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

//#[cfg(feature = "serde")]
//mod serde;

use crate::New;
use arb_sys::arb::*;

use std::ffi::CStr;
use std::fmt;
//use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};


#[derive(Debug)]
pub struct Real {
    pub(crate) inner: arb_struct,
}

impl AsRef<Real> for Real {
    fn as_ref(&self) -> &Real {
        self
    }
}

impl Clone for Real {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = Real::default();
        unsafe {
            arb_set(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
}

impl Default for Real {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            arb_init(z.as_mut_ptr());
            Real::from_raw(z.assume_init())
        }
    }
}

impl fmt::Display for Real {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            // taken from Nemo/src/arb/arb.jl: why bits * log_10(2)?
            // Adding 2 seems to be slightly better. Real::from(123) gives
            // 1.23e+2, with +2 bits we get 123.00. Real::from(12345678) gives
            // [1.2345678e+7 +/- 2.00] with +1 we get 1.2345678e+7, with + 2
            // we get 12345678.0, but Real::from(123) becomes 123.00. Whats best?
            let mut n = self.bits() as f64;
            n *= 0.30102999566398119521;
            let m = n.ceil() as i64;
            let c_str = CStr::from_ptr(arb_get_str(self.as_ptr(), m, 0));
            let s = c_str.to_str().unwrap();
            write!(f, "{}", s)
        }
    }
}

impl Drop for Real {
    #[inline]
    fn drop(&mut self) {
        unsafe { arb_clear(self.as_mut_ptr()) }
    }
}

/* TODO
impl Hash for Real {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_ui_vector().hash(state);
    }
}
*/

impl<T: Into<Real>> New<T> for Real {
    #[inline]
    fn new(src: T) -> Self {
        src.into()
    }
}

impl New<&Real> for Real {
    #[inline]
    fn new(src: &Real) -> Self {
        src.clone()
    }
}

impl Real {
    #[inline]
    pub fn zero() -> Self {
        Real::default()
    }

    #[inline]
    pub fn one() -> Self {
        let mut res = Real::default();
        res.one_assign();
        res
    }

    #[inline]
    pub fn zero_assign(&mut self) {
        unsafe {
            arb_zero(self.as_mut_ptr());
        }
    }

    #[inline]
    pub fn one_assign(&mut self) {
        unsafe {
            arb_one(self.as_mut_ptr());
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe {
            arb_is_zero(self.as_ptr()) != 0
        }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe {
            arb_is_one(self.as_ptr()) != 0
        }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const arb_struct {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut arb_struct {
        &mut self.inner
    }

    #[inline]
    pub const unsafe fn from_raw(inner: arb_struct) -> Self {
        Real { inner }
    }

    #[inline]
    pub const fn into_raw(self) -> arb_struct {
        let ret = self.inner;
        let _ = ManuallyDrop::new(self);
        ret
    }

    #[inline]
    pub fn bits(&self) -> i64 {
        unsafe { arb_bits(self.as_ptr()) }
    }
}

/*
#[derive(Debug)]
pub struct IntMod {
    pub(crate) inner: fmpz::fmpz,
    pub(crate) ctx: IntModCtx,
}

impl AsRef<IntMod> for IntMod {
    #[inline]
    fn as_ref(&self) -> &IntMod {
        self
    }
}

impl Clone for IntMod {
    fn clone(&self) -> Self {
        let mut res = IntMod::zero(self.context());
        unsafe { fmpz::fmpz_set(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
}

impl fmt::Display for IntMod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Integer::from(self).to_string())
    }
}

impl Drop for IntMod {
    fn drop(&mut self) {
        unsafe { fmpz::fmpz_clear(self.as_mut_ptr()) }
    }
}


// TODO: avoid Integer allocation?
impl Hash for IntMod {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.context().hash(state);
        Integer::from(self).hash(state);
    }
}

impl<T: Into<Integer>> NewCtx<T, IntModCtx> for IntMod {
    fn new(src: T, ctx: &IntModCtx) -> Self {
        let mut res = unsafe { 
            IntMod::from_raw(src.into().into_raw(), ctx.clone())
        };
        res.canonicalize();
        res
    }
}

impl IntMod {
    #[inline]
    pub(crate) fn canonicalize(&mut self) {
        unsafe {
            // FIXME: Which to use?
            //fmpz::fmpz_mod(self.as_mut_ptr(), self.as_ptr(), self.modulus_as_ptr());
            fmpz_mod::fmpz_mod_set_fmpz(
                self.as_mut_ptr(), 
                self.as_ptr(), 
                self.ctx_as_ptr()
            );
        }
    }
   
    #[inline]
    pub fn zero(ctx: &IntModCtx) -> IntMod {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz::fmpz_init(z.as_mut_ptr());
            IntMod::from_raw(z.assume_init(), ctx.clone())
        }
    }

    #[inline]
    pub fn one(ctx: &IntModCtx) -> IntMod {
        let mut res = IntMod::zero(ctx);
        unsafe{ fmpz::fmpz_one(res.as_mut_ptr()); }
        res
    }
    
    #[inline]
    pub fn zero_assign(&mut self) {
        unsafe { fmpz::fmpz_zero(self.as_mut_ptr()) }
    }
    
    #[inline]
    pub fn one_assign(&mut self) {
        unsafe { fmpz::fmpz_one(self.as_mut_ptr()) }
    }

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
    
    /// Returns a pointer to the [FLINT context][fmpz_mod::fmpz_mod_ctx_struct].
    #[inline]
    pub fn ctx_as_ptr(&self) -> *const fmpz_mod::fmpz_mod_ctx_struct {
        self.context().as_ptr()
    }
    
    /// Returns a pointer to the [FLINT context][fmpz_mod::fmpz_mod_ctx_struct].
    #[inline]
    pub fn modulus_as_ptr(&self) -> *const fmpz::fmpz {
        self.context().modulus_as_ptr()
    }

    /// Construct an `IntMod` from a raw [fmpz::fmpz] and reference to an 
    /// `IntModRing`. This does not canonicalize the output!
    #[inline]
    pub const unsafe fn from_raw(inner: fmpz::fmpz, ctx: IntModCtx) -> IntMod {
        IntMod { inner, ctx }
    }
  
    #[inline]
    pub const fn into_raw(self) -> fmpz::fmpz {
        let inner = self.inner;
        let _ = ManuallyDrop::new(self);
        //(inner, self.ctx.clone())
        inner
    }
    
    #[inline]
    pub const fn context(&self) -> &IntModCtx {
        &self.ctx
    }
    
    /// Return the modulus of `IntMod`.
    #[inline]
    pub fn modulus(&self) -> Integer {
        self.context().modulus()
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { fmpz::fmpz_is_zero(self.as_ptr()) == 1 }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { fmpz::fmpz_is_one(self.as_ptr()) == 1 }
    }
}
*/
