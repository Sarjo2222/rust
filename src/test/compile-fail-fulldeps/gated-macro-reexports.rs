// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that macro reexports item are gated by `macro_reexport` feature gate.

// aux-build:macro_reexport_1.rs
// ignore-stage1

#![crate_type = "dylib"]

#[macro_reexport(reexported)]
#[macro_use] #[no_link]
extern crate macro_reexport_1;
//~^ ERROR macros reexports are experimental and possibly buggy
//~| HELP add #![feature(macro_reexport)] to the crate attributes to enable
