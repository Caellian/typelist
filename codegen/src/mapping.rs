use proc_macro2::Span;
use quote::format_ident;
use syn::{punctuated::Punctuated, *};

use crate::{
    types::{Pure, TupleT},
    util::{
        fnmut_bound, ident_ty, index_tuple, AsIdentPath, CollectGenerics, Module, ModuleVisibility,
    },
    Options,
};

pub struct TupleMappingTrait<'a> {
    pub target: &'a TupleT<Pure>,
}

impl<'a> TupleMappingTrait<'a> {
    fn mapping_signatures(&self, receiver: Receiver) -> impl Iterator<Item = Signature> + '_ {
        self.target.iter_names().enumerate().map(move |(i, t)| {
            let fn_name = format_ident!("map_{}", i);
            let input_ty = format!("I{}", t).to_type();
            let result = ["Self", format!("Result{}", i).as_str()].to_type();
            parse_quote! {
               fn #fn_name (#receiver, value: #input_ty) -> #result
            }
        })
    }

    fn name_ident(&self) -> Ident {
        format_ident!("Map{}", self.target.len())
    }
    fn name_path(&self, arguments: impl IntoIterator<Item = GenericArgument>) -> Path {
        let mut path = Path::from(self.name_ident());
        let mut args = arguments.into_iter();
        if let Some(first_arg) = args.next() {
            let segment = path.segments.last_mut().unwrap();
            segment.arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Default::default(),
                args: Punctuated::from_iter(std::iter::once(first_arg).chain(args)),
                gt_token: Default::default(),
            })
        }
        path
    }
    fn name_path_with_inputs(&self) -> Path {
        self.name_path(self.input_idents().map(ident_ty).map(GenericArgument::Type))
    }
    fn input_idents(&self) -> impl Iterator<Item = Ident> + '_ {
        self.target
            .iter_names()
            .map(|name| format_ident!("I{}", name))
    }
    fn output_idents(&self) -> impl Iterator<Item = Ident> + '_ {
        self.target
            .iter_names()
            .map(|name| format_ident!("O{}", name))
    }
    fn fn_idents(&self) -> impl Iterator<Item = Ident> + '_ {
        self.target
            .iter_names()
            .map(|name| format_ident!("F{}", name))
    }

    fn result_names(&self) -> impl Iterator<Item = Ident> + '_ {
        (0..self.target.len()).map(|i| format_ident!("Result{}", i))
    }
    fn result_tuple(&self) -> Type {
        Type::Tuple(TypeTuple {
            paren_token: Default::default(),
            elems: Punctuated::from_iter(
                self.result_names()
                    .map(|result| ["Self", result.to_string().as_str()].to_type()),
            ),
        })
    }
    fn apply_all_sig(&self, mut_self: bool) -> Signature {
        Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: Ident::new("apply", Span::call_site()),
            generics: Default::default(),
            paren_token: Default::default(),
            inputs: Punctuated::from_iter([
                if mut_self {
                    parse_quote!(mut self)
                } else {
                    parse_quote!(self)
                },
                FnArg::Typed(PatType {
                    attrs: vec![],
                    pat: Box::new(Pat::Ident(PatIdent {
                        attrs: vec![],
                        by_ref: None,
                        mutability: None,
                        ident: Ident::new("value", Span::call_site()),
                        subpat: None,
                    })),
                    colon_token: Default::default(),
                    ty: Box::new(Type::Tuple(TypeTuple {
                        paren_token: Default::default(),
                        elems: Punctuated::from_iter(self.input_idents().map(ident_ty)),
                    })),
                }),
            ]),
            variadic: None,
            output: ReturnType::Type(Default::default(), Box::new(self.result_tuple())),
        }
    }

    fn declare_base_trait(&self) -> Item {
        let result_type_decl = self.result_names().map(|ident| {
            TraitItem::Type(TraitItemType {
                attrs: vec![],
                type_token: Default::default(),
                ident,
                generics: Default::default(),
                colon_token: None,
                bounds: Default::default(),
                default: None,
                semi_token: Default::default(),
            })
        });

        let map_declarations = self.mapping_signatures(parse_quote!(self)).map(|sig| {
            TraitItem::Fn(TraitItemFn {
                attrs: vec![],
                sig,
                default: None,
                semi_token: Some(Default::default()),
            })
        });

        Item::Trait(ItemTrait {
            attrs: vec![],
            vis: Visibility::Public(Default::default()),
            unsafety: None,
            auto_token: None,
            restriction: None,
            trait_token: Default::default(),
            ident: self.name_ident(),
            generics: Generics {
                lt_token: Default::default(),
                params: Punctuated::from_iter(self.input_idents().map(|ident| {
                    GenericParam::Type(TypeParam {
                        attrs: vec![],
                        ident,
                        colon_token: None,
                        bounds: Default::default(),
                        eq_token: None,
                        default: None,
                    })
                })),
                gt_token: Default::default(),
                where_clause: None,
            },
            colon_token: None,
            supertraits: Default::default(),
            brace_token: Default::default(),
            items: Vec::from_iter(
                result_type_decl
                    .chain(map_declarations)
                    .chain([TraitItem::Fn(TraitItemFn {
                        attrs: vec![parse_quote!(#[allow(clippy::type_complexity)])],
                        sig: self.apply_all_sig(false),
                        default: None,
                        semi_token: Default::default(),
                    })]),
            ),
        })
    }

    fn impl_base_for_fn_tuple(&self) -> Item {
        let generics = self
            .input_idents()
            .chain(self.output_idents())
            .chain(self.fn_idents())
            .collect_generics(Some(WhereClause {
                where_token: Default::default(),
                predicates: Punctuated::from_iter(
                    self.fn_idents()
                        .map(ident_ty)
                        .zip(
                            self.input_idents()
                                .map(ident_ty)
                                .zip(self.output_idents().map(ident_ty)),
                        )
                        .map(|(f, (i, o))| {
                            WherePredicate::Type(PredicateType {
                                lifetimes: None,
                                bounded_ty: f,
                                colon_token: Default::default(),
                                bounds: Punctuated::from_iter([TypeParamBound::Trait(
                                    fnmut_bound([i], o),
                                )]),
                            })
                        }),
                ),
            }));

        let result_types = self
            .result_names()
            .zip(self.output_idents().map(ident_ty))
            .map(|(ident, ty)| {
                ImplItem::Type(ImplItemType {
                    attrs: vec![],
                    vis: Visibility::Inherited,
                    defaultness: None,
                    type_token: Default::default(),
                    ident,
                    generics: Default::default(),
                    eq_token: Default::default(),
                    ty,
                    semi_token: Default::default(),
                })
            });

        let map_impls = self
            .mapping_signatures(parse_quote!(mut self))
            .enumerate()
            .map(|(i, sig)| {
                let f = index_tuple(&"self".to_expr(), i);
                let f = Expr::Paren(ExprParen {
                    attrs: vec![],
                    paren_token: Default::default(),
                    expr: Box::new(f),
                });
                let call = Expr::Call(ExprCall {
                    attrs: vec![],
                    func: Box::new(f),
                    paren_token: Default::default(),
                    args: Punctuated::from_iter(["value".to_expr()]),
                });
                ImplItem::Fn(ImplItemFn {
                    attrs: vec![],
                    vis: Visibility::Inherited,
                    defaultness: None,
                    sig,
                    block: Block {
                        brace_token: Default::default(),
                        stmts: vec![Stmt::Expr(call, None)],
                    },
                })
            });

        let apply_fn = ImplItem::Fn(ImplItemFn {
            attrs: vec![],
            vis: Visibility::Inherited,
            defaultness: None,
            sig: self.apply_all_sig(true),
            block: Block {
                brace_token: Default::default(),
                stmts: vec![Stmt::Expr(
                    Expr::Tuple(ExprTuple {
                        attrs: vec![],
                        paren_token: Default::default(),
                        elems: Punctuated::from_iter((0..self.target.len()).map(|i| {
                            let f = index_tuple(&"self".to_expr(), i);
                            let f = Expr::Paren(ExprParen {
                                attrs: vec![],
                                paren_token: Default::default(),
                                expr: Box::new(f),
                            });
                            Expr::Call(ExprCall {
                                attrs: vec![],
                                func: Box::new(f),
                                paren_token: Default::default(),
                                args: Punctuated::from_iter([index_tuple(&"value".to_expr(), i)]),
                            })
                        })),
                    }),
                    None,
                )],
            },
        });

        Item::Impl(ItemImpl {
            attrs: vec![],
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics,
            trait_: Some((None, self.name_path_with_inputs(), Default::default())),
            self_ty: Box::new(Type::Tuple(TypeTuple {
                paren_token: Default::default(),
                elems: Punctuated::from_iter(self.fn_idents().map(ident_ty)),
            })),
            brace_token: Default::default(),
            items: Vec::from_iter(result_types.chain(map_impls).chain([apply_fn])),
        })
    }

    #[allow(unused)]
    fn todo() {
        /*
        trait HasMap1<IA, FA> {
            type Result: TypeList;
            fn map(self, m: (FA,)) -> Self::Result
            where
                Self::Result: HasElement<0>,
                (FA,): Map1<IA, Result0 = <Self::Result as HasElement<0>>::Value>;
        }
        impl<IA, OA, FA> HasMap1<IA, FA> for (IA,)
        where
            FA: FnOnce(IA) -> OA,
        {
            type Result = (OA,);
            fn map(self, m: (FA,)) -> Self::Result
            where
                Self::Result: HasElement<0>,
                (FA,): Map1<IA, Result0 = OA> {
                m.apply(self)
            }
        }
        */
    }
}

pub(crate) const MAPPING_MODULE: Module = Module {
    visibility: ModuleVisibility::Public,
    name: "mapping",
    generate: generate_module,
};
fn generate_module(items: &mut Vec<Item>, options: &Options) {
    for size in 1..=options.size {
        let variant = TupleT::new(size);
        let mapping = TupleMappingTrait { target: &variant };
        items.push(mapping.declare_base_trait());
        items.push(mapping.impl_base_for_fn_tuple());
    }
}
