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

pub mod macros;

use crate::New;
use flint_sys::fmpz;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};

#[derive(Debug)]
pub struct Integer {
    inner: fmpz::fmpz,
}

impl AsRef<Integer> for Integer {
    fn as_ref(&self) -> &Integer {
        self
    }
}

impl Clone for Integer {
    #[inline]
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz::fmpz_init_set(z.as_mut_ptr(), self.as_ptr());
            Integer::from_raw(z.assume_init())
        }
    }
}

impl Default for Integer {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz::fmpz_init(z.as_mut_ptr());
            Integer::from_raw(z.assume_init())
        }
    }
}

impl fmt::Display for Integer {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str_radix(10))
    }
}

impl Drop for Integer {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpz::fmpz_clear(self.as_mut_ptr()) }
    }
}

impl Hash for Integer {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_ui_vector().hash(state);
    }
}

impl<T: Into<Integer>> New<T> for Integer {
    #[inline]
    fn new(src: T) -> Self {
        src.into()
    }
}

impl New<&Integer> for Integer {
    #[inline]
    fn new(src: &Integer) -> Self {
        src.clone()
    }
}

impl Integer {
    // FFI ///

    /// Returns a pointer to the inner [FLINT integer][fmpz::fmpz].
    #[inline]
    pub const fn as_ptr(&self) -> *const fmpz::fmpz {
        &self.inner
    }

