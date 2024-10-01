use proc_macro2::Span;
use quote::format_ident;
use syn::{punctuated::Punctuated, *};

use crate::{
    types::{Pure, TupleT},
    util::{index_tuple, AsIdentPath, Module, ModuleVisibility},
    Options,
};

pub struct TupleMappingTrait<'a> {
    pub target: &'a TupleT<Pure>,
}

fn generic_types(param: &GenericParam) -> Option<Type> {
    match param {
        GenericParam::Type(t) => Some(Type::Path(TypePath {
            qself: None,
            path: Path::from(t.ident.clone()),
        })),
        _ => None,
    }
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

    pub fn into_items(self) -> Vec<Item> {
        if self.target.is_empty() {
            return vec![];
        }
        let mut result = Vec::new();

        let name = format_ident!("Map{}", self.target.len());
        let (i_generics, o_generics, applicators, io_generics) = {
            let (mut i, mut o, mut a) = {
                let g = self.target.generics(None, None);
                (g.clone(), g.clone(), g)
            };
            for p in i.params.iter_mut() {
                if let GenericParam::Type(ty_p) = p {
                    ty_p.ident = format_ident!("I{}", ty_p.ident);
                }
            }
            for p in o.params.iter_mut() {
                if let GenericParam::Type(ty_p) = p {
                    ty_p.ident = format_ident!("O{}", ty_p.ident);
                }
            }
            for p in a.params.iter_mut() {
                if let GenericParam::Type(ty_p) = p {
                    ty_p.ident = format_ident!("F{}", ty_p.ident);
                }
            }
            let mut io = i.clone();
            for o_item in o.params.iter() {
                io.params.push(o_item.clone());
            }
            for a_item in a.params.iter() {
                io.params.push(a_item.clone());
            }
            (i, o, a, io)
        };

        let result_type_decl: Vec<_> = (0..self.target.len())
            .map(|i| TraitItemType {
                attrs: vec![],
                type_token: Default::default(),
                ident: format_ident!("Result{}", i),
                generics: Default::default(),
                colon_token: None,
                bounds: Default::default(),
                default: None,
                semi_token: Default::default(),
            })
            .collect();

        let self_results: Vec<_> = (0..self.target.len())
            .map(|i| ["Self", format!("Result{}", i).as_str()].to_type())
            .collect();

        let map_declarations: Vec<_> = self
            .mapping_signatures(parse_quote!(self))
            .map(|sig| {
                TraitItem::Fn(TraitItemFn {
                    attrs: vec![],
                    sig,
                    default: None,
                    semi_token: Some(Default::default()),
                })
            })
            .collect();

        let result_union_ty = Type::Tuple(TypeTuple {
            paren_token: Default::default(),
            elems: Punctuated::from_iter(self_results.iter().cloned()),
        });

        let input_tuple = Type::Tuple(TypeTuple {
            paren_token: Default::default(),
            elems: Punctuated::from_iter(i_generics.params.iter().filter_map(generic_types)),
        });

        result.push(Item::Trait(parse_quote! {
            pub trait #name #i_generics {
                #(#result_type_decl)*
                #(#map_declarations)*

                #[allow(clippy::type_complexity)]
                fn apply(self, value: #input_tuple) -> #result_union_ty;
            }
        }));

        let map_fns_invokes: Vec<_> = (0..self.target.len())
            .map(|i| {
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
            })
            .collect();

        let applicator_predicates: Vec<WherePredicate> = applicators
            .params
            .iter()
            .filter_map(generic_types)
            .zip(
                i_generics
                    .params
                    .iter()
                    .filter_map(generic_types)
                    .zip(o_generics.params.iter().filter_map(generic_types)),
            )
            .map(|(f, (i, o))| {
                WherePredicate::Type(PredicateType {
                    lifetimes: None,
                    bounded_ty: f,
                    colon_token: Default::default(),
                    bounds: Punctuated::from_iter([TypeParamBound::Trait(TraitBound {
                        paren_token: None,
                        modifier: TraitBoundModifier::None,
                        lifetimes: None,
                        path: Path {
                            leading_colon: None,
                            segments: Punctuated::from_iter([PathSegment {
                                ident: Ident::new("FnMut", Span::call_site()),
                                arguments: PathArguments::Parenthesized(
                                    ParenthesizedGenericArguments {
                                        paren_token: Default::default(),
                                        inputs: Punctuated::from_iter([i]),
                                        output: ReturnType::Type(Default::default(), Box::new(o)),
                                    },
                                ),
                            }]),
                        },
                    })]),
                })
            })
            .collect();

        let apply_tuple = Type::Tuple(TypeTuple {
            paren_token: Default::default(),
            elems: Punctuated::from_iter(applicators.params.iter().filter_map(generic_types)),
        });

        let result_types: Vec<_> = result_type_decl
            .into_iter()
            .zip(o_generics.params.iter().filter_map(generic_types))
            .map(|(result_t, value_t)| {
                ImplItem::Type(ImplItemType {
                    attrs: vec![],
                    vis: Visibility::Inherited,
                    defaultness: None,
                    type_token: Default::default(),
                    ident: result_t.ident,
                    generics: Default::default(),
                    eq_token: Default::default(),
                    ty: value_t,
                    semi_token: Default::default(),
                })
            })
            .collect();

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

        let mut applicator_tuple: ItemImpl = parse_quote! {
            impl #io_generics #name #i_generics for #apply_tuple {
                #(#result_types)*
                #(#map_impls)*

                fn apply(mut self, value: #input_tuple) -> #result_union_ty {
                    (
                        #(#map_fns_invokes,)*
                    )
                }
            }
        };
        applicator_tuple.generics.where_clause = Some(WhereClause {
            where_token: Default::default(),
            predicates: Punctuated::from_iter(applicator_predicates),
        });
        result.push(Item::Impl(applicator_tuple));

        result
    }
}

pub(crate) const MAPPING_MODULE: Module = Module {
    visibility: ModuleVisibility::Public,
    name: "mapping",
    generate: generate_module,
};
fn generate_module(items: &mut Vec<Item>, options: &Options) {
    for size in 0..=options.size {
        let variant = TupleT::new(size);
        items.append(&mut TupleMappingTrait { target: &variant }.into_items())
    }
}
