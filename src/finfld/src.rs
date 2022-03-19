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

use std::ffi::{CStr, CString};
use std::fmt;
use std::mem::MaybeUninit;
use std::sync::Arc;
use flint_sys::fq_default as fq;
use crate::{Integer, IntPoly, IntoValOrRef};


#[derive(Debug)]
pub struct FqCtx(fq::fq_default_ctx_struct);

impl Drop for FqCtx {
    fn drop(&mut self) {
        unsafe { fq::fq_default_ctx_clear(&mut self.0); }
    }
}

#[derive(Clone, Debug)]
pub struct FiniteField {
    ctx: Arc<FqCtx>
}

// TODO: get rid of this. see intmod.
impl std::ops::Deref for FiniteField {
    type Target = Arc<FqCtx>;
    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

/*
impl Hash for FiniteField {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.modulus().hash(state)
    }
}*/

impl FiniteField {
    
    /// Returns a pointer to the [FLINT context][fq::fq_default_ctx_struct].
    #[inline]
    pub fn ctx_as_ptr(&self) -> &fq::fq_default_ctx_struct {
        &self.ctx.0
    }
   
    #[inline]
    pub fn init<'a, P, K>(p: P, k: K) -> FiniteField where 
        P: IntoValOrRef<'a, Integer>,
        K: TryInto<i64>,
    {
        let p = &*p.val_or_ref();
        match k.try_into() {
            Ok(k) => {
                assert!(p.is_prime());
                assert!(k > 0);

                Self::init_unchecked(p, k)
            },
            Err(_) => panic!("Input cannot be converted into a signed long!")
        }
    }
    
    pub fn init_unchecked<'a, P, K>(p: P, k: K) -> FiniteField where 
        P: IntoValOrRef<'a, Integer>,
        K: TryInto<i64>,
    {
        match k.try_into() {
            Ok(k) => {
                let var = CString::new("o").unwrap();
                let mut ctx = MaybeUninit::uninit();
                unsafe {
                    fq::fq_default_ctx_init(
                        ctx.as_mut_ptr(), 
                        p.val_or_ref().as_ptr(), 
                        k,
                        var.as_ptr()
                    );
                    FiniteField { ctx: Arc::new(FqCtx(ctx.assume_init())) }
                }
            },
            Err(_) => panic!("Input cannot be converted into a signed long!")
        }
    }
    
    #[inline]
    pub fn default(&self) -> FinFldElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fq::fq_default_init(z.as_mut_ptr(), self.ctx_as_ptr());
            FinFldElem { inner: z.assume_init(), ctx: Arc::clone(&self.ctx) }
        }
    }

    #[inline]
    pub fn new<'a, T>(&self, x: T) -> FinFldElem where
        T: IntoValOrRef<'a, IntPoly>
    {
        let mut res = self.default();
        unsafe { 
            fq::fq_default_set_fmpz_poly(
                res.as_mut_ptr(), 
                x.val_or_ref().as_ptr(), 
                self.ctx_as_ptr()
            ); 
        }
        res
    }
    
    /* requires fmpz_mod_poly
    /// Return the modulus of the ring.
    #[inline]
    pub fn modulus(&self) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {
            fq::fq_default_ctx_modulus(res.as_mut_ptr(), self.ctx_as_ptr()); 
        }
        res
    }*/
    
    #[inline]
    pub fn prime(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { fq::fq_default_ctx_prime(res.as_mut_ptr(), self.ctx_as_ptr()); }
        res
    }

    #[inline]
    pub fn degree(&self) -> i64 {
        unsafe { fq::fq_default_ctx_degree(self.ctx_as_ptr()) }
    }

    #[inline]
    pub fn order(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { fq::fq_default_ctx_order(res.as_mut_ptr(), self.ctx_as_ptr()); }
        res
    }
}


#[derive(Debug)]
pub struct FinFldElem {
    inner: fq::fq_default_struct,
    ctx: Arc<FqCtx>,
}

impl Clone for FinFldElem {
    fn clone(&self) -> Self {
        let mut res = self.parent().default();
        unsafe { 
            fq::fq_default_set(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                self.ctx_as_ptr()
            ); 
        }
        res
    }
}

impl Drop for FinFldElem {
    fn drop(&mut self) {
        unsafe { fq::fq_default_clear(self.as_mut_ptr(), self.ctx_as_ptr())}
    }
}

impl fmt::Display for FinFldElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

/*
impl Hash for IntMod {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Integer::from(self).hash(state);
        self.modulus().hash(state);
    }
}
*/
impl FinFldElem {
    
    /// Returns a pointer to the inner [FLINT integer][fmpz::fmpz].
    #[inline]
    pub const fn as_ptr(&self) -> *const fq::fq_default_struct {
        &self.inner
    }

    /// Returns a mutable pointer to the inner [FLINT integer][fmpz::fmpz].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fq::fq_default_struct {
        &mut self.inner
    }
    
    /// Returns a pointer to the [FLINT context][fmpz_mod::fmpz_mod_ctx_struct].
    #[inline]
    pub fn ctx_as_ptr(&self) -> &fq::fq_default_ctx_struct {
        &self.ctx.0
    }
    
    /// Return a [String] representation of a finite field element.
    #[inline]
    pub fn get_str(&self) -> String {
        unsafe {
            let s = fq::fq_default_get_str(self.as_ptr(), self.ctx_as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    
    /// Return a pretty-printed [String] representation of a finite field element.
    #[inline]
    pub fn get_str_pretty(&self) -> String {
        unsafe {
            let s = fq::fq_default_get_str_pretty(
                self.as_ptr(), 
                self.ctx_as_ptr()
            );
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }

    /// Return the parent [ring of integers mod `n`][IntModRing].
    #[inline]
    pub fn parent(&self) -> FiniteField {
        FiniteField { ctx: Arc::clone(&self.ctx) }
    }
   
    /* requires fmpz_mod_poly
    /// Return the modulus of the ring.
    #[inline]
    pub fn modulus(&self) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {
            fq::fq_default_ctx_modulus(res.as_mut_ptr(), self.ctx_as_ptr()); 
        }
        res
    }*/
    
    #[inline]
    pub fn prime(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { fq::fq_default_ctx_prime(res.as_mut_ptr(), self.ctx_as_ptr()); }
        res
    }

    #[inline]
    pub fn degree(&self) -> i64 {
        unsafe { fq::fq_default_ctx_degree(self.ctx_as_ptr()) }
    }

    #[inline]
    pub fn order(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { fq::fq_default_ctx_order(res.as_mut_ptr(), self.ctx_as_ptr()); }
        res
    }
}
