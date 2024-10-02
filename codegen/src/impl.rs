use crate::{
    types::{Pure, TupleT},
    value::TupleV, util::{AsIdentPath as _, index_tuple, default},
};

use proc_macro2::Span;
use quote::format_ident;
use syn::{punctuated::Punctuated, *};

pub trait TupleTrait: Sized {
    fn target(&self) -> &TupleT<Pure>;
    fn for_trait(&self) -> Path;
    fn produce_items(&self, items: &mut Vec<ImplItem>);

    fn into_item(self) -> Item {
        let mut items = Vec::with_capacity(8);
        self.produce_items(&mut items);
        Item::Impl(ItemImpl {
            attrs: vec![],
            defaultness: None,
            unsafety: None,
            impl_token: default(),
            generics: self.target().generics(None, None),
            trait_: Some((None, self.for_trait(), default())),
            self_ty: Box::new(self.target().clone().into()),
            brace_token: default(),
            items,
        })
    }
}

pub struct BaseImpl<'a> {
    pub target: &'a TupleT<Pure>,
}

impl<'a> TupleTrait for BaseImpl<'a> {
    fn target(&self) -> &TupleT<Pure> {
        self.target
    }

    fn for_trait(&self) -> Path {
        "TypeList".to_path()
    }

    fn produce_items(&self, items: &mut Vec<ImplItem>) {
        items.push(simple_type_item("Head", self.target.get_type(0), None));
        items.push(simple_type_item("Tail", self.target.slice(1..), None));
        items.push(simple_type_item("First", self.target.get_type(0), None));
        items.push(simple_type_item(
            "Last",
            self.target.get_type(self.target.len().saturating_sub(1)),
            None,
        ));

        items.push(simple_type_item("Reverse", self.target.reverse(), None));

        {
            let param = Ident::new("Pushed", Span::call_site());
            let front = self.target.push_front(param.clone());
            let back = self.target.push_back(param.clone());

            items.push(simple_type_item(
                "PushFront",
                front,
                Some(Generics {
                    lt_token: default(),
                    params: Punctuated::from_iter(
                        [GenericParam::Type(TypeParam {
                            attrs: vec![],
                            ident: param.clone(),
                            colon_token: None,
                            bounds: Punctuated::default(),
                            eq_token: None,
                            default: None,
                        })],
                    ),
                    gt_token: default(),
                    where_clause: None,
                }),
            ));
            items.push(simple_type_item(
                "PushBack",
                back,
                Some(Generics {
                    lt_token: default(),
                    params: Punctuated::from_iter(
                        [GenericParam::Type(TypeParam {
                            attrs: vec![],
                            ident: param,
                            colon_token: None,
                            bounds: Punctuated::default(),
                            eq_token: None,
                            default: None,
                        })],
                    ),
                    gt_token: default(),
                    where_clause: None,
                }),
            ));
        }

        let len_lit_expr = Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Int(LitInt::new(
                self.target.len().to_string().as_str(),
                Span::call_site(),
            )),
        });

        let tuple_name = format!("Tuple{}", self.target.len());
        items.push(simple_type_item(
            "Variant",
            Type::Path(TypePath {
                qself: None,
                path: (
                    &tuple_name,
                    self.target.iter_types().map(GenericArgument::Type),
                ).to_path(),
            }),
            None,
        ));
        items.push(simple_type_item(
            "Variants",
            Type::Array(TypeArray {
                bracket_token: default(),
                elem: parse_quote! {
                    Self::Variant
                },
                semi_token: default(),
                len: len_lit_expr.clone(),
            }),
            None,
        ));

        items.push(simple_const_item(
            "LEN",
            "usize".to_type(),
            len_lit_expr.clone(),
            None,
        ));
        items.push(simple_const_item(
            "IS_EMPTY",
            "bool".to_type(),
            Expr::Lit(ExprLit {
                attrs: vec![],
                lit: Lit::Bool(LitBool::new(self.target.is_empty(), Span::call_site())),
            }),
            None,
        ));

        items.push({
            let mut v = TupleV::for_type(self.target, "self".to_expr());
            v.push_back("value".to_expr());
            parse_quote!(
                fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
                    #v
                }
            )
        });
        items.push({
            let mut v = TupleV::for_type(self.target, "self".to_expr());
            v.push_front("value".to_expr());
            parse_quote!(
                fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
                    #v
                }
            )
        });
        items.push({
            let mut v = TupleV::for_type(self.target, "self".to_expr());
            v.reverse();
            parse_quote!(
                fn reverse(self) -> Self::Reverse {
                    #v
                }
            )
        });
        items.push({
            let variants = self.target.iter_indices().map(|i| {
                let variant_i = format_ident!("Variant{}", i);
                let value_i = index_tuple(&"self".to_expr(), i);
                Expr::Call(ExprCall {
                    attrs: vec![],
                    func: Box::new(Expr::Path(ExprPath {
                        attrs: vec![],
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: Punctuated::from_iter([
                                PathSegment {
                                    ident: Ident::new(&tuple_name, Span::call_site()),
                                    arguments: PathArguments::None,
                                },
                                PathSegment {
                                    ident: variant_i,
                                    arguments: PathArguments::None,
                                },
                            ]),
                        },
                    })),
                    paren_token: default(),
                    args: Punctuated::from_iter([value_i]),
                })
            });
            parse_quote!(
                fn into_variants(self) -> Self::Variants {
                    [
                        #(#variants ,)*
                    ]
                }
            )
        });
    }
}

