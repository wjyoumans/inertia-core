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

mod arith;
mod conv;

use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;

use crate::*;
use flint_sys::fmpz;
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};

#[derive(Clone, Copy, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct IntegerRing {}
pub type Integers = IntegerRing;

impl Eq for IntegerRing {}

impl PartialEq for IntegerRing {
    fn eq(&self, _rhs: &IntegerRing) -> bool {
        true
    }
}

impl fmt::Display for IntegerRing {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Integer ring")
    }
}

impl IntegerRing {
    /// Initialize an `IntegerRing`.
    ///
    /// ```
    /// use inertia_core::IntegerRing;
    ///
    /// let zz = IntegerRing::init();
    /// let z = zz.new(1);
    ///
    /// assert_eq!(z, 1);
    /// ```
    #[inline]
    pub fn init() -> Self {
        IntegerRing {}
    }

    /// Return the default value of the `IntegerRing`, zero.
    ///
    /// ```
    /// use inertia_core::IntegerRing;
    ///
    /// let zz = IntegerRing::init();
    /// let z = zz.default();
    ///
    /// assert_eq!(z, 0);
    /// ```
    #[inline]
    pub fn default(&self) -> Integer {
        Integer::default()
    }

    /// Initialize an Integer from an integer ring.
    ///
    /// ```
    /// use inertia_core::IntegerRing;
    ///
    /// let zz = IntegerRing::init();
    ///
    /// let z = zz.new(2);
    /// assert_eq!(z, 2);
    /// ```
    #[inline]
    pub fn new<T>(&self, x: T) -> Integer
    where
        T: Into<Integer>,
    {
        x.into()
    }
}

#[derive(Debug)]
pub struct Integer {
    inner: fmpz::fmpz,
}

impl AsRef<Integer> for Integer {
    fn as_ref(&self) -> &Integer {
        self
    }
}

impl<'a, T> Assign<T> for Integer
where
    T: AsRef<Integer>,
{
    fn assign(&mut self, other: T) {
        unsafe {
            fmpz::fmpz_set(self.as_mut_ptr(), other.as_ref().as_ptr());
        }
    }
}

impl Clone for Integer {
    #[inline]
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz::fmpz_init_set(z.as_mut_ptr(), self.as_ptr());
            Integer {
                inner: z.assume_init(),
            }
        }
    }
}

impl Default for Integer {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz::fmpz_init(z.as_mut_ptr());
            Integer {
                inner: z.assume_init(),
            }
        }
    }
}

impl Drop for Integer {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpz::fmpz_clear(self.as_mut_ptr()) }
    }
}

impl fmt::Display for Integer {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str_radix(10))
    }
}

impl Hash for Integer {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_ui_vector().hash(state);
    }
}

impl Integer {
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

    /// Instantiate an Integer from a [FLINT integer][fmpz::fmpz].
    #[inline]
    pub fn from_raw(raw: fmpz::fmpz) -> Integer {
        Integer { inner: raw }
    }

    /// Initialize a new Integer.
    #[inline]
    pub fn new<T>(x: T) -> Integer
    where
        T: Into<Integer>,
    {
        x.into()
    }

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
            Integer {
                inner: z.assume_init(),
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
            vector.set_len(len);

            fmpz::fmpz_get_str(vector.as_mut_ptr() as *mut _, 
                               base as i32, self.as_ptr());

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

    /// Return an `Option` containing the input as a signed long (`libc::c_long`)
    /// if possible.
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

    /// Return an `Option` containing the input as an unsigned long
    /// (`libc::c_ulong`) if possible.
    ///
    /// ```
    /// use inertia_core::Integer;
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
    /// use inertia_core::{
    ///     ops::Pow,
    ///     Integer
    /// };
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
    /// use inertia_core::{
    ///     ops::Pow,
    ///     Integer
    /// };
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
        Integer::from(1)
    }

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
    
    #[inline]
    pub fn zero_assign(&mut self) {
        unsafe { fmpz::fmpz_zero(self.as_mut_ptr()) }
    }
    
    #[inline]
    pub fn one_assign(&mut self) {
        unsafe { fmpz::fmpz_one(self.as_mut_ptr()) }
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

    /// Attempt to invert `self` modulo `modulus`.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(4);
    /// assert_eq!(z.invmod(7).unwrap(), 2);
    /// ```
    #[inline]
    pub fn invmod<'a, T>(&self, modulus: T) -> Option<Integer>
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

    /// Outputs `self * x * y` where `x, y` can be converted to unsigned longs.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let f = Integer::from(-1);
    /// assert_eq!(f.mul2_uiui(10, 3), -30i32);
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
    /// f.mul2_uiui_assign(10, 3);
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
    /// assert_eq!(g.mul_2exp(3), 16);
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
    /// g.mul_2exp_assign(3);
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

    /// Return `self + (x * y)`.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(2);
    /// assert_eq!(z.addmul(Integer::from(3), Integer::from(4)), 14);
    /// ```
    #[inline]
    pub fn addmul<'a, T>(&self, x: T, y: T) -> Integer
    where
        T: AsRef<Integer>,
    {
        let mut res = self.clone();
        unsafe {
            fmpz::fmpz_addmul(res.as_mut_ptr(), 
                              x.as_ref().as_ptr(), y.as_ref().as_ptr());
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
    pub fn addmul_assign<'a, T>(&mut self, x: T, y: T)
    where
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_addmul(self.as_mut_ptr(), x.as_ref().as_ptr(), 
                              y.as_ref().as_ptr());
        }
    }

    /// Return `self + (x * y)` where `y` can be converted to an unsigned long.
    ///
    /// ```
    /// use inertia_core::Integer;
    ///
    /// let z = Integer::from(2);
    /// assert_eq!(z.addmul_ui(Integer::from(3), 4), 14);
    /// ```
    #[inline]
    pub fn addmul_ui<'a, S, T>(&self, x: T, y: S) -> Integer
    where
        S: Into<u64>,
        T: AsRef<Integer>,
    {
        let mut res = self.clone();
        unsafe {
            fmpz::fmpz_addmul_ui(res.as_mut_ptr(), x.as_ref().as_ptr(), y.into());
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
    /// z.addmul_ui_assign(Integer::from(3), 4);
    /// assert_eq!(z, 14);
    /// ```
    #[inline]
    pub fn addmul_ui_assign<'a, S, T>(&mut self, x: T, y: S)
    where
        S: Into<u64>,
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_addmul_ui(self.as_mut_ptr(), x.as_ref().as_ptr(), y.into());
        }
    }
}

impl Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ui_vec = self.get_ui_vector();
        let mut seq = serializer.serialize_seq(Some(ui_vec.len()))?;
        for e in ui_vec.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct IntegerVisitor {}

impl IntegerVisitor {
    fn new() -> Self {
        IntegerVisitor {}
    }
}

impl<'de> Visitor<'de> for IntegerVisitor {
    type Value = Integer;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an Integer")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut vec_ui = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_element()? {
            vec_ui.push(x);
        }

        let mut out = Integer::default();
        out.set_ui_vector(vec_ui);
        Ok(out)
    }
}

impl<'de> Deserialize<'de> for Integer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(IntegerVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::Integer;

    #[test]
    fn serde() {
        let x: Integer = "18446744073709551616".parse().unwrap();
        let ser = bincode::serialize(&x).unwrap();
        let y: Integer = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}
