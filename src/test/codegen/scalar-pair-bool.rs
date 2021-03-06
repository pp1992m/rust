// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: -O

#![crate_type = "lib"]

// CHECK: define { i8, i8 } @pair_bool_bool(i1 zeroext %pair.0, i1 zeroext %pair.1)
#[no_mangle]
pub fn pair_bool_bool(pair: (bool, bool)) -> (bool, bool) {
    pair
}

// CHECK: define { i8, i32 } @pair_bool_i32(i1 zeroext %pair.0, i32 %pair.1)
#[no_mangle]
pub fn pair_bool_i32(pair: (bool, i32)) -> (bool, i32) {
    pair
}

// CHECK: define { i32, i8 } @pair_i32_bool(i32 %pair.0, i1 zeroext %pair.1)
#[no_mangle]
pub fn pair_i32_bool(pair: (i32, bool)) -> (i32, bool) {
    pair
}

// CHECK: define { i8, i8 } @pair_and_or(i1 zeroext %arg0.0, i1 zeroext %arg0.1)
#[no_mangle]
pub fn pair_and_or((a, b): (bool, bool)) -> (bool, bool) {
    // Make sure it can operate directly on the unpacked args
    // CHECK: and i1 %arg0.0, %arg0.1
    // CHECK: or i1 %arg0.0, %arg0.1
    (a && b, a || b)
}

// CHECK: define void @pair_branches(i1 zeroext %arg0.0, i1 zeroext %arg0.1)
#[no_mangle]
pub fn pair_branches((a, b): (bool, bool)) {
    // Make sure it can branch directly on the unpacked bool args
    // CHECK: br i1 %arg0.0
    if a {
        println!("Hello!");
    }
    // CHECK: br i1 %arg0.1
    if b {
        println!("Goodbye!");
    }
}
