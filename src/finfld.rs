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

use crate::*;
use flint_sys::fq_default as fq;
use std::ffi::{CStr, CString};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::rc::Rc;

#[derive(Debug)]
pub struct FqCtx(pub fq::fq_default_ctx_struct);

impl Drop for FqCtx {
    fn drop(&mut self) {
        unsafe {
            fq::fq_default_ctx_clear(&mut self.0);
        }
    }
}

#[derive(Clone, Debug)]
pub struct FiniteField {
    pub ctx: Rc<FqCtx>,
}

impl Eq for FiniteField {}

impl PartialEq for FiniteField {
    fn eq(&self, rhs: &FiniteField) -> bool {
        Rc::ptr_eq(&self.ctx, &rhs.ctx) || self.order() == rhs.order()
    }
}

impl fmt::Display for FiniteField {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Finite field of order {}^{}",
            self.prime(),
            self.degree()
        )
    }
}

impl Hash for FiniteField {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.modulus().hash(state)
    }
}

impl FiniteField {
    /// Returns a pointer to the [FLINT context][fq::fq_default_ctx_struct].
    #[inline]
    pub fn ctx_as_ptr(&self) -> &fq::fq_default_ctx_struct {
        &self.ctx.0
    }

    #[inline]
    pub fn init<'a, P, K>(p: P, k: K) -> FiniteField
    where
        P: AsRef<Integer>,
        K: Into<i64>,
    {
        let p = p.as_ref();
        let k = k.into();
        assert!(p.is_prime());
        assert!(k > 0);
        Self::init_unchecked(p, k)
    }

    pub fn init_unchecked<'a, P, K>(p: P, k: K) -> FiniteField
    where
        P: AsRef<Integer>,
        K: Into<i64>,
    {
        let var = CString::new("o").unwrap();
        let mut ctx = MaybeUninit::uninit();
        unsafe {
            fq::fq_default_ctx_init(
                ctx.as_mut_ptr(), 
                p.as_ref().as_ptr(), 
                k.into(), 
                var.as_ptr()
            );
            FiniteField {
                ctx: Rc::new(FqCtx(ctx.assume_init())),
            }
        }
    }

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
    }

    #[inline]
    pub fn new<'a, T>(&self, x: T) -> FinFldElem
    where
        T: Into<IntPoly>,
    {
        let mut res = self.default();
        unsafe {
            fq::fq_default_set_fmpz_poly(
                res.as_mut_ptr(), x.into().as_ptr(), self.ctx_as_ptr());
        }
        res
    }

    #[inline]
    pub fn modulus(&self) -> IntModPoly {
        let zp = IntModPolyRing::init(self.prime(), "x");
        let mut res = zp.default();
        unsafe {
            fq::fq_default_ctx_modulus(res.as_mut_ptr(), self.ctx_as_ptr());
        }
        res
    }

    /// Return the variable of the finite field elements as a polynomial.
    #[inline]
    pub fn var(&self) -> String {
        //self.var.borrow().to_string()
        String::from("o")
    }

    #[inline]
    pub fn prime(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            fq::fq_default_ctx_prime(res.as_mut_ptr(), self.ctx_as_ptr());
        }
        res
    }

    #[inline]
    pub fn degree(&self) -> i64 {
        unsafe { fq::fq_default_ctx_degree(self.ctx_as_ptr()) }
    }

    #[inline]
    pub fn order(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            fq::fq_default_ctx_order(res.as_mut_ptr(), self.ctx_as_ptr());
        }
        res
    }
}

#[derive(Debug)]
pub struct FinFldElem {
    inner: fq::fq_default_struct,
    ctx: Rc<FqCtx>,
}

impl AsRef<FinFldElem> for FinFldElem {
    fn as_ref(&self) -> &FinFldElem {
        self
    }
}

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

impl Clone for FinFldElem {
    fn clone(&self) -> Self {
        let mut res = self.parent().default();
        unsafe {
            fq::fq_default_set(res.as_mut_ptr(), self.as_ptr(), self.ctx_as_ptr());
        }
        res
    }
}

impl Drop for FinFldElem {
    fn drop(&mut self) {
        unsafe { fq::fq_default_clear(self.as_mut_ptr(), self.ctx_as_ptr()) }
    }
}

impl fmt::Display for FinFldElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let s = fq::fq_default_get_str_pretty(self.as_ptr(), self.ctx_as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => write!(f, "{}", s),
                Err(_) => panic!("Flint returned invalid UTF-8!"),
            }
        }
    }
}

impl Hash for FinFldElem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent().hash(state);
        IntModPoly::from(self).hash(state);
    }
}

impl FinFldElem {
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
        &self.ctx.0
    }

    #[inline]
    pub fn modulus(&self) -> IntModPoly {
        let zp = IntModPolyRing::init(self.parent().prime(), "x");
        let mut res = zp.default();
        unsafe {
            fq::fq_default_ctx_modulus(res.as_mut_ptr(), self.ctx_as_ptr());
        }
        res
    }

    /// Return the variable of the finite field element as a polynomial.
    #[inline]
    pub fn var(&self) -> String {
        //self.var.borrow().to_string()
        String::from("o")
    }

    /// Return the parent [finite field][FiniteField].
    #[inline]
    pub fn parent(&self) -> FiniteField {
        FiniteField {
            ctx: Rc::clone(&self.ctx),
        }
    }
}
