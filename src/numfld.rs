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
//mod conv;

use crate::RatPoly;
use flint_sys::fmpq_poly::{fmpq_poly_struct, fmpq_poly_set};
use antic_sys::{
    nf::*,
    nf_elem::*
};

use std::ffi::{CStr, CString};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};
use std::rc::Rc;


#[derive(Debug)]
pub(crate) struct NfCtx(nf_struct);

impl Drop for NfCtx {
    fn drop(&mut self) {
        unsafe {
            nf_clear(&mut self.0);
        }
    }
}

impl NfCtx {
    pub fn new<T: AsRef<RatPoly>>(pol: T) -> Self {
        let mut ctx = MaybeUninit::uninit();
        unsafe {
            nf_init(ctx.as_mut_ptr(), pol.as_ref().as_ptr());
            NfCtx(ctx.assume_init())
        }
    }

}

#[derive(Clone, Debug)]
pub struct NumFldCtx {
    inner: Rc<NfCtx>
}

impl Eq for NumFldCtx {}

impl PartialEq for NumFldCtx {
    fn eq(&self, rhs: &NumFldCtx) -> bool {
        Rc::ptr_eq(&self.inner, &rhs.inner) 
            || (self.defining_polynomial() == rhs.defining_polynomial())
    }
}

impl fmt::Display for NumFldCtx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Context for number field with defining polynomial {}", 
               self.defining_polynomial())
    }
}

impl Hash for NumFldCtx {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.defining_polynomial().hash(state)
    }
}

impl NumFldCtx {
    #[inline]
    pub fn new<T: Into<RatPoly>>(pol: T) -> Self {
        NumFldCtx {
            inner: Rc::new(NfCtx::new(pol.into()))
        }
    }
    
    #[inline]
    pub fn as_ptr(&self) -> *const nf_struct {
        &self.inner.0
    }

    pub fn poly_as_ptr(&self) -> *const fmpq_poly_struct {
        &self.inner.0.pol[0]
    }
    
    #[inline]
    pub fn defining_polynomial(&self) -> RatPoly {
        let mut res = RatPoly::default();
        unsafe { fmpq_poly_set(res.as_mut_ptr(), self.poly_as_ptr()); }
        res
    }
    
}

// Debug? nf_elem_struct is a union
pub struct NumFldElem {
    pub(crate) inner: nf_elem_struct,
    pub(crate) ctx: NumFldCtx
}

impl AsRef<NumFldElem> for NumFldElem {
    fn as_ref(&self) -> &NumFldElem {
        self
    }
}

impl Clone for NumFldElem {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = NumFldElem::zero(self.context());
        unsafe {
            nf_elem_set_fmpq_poly(
                res.as_mut_ptr(), 
                self.poly_as_ptr(), 
                self.ctx_as_ptr()
            );
        }
        res
    }
}

impl fmt::Display for NumFldElem {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let var = CString::new("x").unwrap();
        unsafe {
            let c_str = CStr::from_ptr(
                nf_elem_get_str_pretty(
                    self.as_ptr(),
                    var.as_ptr(),
                    self.ctx_as_ptr()
                )
            );
            write!(f, "{}", c_str.to_str().unwrap())
        }
    }
}

impl Drop for NumFldElem {
    #[inline]
    fn drop(&mut self) {
        unsafe { nf_elem_clear(self.as_mut_ptr(), self.ctx_as_ptr()) }
    }
}

/*
// TODO
impl Hash for NumFldElem {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
    }
}
*/

// TODO: NewCtx

impl NumFldElem {
    #[inline]
    pub fn zero(ctx: &NumFldCtx) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            nf_elem_init(z.as_mut_ptr(), ctx.as_ptr());
            NumFldElem::from_raw(z.assume_init(), ctx.clone())
        }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const nf_elem_struct {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut nf_elem_struct {
        &mut self.inner
    }
    
    #[inline]
    pub fn ctx_as_ptr(&self) -> *const nf_struct {
        self.context().as_ptr()
    }
    
    #[inline]
    pub fn poly_as_ptr(&self) -> *const fmpq_poly_struct {
        self.context().poly_as_ptr()
    }

    #[inline]
    pub const unsafe fn from_raw(inner: nf_elem_struct, ctx: NumFldCtx) -> Self {
        NumFldElem { inner, ctx }
    }

    #[inline]
    pub const fn into_raw(self) -> nf_elem_struct {
        let ret = self.inner;
        let _ = ManuallyDrop::new(self);
        ret
    }

    #[inline]
    pub const fn context(&self) -> &NumFldCtx {
        &self.ctx
    }

    #[inline]
    pub fn defining_polynomial(&self) -> RatPoly {
        self.context().defining_polynomial()
    }
}
