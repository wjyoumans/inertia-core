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

#[cfg(feature = "serde")]
mod serde;

use crate::Integer;

use flint_sys::{fmpz, fmpq};

use std::any::TypeId;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};

#[derive(Clone, Debug)]
pub struct RationalField {}

impl Eq for RationalField {}

impl PartialEq for RationalField {
    #[inline]
    fn eq(&self, _: &RationalField) -> bool {
        true
    }
}

impl fmt::Display for RationalField {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rational field")
    }
}

impl Hash for RationalField {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        TypeId::of::<Self>().hash(state)
    }
}

impl RationalField {
    #[inline]
    pub fn init() -> Self {
        RationalField {}
    }
    
    #[inline]
    pub fn new<T: Into<Rational>>(&self, value: T) -> Rational {
        Rational::new(value)
    }
}

#[derive(Debug)]
pub struct Rational {
    inner: fmpq::fmpq,
}

impl AsRef<Rational> for Rational {
    #[inline]
    fn as_ref(&self) -> &Rational {
        self
    }
}

impl Clone for Rational {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = Rational::default();
        unsafe {
            fmpq::fmpq_set(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
}

impl Default for Rational {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpq::fmpq_init(z.as_mut_ptr());
            Rational::from_raw(z.assume_init())
        }
    }
}

impl fmt::Display for Rational {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = self.numerator();
        let d = self.denominator();
        if d == 1 {
            write!(f, "{}", n)
        } else {
            write!(f, "{}/{}", n, d)
        }
    }
}

impl Drop for Rational {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpq::fmpq_clear(self.as_mut_ptr()) }
    }
}

impl Hash for Rational {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.numerator().hash(state);
        self.denominator().hash(state);
    }
}

impl Rational {
    #[inline]
    pub fn new<T: Into<Rational>>(value: T) -> Self {
        value.into()
    }

    /// Return zero.
    ///
    /// ```
    /// use inertia_core::Rational;
    ///
    /// assert_eq!(Rational::zero(), 0);
    /// ```
    #[inline]
    pub fn zero() -> Rational {
        Rational::default()
    }

    /// Return one.
    ///
    /// ```
    /// use inertia_core::Rational;
    ///
    /// assert_eq!(Rational::one(), 1);
    /// ```
    #[inline]
    pub fn one() -> Rational {
        let mut res = Rational::default();
        unsafe { fmpq::fmpq_one(res.as_mut_ptr()); }
        res
    }
    
    #[inline]
    pub fn zero_assign(&mut self) {
        unsafe { fmpq::fmpq_zero(self.as_mut_ptr()) }
    }
    
    #[inline]
    pub fn one_assign(&mut self) {
        unsafe { fmpq::fmpq_one(self.as_mut_ptr()) }
    }
    
    /// Return true if the `Rational` is zero.
    ///
    /// ```
    /// use inertia_core::Rational;
    ///
    /// let x = Rational::from(0u32);
    /// assert!(x.is_zero());
    /// ```
    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { fmpq::fmpq_is_zero(self.as_ptr()) == 1 }
    }

    /// Return true if the `Rational` is one.
    ///
    /// ```
    /// use inertia_core::Rational;
    ///
    /// let x = Rational::from(1i16);
    /// assert!(x.is_one());
    /// ```
    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { fmpq::fmpq_is_one(self.as_ptr()) == 1 }
    }

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

    /// Instantiate a rational from a [FLINT rational][fmpq::fmpq].
    #[inline]
    pub const unsafe fn from_raw(raw: fmpq::fmpq) -> Rational {
        Rational { inner: raw }
    }

    #[inline]
    pub const fn into_raw(self) -> fmpq::fmpq {
        let ret = self.inner;
        let _ = ManuallyDrop::new(self);
        ret
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
        let mut res = Integer::zero();
        unsafe {
            fmpz::fmpz_set(res.as_mut_ptr(), &self.inner.num)
        }
        res
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
        let mut res = Integer::zero();
        unsafe {
            fmpz::fmpz_set(res.as_mut_ptr(), &self.inner.den)
        }
        res
    }

    #[inline]
    pub fn floor(&self) -> Integer {
        let mut res = self.numerator();
        res.fdiv_assign(self.denominator());
        res
    }

    #[inline]
    pub fn ceil(&self) -> Integer {
        let mut res = self.numerator();
        res.cdiv_assign(self.denominator());
        res
    }
    
    #[inline]
    pub fn round(&self) -> Integer {
        let mut res = self.numerator();
        res.tdiv_assign(self.denominator());
        res
    }
    
    #[inline]
    pub fn sign(&self) -> i32 {
        unsafe {
            fmpq::fmpq_sgn(self.as_ptr())
        }
    }

    #[inline]
    pub fn abs(&self) -> Rational {
        unsafe {
            let mut res = Rational::default();
            fmpq::fmpq_abs(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }
    
    #[inline]
    pub fn abs_assign(&mut self) {
        unsafe {
            fmpq::fmpq_abs(self.as_mut_ptr(), self.as_ptr());
        }
    }

    #[inline]
    pub fn height(&self) -> Integer {
        unsafe {
            let mut res = Integer::default();
            fmpq::fmpq_height(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }
}

