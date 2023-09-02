#![no_std]
//! `copy_range` provides three structs: [`CopyRange`], [`CopyRangeFrom`], and
//! [`CopyRangeInclusive`].
//!
//! They are similar to `core::ops`'s [`Range`], [`RangeFrom`], and
//! [`RangeInclusive`], respectively, except they implement `Copy` if their
//! element type implements `Copy`, and they implement `IntoIterator` instead of
//! `Iterator`.
//!
//! They are freely convertible to and from their `core::ops` counterparts (with
//! a [note](CopyRangeInclusive::from_std) about `RangeInclusive`), and they
//! implement most of the same (non-iterator-related) traits, notably
//! [`RangeBounds`].
//!
//! Ranges of `usize` are additionally usable as the [`Index`] parameter for
//! [arrays](prim@array), [slices](prim@slice), [string slices](prim@str) and
//! (with the `"alloc"` feature enabled) [`Vec`][alloc::vec::Vec] and
//! [`String`][alloc::string::String].

// Much of this crate is adapted from the stdlib, specifically
// `library/core/src/ops/range.rs`.

#[cfg(feature = "alloc")]
extern crate alloc;

use core::ops::{
    Bound, Index, IndexMut, Range, RangeBounds, RangeFrom, RangeFull,
    RangeInclusive, RangeTo, RangeToInclusive,
};

/// A (half-open) range bounded inclusively below and exclusively above. See
/// [`core::ops::Range`].
///
/// Unlike `Range`, this struct is `Copy` if `Idx` is `Copy`, and implements
/// `IntoIterator` instead of `Interator`.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct CopyRange<Idx> {
    pub start: Idx,
    pub end: Idx,
}

impl<Idx: core::fmt::Debug> core::fmt::Debug for CopyRange<Idx> {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.start.fmt(fmt)?;
        write!(fmt, "..")?;
        self.end.fmt(fmt)?;
        Ok(())
    }
}

impl<Idx> CopyRange<Idx> {
    /// Returns `true` if `item` is contained in the range.
    ///
    /// See [`Range::contains`][core::ops::Range::contains].
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    {
        <Self as RangeBounds<Idx>>::contains(self, item)
    }

    /// Convert a [`Range`] into a `CopyRange`.
    pub fn from_std(range: Range<Idx>) -> Self {
        range.into()
    }

    /// Convert a `CopyRange` into a [`Range`].
    pub fn into_std(self) -> Range<Idx> {
        self.into()
    }

    /// Returns `true` if the range contains no items.
    ///
    /// See [`Range::contains`][core::ops::Range::contains].
    pub fn is_empty(&self) -> bool
    where
        Idx: PartialOrd,
    {
        self.start >= self.end
    }
}

/// Convert a [`Range`] into a `CopyRange`.
impl<Idx> From<Range<Idx>> for CopyRange<Idx> {
    fn from(Range { start, end }: Range<Idx>) -> Self {
        Self { start, end }
    }
}

/// Convert a `CopyRange` into a [`Range`].
impl<Idx> From<CopyRange<Idx>> for Range<Idx> {
    fn from(value: CopyRange<Idx>) -> Self {
        value.start..value.end
    }
}

/// A range only bounded inclusively below. See [`core::ops::RangeFrom`].
///
/// Unlike `RangeFrom`, this struct is `Copy` if `Idx` is `Copy`, and implements
/// `IntoIterator` instead of `Interator`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CopyRangeFrom<Idx> {
    pub start: Idx,
}

impl<Idx: core::fmt::Debug> core::fmt::Debug for CopyRangeFrom<Idx> {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.start.fmt(fmt)?;
        write!(fmt, "..")?;
        Ok(())
    }
}

impl<Idx> CopyRangeFrom<Idx> {
    /// Returns `true` if `item` is contained in the range.
    ///
    /// See [`RangeFrom::contains`][core::ops::RangeFrom::contains].
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    {
        <Self as RangeBounds<Idx>>::contains(self, item)
    }

