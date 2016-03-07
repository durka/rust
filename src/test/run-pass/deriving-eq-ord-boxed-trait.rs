// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test for #24047

#![feature(get_type_id)]

use std::any::Any;
use std::cmp::Ordering;

trait Foo: Any { }

impl PartialEq for Foo {
    fn eq(&self, rhs: &Foo) -> bool {
        self.get_type_id() == rhs.get_type_id()
    }
}

impl PartialOrd for Foo {
    fn partial_cmp(&self, rhs: &Foo) -> Option<Ordering> {
        None
    }
}

#[derive(PartialEq, PartialOrd)]
struct Bar(Box<Foo>);

pub fn main() {
}
