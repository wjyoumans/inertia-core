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

use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use flint_sys::fmpq;
use crate::{Integer, ValOrRef, IntoValOrRef};

#[derive(Clone, Copy, Debug, Hash)]
pub struct RationalField {}
pub type Rationals = RationalField;

impl RationalField {
    #[inline]
    pub fn init() -> Self {
        RationalField {}
    }

    #[inline]
    pub fn default(&self) -> Rational {
        Rational::default()
    }

    #[inline]
    pub fn new<T: Into<Rational>>(&self, x: T) -> Rational {
        x.into()
    }
}

#[derive(Debug)]
pub struct Rational {
    inner: fmpq::fmpq,
}

impl Clone for Rational {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = Rational::default();
        unsafe { fmpq::fmpq_set(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
}

impl Default for Rational {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpq::fmpq_init(z.as_mut_ptr());
            Rational {
                inner: z.assume_init()
            }
        }
    }
}

impl fmt::Display for Rational {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for Rational {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpq::fmpq_clear(self.as_mut_ptr())}
    }
}

impl Hash for Rational {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.numerator().hash(state);
        self.denominator().hash(state);
    }
}

impl<'a, T> IntoValOrRef<'a, Rational> for T where
    T: Into<Rational>
{
    #[inline]
    fn val_or_ref(self) -> ValOrRef<'a, Rational> {
        ValOrRef::Val(self.into())
    }
}

impl Rational {
    
    /// Returns a pointer to the inner [FLINT rational][fmpq::fmpq].
    #[inline]
    pub const fn as_ptr(&self) -> *const fmpq::fmpq {
        &self.inner
    }

    /// Returns a mutable pointer to the inner [FLINT rational][fmpq::fmpq].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpq::fmpq {
        &mut self.inner
    }
    
    /// Returns the numerator of a rational number as an [Integer].
    ///
    /// ```
    /// use inertia_core::Rational;
    ///
    /// let q = Rational::from([3, 4]);
    /// assert_eq!(q.numerator(), 3);
    /// ```
    #[inline]
    pub fn numerator(&self) -> Integer {
        Integer::from_raw(self.inner.num)
    }
    
    /// Returns the denominator of a rational number as an [Integer].
    ///
    /// ```
    /// use inertia_core::Rational;
    ///
    /// let q = Rational::from([3, 4]);
    /// assert_eq!(q.denominator(), 4);
    /// ```
    #[inline]
    pub fn denominator(&self) -> Integer {
        Integer::from_raw(self.inner.den)
    }
}
