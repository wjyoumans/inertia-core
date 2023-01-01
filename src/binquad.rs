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
use flint_sys::fmpz::fmpz_set;
use antic_sys::qfb::*;

use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};


#[derive(Debug)]
pub struct BinQuadForm {
    pub(crate) inner: qfb,
}

impl AsRef<BinQuadForm> for BinQuadForm {
    fn as_ref(&self) -> &BinQuadForm {
        self
    }
}

impl Clone for BinQuadForm {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = BinQuadForm::default();
        unsafe {
            qfb_set(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
}

impl Default for BinQuadForm {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            qfb_init(z.as_mut_ptr());
            BinQuadForm::from_raw(z.assume_init())
        }
    }
}

impl fmt::Display for BinQuadForm {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let a = Integer::from_raw(self.inner.a[0]);
            let b = Integer::from_raw(self.inner.b[0]);
            let c = Integer::from_raw(self.inner.c[0]);
            let s = format!("({}, {}, {})", &a, &b, &c);
            let _ = ManuallyDrop::new(a);
            let _ = ManuallyDrop::new(b);
            let _ = ManuallyDrop::new(c);
            write!(f, "{}", s)
        }
    }
}

impl Drop for BinQuadForm {
    #[inline]
    fn drop(&mut self) {
        unsafe { qfb_clear(self.as_mut_ptr()) }
    }
}

// TODO
impl Hash for BinQuadForm {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let tmp = [self.inner.a, self.inner.b, self.inner.c];
        for coeff in tmp {
            unsafe {
                let a = ManuallyDrop::new(Integer::from_raw(coeff[0]));
                a.hash(state);
            }
        }
    }
}

impl<T: Into<BinQuadForm>> New<T> for BinQuadForm {
    #[inline]
    fn new(src: T) -> Self {
        src.into()
    }
}

impl New<&BinQuadForm> for BinQuadForm {
    #[inline]
    fn new(src: &BinQuadForm) -> Self {
        src.clone()
    }
}

impl BinQuadForm {
    #[inline]
    pub fn zero() -> Self {
        BinQuadForm::default()
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const qfb {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut qfb {
        &mut self.inner
    }

    #[inline]
    pub const unsafe fn from_raw(inner: qfb) -> Self {
        BinQuadForm { inner }
    }

    #[inline]
    pub const fn into_raw(self) -> qfb {
        let ret = self.inner;
        let _ = ManuallyDrop::new(self);
        ret
    }

    pub fn get_coeffs(&self) -> [Integer; 3] {
        let mut a = Integer::default();
        let mut b = Integer::default();
        let mut c = Integer::default();
        unsafe {
            fmpz_set(a.as_mut_ptr(), &self.inner.a[0]);
            fmpz_set(b.as_mut_ptr(), &self.inner.b[0]);
            fmpz_set(c.as_mut_ptr(), &self.inner.c[0]);
        }
        [a, b, c]
    }
}
