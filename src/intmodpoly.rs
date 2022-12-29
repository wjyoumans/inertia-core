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

use crate::*;

use flint_sys::{fmpz, fmpz_mod, fmpz_mod_poly};

use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct IntModPolyRing {
    inner: Rc<FmpzModCtx>
}

impl Eq for IntModPolyRing {}

impl PartialEq for IntModPolyRing {
    fn eq(&self, rhs: &IntModPolyRing) -> bool {
        Rc::ptr_eq(&self.inner, &rhs.inner) || (self.modulus() == rhs.modulus())
    }
}

impl fmt::Display for IntModPolyRing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ring of polynomials over {}", self.base_ring())
    }
}

impl Hash for IntModPolyRing {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.modulus().hash(state)
    }
}

impl IntModPolyRing {
    #[inline]
    pub fn init<T: Into<Integer>>(modulus: T) -> Self {
        IntModPolyRing {
            inner: Rc::new(FmpzModCtx::new(modulus.into()))
        }
    }
    
    pub fn new<T: Into<IntPoly>>(&self, value: T) -> IntModPoly {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_mod_poly::fmpz_mod_poly_init(z.as_mut_ptr(), self.as_ptr());
            fmpz_mod_poly::fmpz_mod_poly_set_fmpz_poly(
                z.as_mut_ptr(), 
                value.into().as_ptr(),
                self.as_ptr()
            );
            IntModPoly::from_raw(z.assume_init(), self.clone())
        }

    }
    
    pub fn with_capacity(&self, capacity: usize) -> IntModPoly {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_mod_poly::fmpz_mod_poly_init2(
                z.as_mut_ptr(),
                capacity.try_into().expect("Cannot convert input to a signed long."),
                self.as_ptr()
            );
            IntModPoly::from_raw(z.assume_init(), self.clone())
        }
    }
    
    #[inline]
    pub fn zero(&self) -> IntModPoly {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_mod_poly::fmpz_mod_poly_init(z.as_mut_ptr(), self.as_ptr());
            IntModPoly::from_raw(z.assume_init(), self.clone())
        }
    }

    #[inline]
    pub fn one(&self) -> IntModPoly {
        let mut res = self.zero();
        unsafe{ fmpz_mod_poly::fmpz_mod_poly_one(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
    
    #[inline]
    pub fn base_ring(&self) -> IntModRing {
        IntModRing { inner: Rc::clone(&self.inner) }
    }
    
    #[inline]
    pub fn as_ptr(&self) -> *const fmpz_mod::fmpz_mod_ctx_struct {
        &self.inner.0
    }
    
    #[inline]
    pub fn modulus_as_ptr(&self) -> *const fmpz::fmpz {
        unsafe { fmpz_mod::fmpz_mod_ctx_modulus(self.as_ptr()) }
    }
   
    #[inline]
    pub fn modulus(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { fmpz::fmpz_set(res.as_mut_ptr(), self.modulus_as_ptr()); }
        res
    }
    
}

#[derive(Debug)]
pub struct IntModPoly {
    inner: fmpz_mod_poly::fmpz_mod_poly_struct,
    parent: IntModPolyRing,
}

impl AsRef<IntModPoly> for IntModPoly {
    #[inline]
    fn as_ref(&self) -> &IntModPoly {
        self
    }
}

impl Clone for IntModPoly {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = self.parent().zero();
        unsafe {
            fmpz_mod_poly::fmpz_mod_poly_set(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                self.ctx_as_ptr()
            );
        }
        res
    }
}

// TODO: avoid allocation
impl fmt::Display for IntModPoly {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", IntPoly::from(self))
    }
}

impl Drop for IntModPoly {
    #[inline]
    fn drop(&mut self) {
        unsafe { 
            fmpz_mod_poly::fmpz_mod_poly_clear(self.as_mut_ptr(), self.ctx_as_ptr())
        }
    }
}

// TODO: avoid IntPoly allocation
impl Hash for IntModPoly {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent().hash(state);
        IntPoly::from(self).hash(state);
    }
}

impl IntModPoly {
    #[inline]
    pub const fn as_ptr(&self) -> *const fmpz_mod_poly::fmpz_mod_poly_struct {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpz_mod_poly::fmpz_mod_poly_struct {
        &mut self.inner
    }

    #[inline]
    pub fn ctx_as_ptr(&self) -> *const fmpz_mod::fmpz_mod_ctx_struct {
        self.parent().as_ptr()
    }
    
    #[inline]
    pub fn modulus_as_ptr(&self) -> *const fmpz::fmpz {
        self.parent().modulus_as_ptr()
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
    pub const unsafe fn from_raw(
        inner: fmpz_mod_poly::fmpz_mod_poly_struct, 
        parent: IntModPolyRing
    ) -> Self {
        IntModPoly { inner, parent }
    }
    
    #[inline]
    pub const fn into_raw(self) -> fmpz_mod_poly::fmpz_mod_poly_struct {
        let inner = self.inner;
        let _ = ManuallyDrop::new(self);
        inner
    }
    
    #[inline]
    pub fn parent(&self) -> &IntModPolyRing {
        &self.parent
    }
    
    #[inline]
    pub fn base_ring(&self) -> IntModRing {
        self.parent().base_ring()
    }
    
    #[inline]
    pub fn modulus(&self) -> Integer {
        self.parent().modulus()
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { 
            fmpz_mod_poly::fmpz_mod_poly_is_zero(
                self.as_ptr(), 
                self.ctx_as_ptr()
            ) == 1
        }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { 
            fmpz_mod_poly::fmpz_mod_poly_is_one(
                self.as_ptr(), 
                self.ctx_as_ptr()
            ) == 1
        }
    }

    #[inline]
    pub fn is_gen(&self) -> bool {
        unsafe { 
            fmpz_mod_poly::fmpz_mod_poly_is_gen(
                self.as_ptr(), 
                self.ctx_as_ptr()
            ) == 1
        }
    }

    
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { 
            fmpz_mod_poly::fmpz_mod_poly_length(
                self.as_ptr(), 
                self.ctx_as_ptr()
            ).try_into().unwrap()
        }
    }

    #[inline]
    pub fn degree(&self) -> i64 {
        unsafe { 
            fmpz_mod_poly::fmpz_mod_poly_degree(self.as_ptr(), self.ctx_as_ptr()) 
        }
    }
    
    pub fn get_coeff(&self, i: usize) -> IntMod {
        let mut res = self.base_ring().zero();
        unsafe { 
            fmpz_mod_poly::fmpz_mod_poly_get_coeff_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                i.try_into().expect("Cannot convert index to a signed long."),
                res.parent().as_ptr()
            )
        }
        res
    }
    
    pub fn set_coeff<T: AsRef<IntMod>>(&mut self, i: usize, coeff: T) {
        unsafe {
            fmpz_mod_poly::fmpz_mod_poly_set_coeff_fmpz(
                self.as_mut_ptr(),                                 
                i.try_into().expect("Cannot convert index to a signed long."), 
                coeff.as_ref().as_ptr(),
                self.ctx_as_ptr()
            );
        }
    }
    
    // TODO: anything better?
    #[inline]
    pub fn get_coeffs(&self) -> Vec<IntMod> {
        let mut res = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            res.push(self.get_coeff(i))
        }
        res
    }
}

