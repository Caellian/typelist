use proc_macro2::Span;
use quote::format_ident;
use syn::{punctuated::Punctuated, *};

use crate::{
    constraints::has_element,
    types::{Pure, TupleT},
    util::{
        default, fnmut_bound, ident_ty, index_tuple, AsIdentPath, CollectGenerics, Module,
        ModuleVisibility,
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
                lt_token: default(),
                args: Punctuated::from_iter(std::iter::once(first_arg).chain(args)),
                gt_token: default(),
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
    fn fn_predicates(&self) -> impl Iterator<Item = WherePredicate> + '_ {
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
                    colon_token: default(),
                    bounds: Punctuated::from_iter([TypeParamBound::Trait(fnmut_bound([i], o))]),
                })
            })
    }

    fn result_names(&self) -> impl Iterator<Item = Ident> + '_ {
        (0..self.target.len()).map(|i| format_ident!("Result{}", i))
    }
    fn result_tuple(&self) -> Type {
        Type::Tuple(TypeTuple {
            paren_token: default(),
            elems: Punctuated::from_iter(
                self.result_names()
                    .map(|result| ["Self", result.to_string().as_str()].to_type()),
            ),
        })
    }
    fn apply_all_signature(&self, mut_self: bool) -> Signature {
        Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: default(),
            ident: Ident::new("apply", Span::call_site()),
            generics: default(),
            paren_token: default(),
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
                    colon_token: default(),
                    ty: Box::new(Type::Tuple(TypeTuple {
                        paren_token: default(),
                        elems: Punctuated::from_iter(self.input_idents().map(ident_ty)),
                    })),
                }),
            ]),
            variadic: None,
            output: ReturnType::Type(default(), Box::new(self.result_tuple())),
        }
    }

    fn declare_base_trait(&self) -> Item {
        let result_type_decl = self.result_names().map(|ident| {
            TraitItem::Type(TraitItemType {
                attrs: vec![],
                type_token: default(),
                ident,
                generics: default(),
                colon_token: None,
                bounds: default(),
                default: None,
                semi_token: default(),
            })
        });

        Item::Trait(simple_trait(
            self.name_ident(),
            Generics {
                lt_token: default(),
                params: Punctuated::from_iter(self.input_idents().map(|ident| {
                    GenericParam::Type(TypeParam {
                        attrs: vec![],
                        ident,
                        colon_token: None,
                        bounds: default(),
                        eq_token: None,
                        default: None,
                    })
                })),
                gt_token: default(),
                where_clause: None,
            },
            result_type_decl
                .chain([TraitItem::Fn(TraitItemFn {
                    attrs: vec![parse_quote!(#[allow(clippy::type_complexity)])],
                    sig: self.apply_all_signature(false),
                    default: None,
                    semi_token: default(),
                })]),
        ))
    }

    fn impl_base_for_fn_tuple(&self) -> Item {
        let generics = self
            .input_idents()
            .chain(self.output_idents())
            .chain(self.fn_idents())
            .collect_generics(Some(WhereClause {
                where_token: default(),
                predicates: Punctuated::from_iter(self.fn_predicates()),
            }));

        let result_types = self
            .result_names()
            .zip(self.output_idents().map(ident_ty))
            .map(|(ident, ty)| {
                ImplItem::Type(ImplItemType {
                    attrs: vec![],
                    vis: Visibility::Inherited,
                    defaultness: None,
                    type_token: default(),
                    ident,
                    generics: default(),
                    eq_token: default(),
                    ty,
                    semi_token: default(),
                })
            });

        let apply_fn = ImplItem::Fn(ImplItemFn {
            attrs: vec![],
            vis: Visibility::Inherited,
            defaultness: None,
            sig: self.apply_all_signature(true),
            block: Block {
                brace_token: default(),
                stmts: vec![Stmt::Expr(
                    Expr::Tuple(ExprTuple {
                        attrs: vec![],
                        paren_token: default(),
                        elems: Punctuated::from_iter((0..self.target.len()).map(|i| {
                            let f = index_tuple(&"self".to_expr(), i);
                            let f = Expr::Paren(ExprParen {
                                attrs: vec![],
                                paren_token: default(),
                                expr: Box::new(f),
                            });
                            Expr::Call(ExprCall {
                                attrs: vec![],
                                func: Box::new(f),
                                paren_token: default(),
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
            impl_token: default(),
            generics,
            trait_: Some((None, self.name_path_with_inputs(), default())),
            self_ty: Box::new(Type::Tuple(TypeTuple {
                paren_token: default(),
                elems: Punctuated::from_iter(self.fn_idents().map(ident_ty)),
            })),
            brace_token: default(),
            items: Vec::from_iter(result_types.chain([apply_fn])),
        })
    }

    fn map_fn_sig<const DISCRETE: bool>(&self) -> Signature {
        let fn_tuple = Type::Tuple(TypeTuple {
            paren_token: default(),
            elems: Punctuated::from_iter(self.fn_idents().map(ident_ty)),
        });
        let result_ty = |index: usize| {
            if DISCRETE {
                self.output_idents().nth(index).map(ident_ty).unwrap()
            } else {
                Type::Path(TypePath {
                    qself: Some(QSelf {
                        lt_token: default(),
                        ty: Box::new(["Self", "Result"].to_type()),
                        position: 1,
                        as_token: default(),
                        gt_token: default(),
                    }),
                    path: Path {
                        leading_colon: None,
                        segments: Punctuated::from_iter(
                            has_element(index).segments.into_iter().chain([
                                PathSegment {
                                    ident: Ident::new("Value", Span::call_site()),
                                    arguments: PathArguments::None,
                                },
                            ]),
                        ),
                    },
                })
            }
        };

        let map_impl_target = |index: usize| Path {
            leading_colon: None,
            segments: Punctuated::from_iter([PathSegment {
                ident: format_ident!("Map{}", self.target.len()),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: default(),
                    args: Punctuated::from_iter(
                        self.input_idents()
                            .map(ident_ty)
                            .map(GenericArgument::Type)
                            .chain([GenericArgument::AssocType(AssocType {
                                ident: format_ident!("Result{}", index),
                                generics: None,
                                eq_token: default(),
                                ty: result_ty(index),
                            })]),
                    ),
                    gt_token: default(),
                }),
            }]),
        };

        let bounds = (0..self.target.len()).flat_map(|i| {
            [
                WherePredicate::Type(PredicateType {
                    lifetimes: None,
                    bounded_ty: ["Self", "Result"].to_type(),
                    colon_token: default(),
                    bounds: Punctuated::from_iter([TypeParamBound::Trait(TraitBound {
                        paren_token: None,
                        modifier: TraitBoundModifier::None,
                        lifetimes: None,
                        path: has_element(i),
                    })]),
                }),
                WherePredicate::Type(PredicateType {
                    lifetimes: None,
                    bounded_ty: fn_tuple.clone(),
                    colon_token: default(),
                    bounds: Punctuated::from_iter([TypeParamBound::Trait(TraitBound {
                        paren_token: None,
                        modifier: TraitBoundModifier::None,
                        lifetimes: None,
                        path: map_impl_target(i),
                    })]),
                }),
            ]
        });

        let generics = Generics {
            lt_token: default(),
            params: default(),
            gt_token: default(),
            where_clause: Some(WhereClause {
                where_token: default(),
                predicates: Punctuated::from_iter(bounds),
            }),
        };

        Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: default(),
            ident: Ident::new("map", Span::call_site()),
            generics,
            paren_token: default(),
            inputs: Punctuated::from_iter([
                parse_quote!(self),
                simple_fn_arg(
                    "mapping",
                    Type::Tuple(TypeTuple {
                        paren_token: default(),
                        elems: Punctuated::from_iter(self.fn_idents().map(ident_ty)),
                    }),
                ),
            ]),
            variadic: None,
            output: ReturnType::Type(default(), Box::new(["Self", "Result"].to_type())),
        }
    }

    fn declare_accessor_trait(&self) -> Item {
        Item::Trait(simple_trait(
            format_ident!("HasMap{}", self.target.len()),
            Generics {
                lt_token: default(),
                params: Punctuated::from_iter(self.input_idents().chain(self.fn_idents()).map(
                    |ident| {
                        GenericParam::Type(TypeParam {
                            attrs: vec![],
                            ident,
                            colon_token: None,
                            bounds: default(),
                            eq_token: None,
                            default: None,
                        })
                    },
                )),
                gt_token: default(),
                where_clause: None,
            },
            [
                TraitItem::Type(TraitItemType {
                    attrs: vec![],
                    type_token: default(),
                    ident: Ident::new("Result", Span::call_site()),
                    generics: default(),
                    colon_token: None,
                    bounds: Punctuated::from_iter((0..self.target.len()).map(|i| {
                        TypeParamBound::Trait(TraitBound {
                            paren_token: None,
                            modifier: TraitBoundModifier::None,
                            lifetimes: None,
                            path: has_element(i),
                        })
                    })),
                    default: None,
                    semi_token: default(),
                }),
                TraitItem::Fn(TraitItemFn {
                    attrs: vec![],
                    sig: self.map_fn_sig::<false>(),
                    default: None,
                    semi_token: default(),
                }),
            ],
        ))
    }

    #[allow(unused)]
    fn impl_accessor_for_tuples(&self) -> Item {
        /*
            type Result = (OA,);
            fn map(self, m: (FA,)) -> Self::Result
            where
                Self::Result: HasElement<0>,
                (FA,): Map1<IA, Result0 = OA> {
                m.apply(self)
            }
        */
        let mut items = Vec::with_capacity(2);
        items.push(ImplItem::Type(ImplItemType {
            attrs: vec![],
            vis: Visibility::Inherited,
            defaultness: None,
            type_token: default(),
            ident: Ident::new("Result", Span::call_site()),
            generics: default(),
            eq_token: default(),
            ty: Type::Tuple(TypeTuple {
                paren_token: default(),
                elems: Punctuated::from_iter(self.output_idents().map(ident_ty)),
            }),
            semi_token: default(),
        }));
        items.push(ImplItem::Fn(ImplItemFn {
            attrs: vec![],
            vis: Visibility::Inherited,
            defaultness: None,
            sig: self.map_fn_sig::<true>(),
            block: Block {
                brace_token: default(),
                stmts: vec![Stmt::Expr(
                    Expr::MethodCall(ExprMethodCall {
                        attrs: vec![],
                        receiver: Box::new("mapping".to_expr()),
                        dot_token: default(),
                        method: Ident::new("apply", Span::call_site()),
                        turbofish: None,
                        paren_token: default(),
                        args: Punctuated::from_iter(["self".to_expr()]),
                    }),
                    None,
                )],
            },
        }));

        Item::Impl(ItemImpl {
            attrs: vec![],
            defaultness: None,
            unsafety: None,
            impl_token: default(),
            generics: self
                .input_idents()
                .chain(self.output_idents())
                .chain(self.fn_idents())
                .collect_generics(Some(WhereClause {
                    where_token: default(),
                    predicates: Punctuated::from_iter(self.fn_predicates()),
                })),
            trait_: Some((
                None,
                Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([PathSegment {
                        ident: format_ident!("HasMap{}", self.target.len()),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: default(),
                            args: Punctuated::from_iter(
                                self.input_idents()
                                    .chain(self.fn_idents())
                                    .map(ident_ty)
                                    .map(GenericArgument::Type),
                            ),
                            gt_token: default(),
                        }),
                    }]),
                },
                default(),
            )),
            self_ty: Box::new(Type::Tuple(TypeTuple {
                paren_token: default(),
                elems: Punctuated::from_iter(self.input_idents().map(ident_ty)),
            })),
            brace_token: default(),
            items,
        })
    }
}

fn simple_trait(
    ident: Ident,
    generics: Generics,
    items: impl IntoIterator<Item = TraitItem>,
) -> ItemTrait {
    ItemTrait {
        attrs: vec![],
        vis: Visibility::Public(default()),
        unsafety: None,
        auto_token: None,
        restriction: None,
        trait_token: default(),
        ident,
        generics,
        colon_token: None,
        supertraits: default(),
        brace_token: default(),
        items: Vec::from_iter(items),
    }
}
fn simple_fn_arg(name: impl AsRef<str>, ty: Type) -> FnArg {
    FnArg::Typed(PatType {
        attrs: vec![],
        pat: Box::new(Pat::Ident(PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: Ident::new(name.as_ref(), Span::call_site()),
            subpat: None,
        })),
        colon_token: default(),
        ty: Box::new(ty),
    })
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
        items.push(mapping.declare_accessor_trait());
        items.push(mapping.impl_accessor_for_tuples());
    }
}
