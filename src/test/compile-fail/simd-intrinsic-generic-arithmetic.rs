// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(repr_simd, platform_intrinsics)]
#![allow(non_camel_case_types)]
#[repr(simd)]
#[derive(Copy, Clone)]
pub struct i32x4(pub i32, pub i32, pub i32, pub i32);

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct u32x4(pub u32, pub u32, pub u32, pub u32);

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct f32x4(pub f32, pub f32, pub f32, pub f32);

extern "platform-intrinsic" {
    fn simd_add<T>(x: T, y: T) -> T;
    fn simd_sub<T>(x: T, y: T) -> T;
    fn simd_mul<T>(x: T, y: T) -> T;
    fn simd_div<T>(x: T, y: T) -> T;
    fn simd_shl<T>(x: T, y: T) -> T;
    fn simd_shr<T>(x: T, y: T) -> T;
    fn simd_and<T>(x: T, y: T) -> T;
    fn simd_or<T>(x: T, y: T) -> T;
    fn simd_xor<T>(x: T, y: T) -> T;
}

fn main() {
    let x = i32x4(0, 0, 0, 0);
    let y = u32x4(0, 0, 0, 0);
    let z = f32x4(0.0, 0.0, 0.0, 0.0);

    unsafe {
        simd_add(x, x);
        simd_add(y, y);
        simd_add(z, z);
        simd_sub(x, x);
        simd_sub(y, y);
        simd_sub(z, z);
        simd_mul(x, x);
        simd_mul(y, y);
        simd_mul(z, z);

        simd_div(z, z);

        simd_shl(x, x);
        simd_shl(y, y);
        simd_shr(x, x);
        simd_shr(y, y);
        simd_and(x, x);
        simd_and(y, y);
        simd_or(x, x);
        simd_or(y, y);
        simd_xor(x, x);
        simd_xor(y, y);


        simd_add(0, 0);
        //~^ ERROR expected SIMD input type, found non-SIMD `i32`
        simd_sub(0, 0);
        //~^ ERROR expected SIMD input type, found non-SIMD `i32`
        simd_mul(0, 0);
        //~^ ERROR expected SIMD input type, found non-SIMD `i32`
        simd_div(0, 0);
        //~^ ERROR expected SIMD input type, found non-SIMD `i32`
        simd_shl(0, 0);
        //~^ ERROR expected SIMD input type, found non-SIMD `i32`
        simd_shr(0, 0);
        //~^ ERROR expected SIMD input type, found non-SIMD `i32`
        simd_and(0, 0);
        //~^ ERROR expected SIMD input type, found non-SIMD `i32`
        simd_or(0, 0);
        //~^ ERROR expected SIMD input type, found non-SIMD `i32`
        simd_xor(0, 0);
        //~^ ERROR expected SIMD input type, found non-SIMD `i32`


        simd_div(x, x);
//~^ ERROR unsupported operation on `i32x4` with element `i32`
        simd_div(y, y);
//~^ ERROR unsupported operation on `u32x4` with element `u32`
        simd_shl(z, z);
//~^ ERROR unsupported operation on `f32x4` with element `f32`
        simd_shr(z, z);
//~^ ERROR unsupported operation on `f32x4` with element `f32`
        simd_and(z, z);
//~^ ERROR unsupported operation on `f32x4` with element `f32`
        simd_or(z, z);
//~^ ERROR unsupported operation on `f32x4` with element `f32`
        simd_xor(z, z);
//~^ ERROR unsupported operation on `f32x4` with element `f32`
    }
}