pub struct HasElementImpl<'a> {
    pub target: &'a TupleT<Pure>,
    pub index: usize,
}

impl<'a> TupleTrait for HasElementImpl<'a> {
    fn target(&self) -> &TupleT<Pure> {
        self.target
    }

    fn for_trait(&self) -> Path {
        (
            "HasElement",
            [GenericArgument::Const(Expr::Lit(ExprLit {
                attrs: vec![],
                lit: Lit::Int(LitInt::new(
                    self.index.to_string().as_str(),
                    Span::call_site(),
                )),
            }))],
        ).to_path()
    }

    fn produce_items(&self, items: &mut Vec<ImplItem>) {
        items.push(simple_type_item(
            "Value",
            self.target.get_type(self.index),
            None,
        ));
        items.push(simple_type_item(
            "Other",
            self.target.without(self.index),
            None,
        ));

        let indexed = index_tuple(&"self".to_expr(), self.index);
        items.push(parse_quote! {
            fn get(&self) -> &Self::Value {
                &#indexed
            }
        });

        let other = TupleV::for_type(self.target, "self".to_expr()).without(self.index);
        items.push(parse_quote! {
            fn remove(self) -> (Self::Value, Self::Other) {
                (#indexed, #other)
            }
        });
    }
}

pub struct NonEmptyImpl<'a> {
    pub target: &'a TupleT<Pure>,
}

impl<'a> TupleTrait for NonEmptyImpl<'a> {
    fn target(&self) -> &TupleT<Pure> {
        self.target
    }

    fn for_trait(&self) -> Path {
        "NonEmpty".to_path()
    }

    fn produce_items(&self, items: &mut Vec<ImplItem>) {
        items.push(simple_type_item(
            "LTail",
            self.target.slice(..self.target.len()),
            None,
        ));

        items.push({
            let result = index_tuple(&"self".to_expr(), 0);
            parse_quote!(
                fn first(&self) -> &Self::First {
                    &#result
                }
            )
        });
        items.push({
            let result = index_tuple(&"self".to_expr(), self.target.len() - 1);
            parse_quote!(
                fn last(&self) -> &Self::Last {
                    &#result
                }
            )
        });
        items.push({
            let mut v = TupleV::for_type(self.target, "self".to_expr());
            let result = v.pop_back();
            parse_quote!(
                fn pop_back(self) -> (Self::Last, Self::LTail) {
                    (#result, #v)
                }
            )
        });
        items.push({
            let mut v = TupleV::for_type(self.target, "self".to_expr());
            let result = v.pop_front();
            parse_quote!(
                fn pop_front(self) -> (Self::Head, Self::Tail) {
                    (#result, #v)
                }
            )
        });
    }
}

fn simple_type_item(
    name: impl AsRef<str>,
    ty: impl Into<Type>,
    generics: Option<Generics>,
) -> ImplItem {
    ImplItem::Type(ImplItemType {
        attrs: vec![],
        vis: Visibility::Inherited,
        defaultness: None,
        type_token: default(),
        ident: Ident::new(name.as_ref(), Span::call_site()),
        generics: generics.unwrap_or_default(),
        eq_token: default(),
        ty: ty.into(),
        semi_token: default(),
    })
}

fn simple_const_item(
    name: impl AsRef<str>,
    ty: impl Into<Type>,
    value: impl Into<Expr>,
    generics: Option<Generics>,
) -> ImplItem {
    ImplItem::Const(ImplItemConst {
        attrs: vec![],
        vis: Visibility::Inherited,
        defaultness: None,
        const_token: default(),
        ident: Ident::new(name.as_ref(), Span::call_site()),
        generics: generics.unwrap_or_default(),
        colon_token: default(),
        ty: ty.into(),
        eq_token: default(),
        expr: value.into(),
        semi_token: default(),
    })
}
