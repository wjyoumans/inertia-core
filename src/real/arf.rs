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

use crate::{New, Integer};
use arb_sys::arf::*;

use std::ffi::CStr;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};


#[derive(Debug)]
pub struct Arf {
    pub(crate) inner: arf_struct,
}

impl AsRef<Arf> for Arf {
    fn as_ref(&self) -> &Arf {
        self
    }
}

impl Clone for Arf {
    fn clone(&self) -> Self {
        let mut res = Arf::default();
        unsafe {
            arf_set(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
}

impl Default for Arf {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            arf_init(z.as_mut_ptr());
            Arf::from_raw(z.assume_init())
        }
    }
}

impl fmt::Display for Arf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let mut n = self.bits() as f64;
            n *= 0.30102999566398119521; // log_10(2)
            let digits = n.ceil() as i64 + 1;
            let c_str = CStr::from_ptr(arf_get_str(self.as_ptr(), digits));
            let s = c_str.to_str().unwrap();
            write!(f, "{}", s)
        }
    }
}

impl Drop for Arf {
    #[inline]
    fn drop(&mut self) {
        unsafe { arf_clear(self.as_mut_ptr()) }
    }
}

impl Hash for Arf {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (m, exp) = self.mantissa_exponent();
        m.hash(state);
        exp.hash(state);
    }
}

impl<T: Into<Arf>> New<T> for Arf {
    #[inline]
    fn new(src: T) -> Self {
        src.into()
    }
}

impl New<&Arf> for Arf {
    #[inline]
    fn new(src: &Arf) -> Self {
        src.clone()
    }
}

impl Arf {
    #[inline]
    pub fn zero() -> Self {
        Arf::default()
    }

    #[inline]
    pub fn one() -> Self {
        let mut res = Arf::default();
        res.one_assign();
        res
    }

    #[inline]
    pub fn zero_assign(&mut self) {
        unsafe {
            arf_zero(self.as_mut_ptr());
        }
    }

    #[inline]
    pub fn one_assign(&mut self) {
        unsafe {
            arf_one(self.as_mut_ptr());
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe {
            arf_is_zero(self.as_ptr()) != 0
        }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe {
            arf_is_one(self.as_ptr()) != 0
        }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const arf_struct {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut arf_struct {
        &mut self.inner
    }

    #[inline]
    pub const unsafe fn from_raw(inner: arf_struct) -> Self {
        Arf { inner }
    }

    #[inline]
    pub const fn into_raw(self) -> arf_struct {
        let ret = self.inner;
        let _ = ManuallyDrop::new(self);
        ret
    }

    #[inline]
    pub fn bits(&self) -> i64 {
        unsafe { arf_bits(self.as_ptr()) }
    }
    
    /// Return the mantissa `m` and exponent `exp` such that `x = m*2^exp`.
    pub fn mantissa_exponent(&self) -> (Integer, Integer) {
        let mut m = Integer::default();
        let mut exp = Integer::default();
        unsafe { 
            arf_get_fmpz_2exp(m.as_mut_ptr(), exp.as_mut_ptr(), self.as_ptr()); 
        }
        (m, exp)
    }
}

