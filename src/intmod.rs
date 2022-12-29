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

use crate::Integer;
use flint_sys::{fmpz, fmpz_mod};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};
use std::rc::Rc;

pub(crate) struct FmpzModCtx(pub(crate) fmpz_mod::fmpz_mod_ctx_struct);

// Certain fields can be uninitialized so manually implement.
impl fmt::Debug for FmpzModCtx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FmpzModCtx")
            .field("n", &self.0.n)
            .field("add_fxn", &self.0.add_fxn)
            .field("sub_fxn", &self.0.sub_fxn)
            .field("mul_fxn", &self.0.mul_fxn)
            // Complains here if modulus fits in machine word
            //.field("mod_", &self.0.mod_)
            .field("n_limbs", &self.0.n_limbs)
            // Seems to always complain here, why?
            //.field("ninv_limbs", &self.0.ninv_limbs)
            .finish()
    }
}

impl Drop for FmpzModCtx {
    fn drop(&mut self) {
        unsafe {
            fmpz_mod::fmpz_mod_ctx_clear(&mut self.0);
        }
    }
}

impl FmpzModCtx {
    pub fn new<T: AsRef<Integer>>(modulus: T) -> Self {
        let mut ctx = MaybeUninit::uninit();
        unsafe {
            fmpz_mod::fmpz_mod_ctx_init(ctx.as_mut_ptr(), modulus.as_ref().as_ptr());
            FmpzModCtx(ctx.assume_init())
        }
    }

}

#[derive(Clone, Debug)]
pub struct IntModRing {
    pub(crate) inner: Rc<FmpzModCtx>
}

impl Eq for IntModRing {}

impl PartialEq for IntModRing {
    fn eq(&self, rhs: &IntModRing) -> bool {
        Rc::ptr_eq(&self.inner, &rhs.inner) || (self.modulus() == rhs.modulus())
    }
}

impl fmt::Display for IntModRing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ring of integers mod {}", self.modulus())
    }
}

impl Hash for IntModRing {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.modulus().hash(state)
    }
}

impl IntModRing {
    #[inline]
    pub fn init<T: Into<Integer>>(modulus: T) -> Self {
        IntModRing {
            inner: Rc::new(FmpzModCtx::new(modulus.into()))
        }
    }
    
    pub fn new<T: Into<Integer>>(&self, value: T) -> IntMod {
        let mut res = unsafe { 
            IntMod::from_raw(value.into().into_raw(), self.clone())
        };
        res.canonicalize();
        res
    }
    
    pub fn zero(&self) -> IntMod {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz::fmpz_init(z.as_mut_ptr());
            IntMod::from_raw(z.assume_init(), self.clone())
        }
    }

    pub fn one(&self) -> IntMod {
        let mut res = self.zero();
        unsafe{ fmpz::fmpz_one(res.as_mut_ptr()); }
        res
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
pub struct IntMod {
    pub(crate) inner: fmpz::fmpz,
    pub(crate) parent: IntModRing,
}

impl AsRef<IntMod> for IntMod {
    #[inline]
    fn as_ref(&self) -> &IntMod {
        self
    }
}

impl Clone for IntMod {
    fn clone(&self) -> Self {
        let mut res = self.parent().zero();
        unsafe { fmpz::fmpz_set(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
}

// TODO: avoid Integer allocation
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


// TODO: avoid Integer allocation
impl Hash for IntMod {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent().hash(state);
        Integer::from(self).hash(state);
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
   
    /* TODO: Maybe do new/zero/one here as well
    #[inline]
    pub fn new<T: Into<Integer>>(value: T, ctx: &IntModCtx) -> Self {
        let mut res = unsafe { 
            IntMod::from_raw(value.into().into_raw(), ctx.clone())
        };
        res.canonicalize();
        res
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
    */
    
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
        self.parent().as_ptr()
    }
    
    /// Returns a pointer to the [FLINT context][fmpz_mod::fmpz_mod_ctx_struct].
    #[inline]
    pub fn modulus_as_ptr(&self) -> *const fmpz::fmpz {
        self.parent().modulus_as_ptr()
    }

    /// Construct an `IntMod` from a raw [fmpz::fmpz] and reference to an 
    /// `IntModRing`. This does not canonicalize the output!
    #[inline]
    pub const unsafe fn from_raw(inner: fmpz::fmpz, parent: IntModRing) -> IntMod {
        IntMod { inner, parent }
    }
  
    #[inline]
    pub const fn into_raw(self) -> fmpz::fmpz {
        let inner = self.inner;
        let _ = ManuallyDrop::new(self);
        //(inner, self.ctx.clone())
        inner
    }
    
    #[inline]
    pub const fn parent(&self) -> &IntModRing {
        &self.parent
    }
    
    /// Return the modulus of `IntMod`.
    #[inline]
    pub fn modulus(&self) -> Integer {
        self.parent().modulus()
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
