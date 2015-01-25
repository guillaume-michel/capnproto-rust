// Copyright (c) 2013-2015 Sandstorm Development Group, Inc. and contributors
// Licensed under the MIT License:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

pub trait Mask {
    fn mask(value : Self, mask : Self) -> Self;
}

// There's got to be a way to use a standard trait like Bitwise here,
// but I keep getting "conflicting implementation" errors.

macro_rules! int_mask(
    ($t:ident) => (
        impl Mask for $t {
            #[inline]
            fn mask(value : $t, mask : $t) -> $t {
                value ^ mask
            }
        }
    )
);

int_mask!(i8);
int_mask!(i16);
int_mask!(i32);
int_mask!(i64);
int_mask!(u8);
int_mask!(u16);
int_mask!(u32);
int_mask!(u64);

impl Mask for f32 {
    #[inline]
    fn mask(value : f32, mask : f32) -> f32 {
        use std;
        unsafe {
            let v : u32 = std::mem::transmute(value);
            let m : u32 = std::mem::transmute(mask);
            std::mem::transmute(v ^ m)
        }
    }
}

impl Mask for f64 {
    #[inline]
    fn mask(value : f64, mask : f64) -> f64 {
        use std;
        unsafe {
            let v : u64 = std::mem::transmute(value);
            let m : u64 = std::mem::transmute(mask);
            std::mem::transmute(v ^ m)
        }
    }
}
