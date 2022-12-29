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
use flint_sys::fq_default as fq;
use std::ffi::CString;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct FqCtx(fq::fq_default_ctx_struct);

impl Drop for FqCtx {
    fn drop(&mut self) {
        unsafe {
            fq::fq_default_ctx_clear(&mut self.0);
        }
    }
}

impl FqCtx {
    #[inline]
    pub fn new<P, K>(p: P, k: K) -> Self 
    where
        P: AsRef<Integer>,
        K: TryInto<i64>,
        <K as TryInto<i64>>::Error: fmt::Debug
    {
        let p = p.as_ref();
        assert!(p.is_prime());
        unsafe { Self::new_unchecked(p, k) }
    }

    /// Use `new_unchecked` to avoid primality testing. This will cause
    /// undefined behavior if `p` is not prime.
    pub unsafe fn new_unchecked<P, K>(p: P, k: K) -> Self
    where
        P: AsRef<Integer>,
        K: TryInto<i64>,
        <K as TryInto<i64>>::Error: fmt::Debug
    {
        let k = k.try_into().expect("Exponent too large!");
        assert!(k > 0);

        let var = CString::new("o").unwrap();
        let mut ctx = MaybeUninit::uninit();
        fq::fq_default_ctx_init(
            ctx.as_mut_ptr(), 
            p.as_ref().as_ptr(), 
            k,
            var.as_ptr()
        );
        FqCtx(ctx.assume_init())
    }
}


#[derive(Clone, Debug)]
pub struct FinFldCtx {
    inner: Rc<FqCtx>,
}

impl Eq for FinFldCtx {}

impl PartialEq for FinFldCtx {
    fn eq(&self, rhs: &FinFldCtx) -> bool {
        Rc::ptr_eq(&self.inner, &rhs.inner) || self.modulus() == rhs.modulus()
    }
}

impl fmt::Display for FinFldCtx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Context for finite field of order {}^{}",
            self.prime(),
            self.degree()
        )
    }
}

impl Hash for FinFldCtx {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.modulus().hash(state)
    }
}

impl FinFldCtx {
    #[inline]
    pub fn new<P, K>(p: P, k: K) -> Self 
    where
        P: Into<Integer>,
        K: TryInto<i64>,
        <K as TryInto<i64>>::Error: fmt::Debug
    {
        FinFldCtx {
            inner: Rc::new(FqCtx::new(p.into(), k))
        }
    }
    
