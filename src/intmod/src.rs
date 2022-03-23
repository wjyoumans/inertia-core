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

use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::sync::Arc;

use flint_sys::{fmpz, fmpz_mod};
use serde::ser::{Serialize, Serializer, SerializeTuple};
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess};
use crate::{Integer, ValOrRef};


#[derive(Debug)]
pub struct FmpzModCtx(pub fmpz_mod::fmpz_mod_ctx_struct);

impl Drop for FmpzModCtx {
    fn drop(&mut self) {
        unsafe { fmpz_mod::fmpz_mod_ctx_clear(&mut self.0); }
    }
}

#[derive(Clone, Debug)]
pub struct IntModRing {
    pub ctx: Arc<FmpzModCtx>
}

// TODO: get rid of this. Used for checking Arc::ptr_eq to see if two rings point to the same
// location in memory. Replace with new Eq type? StrictEq etc.?
impl std::ops::Deref for IntModRing {
    type Target = Arc<FmpzModCtx>;
    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl fmt::Display for IntModRing {
    #[inline]
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

    /// Returns a pointer to the [FLINT context][fmpz_mod::fmpz_mod_ctx_struct].
    #[inline]
    pub fn ctx_as_ptr(&self) -> &fmpz_mod::fmpz_mod_ctx_struct {
        &self.ctx.0
    }
   
    #[inline]
    pub fn init<'a, T>(n: T) -> IntModRing where 
        T: Into<ValOrRef<'a, Integer>>
    {
        let mut ctx = MaybeUninit::uninit();
        unsafe {
            fmpz_mod::fmpz_mod_ctx_init(ctx.as_mut_ptr(), n.into().as_ptr());
            IntModRing { ctx: Arc::new(FmpzModCtx(ctx.assume_init())) }
        }
    }

    #[inline]
    pub fn new<'a, T>(&self, x: T) -> IntMod where
        T: Into<ValOrRef<'a, Integer>>
    {
        let mut res = self.default();
        unsafe { 
            fmpz::fmpz_set(res.as_mut_ptr(), x.into().as_ptr());
            fmpz::fmpz_mod(res.as_mut_ptr(), res.as_ptr(), self.modulus().as_ptr());
        }
        res
    }

    #[inline]
    pub fn default(&self) -> IntMod {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz::fmpz_init(z.as_mut_ptr());
            IntMod { inner: z.assume_init(), ctx: Arc::clone(&self.ctx) }
        }
    }

    /// Return the modulus of the ring.
    #[inline]
    pub fn modulus(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            let n = fmpz_mod::fmpz_mod_ctx_modulus(self.ctx_as_ptr()); 
            fmpz::fmpz_set(res.as_mut_ptr(), n);
        }
        res
    }
}

#[derive(Debug)]
pub struct IntMod {
    inner: fmpz::fmpz,
    ctx: Arc<FmpzModCtx>,
}

impl Clone for IntMod {
    fn clone(&self) -> Self {
        let mut res = self.parent().default();
        unsafe { 
            fmpz_mod::fmpz_mod_set_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                self.ctx_as_ptr()
            ); 
        }
        res
    }
}

impl fmt::Display for IntMod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for IntMod {
    fn drop(&mut self) {
        unsafe { fmpz::fmpz_clear(self.as_mut_ptr())}
    }
}

impl Hash for IntMod {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent().hash(state);
        Integer::from(self).hash(state);
    }
}

impl IntMod {
    
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
    pub fn ctx_as_ptr(&self) -> &fmpz_mod::fmpz_mod_ctx_struct {
        &self.ctx.0
    }

    /// Return the parent [ring of integers mod `n`][IntModRing].
    #[inline]
    pub fn parent(&self) -> IntModRing {
        IntModRing { ctx: Arc::clone(&self.ctx) }
    }
    
    /// Return the modulus of the ring.
    #[inline]
    pub fn modulus(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            let n = fmpz_mod::fmpz_mod_ctx_modulus(self.ctx_as_ptr()); 
            fmpz::fmpz_set(res.as_mut_ptr(), n);
        }
        res
    }
}


impl Serialize for IntMod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_tuple(2)?;
        state.serialize_element(&Integer::from(self))?;
        state.serialize_element(&self.modulus())?;
        state.end()
    }
}

struct IntModVisitor {}

impl IntModVisitor {
    fn new() -> Self {
        IntModVisitor {}
    }
}

impl<'de> Visitor<'de> for IntModVisitor {
    type Value = IntMod;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an IntMod")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let val: Integer = access.next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let modulus: Integer = access.next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;

        let zn = IntModRing::init(modulus);
        Ok(zn.new(val))
    }
}

impl<'de> Deserialize<'de> for IntMod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(IntModVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Integer, IntModRing};

    #[test]
    fn serde() {
        let zn = IntModRing::init(12);
        let x = zn.new("18446744073709551616");
        let ser = bincode::serialize(&x).unwrap();
        let y: Integer = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}