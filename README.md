# `copy_range`

This crate provides three structs: `CopyRange`, `CopyRangeFrom`, and
`CopyRangeInclusive`.

They are similar to `core::ops`'s `Range`, `RangeFrom`, and
`RangeInclusive`, respectively, except they implement `Copy` if their
element type implements `Copy`, and they implement `IntoIterator` instead of
`Iterator`.

They are freely convertible to and from their `core::ops` counterparts (with
a note about `RangeInclusive`), and they
implement most of the same (non-iterator-related) traits, notably
`RangeBounds`.

Ranges of `usize` are additionally usable as the `Index` parameter for
arrays, slices, string slices and
(with the `"alloc"` feature enabled) `Vec` and
`String`.
