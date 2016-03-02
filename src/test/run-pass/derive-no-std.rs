// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(libc, lang_items, start)]
#![no_std]

extern crate libc; // to pull in libSystem on OSX

#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
#[lang = "eh_personality"] extern fn eh_personality() {}

// Issue #16803

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
         Debug, Default, Copy)]
struct Foo {
    x: u32,
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
         Debug, Copy)]
enum Bar {
    Qux,
    Quux(u32),
}

enum Baz { A=0, B=5, }

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
         Debug, Copy)]
enum Void {}
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
         Debug, Copy)]
struct Empty;
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
         Debug, Copy)]
struct AlsoEmpty {}

#[start]
fn main(argc: isize, argv: *const *const u8) -> isize {
    Foo { x: 0 };
    Bar::Quux(3);
    Baz::A;

    0
}
