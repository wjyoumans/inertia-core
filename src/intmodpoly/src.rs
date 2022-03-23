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


use std::hash::{Hash, Hasher};
use std::fmt;
use std::mem::MaybeUninit;
use std::sync::{Arc, RwLock};

use flint_sys::{fmpz_mod, fmpz_mod_poly};
use crate::{FmpzModCtx, Integer, IntPoly, IntMod, IntModRing, ValOrRef};

#[derive(Clone, Debug)]
pub struct IntModPolyRing {
    ctx: Arc<FmpzModCtx>,
    var: Arc<RwLock<String>>
}

impl fmt::Display for IntModPolyRing {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Univariate polynomial ring in {} over {}", self.var(), self.base_ring())
    }
}

impl Hash for IntModPolyRing {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nvars().hash(state);
    }
}

impl IntModPolyRing {

    /// Returns a pointer to the [FLINT context][fmpz_mod::fmpz_mod_ctx_struct].
    #[inline]
    pub fn ctx_as_ptr(&self) -> &fmpz_mod::fmpz_mod_ctx_struct {
        &self.ctx.0
    }

    #[inline]
    pub fn init<'a, T>(n: T, var: &str) -> Self where
        T: Into<ValOrRef<'a, Integer>>
    {
        let mut ctx = MaybeUninit::uninit();
        unsafe{
            fmpz_mod::fmpz_mod_ctx_init(ctx.as_mut_ptr(), n.into().as_ptr());
            IntModPolyRing { 
                ctx: Arc::new(FmpzModCtx(ctx.assume_init())),
                var: Arc::new(RwLock::new(var.to_string())) 
            }
        }
    }

    #[inline]
    pub fn default(&self) -> IntModPoly {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_mod_poly::fmpz_mod_poly_init(z.as_mut_ptr(), self.ctx_as_ptr());
            IntModPoly {
                inner: z.assume_init(),
                ctx: Arc::clone(&self.ctx),
                var: Arc::clone(&self.var),
            }
        }
    }

    #[inline]
    pub fn new<T: Into<IntPoly>>(&self, x: T) -> IntModPoly {
        let mut res = self.default();
        unsafe {
            fmpz_mod_poly::fmpz_mod_poly_set_fmpz_poly(
                res.as_mut_ptr(), 
                x.into().as_ptr(), 
                self.ctx_as_ptr()
            );
        }
        res
    }
    
    pub fn nvars(&self) -> i64 {
        1
    }
    
    /// Return the variable of the polynomial as a `&str`.
    pub fn var(&self) -> String {
        self.var.read().unwrap().to_string()
    }
    
    /// Change the variable of the polynomial.
    pub fn set_var<T: AsRef<String>>(&self, var: T) {
        *self.var.write().unwrap() = var.as_ref().to_string()
    }

    pub fn base_ring(&self) -> IntModRing {
        IntModRing { ctx: Arc::clone(&self.ctx) }
    }
}

#[derive(Debug)]
pub struct IntModPoly {
    inner: fmpz_mod_poly::fmpz_mod_poly_struct,
    ctx: Arc<FmpzModCtx>,
    var: Arc<RwLock<String>>,
}

impl Clone for IntModPoly {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = self.parent().default();
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

/*
impl fmt::Display for IntModPoly {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
*/

impl Drop for IntModPoly {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpz_mod_poly::fmpz_mod_poly_clear(self.as_mut_ptr(), self.ctx_as_ptr())}
    }
}

impl Hash for IntModPoly {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent().hash(state);
        self.coefficients().hash(state);
    }
}

impl IntModPoly {
    
    /// Returns a pointer to the inner [FLINT integer polynomial][fmpz_poly::fmpz_poly].
    #[inline]
    pub const fn as_ptr(&self) -> *const fmpz_mod_poly::fmpz_mod_poly_struct {
        &self.inner
    }

    /// Returns a mutable pointer to the inner [FLINT integer polynomial][fmpz_poly::fmpz_poly].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpz_mod_poly::fmpz_mod_poly_struct {
        &mut self.inner
    }
    
    /// Returns a pointer to the [FLINT context][fmpz_mod::fmpz_mod_ctx_struct].
    #[inline]
    pub fn ctx_as_ptr(&self) -> &fmpz_mod::fmpz_mod_ctx_struct {
        &self.ctx.0
    }
  
    /// Return the parent [ring of polynomials with integer coefficients][IntPolyRing].
    #[inline]
    pub fn parent(&self) -> IntModPolyRing {
        IntModPolyRing {
            ctx: Arc::clone(&self.ctx),
            var: Arc::clone(&self.var)
        }
    }
    
    #[inline]
    pub fn base_ring(&self) -> IntModRing {
        IntModRing {
            ctx: Arc::clone(&self.ctx),
        }
    }

    /// Return the variable of the polynomial as a string.
    #[inline]
    pub fn var(&self) -> String {
        self.var.read().unwrap().to_string()
    }
    
    /// Change the variable of the polynomial.
    #[inline]
    pub fn set_var<T: AsRef<String>>(&self, var: T) {
        *self.var.write().unwrap() = var.as_ref().to_string()
    }
    
    #[inline]
    pub fn len(&self) -> i64 {
        unsafe { fmpz_mod_poly::fmpz_mod_poly_length(self.as_ptr(), self.ctx_as_ptr())}
    }

    #[inline]
    pub fn degree(&self) -> i64 {
        unsafe { fmpz_mod_poly::fmpz_mod_poly_degree(self.as_ptr(), self.ctx_as_ptr())}
    }

    #[inline]
    pub fn get_coeff(&self, i: i64) -> IntMod {
        let mut res = self.base_ring().default();
        unsafe {
            fmpz_mod_poly::fmpz_mod_poly_get_coeff_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                i, 
                self.ctx_as_ptr()
            );
        }
        res
    }
    
    #[inline]
    pub fn set_coeff<'a, T>(&mut self, i: i64, coeff: T) where
        T: Into<ValOrRef<'a, Integer>>
    {
        unsafe {
            fmpz_mod_poly::fmpz_mod_poly_set_coeff_fmpz(
                self.as_mut_ptr(), 
                i, 
                coeff.into().as_ptr(),
                self.ctx_as_ptr()
            );
        }
    }

    #[inline]
    pub fn coefficients(&self) -> Vec<IntMod> {
        let len = self.len();

        let mut vec = Vec::with_capacity(usize::try_from(len).ok().unwrap());
        for i in 0..len {
            vec.push(self.get_coeff(i));
        }
        vec
    }
}

/*
impl Serialize for IntPoly {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let coeffs = self.coefficients();
        let mut seq = serializer.serialize_seq(Some(coeffs.len()))?;
        for e in coeffs.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct IntPolyVisitor {}

impl IntPolyVisitor {
    fn new() -> Self {
        IntPolyVisitor {}
    }
}

impl<'de> Visitor<'de> for IntPolyVisitor {
    type Value = IntPoly;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an IntPoly")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut coeffs: Vec<Integer> = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_element()? {
            coeffs.push(x);
        }

        Ok(IntPoly::from(coeffs))
    }
}

impl<'de> Deserialize<'de> for IntPoly {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(IntPolyVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::IntPoly;

    #[test]
    fn serde() {
        let x = IntPoly::from(vec![1, 0, 0, 2, 1]);
        let ser = bincode::serialize(&x).unwrap();
        let y: IntPoly = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}*/
