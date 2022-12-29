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

use crate::{New, Integer};
use flint_sys::fmpz_poly;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};

#[derive(Debug)]
pub struct IntPoly {
    inner: fmpz_poly::fmpz_poly_struct,
}

impl AsRef<IntPoly> for IntPoly {
    fn as_ref(&self) -> &IntPoly {
        self
    }
}

impl Clone for IntPoly {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = IntPoly::default();
        unsafe {
            fmpz_poly::fmpz_poly_set(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
}

impl Default for IntPoly {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_poly::fmpz_poly_init(z.as_mut_ptr());
            IntPoly::from_raw(z.assume_init())
        }
    }
}

// Note: Flint `get_str_pretty` doesnt space between terms.
impl fmt::Display for IntPoly {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let deg = self.degree();
        if deg < 0 {
            return write!(f, "0");
        } else if deg == 0 {
            return write!(f, "{}", self.get_coeff(0).to_string());
        }

        let deg = deg.try_into().unwrap();
        let mut out = String::new();
        let coeffs = self.get_coeffs();

        let sign = |s| {
            if s > 0 { " + " }
            else if s < 0 { " - " }
            else { unreachable!() }
        };
       
        for (k, c) in coeffs.iter().enumerate().rev() {
            let s = c.sign();
            if s == 0 {
                continue;
            }

            let abs = c.abs();
            if k == 0 {
                out.push_str(&format!("{}{}", sign(s), abs));
            } else if k == deg {
                if abs.is_one() && s > 0 {
                    if k == 1 {
                        out.push_str("x")
                    } else {
                        out.push_str(&format!("x^{}", k));
                    }
                } else if abs.is_one() && s < 0 {
                    if k == 1 {
                        out.push_str("-x")
                    } else {
                        out.push_str(&format!("-x^{}", k));
                    }
                } else {
                    if k == 1 {
                        out.push_str(&format!("{}*x", c));
                    } else {
                        out.push_str(&format!("{}*x^{}", c, k));
                    }
                }
            } else if k == 1 {
                if abs.is_one() {
                    out.push_str(&format!("{}x", sign(s)));
                } else {
                    out.push_str(&format!("{}{}*x", sign(s), abs));
                }
            } else {
                if abs.is_one() {
                    out.push_str(&format!("{}x^{}", sign(s), k));
                } else {
                    out.push_str(&format!("{}{}*x^{}", sign(s), abs, k));
                }
            }
        }
        write!(f, "{}", out)
    }
}

impl Drop for IntPoly {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpz_poly::fmpz_poly_clear(self.as_mut_ptr()) }
    }
}

impl Hash for IntPoly {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_coeffs().hash(state);
    }
}

impl<T: Into<IntPoly>> New<T> for IntPoly {
    #[inline]
    fn new(src: T) -> Self {
        src.into()
    }
}

impl New<&IntPoly> for IntPoly {
    #[inline]
    fn new(src: &IntPoly) -> Self {
        src.clone()
    }
}

impl IntPoly {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_poly::fmpz_poly_init2(
                z.as_mut_ptr(), 
                capacity.try_into().expect("Cannot convert input to a signed long.")
            );
            IntPoly::from_raw(z.assume_init())
        }
    }

    #[inline]
    pub fn zero() -> IntPoly {
        IntPoly::default()
    }

    #[inline]
    pub fn one() -> IntPoly {
        let mut res = IntPoly::default();
        unsafe { fmpz_poly::fmpz_poly_one(res.as_mut_ptr()); }
        res
    }
    
    #[inline]
    pub fn zero_assign(&mut self) {
        unsafe { fmpz_poly::fmpz_poly_zero(self.as_mut_ptr()) }
    }
    
    #[inline]
    pub fn one_assign(&mut self) {
        unsafe { fmpz_poly::fmpz_poly_one(self.as_mut_ptr()) }
    }
    
    #[inline]
    pub const fn as_ptr(&self) -> *const fmpz_poly::fmpz_poly_struct {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpz_poly::fmpz_poly_struct {
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
    pub const unsafe fn from_raw(inner: fmpz_poly::fmpz_poly_struct) -> IntPoly {
        IntPoly { inner }
    }
    
    #[inline]
    pub const fn into_raw(self) -> fmpz_poly::fmpz_poly_struct {
        let inner = self.inner;
        let _ = ManuallyDrop::new(self);
        inner
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == 0
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { fmpz_poly::fmpz_poly_is_one(self.as_ptr()) == 1}
    }

    #[inline]
    pub fn is_unit(&self) -> bool {
        unsafe { fmpz_poly::fmpz_poly_is_unit(self.as_ptr()) == 1}
    }
    
    #[inline]
    pub fn is_gen(&self) -> bool {
        unsafe { fmpz_poly::fmpz_poly_is_gen(self.as_ptr()) == 1}
    }
    
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
}