    #[inline]
    pub unsafe fn new_unchecked<P, K>(p: P, k: K) -> Self 
    where
        P: Into<Integer>,
        K: TryInto<i64>,
        <K as TryInto<i64>>::Error: fmt::Debug
    {
        FinFldCtx {
            inner: Rc::new(FqCtx::new_unchecked(p.into(), k))
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> &fq::fq_default_ctx_struct {
        &self.inner.0
    }
    
    /* Cant (easily) get pointer since the modulus could be an nmod_poly
    #[inline]
    pub fn modulus_as_ptr(&self) -> &fmpz_mod_poly::fmpz_mod_poly_struct {
    }
    */

    /*
    #[inline]
    pub fn default(&self) -> FinFldElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fq::fq_default_init(z.as_mut_ptr(), self.ctx_as_ptr());
            FinFldElem {
                inner: z.assume_init(),
                ctx: Rc::clone(&self.ctx),
            }
        }
    }*/

    #[inline]
    pub fn modulus(&self) -> IntModPoly {
        let ctx = IntModCtx::new(self.prime());
        let mut res = IntModPoly::zero(&ctx);
        unsafe {
            fq::fq_default_ctx_modulus(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }

    #[inline]
    pub fn prime(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            fq::fq_default_ctx_prime(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }

    #[inline]
    pub fn degree(&self) -> i64 {
        unsafe { fq::fq_default_ctx_degree(self.as_ptr()) }
    }

    #[inline]
    pub fn order(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            fq::fq_default_ctx_order(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
}

#[derive(Debug)]
pub struct FinFldElem {
    inner: fq::fq_default_struct,
    ctx: FinFldCtx,
}

impl AsRef<FinFldElem> for FinFldElem {
    fn as_ref(&self) -> &FinFldElem {
        self
    }
}

/*
impl<'a, T> Assign<T> for FinFldElem
where
    T: AsRef<FinFldElem>,
{
    fn assign(&mut self, other: T) {
        let other = other.as_ref();
        assert_eq!(self.parent(), other.parent());
        unsafe {
            fq::fq_default_set(self.as_mut_ptr(), other.as_ptr(), self.ctx_as_ptr());
        }
    }
}
*/

impl Clone for FinFldElem {
    fn clone(&self) -> Self {
        let mut res = FinFldElem::zero(self.context());
        unsafe {
            fq::fq_default_set(res.as_mut_ptr(), self.as_ptr(), self.ctx_as_ptr());
        }
        res
    }
}

// FIXME: use 'o' for variable like flint default
impl fmt::Display for FinFldElem {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // IntModPoly makes coeffs positive
        IntModPoly::from(self).fmt(f)

        // Flint does weird formatting: outputs o^0 for 1? Doesnt reduce mod modulus?
        /*
        unsafe {
            let s = fq::fq_default_get_str_pretty(self.as_ptr(), self.ctx_as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => write!(f, "{}", s),
                Err(_) => panic!("Flint returned invalid UTF-8!"),
            }
        }
        */
    }
}

impl Drop for FinFldElem {
    fn drop(&mut self) {
        unsafe { fq::fq_default_clear(self.as_mut_ptr(), self.ctx_as_ptr()) }
    }
}

impl Hash for FinFldElem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.context().hash(state);
        IntPoly::from(self).hash(state);
    }
}

impl FinFldElem {
    pub fn new<T: Into<IntPoly>>(value: T, ctx: &FinFldCtx) -> FinFldElem {
        let mut res = FinFldElem::zero(ctx);
        unsafe {
            fq::fq_default_set_fmpz_poly(
                res.as_mut_ptr(), 
                value.into().as_ptr(), 
                ctx.as_ptr()
            );
        }
        res
    }

    #[inline]
    pub fn zero(ctx: &FinFldCtx) -> FinFldElem {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fq::fq_default_init(z.as_mut_ptr(), ctx.as_ptr());
            FinFldElem::from_raw(z.assume_init(), ctx.clone())
        }
    }
    
    #[inline]
    pub fn one(ctx: &FinFldCtx) -> FinFldElem {
        let mut res = FinFldElem::zero(ctx);
        unsafe {
            fq::fq_default_one(res.as_mut_ptr(), ctx.as_ptr());
        }
        res
    }
    
    #[inline]
    pub fn zero_assign(&mut self) {
        unsafe { fq::fq_default_zero(self.as_mut_ptr(), self.ctx_as_ptr()) }
    }
    
    #[inline]
    pub fn one_assign(&mut self) {
        unsafe { fq::fq_default_one(self.as_mut_ptr(), self.ctx_as_ptr()) }
    }

    /// Returns a pointer to the inner [fq::fq_default_struct].
    #[inline]
    pub const fn as_ptr(&self) -> *const fq::fq_default_struct {
        &self.inner
    }

    /// Returns a mutable pointer to the inner [fq::fq_default_struct].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fq::fq_default_struct {
        &mut self.inner
    }

    /// Returns a pointer to the [FLINT context][fq::fq_default_ctx_struct].
    #[inline]
    pub fn ctx_as_ptr(&self) -> &fq::fq_default_ctx_struct {
        self.context().as_ptr()
    }
    
    #[inline]
    pub const unsafe fn from_raw(
        inner: fq::fq_default_struct, 
        ctx: FinFldCtx
    ) -> FinFldElem {
        FinFldElem { inner, ctx }
    }
  
    #[inline]
    pub const fn into_raw(self) -> fq::fq_default_struct {
        let inner = self.inner;
        let _ = ManuallyDrop::new(self);
        inner
    }
    
    #[inline]
    pub const fn context(&self) -> &FinFldCtx {
        &self.ctx
    }

    #[inline]
    pub fn modulus(&self) -> IntModPoly {
        self.context().modulus()
    }
    
    #[inline]
    pub fn prime(&self) -> Integer {
        self.context().prime()
    }
    
    #[inline]
    pub fn degree(&self) -> i64 {
        self.context().degree()
    }
    
    #[inline]
    pub fn order(&self) -> Integer {
        self.context().order()
    }
}
