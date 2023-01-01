/*
 *  Copyright (C) 2021 William Youmans
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

//mod ops;
mod conv;

use crate::{New, Real};
use arb_sys::acb::*;

use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};


#[derive(Debug)]
pub struct Acb {
    pub(crate) inner: acb_struct,
}

pub type Complex = Acb;

impl AsRef<Complex> for Complex {
    fn as_ref(&self) -> &Complex {
        self
    }
}

impl Clone for Complex {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = Complex::default();
        unsafe {
            acb_set(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
}

impl Default for Complex {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            acb_init(z.as_mut_ptr());
            Complex::from_raw(z.assume_init())
        }
    }
}

// TODO
impl fmt::Display for Complex {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + i*{}", self.re(), self.im())
    }
}

impl Drop for Complex {
    #[inline]
    fn drop(&mut self) {
        unsafe { acb_clear(self.as_mut_ptr()) }
    }
}

// TODO: avoid allocations
impl Hash for Complex {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.re().hash(state);
        self.im().hash(state);
    }
}

impl<T: Into<Complex>> New<T> for Complex {
    #[inline]
    fn new(src: T) -> Self {
        src.into()
    }
}

impl New<&Complex> for Complex {
    #[inline]
    fn new(src: &Complex) -> Self {
        src.clone()
    }
}

impl Complex {
    #[inline]
    pub fn zero() -> Self {
        Complex::default()
    }

    #[inline]
    pub fn one() -> Self {
        let mut res = Complex::default();
        res.one_assign();
        res
    }
    
    #[inline]
    pub fn onei() -> Self {
        let mut res = Complex::default();
        res.onei_assign();
        res
    }

    #[inline]
    pub fn zero_assign(&mut self) {
        unsafe {
            acb_zero(self.as_mut_ptr());
        }
    }

    #[inline]
    pub fn one_assign(&mut self) {
        unsafe {
            acb_one(self.as_mut_ptr());
        }
    }
    
    #[inline]
    pub fn onei_assign(&mut self) {
        unsafe {
            acb_onei(self.as_mut_ptr());
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe {
            acb_is_zero(self.as_ptr()) != 0
        }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe {
            acb_is_one(self.as_ptr()) != 0
        }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const acb_struct {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut acb_struct {
        &mut self.inner
    }

    #[inline]
    pub const unsafe fn from_raw(inner: acb_struct) -> Self {
        Complex { inner }
    }

    #[inline]
    pub const fn into_raw(self) -> acb_struct {
        let ret = self.inner;
        let _ = ManuallyDrop::new(self);
        ret
    }

    #[inline]
    pub fn re(&self) -> Real {
        let mut res = Real::default();
        unsafe {
            acb_get_real(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
    
    #[inline]
    pub fn im(&self) -> Real {
        let mut res = Real::default();
        unsafe {
            acb_get_imag(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
}
