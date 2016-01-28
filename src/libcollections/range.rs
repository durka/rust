// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![unstable(feature = "collections_range",
            reason = "waiting for dust to settle on inclusive ranges",
            issue = "30877")]

//! Range syntax.

use core::option::Option::{self, None, Some};
use core::ops::{RangeFull, Range, RangeTo, RangeFrom, RangeInclusive, RangeToInclusive};

/// An endpoint of a range, which may be open or closed
pub enum Bounded<T> {
    /// An endpoint which is not included by range it bounds
    Open(T),

    /// An endpoint which is included by range it bounds
    Closed(T),
}

/// **RangeArgument** is implemented by Rust's built-in range types, produced
/// by range syntax like `..`, `a..`, `..b`, `c..d`, `...e` or `f...g`.
pub trait RangeArgument<T> {
    /// Start index
    ///
    /// Return start value if present, else `None`.
    fn start(&self) -> Option<Bounded<&T>> {
        None
    }

    /// End index
    ///
    /// Return end value if present, else `None`.
    fn end(&self) -> Option<Bounded<&T>> {
        None
    }
}

impl<T> RangeArgument<T> for RangeFull {}

impl<T> RangeArgument<T> for RangeFrom<T> {
    fn start(&self) -> Option<Bounded<&T>> {
        Some(Bounded::Closed(&self.start))
    }
}

impl<T> RangeArgument<T> for RangeTo<T> {
    fn end(&self) -> Option<Bounded<&T>> {
        Some(Bounded::Open(&self.end))
    }
}

impl<T> RangeArgument<T> for Range<T> {
    fn start(&self) -> Option<Bounded<&T>> {
        Some(Bounded::Closed(&self.start))
    }
    fn end(&self) -> Option<Bounded<&T>> {
        Some(Bounded::Open(&self.end))
    }
}

impl<T> RangeArgument<T> for RangeToInclusive<T> {
    fn end(&self) -> Option<Bounded<&T>> {
        Some(Bounded::Closed(&self.end))
    }
}

impl<T> RangeArgument<T> for RangeInclusive<T> {
    fn start(&self) -> Option<Bounded<&T>> {
        match *self {
            RangeInclusive::Empty { ref at } => Some(Bounded::Closed(at)),
            RangeInclusive::NonEmpty { ref start, .. } => Some(Bounded::Closed(start)),
        }
    }
    fn end(&self) -> Option<Bounded<&T>> {
        match *self {
            RangeInclusive::Empty { ref at } => Some(Bounded::Closed(at)),
            RangeInclusive::NonEmpty { ref end, .. } => Some(Bounded::Closed(end)),
        }
    }
}

