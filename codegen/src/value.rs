use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::*;

use crate::{
    types::{Pure, TupleT},
    util::index_tuple,
};

#[derive(Default, Clone)]
pub struct TupleV {
    items: Vec<Expr>,
}

impl TupleV {
    pub fn for_type_mapped<F>(t: &TupleT<Pure>, on: Expr, mapping: F) -> Self
    where
        F: Fn(Expr) -> Expr,
    {
        let items = t
            .iter_indices()
            .map(|index| mapping(index_tuple(&on, index)))
            .collect();

        Self { items }
    }
    #[inline]
    pub fn for_type(t: &TupleT<Pure>, on: Expr) -> Self {
        Self::for_type_mapped(t, on, |it| it)
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    #[inline]
    pub fn push_back(&mut self, value: Expr) {
        self.items.push(value)
    }
    #[inline]
    pub fn push_front(&mut self, value: Expr) {
        self.items.insert(0, value)
    }

    #[inline]
    pub fn pop_back(&mut self) -> Option<Expr> {
        self.items.pop()
    }
    #[inline]
    pub fn pop_front(&mut self) -> Option<Expr> {
        if self.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    pub fn without(&self, index: usize) -> Self {
        let mut result = self.clone();
        result.items.remove(index);
        result
    }

    pub fn reverse(&mut self) {
        self.items.reverse();
    }
}

impl ToTokens for TupleV {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        Expr::Tuple(ExprTuple {
            attrs: vec![],
            paren_token: Default::default(),
            elems: self.items.iter().cloned().collect(),
        })
        .to_tokens(tokens)
    }
}