    /// Convert a [`RangeFrom`] into a `CopyRangeFrom`.
    pub fn from_std(range: RangeFrom<Idx>) -> Self {
        range.into()
    }

    /// Convert a `CopyRangeFrom` into a [`RangeFrom`].
    pub fn into_std(self) -> RangeFrom<Idx> {
        self.into()
    }
}

/// Convert a [`RangeFrom`] into a `CopyRangeFrom`.
impl<Idx> From<RangeFrom<Idx>> for CopyRangeFrom<Idx> {
    fn from(RangeFrom { start }: RangeFrom<Idx>) -> Self {
        Self { start }
    }
}

/// Convert a `CopyRangeFrom` into a [`RangeFrom`].
impl<Idx> From<CopyRangeFrom<Idx>> for RangeFrom<Idx> {
    fn from(value: CopyRangeFrom<Idx>) -> Self {
        value.start..
    }
}

/// A range bounded inclusively above and below. See
/// [`core::ops::RangeInclusive`].
///
/// Unlike `RangeInclusive`, this struct is `Copy` if `Idx` is `Copy`, and
/// implements `IntoIterator` instead of `Interator`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CopyRangeInclusive<Idx> {
    pub start: Idx,
    pub end: Idx,
}

impl<Idx: core::fmt::Debug> core::fmt::Debug for CopyRangeInclusive<Idx> {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.start.fmt(fmt)?;
        write!(fmt, "..=")?;
        self.end.fmt(fmt)?;
        Ok(())
    }
}

impl<Idx> CopyRangeInclusive<Idx> {
    /// Returns `true` if `item` is contained in the range.
    ///
    /// See [`RangeInclusive::contains`][core::ops::RangeInclusive::contains].
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    {
        <Self as RangeBounds<Idx>>::contains(self, item)
    }

    /// Returns `true` if the range contains no items.
    ///
    /// See [`RangeInclusive::contains`][core::ops::RangeInclusive::contains].
    pub fn is_empty(&self) -> bool
    where
        Idx: PartialOrd,
    {
        !(self.start <= self.end)
    }

    /// Convert a [`RangeInclusive`] into a `CopyRangeInclusive`.
    ///
    /// Note: the value returned by this conversion is unspecified after the
    /// `RangeInclusive` has been iterated to exhaustion.
    pub fn from_std(range: RangeInclusive<Idx>) -> Self {
        range.into()
    }

    /// Convert a `CopyRangeInclusive` into a [`RangeInclusive`].
    pub fn into_std(self) -> RangeInclusive<Idx> {
        self.into()
    }
}

/// Convert a [`RangeInclusive`] into a `CopyRangeInclusive`.
///
/// Note: the value returned by this conversion is unspecified after the
/// `RangeInclusive` has been iterated to exhaustion.
impl<Idx> From<RangeInclusive<Idx>> for CopyRangeInclusive<Idx> {
    fn from(range: RangeInclusive<Idx>) -> Self {
        let (start, end) = range.into_inner();
        Self { start, end }
    }
}

/// Convert a `CopyRangeInclusive` into a [`RangeInclusive`].
impl<Idx> From<CopyRangeInclusive<Idx>> for RangeInclusive<Idx> {
    fn from(value: CopyRangeInclusive<Idx>) -> Self {
        value.start..=value.end
    }
}

impl<Idx> RangeBounds<Idx> for CopyRange<Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Excluded(&self.end)
    }
}

impl<Idx> RangeBounds<Idx> for CopyRange<&Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Included(self.start)
    }

    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Excluded(self.end)
    }
}

impl<Idx> IntoIterator for CopyRange<Idx>
where
    Range<Idx>: Iterator<Item = Idx>,
{
    type Item = Idx;

    type IntoIter = Range<Idx>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..self.end
    }
}

