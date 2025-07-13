// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Integer trait and functions.
//!
//! ## Compatibility
//!
//! The `num-integer` crate is tested for rustc 1.31 and greater.

// Copyright 2025 Matthew Yang
// This file is part of dproc.
// This is the stripped down version of the num-integer crate, only including what is necessary for dproc to function.
// The main source code of the num-integer crate remains intact in the other files.

// gcd and lcm
pub trait Integer: Sized + Copy + PartialEq {
    fn gcd(&self, other: &Self) -> Self;
    fn lcm(&self, other: &Self) -> Self;
}

impl Integer for u32 {
    fn gcd(&self, other: &Self) -> Self {
        let mut m = *self;
        let mut n = *other;
        while n != 0 {
            let r = m % n;
            m = n;
            n = r;
        }
        m
    }
    fn lcm(&self, other: &Self) -> Self {
        (*self / self.gcd(other)) * *other
    }
}

/// Calculates the Greatest Common Divisor (GCD) of the number and `other`. The
/// result is always non-negative.
#[inline(always)]
pub fn gcd<T: Integer>(x: T, y: T) -> T {
    x.gcd(&y)
}
/// Calculates the Lowest Common Multiple (LCM) of the number and `other`.
#[inline(always)]
pub fn lcm<T: Integer>(x: T, y: T) -> T {
    x.lcm(&y)
}
