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

use crate::{
    FinFldElem, 
    FiniteField, 
    FqCtx, 
    IntModPoly, 
    IntModPolyRing, 
    IntPoly, 
    Integer, 
    ValOrRef,
};
//use flint_sys::{fmpz, fmpz_mod, fmpz_mod_poly};
use flint_sys::fq_default as fq;
use flint_sys::fq_default_poly as fq_poly;
//use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
//use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::cell::RefCell;
use std::ffi::CString;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct FinFldPolyRing {
    ctx: Rc<FqCtx>,
    var: Rc<RefCell<String>>,
}

impl Eq for FinFldPolyRing {}

impl PartialEq for FinFldPolyRing {
    fn eq(&self, rhs: &FinFldPolyRing) -> bool {
        self.base_ring() == rhs.base_ring()
    }
}

impl fmt::Display for FinFldPolyRing {
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

impl Hash for FinFldPolyRing {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nvars().hash(state);
    }
}

impl FinFldPolyRing {
    /// Returns a pointer to the [FLINT context][fq_default::fq_default_ctx_struct].
    #[inline]
    pub fn ctx_as_ptr(&self) -> &fq::fq_default_ctx_struct {
        &self.ctx.0
    }

    /// Initialize the polynomial ring with the given variable.
    #[inline]
    pub fn init<'a, P, K>(p: P, k: K, var: &str) -> Self
    where
        P: Into<ValOrRef<'a, Integer>>,
        K: TryInto<i64>,
    {
        let p = &*p.into();
        match k.try_into() {
            Ok(k) => {
                assert!(p.is_prime());
                assert!(k > 0);

                Self::init_unchecked(p, k, var)
            }
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }

    pub fn init_unchecked<'a, P, K>(p: P, k: K, var: &str) -> Self
    where
        P: Into<ValOrRef<'a, Integer>>,
        K: TryInto<i64>,
    {
        match k.try_into() {
            Ok(k) => {
                let v = CString::new("o").unwrap();
                let mut ctx = MaybeUninit::uninit();
                unsafe {
                    fq::fq_default_ctx_init(ctx.as_mut_ptr(), p.into().as_ptr(), k, v.as_ptr());
                    FinFldPolyRing {
                        ctx: Rc::new(FqCtx(ctx.assume_init())),
                        var: Rc::new(RefCell::new(var.to_string())),
                    }
                }
            }
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }

    #[inline]
    pub fn modulus(&self) -> IntModPoly {
        let zp = IntModPolyRing::init(self.base_ring().prime(), "x");
        let mut res = zp.default();
        unsafe {
            fq::fq_default_ctx_modulus(res.as_mut_ptr(), self.ctx_as_ptr());
        }
        res
    }

    #[inline]
    pub fn default(&self) -> FinFldPoly {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fq_poly::fq_default_poly_init(z.as_mut_ptr(), self.ctx_as_ptr());
            FinFldPoly {
                inner: z.assume_init(),
                ctx: Rc::clone(&self.ctx),
                var: Rc::clone(&self.var),
            }
        }
    }

    #[inline]
    pub fn new<T: Into<IntPoly>>(&self, x: T) -> FinFldPoly {
        let mut res = self.default();
        unsafe {
            fq_poly::fq_default_poly_set_fmpz_poly(
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
    pub fn base_ring(&self) -> FiniteField {
        FiniteField {
            ctx: Rc::clone(&self.ctx),
        }
    }
}

//#[derive(Debug)]
pub struct FinFldPoly {
    inner: fq_poly::fq_default_poly_struct,
    ctx: Rc<FqCtx>,
    var: Rc<RefCell<String>>,
}

impl Clone for FinFldPoly {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = self.parent().default();
        unsafe {
            fq_poly::fq_default_poly_set(res.as_mut_ptr(), self.as_ptr(), self.ctx_as_ptr());
        }
        res
    }
}

/*
impl fmt::Display for FinFldPoly {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
*/

impl Drop for FinFldPoly {
    #[inline]
    fn drop(&mut self) {
        unsafe { fq_poly::fq_default_poly_clear(self.as_mut_ptr(), self.ctx_as_ptr()) }
    }
}

impl Hash for FinFldPoly {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent().hash(state);
        self.coefficients().hash(state);
    }
}

impl FinFldPoly {
    /// Returns a pointer to the inner [fq_poly::fq_default_poly_struct].
    #[inline]
    pub const fn as_ptr(&self) -> *const fq_poly::fq_default_poly_struct {
        &self.inner
    }

    /// Returns a mutable pointer to the inner [fq_poly::fq_default_poly_struct].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fq_poly::fq_default_poly_struct {
        &mut self.inner
    }

    /// Returns a pointer to the [FLINT context][fq::fq_default_ctx_struct].
    #[inline]
    pub fn ctx_as_ptr(&self) -> &fq::fq_default_ctx_struct {
        &self.ctx.0
    }

    #[inline]
    pub fn modulus(&self) -> IntModPoly {
        let zp = IntModPolyRing::init(self.base_ring().prime(), "x");
        let mut res = zp.default();
        unsafe {
            fq::fq_default_ctx_modulus(res.as_mut_ptr(), self.ctx_as_ptr());
        }
        res
    }

    /// Return the parent [ring of polynomials][FinFldPolyRing].
    #[inline]
    pub fn parent(&self) -> FinFldPolyRing {
        FinFldPolyRing {
            ctx: Rc::clone(&self.ctx),
            var: Rc::clone(&self.var),
        }
    }

    #[inline]
    pub fn base_ring(&self) -> FiniteField {
        FiniteField {
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
        unsafe { fq_poly::fq_default_poly_length(self.as_ptr(), self.ctx_as_ptr()) }
    }

    #[inline]
    pub fn degree(&self) -> i64 {
        unsafe { fq_poly::fq_default_poly_degree(self.as_ptr(), self.ctx_as_ptr()) }
    }
    
    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { 
            fq_poly::fq_default_poly_is_zero(
                self.as_ptr(), 
                self.ctx_as_ptr()
            ) == 1
        }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { 
            fq_poly::fq_default_poly_is_one(
                self.as_ptr(), 
                self.ctx_as_ptr()
            ) == 1
        }
    }
    
    #[inline]
    pub fn is_gen(&self) -> bool {
        unsafe { 
            fq_poly::fq_default_poly_is_gen(
                self.as_ptr(), 
                self.ctx_as_ptr()
            ) == 1
        }
    }
    
    #[inline]
    pub fn is_unit(&self) -> bool {
        unsafe { 
            fq_poly::fq_default_poly_is_unit(
                self.as_ptr(), 
                self.ctx_as_ptr()
            ) == 1
        }
    }

    #[inline]
    pub fn get_coeff(&self, i: i64) -> FinFldElem {
        let mut res = self.base_ring().default();
        unsafe {
            fq_poly::fq_default_poly_get_coeff(
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
        T: Into<ValOrRef<'a, FinFldElem>>,
    {
        unsafe {
            fq_poly::fq_default_poly_set_coeff(
                self.as_mut_ptr(),
                i,
                coeff.into().as_ptr(),
                self.ctx_as_ptr(),
            );
        }
    }

    #[inline]
    pub fn coefficients(&self) -> Vec<FinFldElem> {
        let len = self.len();

        let mut vec = Vec::with_capacity(usize::try_from(len).ok().unwrap());
        for i in 0..len {
            vec.push(self.get_coeff(i));
        }
        vec
    }
}

/*
impl Serialize for FinFldPoly {
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

struct FinFldPolyVisitor {}

impl FinFldPolyVisitor {
    fn new() -> Self {
        FinFldPolyVisitor {}
    }
}

impl<'de> Visitor<'de> for FinFldPolyVisitor {
    type Value = FinFldPoly;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an FinFldPoly")
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
        let zn = FinFldPolyRing::init(m, "x");
        Ok(zn.new(coeffs))
    }
}

impl<'de> Deserialize<'de> for FinFldPoly {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(FinFldPolyVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::{FinFldPoly, FinFldPolyRing};

    #[test]
    fn serde() {
        let zn = FinFldPolyRing::init(72u32, "x");
        let x = zn.new(vec![1, 0, 0, 2, -19]);
        let ser = bincode::serialize(&x).unwrap();
        let y: FinFldPoly = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}*/
