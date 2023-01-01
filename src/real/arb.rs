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

mod ops;
mod conv;

use crate::{New, Real, arf::Arf, mag::Mag};
use arb_sys::{
    arb::*,
    arf::arf_set,
    mag::mag_set
};

use std::ffi::CStr;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};


#[derive(Debug)]
pub struct Arb {
    pub(crate) inner: arb_struct,
}

impl AsRef<Real> for Real {
    fn as_ref(&self) -> &Real {
        self
    }
}

impl Clone for Real {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = Real::default();
        unsafe {
            arb_set(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
}

impl Default for Real {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            arb_init(z.as_mut_ptr());
            Real::from_raw(z.assume_init())
        }
    }
}

impl fmt::Display for Real {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let mut n = self.bits() as f64;
            n *= 0.30102999566398119521; // log_10(2)
            let digits = n.ceil() as i64 + 1;
            let c_str = CStr::from_ptr(arb_get_str(self.as_ptr(), digits, 0));
            let s = c_str.to_str().unwrap();
            write!(f, "{}", s)
        }
    }
}

impl Drop for Real {
    #[inline]
    fn drop(&mut self) {
        unsafe { arb_clear(self.as_mut_ptr()) }
    }
}

// TODO: avoid allocations
impl Hash for Real {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.midpoint_as_arf().hash(state);
        self.radius_as_mag().hash(state);
    }
}

impl<T: Into<Real>> New<T> for Real {
    #[inline]
    fn new(src: T) -> Self {
        src.into()
    }
}

impl New<&Real> for Real {
    #[inline]
    fn new(src: &Real) -> Self {
        src.clone()
    }
}

impl Real {
    #[inline]
    pub fn zero() -> Self {
        Real::default()
    }

    #[inline]
    pub fn one() -> Self {
        let mut res = Real::default();
        res.one_assign();
        res
    }

    #[inline]
    pub fn zero_assign(&mut self) {
        unsafe {
            arb_zero(self.as_mut_ptr());
        }
    }

    #[inline]
    pub fn one_assign(&mut self) {
        unsafe {
            arb_one(self.as_mut_ptr());
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe {
            arb_is_zero(self.as_ptr()) != 0
        }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe {
            arb_is_one(self.as_ptr()) != 0
        }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const arb_struct {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut arb_struct {
        &mut self.inner
    }

    #[inline]
    pub const unsafe fn from_raw(inner: arb_struct) -> Self {
        Real { inner }
    }

    #[inline]
    pub const fn into_raw(self) -> arb_struct {
        let ret = self.inner;
        let _ = ManuallyDrop::new(self);
        ret
    }

    #[inline]
    pub fn bits(&self) -> i64 {
        unsafe { arb_bits(self.as_ptr()) }
    }

    pub fn midpoint(&self) -> Self {
        let mut res = Real::default();
        unsafe {
            arb_get_mid_arb(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }

    pub fn radius(&self) -> Self {
        let mut res = Real::default();
        unsafe {
            arb_get_rad_arb(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }

    pub fn midpoint_as_arf(&self) -> Arf {
        let mut res = Arf::default();
        unsafe {
            arf_set(res.as_mut_ptr(), &self.inner.mid);
        }
        res
    }
    
    pub fn radius_as_mag(&self) -> Mag {
        let mut res = Mag::default();
        unsafe {
            mag_set(res.as_mut_ptr(), &self.inner.rad);
        }
        res
    }
}
