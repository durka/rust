// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(question_mark)]

fn does_not_return_result() -> i32 {
    try!(Ok(42)); //~ERROR expected i32, found enum `std::result::Result`
    //^~NOTE `try!` can only be used in a function that returns a `Result`
    
    Ok(42)?; //~ERROR expected i32, found enum `std::result::Result`
    //^~NOTE `?` can only be used in a function that returns a `Result`
    
    42
}

fn returns_result() -> Result<i32, ()> {
    // no note because the type mismatch is not regarding the return statement
    
    let x: i32 = try!(Ok(42.0)); //~ERROR expected i32, found f32
    let y: i32 = Ok(42.0)?; //~ERROR expected i32, found f32

    try!(Err(42)); //~ERROR expected (), found {integer}
    Err(42)?; //~ERROR expected (), found {integer}

    Ok(42)
}

fn returns_result_result() -> Result<Result<i32, ()>, ()> {
    // tricky case: type mismatch is "expected Result, found i32" but still not regarding the
    // return statement
    
    let x: Result<i32, ()> = try!(Ok(42)); //~ERROR expected Result<i32, ()>, found i32
    let y: Result<i32, ()> = Ok(42)?; //~ERROR expected Result<i32, ()>, found i32

    Ok(Ok(42))
}

fn main() {}

