use std::marker::PhantomData;
use std::ops::RangeBounds;

use crate::{
    alphabet::{Alphabet, AlphabetGenerator},
    util::AsIdentPath as _,
};
use proc_macro2::Span;
use syn::{punctuated::Punctuated, *};

pub trait TuplePurity {}
pub struct Pure;
impl TuplePurity for Pure {}
pub struct Dirty;
impl TuplePurity for Dirty {}

pub struct TupleT<P: TuplePurity = Dirty> {
    names: Vec<Ident>,
    _phantom: PhantomData<P>,
}

impl TupleT<Pure> {
    pub fn new(length: usize) -> Self {
        Self {
            names: AlphabetGenerator::<Ident>::default().take(length).collect(),
            _phantom: PhantomData,
        }
    }

    pub fn push_index(&mut self, value: usize) {
        let name = Alphabet::DEFAULT.encode(value);
        self.names.push(Ident::new(&name, Span::call_site()));
    }

    pub fn iter_names(&self) -> impl Iterator<Item = String> + '_ {
        self.names.iter().map(|it| it.to_string())
    }

    pub fn iter_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.iter()
            .map(|it| Alphabet::DEFAULT.decode(it.to_string()))
    }

    pub fn reverse(&self) -> Self {
        let mut result = self.clone();
        result.names.reverse();
        result
    }
}

impl<P: TuplePurity> TupleT<P> {
    pub fn empty() -> Self {
        Self {
            names: vec![],
            _phantom: PhantomData,
        }
    }

    pub fn generics(
        &self,
        where_item: Option<Punctuated<TypeParamBound, Token![+]>>,
        where_top_level: Option<WhereClause>,
    ) -> Generics {
        Generics {
            lt_token: Some(Default::default()),
            params: self
                .names
                .iter()
                .cloned()
                .map(|ident| {
                    GenericParam::Type(TypeParam {
                        attrs: vec![],
                        ident,
                        colon_token: where_item.as_ref().map(|_| Default::default()),
                        bounds: where_item.clone().unwrap_or_default(),
                        eq_token: None,
                        default: None,
                    })
                })
                .collect(),
            gt_token: Some(Default::default()),
            where_clause: where_top_level,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.names.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    #[inline]
    pub fn first(&self) -> Option<Ident> {
        self.names.first().cloned()
    }
    #[inline]
    pub fn first_type(&self) -> Type {
        self.names
            .first()
            .cloned()
            .map(|it| {
                Type::Path(TypePath {
                    qself: None,
                    path: Path::from(it),
                })
            })
            .unwrap_or_else(|| ["crate", "Never"].to_type())
    }

    #[inline]
    pub fn last(&self) -> Option<Ident> {
        self.names.last().cloned()
    }
    #[inline]
    pub fn last_type(&self) -> Type {
        self.names
            .last()
            .cloned()
            .map(|it| {
                Type::Path(TypePath {
                    qself: None,
                    path: Path::from(it),
                })
            })
            .unwrap_or_else(|| ["crate", "Never"].to_type())
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<Ident> {
        self.names.get(index).cloned()
    }

    pub fn get_type(&self, index: usize) -> Type {
        self.get(index)
            .map(|it| {
                Type::Path(TypePath {
                    qself: None,
                    path: Path::from(it),
                })
            })
            .unwrap_or_else(|| ["crate", "Never"].to_type())
    }

    pub fn without(&self, index: usize) -> Self {
        let mut result = self.clone();
        result.names.remove(index);
        result
    }

    pub fn iter(&self) -> impl Iterator<Item = Ident> + '_ {
        self.names.iter().cloned()
    }

    pub fn iter_types(&self) -> impl Iterator<Item = Type> + '_ {
        self.iter().map(|it| {
            Type::Path(TypePath {
                qself: None,
                path: Path::from(it),
            })
        })
    }

    pub fn slice<R>(&self, range: R) -> Self
    where
        R: RangeBounds<usize>,
    {
        if self.is_empty() {
            return Self::empty();
        }
        let last = self.len() - 1;

        let min_index = match range.start_bound() {
            std::ops::Bound::Included(from) => *from,
            std::ops::Bound::Excluded(from) => *from + 1,
            std::ops::Bound::Unbounded => 0,
        };
        if min_index > last {
            return Self::empty();
        }

        let max_index = match range.end_bound() {
            std::ops::Bound::Included(to) => *to,
            std::ops::Bound::Excluded(to) => to.saturating_sub(1),
            std::ops::Bound::Unbounded => self.len(),
        };

        if max_index <= min_index {
            return Self::empty();
        }

        Self {
            names: self.names[min_index..max_index].to_vec(),
            _phantom: PhantomData,
        }
    }

    pub fn push_back(&self, ty: Ident) -> TupleT<Dirty> {
        let mut result = self.clone();
        result.names.push(ty);
        unsafe { std::mem::transmute(result) }
    }

    pub fn push_front(&self, ty: Ident) -> TupleT<Dirty> {
        let mut result = self.clone();
        result.names.insert(0, ty);
        unsafe { std::mem::transmute(result) }
    }
}

impl<P: TuplePurity> Default for TupleT<P> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<P: TuplePurity> Clone for TupleT<P> {
    fn clone(&self) -> Self {
        Self {
            names: self.names.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<P: TuplePurity> From<TupleT<P>> for Type {
    fn from(value: TupleT<P>) -> Self {
        Type::Tuple(TypeTuple {
            paren_token: Default::default(),
            elems: value.iter_types().collect(),
        })
    }
}
