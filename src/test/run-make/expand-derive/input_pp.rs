// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// minimal junk
#![feature(no_core)]
#![no_core]

struct A {
    a: i32,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::clone::Clone for A {
    #[inline]
    fn clone(&self) -> A {
        {
            match *self {
                A { a: ref __self_0_0 } => {
                    ::clone::assert_receiver_is_clone(&(*__self_0_0));
                }
            };
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::marker::Copy for A { }

struct B<T> {
    a: T,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::clone::Clone> ::clone::Clone for B<T> {
    #[inline]
    fn clone(&self) -> B<T> {
        match *self {
            B { a: ref __self_0_0 } => B { a: ::clone::Clone::clone(&(*__self_0_0)), },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::marker::Copy> ::marker::Copy for B<T> { }