    /// Returns a mutable pointer to the inner [FLINT integer][fmpz::fmpz].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpz::fmpz {
        &mut self.inner
    }

    /// Create an Integer from an initialized [FLINT integer][fmpz::fmpz].
    ///
    /// # Safety
    ///
    ///   * The function must *not* be used to create a constant [`Integer`],
    ///     though it can be used to create a static [`Integer`]. This is
    ///     because constant values are *copied* on use, leading to undefined
    ///     behavior when they are dropped.
    ///   * The value must be initialized.
    ///   * The [`fmpz::fmpz`] type can be considered as a kind of pointer, so there
    ///     can be multiple copies of it. Since this function takes over
    ///     ownership, no other copies of the passed value should exist.
    #[inline]
    pub const unsafe fn from_raw(raw: fmpz::fmpz) -> Integer {
        Integer { inner: raw }
    }

    /// The returned object should be freed to avoid memory leaks.
    #[inline]
    pub const fn into_raw(self) -> fmpz::fmpz {
        let ret = self.inner;
        let _ = ManuallyDrop::new(self);
        ret
    }

    // Construction //

    /// Initialize a new `Integer` with the given number of limbs.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let x = Integer::with_capacity(2);
    /// assert_eq!(x, 0);
    /// ```
    #[inline]
    pub fn with_capacity(limbs: u64) -> Integer {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz::fmpz_init2(z.as_mut_ptr(), limbs);
            Integer::from_raw(z.assume_init())
        }
    }

    /// Return zero.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// assert_eq!(Integer::zero(), 0);
    /// ```
    #[inline]
    pub fn zero() -> Integer {
        Integer::default()
    }

    /// Return one.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// assert_eq!(Integer::one(), 1);
    /// ```
    #[inline]
    pub fn one() -> Integer {
        let mut res = Integer::default();
        unsafe { fmpz::fmpz_one(res.as_mut_ptr()); }
        res
    }

    /// Set the integer to zero.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    /// 
    /// let mut a = Integer::new(5);
    /// a.zero_assign();
    /// assert!(a.is_zero());
    /// ```
    #[inline]
    pub fn zero_assign(&mut self) {
        unsafe { fmpz::fmpz_zero(self.as_mut_ptr()) }
    }
    
    /// Set the integer to one.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    /// 
    /// let mut a = Integer::new(5);
    /// a.one_assign();
    /// assert!(a.is_one());
    /// ```
    #[inline]
    pub fn one_assign(&mut self) {
        unsafe { fmpz::fmpz_one(self.as_mut_ptr()) }
    }

    // Random generation //

    // Conversion //

    /// Return an `Option` containing the input as a signed long if possible.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(-1234);
    /// assert_eq!(z.get_si().unwrap(), -1234);
    /// ```
    #[inline]
    pub fn get_si(&self) -> Option<i64> {
        if self.fits_si() {
            unsafe { Some(fmpz::fmpz_get_si(self.as_ptr())) }
        } else {
            None
        }
    }

    /// Return an `Option` containing the input as an unsigned long if possible.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(1234);
    /// assert_eq!(z.get_ui().unwrap(), 1234);
    ///
    /// let z = Integer::from(-1234);
    /// assert!(z.get_ui().is_none());
    /// ```
    #[inline]
    pub fn get_ui(&self) -> Option<u64> {
        if self.sign() < 0 {
            return None;
        }
        if self.abs_fits_ui() {
            unsafe { Some(flint_sys::fmpz::fmpz_get_ui(self.as_ptr())) }
        } else {
            None
        }
    }

    /// Return a vector `A` of unsigned longs such that the original [Integer] can
    /// be written as `a[0] + a[1]*x + ... + a[n-1]*x^(n-1)` where `x = 2^FLINT_BITS`.
    ///
    /// ```
    /// use inertia_core::*;
    ///
    /// let z = Integer::from(2).pow(65u8);
    /// let v = z.get_ui_vector();
    /// assert!(v == vec![0, 2]);
    ///
    /// let mut t = Integer::default();
    /// t.set_ui_vector(v);
    /// assert_eq!(z, t);
    ///
    /// let x: Integer = "18446744073709551616".parse().unwrap();
    /// let v = x.get_ui_vector();
    /// let mut t = Integer::default();
    /// t.set_ui_vector(v);
    /// assert_eq!(x, t);
    /// ```
    #[inline]
    pub fn get_ui_vector(&self) -> Vec<u64> {
        if self.is_zero() {
            vec![]
        } else {
            let n = self.size();
            let mut out = Vec::with_capacity(n as usize);
            unsafe {
                fmpz::fmpz_get_ui_array(out.as_mut_ptr(), n, self.as_ptr());
                out.set_len(n as usize);
            }
            out
        }
    }

    /// Set `self` to the nonnegative [Integer]
    /// `vec[0] + vec[1]*x + ... + vec[n-1]*x^(n-1)` where `x = 2^FLINT_BITS`.
    ///
    /// ```
    /// use inertia_core::*;
    ///
    /// let mut z = Integer::default();
    /// z.set_ui_vector(vec![0,2]);
    /// assert_eq!(z, Integer::from(2).pow(65u8));
    /// ```
    #[inline]
    pub fn set_ui_vector(&mut self, vec: Vec<u64>) {
        if vec.is_empty() {
            self.zero_assign();
        } else {
            unsafe {
                fmpz::fmpz_set_ui_array(
                    self.as_mut_ptr(), vec.as_ptr(), vec.len() as i64);
            }
        }
    }

    /// Convert the `Integer` to a string in base `base`.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let x = Integer::from(1024);
    /// assert_eq!(x.to_str_radix(2), "10000000000")
    /// ```
    pub fn to_str_radix(&self, base: u8) -> String {
        unsafe {
            // Extra two bytes are for possible minus sign and null terminator
            let len = fmpz::fmpz_sizeinbase(self.as_ptr(), base as i32) as usize + 2;

            // Allocate and write into a raw *c_char of the correct length
            let mut vector: Vec<u8> = Vec::with_capacity(len);
            fmpz::fmpz_get_str(vector.as_mut_ptr() as *mut _, 
                               base as i32, self.as_ptr());
            
            vector.set_len(len);
            let mut first_nul = None;
            let mut index: usize = 0;
            for elem in &vector {
                if *elem == 0 {
                    first_nul = Some(index);
                    break;
                }
                index += 1;
            }
            let first_nul = first_nul.unwrap_or(len);

            vector.truncate(first_nul);
            match String::from_utf8(vector) {
                Ok(s) => s,
                Err(_) => panic!("FLINT returned invalid UTF-8!"),
            }
        }
    }
    
    // Basic properties //

    /// Determines the size of the absolute value of an `Integer` in base `base`
    /// in terms of number of digits. The base can be between 2 and 62, inclusive.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(1000001);
    /// assert_eq!(8, z.sizeinbase(7));
    /// ```
    #[inline]
    pub fn sizeinbase(&self, base: i32) -> usize {
        assert!(1 < base && base < 63);
        unsafe { fmpz::fmpz_sizeinbase(self.as_ptr(), base) }
    }

    /// Returns the number of bits required to store the absolute value of an
    /// `Integer`. Returns zero if the `Integer` is zero.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let x = Integer::from(16);
    /// assert_eq!(x.bits(), 5);
    /// ```
    #[inline]
    pub fn bits(&self) -> u64 {
        unsafe { fmpz::fmpz_bits(self.as_ptr()) }
    }

    /// Returns the number of limbs required to store the absolute value of an
    /// `Integer`. Returns zero if the `Integer` is zero.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z: Integer = "18446744073709551616".parse().unwrap();
    /// assert_eq!(2, z.size());
    /// ```
    #[inline]
    pub fn size(&self) -> i64 {
        unsafe { flint_sys::fmpz::fmpz_size(self.as_ptr()) }
    }
    
    /// Returns -1 if the `Integer` is negative, +1 if the `Integer`
    /// is positive, and 0 otherwise.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(-12);
    /// assert_eq!(z.sign(), -1);
    ///
    /// let z = Integer::from(0);
    /// assert_eq!(z.sign(), 0);
    ///
    /// let z = Integer::from(12);
    /// assert_eq!(z.sign(), 1);
    /// ```
    #[inline]
    pub fn sign(&self) -> i32 {
        unsafe { fmpz::fmpz_sgn(self.as_ptr()) }
    }

    /// Determine if the `Integer` fits in a signed long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z: Integer = "18446744073709551616".parse().unwrap();
    /// assert_eq!(z.fits_si(), false);
    /// ```
    #[inline]
    pub fn fits_si(&self) -> bool {
        unsafe { fmpz::fmpz_fits_si(self.as_ptr()) == 1 }
    }

    /// Determine if the absolute value of an `Integer` fits in an unsigned long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z: Integer = "18446744073709551614".parse().unwrap();
    /// assert_eq!(z.abs_fits_ui(), true);
    /// ```
    #[inline]
    pub fn abs_fits_ui(&self) -> bool {
        unsafe { fmpz::fmpz_abs_fits_ui(self.as_ptr()) == 1 }
    }
    
    /// Sets the bit index `bit_index` of an `Integer`.
    ///
    /// ```
    /// use inertia_core::*;
    ///
    /// let mut z = Integer::from(1024);
    /// z.setbit(0);
    /// assert_eq!(1025, z);
    /// ```
    #[inline]
    pub fn setbit(&mut self, bit_index: u64) {
        unsafe { fmpz::fmpz_setbit(self.as_mut_ptr(), bit_index) }
    }

    /// Test the bit index `bit_index` of an `Integer`. Return `true` if it is 1,
    /// `false` if it is zero.
    ///
    /// ```
    /// use inertia_core::*;
    ///
    /// let z = Integer::from(1025);
    /// assert!(z.testbit(0));
    /// ```
    #[inline]
    pub fn testbit(&self, bit_index: u64) -> bool {
        unsafe { fmpz::fmpz_tstbit(self.as_ptr(), bit_index) == 1 }
    }

    // Comparison //
    
    /// Return true if the `Integer` is zero.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let x = Integer::from(0u32);
    /// assert!(x.is_zero());
    /// ```
    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { fmpz::fmpz_is_zero(self.as_ptr()) == 1 }
    }

    /// Return true if the `Integer` is one.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let x = Integer::from(1i16);
    /// assert!(x.is_one());
    /// ```
    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { fmpz::fmpz_is_one(self.as_ptr()) == 1 }
    }
    
    /// Return true if the `Integer` is 1 or -1.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let x = Integer::from(-1i32);
    /// assert!(x.is_pm1());
    /// ```
    #[inline]
    pub fn is_pm1(&self) -> bool {
        unsafe { fmpz::fmpz_is_pm1(self.as_ptr()) == 1 }
    }

    /// Check if the `Integer` is even.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(102);
    /// assert!(z.is_even());
    /// ```
    #[inline]
    pub fn is_even(&self) -> bool {
        unsafe { fmpz::fmpz_is_even(self.as_ptr()) == 1 }
    }

    /// Check if the `Integer` is odd.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(103);
    /// assert!(z.is_odd());
    /// ```
    #[inline]
    pub fn is_odd(&self) -> bool {
        unsafe { fmpz::fmpz_is_odd(self.as_ptr()) == 1 }
    }

    // Basic arithmetic //

    /// Returns the absolute value of an `Integer`
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(-99);
    /// assert_eq!(z.abs(), Integer::from(99));
    /// ```
    #[inline]
    pub fn abs(&self) -> Integer {
        unsafe {
            let mut res = Integer::default();
            fmpz::fmpz_abs(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }

    /// Set the input to its absolute value.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let mut z = Integer::from(-99);
    /// z.abs_assign();
    /// assert_eq!(z, Integer::from(99));
    /// ```
    #[inline]
    pub fn abs_assign(&mut self) {
        unsafe {
            fmpz::fmpz_abs(self.as_mut_ptr(), self.as_ptr());
        }
    }

    /// Outputs `self * x * y` where `x, y` can be converted to unsigned longs.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let f = Integer::from(-1);
    /// assert_eq!(f.mul2_uiui(10u32, 3u32), -30i32);
    /// ```
    #[inline]
    pub fn mul2_uiui<S>(&self, x: S, y: S) -> Integer
    where
        S: Into<u64>,
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_mul2_uiui(res.as_mut_ptr(), self.as_ptr(), 
                                 x.into(), y.into());
        }
        res
    }

    /// Set `self` to `self * x * y` where `x, y` can be converted to unsigned longs.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let mut f = Integer::from(-1);
    /// f.mul2_uiui_assign(10u8, 3u8);
    /// assert_eq!(f, -30);
    /// ```
    #[inline]
    pub fn mul2_uiui_assign<S>(&mut self, x: S, y: S)
    where
        S: Into<u64>,
    {
        unsafe {
            fmpz::fmpz_mul2_uiui(self.as_mut_ptr(), self.as_ptr(), 
                                 x.into(), y.into());
        }
    }

    /// Output `self * 2^exp`.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let g = Integer::from(2);
    /// assert_eq!(g.mul_2exp(3u64), 16);
    /// ```
    #[inline]
    pub fn mul_2exp<S>(&self, exp: S) -> Integer
    where
        S: Into<u64>,
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_mul_2exp(res.as_mut_ptr(), self.as_ptr(), exp.into());
        }
        res
    }

    /// Compute `self * 2^exp` in place.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let mut g = Integer::from(2);
    /// g.mul_2exp_assign(3u32);
    /// assert_eq!(g, 16);
    /// ```
    #[inline]
    pub fn mul_2exp_assign<S>(&mut self, exp: S)
    where
        S: Into<u64>,
    {
        unsafe {
            fmpz::fmpz_mul_2exp(self.as_mut_ptr(), self.as_ptr(), exp.into());
        }
    }
    
    /* TODO: Flint 3
    /// Return the power of two `2^exp`.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// assert_eq!(Integer::one_2exp(3u64), 8);
    /// ```
    #[inline]
    pub fn one_2exp<S>(exp: S) -> Integer
    where
        S: Into<u64>,
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_one_2exp(res.as_mut_ptr(), exp.into());
        }
        res
    }
    
    /// Set the input to the power of two `2^exp`.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let mut g = Integer::from(2);
    /// g.one_2exp_assign(3u32);
    /// assert_eq!(g, 8);
    /// ```
    #[inline]
    pub fn one_2exp_assign<S>(&mut self, exp: S)
    where
        S: Into<u64>,
    {
        unsafe {
            fmpz::fmpz_mul_2exp(self.as_mut_ptr(), self.as_ptr(), exp.into());
        }
    }
    */

    /// Return `self + (x * y)`.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(2);
    /// assert_eq!(z.addmul(Integer::from(3), Integer::from(4)), 14);
    /// ```
    #[inline]
    pub fn addmul<T>(&self, x: T, y: T) -> Integer
    where
        T: AsRef<Integer>,
    {
        let mut res = self.clone();
        unsafe {
            fmpz::fmpz_addmul(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.as_ref().as_ptr()
            );
        }
        res
    }

    /// Compute `self + (x * y)` in place.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let mut z = Integer::from(2);
    /// z.addmul_assign(Integer::from(3), Integer::from(4));
    /// assert_eq!(z, 14);
    /// ```
    #[inline]
    pub fn addmul_assign<T>(&mut self, x: T, y: T)
    where
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_addmul(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.as_ref().as_ptr()
            );
        }
    }

    /// Return `self + (x * y)` where `y` can be converted to an unsigned long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(2);
    /// assert_eq!(z.addmul_ui(Integer::from(3), 4u32), 14);
    /// ```
    #[inline]
    pub fn addmul_ui<S, T>(&self, x: T, y: S) -> Integer
    where
        S: Into<u64>,
        T: AsRef<Integer>,
    {
        let mut res = self.clone();
        unsafe {
            fmpz::fmpz_addmul_ui(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.into()
            );
        }
        res
    }

    /// Compute `self + (x * y)` in place where `y` can be converted to an
    /// unsigned long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let mut z = Integer::from(2);
    /// z.addmul_ui_assign(Integer::from(3), 4u8);
    /// assert_eq!(z, 14);
    /// ```
    #[inline]
    pub fn addmul_ui_assign<S, T>(&mut self, x: T, y: S)
    where
        S: Into<u64>,
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_addmul_ui(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.into()
            );
        }
    }
    
    /// Return `self + (x * y)` where `y` can be converted to a signed long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(2);
    /// assert_eq!(z.addmul_si(Integer::from(3), 4i32), 14);
    /// ```
    #[inline]
    pub fn addmul_si<S, T>(&self, x: T, y: S) -> Integer
    where
        S: Into<i64>,
        T: AsRef<Integer>,
    {
        let mut res = self.clone();
        unsafe {
            fmpz::fmpz_addmul_si(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.into()
            );
        }
        res
    }

    /// Compute `self + (x * y)` in place where `y` can be converted to an
    /// signed long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let mut z = Integer::from(2);
    /// z.addmul_si_assign(Integer::from(3), 4i8);
    /// assert_eq!(z, 14);
    /// ```
    #[inline]
    pub fn addmul_si_assign<S, T>(&mut self, x: T, y: S)
    where
        S: Into<i64>,
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_addmul_si(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.into()
            );
        }
    }
    
    /// Return `self - (x * y)`.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(13);
    /// assert_eq!(z.submul(Integer::from(3), Integer::from(4)), 1);
    /// ```
    #[inline]
    pub fn submul<T>(&self, x: T, y: T) -> Integer
    where
        T: AsRef<Integer>,
    {
        let mut res = self.clone();
        unsafe {
            fmpz::fmpz_submul(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.as_ref().as_ptr()
            );
        }
        res
    }

    /// Compute `self - (x * y)` in place.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let mut z = Integer::from(13);
    /// z.submul_assign(Integer::from(3), Integer::from(4));
    /// assert_eq!(z, 1);
    /// ```
    #[inline]
    pub fn submul_assign<T>(&mut self, x: T, y: T)
    where
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_submul(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.as_ref().as_ptr()
            );
        }
    }

    /// Return `self - (x * y)` where `y` can be converted to an unsigned long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(13);
    /// assert_eq!(z.submul_ui(Integer::from(3), 4u32), 1);
    /// ```
    #[inline]
    pub fn submul_ui<S, T>(&self, x: T, y: S) -> Integer
    where
        S: Into<u64>,
        T: AsRef<Integer>,
    {
        let mut res = self.clone();
        unsafe {
            fmpz::fmpz_submul_ui(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.into()
            );
        }
        res
    }

    /// Compute `self - (x * y)` in place where `y` can be converted to an
    /// unsigned long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let mut z = Integer::from(13);
    /// z.submul_ui_assign(Integer::from(3), 4u8);
    /// assert_eq!(z, 1);
    /// ```
    #[inline]
    pub fn submul_ui_assign<S, T>(&mut self, x: T, y: S)
    where
        S: Into<u64>,
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_submul_ui(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.into()
            );
        }
    }
    
    /// Return `self - (x * y)` where `y` can be converted to a signed long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(13);
    /// assert_eq!(z.submul_si(Integer::from(3), 4i32), 1);
    /// ```
    #[inline]
    pub fn submul_si<S, T>(&self, x: T, y: S) -> Integer
    where
        S: Into<i64>,
        T: AsRef<Integer>,
    {
        let mut res = self.clone();
        unsafe {
            fmpz::fmpz_submul_si(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.into()
            );
        }
        res
    }

    /// Compute `self - (x * y)` in place where `y` can be converted to an
    /// signed long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let mut z = Integer::from(13);
    /// z.submul_si_assign(Integer::from(3), 4i8);
    /// assert_eq!(z, 1);
    /// ```
    #[inline]
    pub fn submul_si_assign<S, T>(&mut self, x: T, y: S)
    where
        S: Into<i64>,
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_submul_si(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.into()
            );
        }
    }
    
    /// Return `(a * b) + (c * d)`.
    ///
    /// ```
    /// use inertia_core::Integer;
    /// 
    /// let v: Vec<Integer> = [1, 2, 3, 4].into_iter()
    ///     .map(|x| Integer::from(x)).collect();
    ///
    /// assert_eq!(Integer::fmma(&v[0], &v[1], &v[2], &v[3]), 14);
    /// ```
    #[inline]
    pub fn fmma<T>(a: T, b: T, c: T, d: T) -> Integer
    where
        T: AsRef<Integer>,
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_fmma(
                res.as_mut_ptr(), 
                a.as_ref().as_ptr(), 
                b.as_ref().as_ptr(),
                c.as_ref().as_ptr(),
                d.as_ref().as_ptr()
            );
        }
        res
    }
    
    /// Return `(a * b) - (c * d)`.
    ///
    /// ```
    /// use inertia_core::Integer;
    /// 
    /// let v: Vec<Integer> = [4, 3, 2, 1].into_iter()
    ///     .map(|x| Integer::from(x)).collect();
    ///
    /// assert_eq!(Integer::fmms(&v[0], &v[1], &v[2], &v[3]), 10);
    /// ```
    #[inline]
    pub fn fmms<T>(a: T, b: T, c: T, d: T) -> Integer
    where
        T: AsRef<Integer>,
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_fmms(
                res.as_mut_ptr(), 
                a.as_ref().as_ptr(), 
                b.as_ref().as_ptr(),
                c.as_ref().as_ptr(),
                d.as_ref().as_ptr()
            );
        }
        res
    }
    
    /// Return the quotient and remainder of self/other rounded up towards 
    /// infinity.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let x = Integer::from(11);
    /// let y = Integer::from(2);
    /// let (q, r) = x.cdiv_qr(y);
    /// assert_eq!(q, 6);
    /// assert_eq!(r, -1);
    /// ```
    #[inline]
    pub fn cdiv_qr<T>(&self, other: T) -> (Integer, Integer)
    where 
        T: AsRef<Integer> 
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        unsafe {
            let mut q = Integer::default();
            let mut r = Integer::default();
            fmpz::fmpz_cdiv_qr(
                q.as_mut_ptr(), 
                r.as_mut_ptr(),
                self.as_ptr(), 
                other.as_ptr()
            );
            (q, r)
        }
    }
    
    /// Return the quotient self/other rounded up towards infinity.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let x = Integer::from(11);
    /// let y = Integer::from(2);
    /// assert_eq!(x.cdiv_q(y), 6);
    /// ```
    #[inline]
    pub fn cdiv_q<T>(&self, other: T) -> Integer 
    where 
        T: AsRef<Integer> 
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        unsafe {
            let mut res = Integer::default();
            fmpz::fmpz_cdiv_q(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }
    
    /// Compute the quotient self/other rounded up towards infinity and assign
    /// it to the input.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let mut x = Integer::from(11);
    /// let y = Integer::from(2);
    /// x.cdiv_q_assign(y);
    /// assert_eq!(x, 6);
    /// ```
    #[inline]
    pub fn cdiv_q_assign<T>(&mut self, other: T) 
    where 
        T: AsRef<Integer> 
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        unsafe {
            fmpz::fmpz_cdiv_q(self.as_mut_ptr(), self.as_ptr(), other.as_ptr());
        }
    }
    
    /// Return the quotient and remainder of self/other rounded down towards
    /// negative infinity.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let x = Integer::from(11);
    /// let y = Integer::from(2);
    /// let (q, r) = x.fdiv_qr(y);
    /// assert_eq!(q, 5);
    /// assert_eq!(r, 1);
    /// ```
    #[inline]
    pub fn fdiv_qr<T>(&self, other: T) -> (Integer, Integer)
    where 
        T: AsRef<Integer> 
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        unsafe {
            let mut q = Integer::default();
            let mut r = Integer::default();
            fmpz::fmpz_fdiv_qr(
                q.as_mut_ptr(), 
                r.as_mut_ptr(),
                self.as_ptr(), 
                other.as_ptr()
            );
            (q, r)
        }
    }
    
    /// Return the quotient self/other rounded down towards negative infinity.
    ///
    /// ```
    /// use inertia_core::*;
    ///
    /// let x = Integer::from(11);
    /// let y = Integer::from(2);
    /// assert_eq!(x.fdiv_q(y), 5);
    /// ```
    #[inline]
    pub fn fdiv_q<T>(&self, other: T) -> Integer 
    where 
        T: AsRef<Integer> 
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        unsafe {
            let mut res = Integer::default();
            fmpz::fmpz_fdiv_q(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }
    
    /// Return the remainder of the quotient self/other rounded down towards 
    /// negative infinity.
    ///
    /// ```
    /// use inertia_core::*;
    ///
    /// let x = Integer::from(11);
    /// let y = Integer::from(2);
    /// assert_eq!(x.fdiv_r(y), 1);
    /// ```
    #[inline]
    pub fn fdiv_r<T>(&self, other: T) -> Integer 
    where 
        T: AsRef<Integer> 
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        unsafe {
            let mut res = Integer::default();
            fmpz::fmpz_fdiv_r(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }
    
    /// Compute the quotient self/other rounded down towards negative infinity
    /// and assign it to the input.
    ///
    /// ```
    /// use inertia_core::*;
    ///
    /// let mut x = Integer::from(11);
    /// let y = Integer::from(2);
    /// x.fdiv_q_assign(y);
    /// assert_eq!(x, 5);
    /// ```
    #[inline]
    pub fn fdiv_q_assign<T>(&mut self, other: T)
    where 
        T: AsRef<Integer> 
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        unsafe {
            fmpz::fmpz_fdiv_q(self.as_mut_ptr(), self.as_ptr(), other.as_ptr());
        }
    }
 
    /// Return the quotient and remainder of self/other rounded towards zero.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let x = Integer::from(-19);
    /// let y = Integer::from(10);
    /// let (q, r) = x.tdiv_qr(y);
    /// assert_eq!(q, -1);
    /// assert_eq!(r, -9);
    /// ```
    #[inline]
    pub fn tdiv_qr<T>(&self, other: T) -> (Integer, Integer)
    where 
        T: AsRef<Integer> 
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        unsafe {
            let mut q = Integer::default();
            let mut r = Integer::default();
            fmpz::fmpz_tdiv_qr(
                q.as_mut_ptr(), 
                r.as_mut_ptr(),
                self.as_ptr(), 
                other.as_ptr()
            );
            (q, r)
        }
    }
    
    /// Return the quotient self/other rounded towards zero.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let x = Integer::from(-19);
    /// let y = Integer::from(10);
    /// assert_eq!(x.tdiv_q(y), -1);
    /// ```
    #[inline]
    pub fn tdiv_q<T>(&self, other: T) -> Integer 
    where 
        T: AsRef<Integer> 
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        unsafe {
            let mut res = Integer::default();
            fmpz::fmpz_tdiv_q(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }
    
    /// Compute the quotient self/other rounded towards zero and assign
    /// it to the input.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let mut x = Integer::from(-19);
    /// let y = Integer::from(10);
    /// x.tdiv_q_assign(y);
    /// assert_eq!(x, -1);
    /// ```
    #[inline]
    pub fn tdiv_q_assign<T>(&mut self, other: T) 
    where 
        T: AsRef<Integer> 
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        unsafe {
            fmpz::fmpz_tdiv_q(self.as_mut_ptr(), self.as_ptr(), other.as_ptr());
        }
    }
    
    /// Return the quotient and remainder of self/other rounded towards the 
    /// nearest integer.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let x = Integer::from(11);
    /// let y = Integer::from(3);
    /// let (q, r) = x.ndiv_qr(y);
    /// assert_eq!(q, 4);
    /// assert_eq!(r, -1);
    /// ```
    #[inline]
    pub fn ndiv_qr<T>(&self, other: T) -> (Integer, Integer)
    where 
        T: AsRef<Integer> 
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        unsafe {
            let mut q = Integer::default();
            let mut r = Integer::default();
            fmpz::fmpz_ndiv_qr(
                q.as_mut_ptr(), 
                r.as_mut_ptr(),
                self.as_ptr(), 
                other.as_ptr()
            );
            (q, r)
        }
    }
    
    // fdiv_q_ui/si, fdiv_q_2exp, fdiv_r_2exp
    // tdiv_q_ui/si, tdiv_q_2exp, tdiv_r_2exp etc.
    
    /// Return an option containing the quotient of self and h if the division is 
    /// exact.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(15);
    /// assert_eq!(z.divexact(Integer::from(3)).unwrap(), 5);
    /// assert!(z.divexact(Integer::from(2)).is_none());
    /// ```
    #[inline]
    pub fn divexact<T>(&self, x: T) -> Option<Integer>
    where
        T: AsRef<Integer>,
    {
        assert!(!x.as_ref().is_zero());
        let (q, r) = self.tdiv_qr(x);
        if r.is_zero() {
            Some(q)
        } else {
            None
        }
    }

    /// Return the quotient of self and h assuming the division is exact.
    /// FLINT may raise an exception if the division is not exact or x is not 0.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(15);
    /// assert_eq!(z.divexact_unchecked(Integer::from(3)), 5);
    /// ```
    #[inline]
    pub fn divexact_unchecked<T>(&self, x: T) -> Integer
    where
        T: AsRef<Integer>,
    {
        assert!(!x.as_ref().is_zero());
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_divexact(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                x.as_ref().as_ptr()
            );
        }
        res
    }
    
    /// Set the input to the quotient of itself and h, assuming the division is 
    /// exact. FLINT may raise an exception if the division is not exact or x is 
    /// not 0.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let mut z = Integer::from(15);
    /// z.divexact_unchecked_assign(Integer::from(3));
    /// assert_eq!(z, 5);
    /// ```
    #[inline]
    pub fn divexact_unchecked_assign<T>(&mut self, x: T)
    where
        T: AsRef<Integer>,
    {
        assert!(!x.as_ref().is_zero());
        unsafe {
            fmpz::fmpz_divexact(
                self.as_mut_ptr(), 
                self.as_ptr(), 
                x.as_ref().as_ptr()
            );
        }
    }
    
    // divexact_si
    // divexact_ui
    // divexact2_uiui

    /// Return true if self is divisible by `x`, false otherwise
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(6);
    /// assert!(z.divisible(Integer::new(3)));
    /// assert!(!z.divisible(Integer::zero()));
    /// ```
    #[inline]
    pub fn divisible<T>(&self, x: T) -> bool
    where
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_divisible(
                self.as_ptr(), 
                x.as_ref().as_ptr()
            ) == 1
        }
    }
    
    /// Return true if self is divisible by the signed integer `x`, false 
    /// otherwise
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(6);
    /// assert!(z.divisible_si(3i16));
    /// //assert!(!z.divisible_si(0));
    /// ```
    #[inline]
    pub fn divisible_si<T>(&self, x: T) -> bool
    where
        T: Into<i64>,
    {
        unsafe {
            fmpz::fmpz_divisible_si(self.as_ptr(), x.into()) == 1
        }
    }
    
    /// Return true if self divides `x`, false otherwise.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(3);
    /// assert!(z.divides(Integer::new(12)));
    /// assert!(!z.divides(Integer::one()));
    /// ```
    #[inline]
    pub fn divides<T>(&self, x: T) -> bool
    where
        T: AsRef<Integer>,
    {
        /*
        unsafe {
            fmpz::fmpz_divides(
                self.as_ptr(), 
                x.as_ref().as_ptr()
            ) == 1
        }
        */
        x.as_ref().divisible(self)
    }
    
    /// Return the signed remainder of self/x symmetric around 0.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(3);
    /// assert_eq!(z.srem(Integer::new(5)), -2);
    /// ```
    #[inline]
    pub fn srem<T>(&self, x: T) -> Integer
    where
        T: AsRef<Integer>,
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_smod(
                res.as_mut_ptr(),
                self.as_ptr(), 
                x.as_ref().as_ptr()
            );
        }
        res
    }
    
    /// Set self to the signed remainder of self/x symmetric around 0.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let mut z = Integer::new(3);
    /// z.srem_assign(Integer::new(5));
    /// assert_eq!(z, -2);
    /// ```
    #[inline]
    pub fn srem_assign<T>(&mut self, x: T)
    where
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_smod(
                self.as_mut_ptr(),
                self.as_ptr(), 
                x.as_ref().as_ptr()
            );
        }
    }
   
    /// Return self^x mod modulus.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// assert_eq!(Integer::new(5).powm(Integer::new(2), Integer::new(3)), 1);
    /// ```
    #[inline]
    pub fn powm<T>(&self, x: T, modulus: T) -> Integer
    where
        T: AsRef<Integer>,
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_powm(
                res.as_mut_ptr(),
                self.as_ptr(), 
                x.as_ref().as_ptr(),
                modulus.as_ref().as_ptr()
            );
        }
        res
    }
    
    /// Set self to self^x mod modulus.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let mut z = Integer::new(5);
    /// z.powm_assign(Integer::new(2), Integer::new(3));
    /// assert_eq!(z, 1);
    /// ```
    #[inline]
    pub fn powm_assign<T>(&mut self, x: T, modulus: T)
    where
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_powm(
                self.as_mut_ptr(),
                self.as_ptr(), 
                x.as_ref().as_ptr(),
                modulus.as_ref().as_ptr()
            );
        }
    }
    
    /// Return self^x mod modulus where x fits in an unsigned long.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// assert_eq!(Integer::new(5).powm_ui(2u64, Integer::new(3)), 1);
    /// ```
    #[inline]
    pub fn powm_ui<S, T>(&self, x: S, modulus: T) -> Integer
    where
        S: Into<u64>,
        T: AsRef<Integer>,
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_powm_ui(
                res.as_mut_ptr(),
                self.as_ptr(), 
                x.into(),
                modulus.as_ref().as_ptr()
            );
        }
        res
    }
    
    /// Set self to self^x mod modulus.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let mut z = Integer::new(5);
    /// z.powm_ui_assign(2u64, Integer::new(3));
    /// assert_eq!(z, 1);
    /// ```
    #[inline]
    pub fn powm_ui_assign<S, T>(&mut self, x: S, modulus: T)
    where
        S: Into<u64>,
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_powm_ui(
                self.as_mut_ptr(),
                self.as_ptr(), 
                x.into(),
                modulus.as_ref().as_ptr()
            );
        }
    }
   
    /// Return the logarithm of `self` with base `b` rounded up to the nearest 
    /// integer. Assumes the result fits in a signed long.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(100);
    /// assert_eq!(z.clog(Integer::new(3)), 5);
    /// ```
    #[inline]
    pub fn clog<T>(&self, b: T) -> i64
    where
        T: AsRef<Integer>
    {
        assert!(self >= &1);
        assert!(b.as_ref() >= &2);

        unsafe {
            fmpz::fmpz_clog(self.as_ptr(), b.as_ref().as_ptr())
        }
    }
    
    /// Return the logarithm of `self` with unsigned long base `b` rounded up to 
    /// the nearest integer. Assumes the result fits in a signed long.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(100);
    /// assert_eq!(z.clog_ui(3u32), 5);
    /// ```
    #[inline]
    pub fn clog_ui<S>(&self, b: S) -> i64
    where
        S: Into<u64>
    {
        assert!(self >= &1);

        let b = b.into();
        assert!(b >= 2);

        unsafe {
            fmpz::fmpz_clog_ui(self.as_ptr(), b)
        }
    }
    
    /// Return the logarithm of `self` with base `b` rounded down to the nearest 
    /// integer. Assumes the result fits in a signed long.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(100);
    /// assert_eq!(z.flog(Integer::new(3)), 4);
    /// ```
    #[inline]
    pub fn flog<T>(&self, b: T) -> i64
    where
        T: AsRef<Integer>
    {
        assert!(self >= &1);
        assert!(b.as_ref() >= &2);

        unsafe {
            fmpz::fmpz_flog(self.as_ptr(), b.as_ref().as_ptr())
        }
    }
    
    /// Return the logarithm of `self` with unsigned long base `b` rounded down to 
    /// the nearest integer. Assumes the result fits in a signed long.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(100);
    /// assert_eq!(z.flog_ui(3u32), 4);
    /// ```
    #[inline]
    pub fn flog_ui<S>(&self, b: S) -> i64
    where
        S: Into<u64>
    {
        assert!(self >= &1);

        let b = b.into();
        assert!(b >= 2);

        unsafe {
            fmpz::fmpz_flog_ui(self.as_ptr(), b)
        }
    }
    
    /// Return the integer part of the square root of `self`.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(7);
    /// assert_eq!(z.sqrt(), 2);
    /// ```
    #[inline]
    pub fn sqrt(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_sqrt(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
    
    /// Set `self` to the integer part its square root.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let mut z = Integer::new(7);
    /// z.sqrt_assign();
    /// assert_eq!(z, 2);
    /// ```
    #[inline]
    pub fn sqrt_assign(&mut self) {
        unsafe {
            fmpz::fmpz_sqrt(self.as_mut_ptr(), self.as_ptr());
        }
    }
   
    /// If `p` is prime, return an `Option` with the the square root of `self` 
    /// modulo `p` if `self` is a quadratic residue modulo `p`, otherwise `None`. 
    /// If `p` is not prime the return value is with high probability `None`, 
    /// indicating that `p` is not prime, or is not a square modulo `p`. If `p` 
    /// is not prime and the return value is not `None`, the value is meaningless.
    ///
    /// Note: The quadratic residue is well-defined for composite modulus, this
    /// is a limitation of FLINTs algorithm (which likely avoids factorization).
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(12);
    /// assert_eq!(z.sqrtmod(Integer::new(13)).unwrap(), 5);
    /// ```
    #[inline]
    pub fn sqrtmod<T>(&self, p: T) -> Option<Integer>
    where
        T: AsRef<Integer>
    {
        let mut res = Integer::default();
        unsafe {
            let b = fmpz::fmpz_sqrtmod(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                p.as_ref().as_ptr()
            );
            if b == 1 {
                Some(res)
            } else {
                None
            }
        }
    }
 
    /// Return `f`, the integer part of the square root of `self`, and the remainder
    /// `r`, that is, the difference `self - f^2`. Requires `self` to be non-negative.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(12);
    /// let (f, r) = z.sqrtrem();
    /// assert_eq!(f, 3);
    /// assert_eq!(r, 3);
    /// ```
    #[inline]
    pub fn sqrtrem(&self) -> (Integer, Integer) {
        assert!(self >= &0);
        let mut f = Integer::default();
        let mut r = Integer::default();
        unsafe {
            fmpz::fmpz_sqrtrem(
                f.as_mut_ptr(), 
                r.as_mut_ptr(), 
                self.as_ptr()
            );
        }
        (f, r)
    }

    /// Return `true` if `self` is a perfect square, `false` otherwise.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(16);
    /// assert!(z.is_square());
    ///
    /// let z = Integer::new(17);
    /// assert!(!z.is_square());
    /// ```
    #[inline]
    pub fn is_square(&self) -> bool {
        unsafe { !(fmpz::fmpz_is_square(self.as_ptr()) == 0) }
    }

    /// Return the integer part of the `n`-th root of `self`. Requires that `n > 0`
    /// and if `n` is even then `self` is non-negative.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(37);
    /// assert_eq!(z.root(4), 2);
    ///
    /// // fails on:
    /// // assert_eq!(z.root(4), -2);
    /// //let z = -z;
    /// //assert_eq!(z.root(4), -2);
    /// ```
    #[inline]
    pub fn root<S>(&self, n: S) -> Integer
    where
        S: Into<i64>
    {
        let n = n.into();
        assert!(n > 0);
        if (n % 2) == 0 {
            assert!(self >= &0);
        }

        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_root(res.as_mut_ptr(), self.as_ptr(), n);
        }
        res
    }
    
    /// Set `self` to the integer part of the `n`-th root of `self`. Requires that 
    /// `n > 0` and if `n` is even then `self` is non-negative.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let mut z = Integer::new(37);
    /// z.root_assign(4);
    /// assert_eq!(z, 2);
    /// ```
    #[inline]
    pub fn root_assign<S>(&mut self, n: S)
    where
        S: Into<i64>
    {
        let n = n.into();
        assert!(n > 0);
        if (n % 2) == 0 {
            assert!(&*self >= &0);
        }

        unsafe {
            fmpz::fmpz_root(self.as_mut_ptr(), self.as_ptr(), n);
        }
    }
    
    /// If `self` is a perfect power `r^k` return `(r, k)`, otherwise `None`. 
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let mut z = Integer::new(32);
    /// assert_eq!(z.is_perfect_power().unwrap(), (Integer::new(2), 5));
    /// ```
    #[inline]
    pub fn is_perfect_power(&self) -> Option<(Integer, i32)> {
        let mut r = Integer::default();
        unsafe {
            let k = fmpz::fmpz_is_perfect_power(r.as_mut_ptr(), self.as_ptr());
            if k == 0 {
                None
            } else {
                Some((r, k))
            }
        }
    }

    /// Return the factorial `n!` where `n` is an unsigned long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// assert_eq!(Integer::fac_ui(3u32), 6);
    /// ```
    #[inline]
    pub fn fac_ui<S>(n: S) -> Integer 
    where
        S: Into<u64>
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_fac_ui(res.as_mut_ptr(), n.into());
        }
        res
    }

    /// Return the factorial `n!` where `n` is an unsigned long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// assert_eq!(Integer::factorial(3u32), 6);
    /// ```
    #[inline]
    pub fn factorial<S>(n: S) -> Integer 
    where
        S: Into<u64>
    {
        Integer::fac_ui(n)
    }
    
    /// Return the Fibonacci number `F_n` where `n` is an unsigned long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// assert_eq!(Integer::fib_ui(11u32), 89);
    /// ```
    #[inline]
    pub fn fib_ui<S>(n: S) -> Integer 
    where
        S: Into<u64>
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_fib_ui(res.as_mut_ptr(), n.into());
        }
        res
    }
    
    /// Return the Fibonacci number `F_n` where `n` is an unsigned long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// assert_eq!(Integer::fibonacci(11u32), 89);
    /// ```
    #[inline]
    pub fn fibonacci<S>(n: S) -> Integer 
    where
        S: Into<u64>
    {
        Integer::fib_ui(n)
    }
    
    /// Return the binomial coefficient `nCk`.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// assert_eq!(Integer::bin_uiui(11u32, 4u32), 330);
    /// ```
    #[inline]
    pub fn bin_uiui<S>(n: S, k: S) -> Integer 
    where
        S: Into<u64>
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_bin_uiui(res.as_mut_ptr(), n.into(), k.into());
        }
        res
    }
    
    /// Return the binomial coefficient `nCk`.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// assert_eq!(Integer::binomial(11u32, 4u32), 330);
    /// ```
    #[inline]
    pub fn binomial<S>(n: S, k: S) -> Integer 
    where
        S: Into<u64>
    {
        Integer::bin_uiui(n, k)
    }

    /// Return the rising factorial `x(x + 1)(x + 2)...(x + k - 1)` (`self` = `x`).
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(3);
    /// assert_eq!(z.rfac_ui(3u32), 60);
    /// ```
    #[inline]
    pub fn rfac_ui<S>(&self, k: S) -> Integer 
    where
        S: Into<u64>
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_rfac_ui(res.as_mut_ptr(), self.as_ptr(), k.into());
        }
        res
    }
    
    /// Return the rising factorial `x(x + 1)(x + 2)...(x + k - 1)` (`self` = `x`).
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(3);
    /// assert_eq!(z.rising_factorial(3u32), 60);
    /// ```
    #[inline]
    pub fn rising_factorial<S>(&self, k: S) -> Integer 
    where
        S: Into<u64>
    {
        self.rfac_ui(k)
    }
    
    /// Return the rising factorial `x(x + 1)(x + 2)...(x + k - 1)`.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// assert_eq!(Integer::rfac_uiui(3u32, 3u32), 60);
    /// ```
    #[inline]
    pub fn rfac_uiui<S>(x: S, k: S) -> Integer 
    where
        S: Into<u64>
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_rfac_uiui(res.as_mut_ptr(), x.into(), k.into());
        }
        res
    }
   
    /* TODO: fix signature in flint-sys
    /// Return the product of `self` and `h` divided by `2^exp` rounded down towards
    /// zero.
    ///
    /// ```
    /// use inertia_core::{Integer, New};
    ///
    /// let z = Integer::new(3);
    /// assert_eq!(z.mul_tdiv_q_2exp(Integer::new(2), 2u32), 30);
    /// ```
    #[inline]
    pub fn mul_tdiv_q_2exp<S, T>(&self, h: T, exp: S) -> Integer 
    where
        S: Into<u64>,
        T: AsRef<Integer>
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_mul_tdiv_q_2exp(
                res.as_mut_ptr(), 
                self.as_ref().as_ptr(), 
                h.as_ref().as_ptr(), 
                exp.into()
            );
        }
        res
    }
    */

    // mul_si_tdiv_q_2exp

    // Greatest common divisor //

    #[inline]
    pub fn gcd<T>(&self, other: T) -> Integer 
    where
        T: AsRef<Integer>
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_gcd(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ref().as_ptr()
            );
        }
        res
    }

    // gcd_ui
    // gcd3

    #[inline]
    pub fn lcm<T>(&self, other: T) -> Integer
    where
        T: AsRef<Integer>
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_lcm(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ref().as_ptr()
            );
        }
        res
    }

    // gcdinv

    #[inline]
    pub fn xgcd<T>(&self, other: T) -> (Integer, Integer, Integer) 
    where
        T: AsRef<Integer>
    {
        let mut d = Integer::default();
        let mut a = Integer::default();
        let mut b = Integer::default();
        unsafe {
            fmpz::fmpz_xgcd(
                d.as_mut_ptr(), 
                a.as_mut_ptr(), 
                b.as_mut_ptr(),
                self.as_ptr(), 
                other.as_ref().as_ptr()
            );
        }
        (d, a, b)
    } 

    // xgcd_canonical_bezout
    // xgcd_partial
    
    // Modular arithmetic //

    // remove

    /// Attempt to invert `self` modulo `modulus`.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(4);
    /// assert_eq!(z.invmod(Integer::from(7)).unwrap(), 2);
    /// ```
    #[inline]
    pub fn invmod<T>(&self, modulus: T) -> Option<Integer>
    where
        T: AsRef<Integer>,
    {
        let modulus = modulus.as_ref();
        assert!(modulus > &0);

        let mut res = Integer::default();
        unsafe {
            let r = fmpz::fmpz_invmod(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                modulus.as_ptr()
            );

            if r == 0 {
                None
            } else {
                Some(res)
            }
        }
    }

    // negmod
    // jacobi
    // kronecker
    // divides_mod_list

    // Bit packing //

    // bit_pack
    // bit_unpack

    // Logic operations //

    // complement
    // clrbit
    // combit
    // popcnt

    // Chinese remaindering //

    // crt_ui
    // crt
    // multi_crt

    // Primality testing //

    /// Returns true if `self` is a prime.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let a = Integer::from(3);
    /// assert!(a.is_prime());
    ///
    /// let b = Integer::from(6);
    /// assert!(!b.is_prime());
    /// ```
    #[inline]
    pub fn is_prime(&self) -> bool {
        unsafe { fmpz::fmpz_is_prime(self.as_ptr()) == 1 }
    }
   
    /*
    #[inline]
    pub fn reconstruct(&self, modulus: T) -> Rational
    where
        T: AsRef<Integer>
    {
        let mut res = Rational::default();
        unsafe {
            fmpq::fmpq_reconstruct_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                modulus.as_ptr()
            );
        }
        res
    }
    
    #[inline]
    pub fn reconstruct_2(&self, modulus: T, n: T, d: T) -> Rational 
    where
        T: AsRef<Integer>
    {
        let mut res = Rational::default();
        unsafe {
            fmpq::fmpq_reconstruct_fmpz_2(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                modulus.as_ptr(),
                n.as_ptr(),
                d.as_ptr()
            );
        }
        res
    }
    */

    // Special functions //
}
