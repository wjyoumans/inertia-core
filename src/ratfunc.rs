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

//#[cfg(feature = "serde")]
//mod serde;

use crate::{New, IntPoly};
use flint_sys::{
    fmpz_poly::fmpz_poly_set,
    fmpz_poly_q::*
};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};

#[derive(Debug)]
pub struct RatFunc {
    inner: fmpz_poly_q_struct,
}

impl AsRef<RatFunc> for RatFunc {
    fn as_ref(&self) -> &RatFunc {
        self
    }
}

impl Clone for RatFunc {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = RatFunc::default();
        unsafe {
            fmpz_poly_q_set(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
}

impl Default for RatFunc {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_poly_q_init(z.as_mut_ptr());
            RatFunc::from_raw(z.assume_init())
        }
    }
}

// TODO: add parens to num or den if not constant, omit den if 1, etc
impl fmt::Display for RatFunc {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let den = self.denominator();
        if den.is_one() {
            write!(f, "{}", self.numerator())
        } else {
            let num = self.numerator();
            if num.degree() < 1 && den.degree() < 1 {
                write!(f, "{}/{}", num, den)
            } else if num.degree() < 1 {
                write!(f, "{}/({})", num, den)
            } else if den.degree() < 1 {
                write!(f, "({})/{}", num, den)
            } else {
                write!(f, "({})/({})", num, den)
            }
        }
    }
}

impl Drop for RatFunc {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpz_poly_q_clear(self.as_mut_ptr()) }
    }
}

impl Hash for RatFunc {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.numerator().hash(state);
        self.denominator().hash(state);
    }
}

impl<T: Into<RatFunc>> New<T> for RatFunc {
    #[inline]
    fn new(src: T) -> Self {
        src.into()
    }
}

impl New<&RatFunc> for RatFunc {
    #[inline]
    fn new(src: &RatFunc) -> Self {
        src.clone()
    }
}

impl RatFunc {
    #[inline]
    pub fn zero() -> RatFunc {
        RatFunc::default()
    }

    #[inline]
    pub fn one() -> RatFunc {
        let mut res = RatFunc::default();
        unsafe { fmpz_poly_q_one(res.as_mut_ptr()); }
        res
    }
    
    #[inline]
    pub fn zero_assign(&mut self) {
        unsafe { fmpz_poly_q_zero(self.as_mut_ptr()) }
    }
    
    #[inline]
    pub fn one_assign(&mut self) {
        unsafe { fmpz_poly_q_one(self.as_mut_ptr()) }
    }
    
    #[inline]
    pub const fn as_ptr(&self) -> *const fmpz_poly_q_struct {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpz_poly_q_struct {
        &mut self.inner
    }

    /*
    // TODO: safety?
    #[inline]
    pub unsafe fn as_slice<'a>(&'a self) -> &'a [fmpz::fmpz] {
        std::slice::from_raw_parts((*self.as_ptr()).coeffs, self.len())
    }
    
    // TODO: safety?
    #[inline]
    pub unsafe fn as_mut_slice<'a>(&'a mut self) -> &'a mut [fmpz::fmpz] {
        std::slice::from_raw_parts_mut((*self.as_ptr()).coeffs, self.len())
    }*/
    
    #[inline]
    pub const unsafe fn from_raw(inner: fmpz_poly_q_struct) -> RatFunc {
        RatFunc { inner }
    }
    
    #[inline]
    pub const fn into_raw(self) -> fmpz_poly_q_struct {
        let inner = self.inner;
        let _ = ManuallyDrop::new(self);
        inner
    }

    #[inline]
    pub fn numerator(&self) -> IntPoly {
        let mut res = IntPoly::zero();
        unsafe {
            fmpz_poly_set(res.as_mut_ptr(), self.inner.num)
        }
        res
    }

    #[inline]
    pub fn denominator(&self) -> IntPoly {
        let mut res = IntPoly::zero();
        unsafe {
            fmpz_poly_set(res.as_mut_ptr(), self.inner.den)
        }
        res
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { fmpz_poly_q_is_zero(self.as_ptr()) == 1 }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { fmpz_poly_q_is_one(self.as_ptr()) == 1}
    }
    
    #[inline]
    pub fn is_gen(&self) -> bool {
        self.denominator().is_one() && self.numerator().is_gen()
    }
   
    /*
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { 
            let len = fmpz_poly::fmpz_poly_length(self.as_ptr()); 
            len.try_into().expect("Cannot convert length to a usize.")
        }
    }

    #[inline]
    pub fn degree(&self) -> i64 {
        unsafe { fmpz_poly::fmpz_poly_degree(self.as_ptr()) }
    }

    pub fn get_coeff(&self, i: usize) -> Integer {
        let mut res = Integer::default();
        unsafe { 
            fmpz_poly::fmpz_poly_get_coeff_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                i.try_into().expect("Cannot convert index to a signed long.")
            )
        }
        res
    }
   
    // Check coeff fits ui
    #[inline]
    pub unsafe fn get_coeff_ui(&self, i: usize) -> u64 {
        fmpz_poly::fmpz_poly_get_coeff_ui(
            self.as_ptr(), 
            i.try_into().expect("Cannot convert index to a signed long.")
        )
    }
    
    // Check coeff fits si
    pub unsafe fn get_coeff_si(&self, i: usize) -> i64 {
        fmpz_poly::fmpz_poly_get_coeff_si(
            self.as_ptr(), 
            i.try_into().expect("Cannot convert index to a signed long.")
        )
    }
    
    pub fn set_coeff<T: AsRef<Integer>>(&mut self, i: usize, coeff: T) {
        unsafe {
            fmpz_poly::fmpz_poly_set_coeff_fmpz(
                self.as_mut_ptr(),                                 
                i.try_into().expect("Cannot convert index to a signed long."), 
                coeff.as_ref().as_ptr()
            );
        }
    }
    
    pub fn set_coeff_ui<T>(&mut self, i: usize, coeff: T)
    where
        T: TryInto<u64>,
        <T as TryInto<u64>>::Error: fmt::Debug
    {
        unsafe {
            fmpz_poly::fmpz_poly_set_coeff_ui(
                self.as_mut_ptr(),                                 
                i.try_into().expect("Cannot convert index to a signed long."), 
                coeff.try_into().expect("Cannot convert coeff to an usigned long.")
            );
        }
    }
    
    pub fn set_coeff_si<T>(&mut self, i: usize, coeff: T)
    where
        T: TryInto<i64>,
        <T as TryInto<i64>>::Error: fmt::Debug
    {
        unsafe {
            fmpz_poly::fmpz_poly_set_coeff_si(
                self.as_mut_ptr(),                                 
                i.try_into().expect("Cannot convert index to a signed long."), 
                coeff.try_into().expect("Cannot convert coeff to a signed long.")
            );
        }
    }

    // TODO: anything better?
    #[inline]
    pub fn get_coeffs(&self) -> Vec<Integer> {
        let mut res = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            res.push(self.get_coeff(i))
        }
        res
    }
    */
}

