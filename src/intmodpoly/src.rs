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

use crate::{FmpzModCtx, IntMod, IntModRing, IntPoly, Integer, ValOrRef};
use flint_sys::{fmpz_mod, fmpz_mod_poly};
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::cell::RefCell;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct IntModPolyRing {
    ctx: Rc<FmpzModCtx>,
    var: Rc<RefCell<String>>,
}

impl Eq for IntModPolyRing {}

impl PartialEq for IntModPolyRing {
    fn eq(&self, rhs: &IntModPolyRing) -> bool {
        self.base_ring() == rhs.base_ring()
    }
}

impl fmt::Display for IntModPolyRing {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Univariate polynomial ring in {} over {}",
            self.var(),
            self.base_ring()
        )
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

    /// Return the modulus of the ring.
    #[inline]
    pub fn modulus(&self) -> Integer {
        unsafe {
            let n = fmpz_mod::fmpz_mod_ctx_modulus(self.ctx_as_ptr());
            Integer::from_raw(*n)
        }
    }

    #[inline]
    pub fn init<'a, T>(n: T, var: &str) -> Self
    where
        T: Into<ValOrRef<'a, Integer>>,
    {
        let mut ctx = MaybeUninit::uninit();
        unsafe {
            fmpz_mod::fmpz_mod_ctx_init(ctx.as_mut_ptr(), n.into().as_ptr());
            IntModPolyRing {
                ctx: Rc::new(FmpzModCtx(ctx.assume_init())),
                var: Rc::new(RefCell::new(var.to_string())),
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
                ctx: Rc::clone(&self.ctx),
                var: Rc::clone(&self.var),
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
                self.ctx_as_ptr(),
            );
        }
        res
    }

    #[inline]
    pub fn nvars(&self) -> i64 {
        1
    }

    /// Return the variable of the polynomial as a `&str`.
    #[inline]
    pub fn var(&self) -> String {
        self.var.borrow().to_string()
    }

    /// Change the variable of the polynomial.
    #[inline]
    pub fn set_var<T: AsRef<str>>(&self, var: T) {
        self.var.replace(var.as_ref().to_string());
    }

    #[inline]
    pub fn base_ring(&self) -> IntModRing {
        IntModRing {
            ctx: Rc::clone(&self.ctx),
        }
    }
}

#[derive(Debug)]
pub struct IntModPoly {
    inner: fmpz_mod_poly::fmpz_mod_poly_struct,
    ctx: Rc<FmpzModCtx>,
    var: Rc<RefCell<String>>,
}

impl Clone for IntModPoly {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = self.parent().default();
        unsafe {
            fmpz_mod_poly::fmpz_mod_poly_set(res.as_mut_ptr(), self.as_ptr(), self.ctx_as_ptr());
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
        unsafe { fmpz_mod_poly::fmpz_mod_poly_clear(self.as_mut_ptr(), self.ctx_as_ptr()) }
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

    /// Return the modulus of the ring.
    #[inline]
    pub fn modulus(&self) -> Integer {
        unsafe {
            let n = fmpz_mod::fmpz_mod_ctx_modulus(self.ctx_as_ptr());
            Integer::from_raw(*n)
        }
    }

    /// Return the parent [ring of polynomials with integer coefficients][IntPolyRing].
    #[inline]
    pub fn parent(&self) -> IntModPolyRing {
        IntModPolyRing {
            ctx: Rc::clone(&self.ctx),
            var: Rc::clone(&self.var),
        }
    }

    #[inline]
    pub fn base_ring(&self) -> IntModRing {
        IntModRing {
            ctx: Rc::clone(&self.ctx),
        }
    }

    /// Return the variable of the polynomial as a string.
    #[inline]
    pub fn var(&self) -> String {
        self.var.borrow().to_string()
    }

    /// Change the variable of the polynomial.
    #[inline]
    pub fn set_var<T: AsRef<str>>(&self, var: T) {
        self.var.replace(var.as_ref().to_string());
    }

    #[inline]
    pub fn len(&self) -> i64 {
        unsafe { fmpz_mod_poly::fmpz_mod_poly_length(self.as_ptr(), self.ctx_as_ptr()) }
    }

    #[inline]
    pub fn degree(&self) -> i64 {
        unsafe { fmpz_mod_poly::fmpz_mod_poly_degree(self.as_ptr(), self.ctx_as_ptr()) }
    }

    #[inline]
    pub fn get_coeff(&self, i: i64) -> IntMod {
        let mut res = self.base_ring().default();
        unsafe {
            fmpz_mod_poly::fmpz_mod_poly_get_coeff_fmpz(
                res.as_mut_ptr(),
                self.as_ptr(),
                i,
                self.ctx_as_ptr(),
            );
        }
        res
    }

    #[inline]
    pub fn set_coeff<'a, T>(&mut self, i: i64, coeff: T)
    where
        T: Into<ValOrRef<'a, Integer>>,
    {
        unsafe {
            fmpz_mod_poly::fmpz_mod_poly_set_coeff_fmpz(
                self.as_mut_ptr(),
                i,
                coeff.into().as_ptr(),
                self.ctx_as_ptr(),
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

impl Serialize for IntModPoly {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let coeffs = self
            .coefficients()
            .iter()
            .map(|x| Integer::from(x))
            .collect::<Vec<_>>();
        let mut seq = serializer.serialize_seq(Some(coeffs.len() + 1))?;
        seq.serialize_element(&self.modulus())?;
        for e in coeffs.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct IntModPolyVisitor {}

impl IntModPolyVisitor {
    fn new() -> Self {
        IntModPolyVisitor {}
    }
}

impl<'de> Visitor<'de> for IntModPolyVisitor {
    type Value = IntModPoly;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an IntModPoly")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut coeffs: Vec<Integer> = Vec::with_capacity(access.size_hint().unwrap_or(0));
        let m: Integer = access
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        while let Some(x) = access.next_element()? {
            coeffs.push(x);
        }
        let zn = IntModPolyRing::init(m, "x");
        Ok(zn.new(coeffs))
    }
}

impl<'de> Deserialize<'de> for IntModPoly {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(IntModPolyVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::{IntModPoly, IntModPolyRing};

    #[test]
    fn serde() {
        let zn = IntModPolyRing::init(72u32, "x");
        let x = zn.new(vec![1, 0, 0, 2, -19]);
        let ser = bincode::serialize(&x).unwrap();
        let y: IntModPoly = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}
