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

use crate::{New, Integer, arf::Arf};

use arb_sys::mag::*;
use flint_sys::fmpz::fmpz_set;

use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};


#[derive(Debug)]
pub struct Mag {
    pub(crate) inner: mag_struct,
}

impl AsRef<Mag> for Mag {
    fn as_ref(&self) -> &Mag {
        self
    }
}

impl Clone for Mag {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            mag_init_set(z.as_mut_ptr(), self.as_ptr());
            Mag::from_raw(z.assume_init())
        }
    }
}

impl Default for Mag {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            mag_init(z.as_mut_ptr());
            Mag::from_raw(z.assume_init())
        }
    }
}

// TODO: use mag_fprint_d
impl fmt::Display for Mag {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Arf::from(self))
    }
}

impl Drop for Mag {
    #[inline]
    fn drop(&mut self) {
        unsafe { mag_clear(self.as_mut_ptr()) }
    }
}

impl Hash for Mag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (m, exp) = self.mantissa_exponent();
        m.hash(state);
        exp.hash(state);
    }
}

impl<T: Into<Mag>> New<T> for Mag {
    #[inline]
    fn new(src: T) -> Self {
        src.into()
    }
}

impl New<&Mag> for Mag {
    #[inline]
    fn new(src: &Mag) -> Self {
        src.clone()
    }
}

impl Mag {
    #[inline]
    pub fn zero() -> Self {
        Mag::default()
    }

    #[inline]
    pub fn one() -> Self {
        let mut res = Mag::default();
        res.one_assign();
        res
    }

    #[inline]
    pub fn zero_assign(&mut self) {
        unsafe {
            mag_zero(self.as_mut_ptr());
        }
    }

    #[inline]
    pub fn one_assign(&mut self) {
        unsafe {
            mag_one(self.as_mut_ptr());
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe {
            mag_is_zero(self.as_ptr()) != 0
        }
    }

    /* TODO no mag_is_one function
    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe {
            mag_is_one(self.as_ptr()) != 0
        }
    }*/

    #[inline]
    pub const fn as_ptr(&self) -> *const mag_struct {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut mag_struct {
        &mut self.inner
    }

    #[inline]
    pub const unsafe fn from_raw(inner: mag_struct) -> Self {
        Mag { inner }
    }

    #[inline]
    pub const fn into_raw(self) -> mag_struct {
        let ret = self.inner;
        let _ = ManuallyDrop::new(self);
        ret
    }

    /* TODO: no mag_bits. use get_d_log2_approx or convert to Arf
    #[inline]
    pub fn bits(&self) -> i64 {
        unsafe { mag_bits(self.as_ptr()) }
    }
    */
    
    /// Return the mantissa `m` and exponent `exp` such that `x = m*2^exp`.
    pub fn mantissa_exponent(&self) -> (u64, Integer) {
        let m = self.inner.man;
        let mut exp = Integer::default();
        unsafe {
            fmpz_set(exp.as_mut_ptr(), &self.inner.exp); 
        }
        (m, exp)
    }
}

