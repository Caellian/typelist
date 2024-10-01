#![cfg_attr(feature = "type_defaults", feature(associated_type_defaults))]

pub trait TypeList {
    /// Head or first eleemnt of the type list.
    type Head;
    /// Tailing elements after the first one.
    type Tail: TypeList;
    /// First tuple element.
    type First;
    /// Last tuple element.
    type Last;

    /// Type stored at `INDEX`.
    #[cfg(feature = "type_defaults")]
    type Value<const INDEX: usize> = <Self as HasElement<INDEX>>::Value where Self: HasElement<INDEX>;

    /// Tuple types in reverse order.
    type Reverse: TypeList;

    /// Resulting tuple produced by pushing `T` to the back of this tuple.
    type PushBack<T>;
    /// Resulting tuple produced by pushing `T` to the front of this tuple.
    type PushFront<T>;

    /// Dependent type (tagged enum) capable of representing any value of this
    /// tuple while preserving topological information.
    ///
    /// In other terms, duplicate types will map to separate variants as
    /// collapsing them would both erase their position information and require
    /// `N!` implementations.
    type Variant;
    /// An array of `Variant`s capable of storing all tuple elements.
    type Variants: IntoIterator<Item = Self::Variant>;

    /// Number of elements stored by this tuple.
    const LEN: usize;
    /// Indicates whether this tuple is empty (a unit).
    const IS_EMPTY: bool;

    #[inline]
    fn len(&self) -> usize {
        Self::LEN
    }
    #[inline]
    fn is_empty(&self) -> bool {
        Self::IS_EMPTY
    }

    #[inline]
    fn first(&self) -> &Self::First
    where
        Self: NonEmpty,
    {
        <Self as NonEmpty>::first(self)
    }
    #[inline]
    fn last(&self) -> &Self::Last
    where
        Self: NonEmpty,
    {
        <Self as NonEmpty>::last(self)
    }

    fn push_back<T>(self, value: T) -> Self::PushBack<T>;
    fn push_front<T>(self, value: T) -> Self::PushFront<T>;

    #[inline]
    fn get<const INDEX: usize>(&self) -> &<Self as HasElement<INDEX>>::Value
    where
        Self: HasElement<INDEX>,
    {
        <Self as HasElement<INDEX>>::get(self)
    }
    #[inline]
    fn remove<const INDEX: usize>(
        self,
    ) -> (
        <Self as HasElement<INDEX>>::Value,
        <Self as HasElement<INDEX>>::Other,
    )
    where
        Self: Sized + HasElement<INDEX>,
    {
        <Self as HasElement<INDEX>>::remove(self)
    }

    #[inline]
    fn pop_back(self) -> (Self::Last, Self::LTail)
    where
        Self: Sized + NonEmpty,
    {
        <Self as NonEmpty>::pop_back(self)
    }
    #[inline]
    fn pop_front(self) -> (Self::Head, Self::Tail)
    where
        Self: Sized + NonEmpty,
    {
        <Self as NonEmpty>::pop_front(self)
    }

    fn reverse(self) -> Self::Reverse;

    fn into_variants(self) -> Self::Variants;
    #[inline]
    fn fold(self) -> impl Iterator<Item = Self::Variant>
    where
        Self: Sized,
    {
        self.into_variants().into_iter()
    }

    fn map_variants<F, R>(self, reducer: F) -> impl Iterator<Item = R>
    where
        Self: Sized,
        F: Fn(Self::Variant) -> R,
    {
        self.into_variants().into_iter().map(reducer)
    }
}

mod _properties {
    use super::TypeList;

    /// Denotes that a tuple has `INDEX` element.
    pub trait HasElement<const INDEX: usize> {
        type Value;
        type Other;

        fn get(&self) -> &Self::Value;
        fn remove(self) -> (Self::Value, Self::Other);
    }

    /// Denotes that a tuple has at least one element.
    pub trait NonEmpty: TypeList {
        /// All elements leading up to the last one.
        type LTail: TypeList;

        fn first(&self) -> &Self::First;
        fn last(&self) -> &Self::Last;

        fn pop_back(self) -> (Self::Last, Self::LTail);
        fn pop_front(self) -> (Self::Head, Self::Tail);
    }

    /// A type that can never exist.
    ///
    /// Polyfill for `!` type until 2024 edition; see
    /// [#35121](https://github.com/rust-lang/rust/issues/35121).
    pub enum Never {}
}
use _properties::*;

#[cfg(feature = "generic_impl")]
include!("./gen.rs");