impl<Idx> RangeBounds<Idx> for CopyRangeFrom<Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Unbounded
    }
}

impl<Idx> RangeBounds<Idx> for CopyRangeFrom<&Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Included(self.start)
    }

    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Unbounded
    }
}

impl<Idx> IntoIterator for CopyRangeFrom<Idx>
where
    RangeFrom<Idx>: Iterator<Item = Idx>,
{
    type Item = Idx;

    type IntoIter = RangeFrom<Idx>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..
    }
}

impl<Idx> RangeBounds<Idx> for CopyRangeInclusive<Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Included(&self.end)
    }
}

impl<Idx> RangeBounds<Idx> for CopyRangeInclusive<&Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        Bound::Included(self.start)
    }

    fn end_bound(&self) -> Bound<&Idx> {
        Bound::Included(self.end)
    }
}

impl<Idx> IntoIterator for CopyRangeInclusive<Idx>
where
    RangeInclusive<Idx>: Iterator<Item = Idx>,
{
    type Item = Idx;

    type IntoIter = RangeInclusive<Idx>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

/// [`core::ops::RangeFull`] is already `Copy`, so we just reexport it.
pub type CopyRangeFull = RangeFull;
/// [`core::ops::RangeTo`] is already `Copy` if `Idx` is `Copy`, so we just
/// reexport it.
pub type CopyRangeTo<Idx> = RangeTo<Idx>;
/// [`core::ops::RangeToInclusive`] is already `Copy` if `Idx` is `Copy`, so we
/// just reexport it.
pub type CopyRangeToInclusive<Idx> = RangeToInclusive<Idx>;

macro_rules! impl_index {
    ([$($generics:tt)*], $ty:ty) => {
        impl<$($generics)*> Index<CopyRange<usize>> for $ty
        where
            $ty: Index<Range<usize>>,
        {
            type Output = <$ty as Index<Range<usize>>>::Output;

            fn index(&self, index: CopyRange<usize>) -> &Self::Output {
                self.index(index.into_std())
            }
        }
        impl<$($generics)*> IndexMut<CopyRange<usize>> for $ty
        where
            $ty: IndexMut<Range<usize>>,
        {
            fn index_mut(&mut self, index: CopyRange<usize>) -> &mut Self::Output {
                self.index_mut(index.into_std())
            }
        }
        impl<$($generics)*> Index<CopyRangeFrom<usize>> for $ty
        where
            $ty: Index<RangeFrom<usize>>,
        {
            type Output = <$ty as Index<RangeFrom<usize>>>::Output;

            fn index(&self, index: CopyRangeFrom<usize>) -> &Self::Output {
                self.index(index.into_std())
            }
        }
        impl<$($generics)*> IndexMut<CopyRangeFrom<usize>> for $ty
        where
            $ty: IndexMut<RangeFrom<usize>>,
        {
            fn index_mut(&mut self, index: CopyRangeFrom<usize>) -> &mut Self::Output {
                self.index_mut(index.into_std())
            }
        }
        impl<$($generics)*> Index<CopyRangeInclusive<usize>> for $ty
        where
            $ty: Index<RangeInclusive<usize>>,
        {
            type Output = <$ty as Index<RangeInclusive<usize>>>::Output;

            fn index(&self, index: CopyRangeInclusive<usize>) -> &Self::Output {
                self.index(index.into_std())
            }
        }
        impl<$($generics)*> IndexMut<CopyRangeInclusive<usize>> for $ty
        where
            $ty: IndexMut<RangeInclusive<usize>>,
        {
            fn index_mut(&mut self, index: CopyRangeInclusive<usize>) -> &mut Self::Output {
                self.index_mut(index.into_std())
            }
        }
    };
}

impl_index!([T], [T]);
impl_index!([], str);
#[cfg(feature = "alloc")]
impl_index!([T], ::alloc::vec::Vec<T>);
#[cfg(feature = "alloc")]
impl_index!([], ::alloc::string::String);
