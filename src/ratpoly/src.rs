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

use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::rc::Rc;
use flint_sys::fmpq_poly;
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use crate::{ops::Assign, Integer, Rational, RationalField, ValOrRef};

#[derive(Clone, Debug)]
pub struct RatPolyRing {
    var: Rc<RefCell<String>>,
}

impl Eq for RatPolyRing {}

impl PartialEq for RatPolyRing {
    fn eq(&self, _: &RatPolyRing) -> bool {
        true
    }
}

impl fmt::Display for RatPolyRing {
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

impl Hash for RatPolyRing {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nvars().hash(state);
    }
}

impl RatPolyRing {
    #[inline]
    pub fn init(var: &str) -> Self {
        RatPolyRing {
            var: Rc::new(RefCell::new(var.to_string())),
        }
    }

    #[inline]
    pub fn default(&self) -> RatPoly {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpq_poly::fmpq_poly_init(z.as_mut_ptr());
            RatPoly {
                inner: z.assume_init(),
                var: Rc::clone(&self.var),
            }
        }
    }

    #[inline]
    pub fn new<T: Into<RatPoly>>(&self, x: T) -> RatPoly {
        x.into()
    }

    pub fn nvars(&self) -> i64 {
        1
    }

    /// Return the variable of the polynomial as a `&str`.
    pub fn var(&self) -> String {
        self.var.borrow().to_string()
    }

    /// Change the variable of the polynomial.
    pub fn set_var<T: AsRef<String>>(&self, var: T) {
        self.var.replace(var.as_ref().to_string());
    }

    pub fn base_ring(&self) -> RationalField {
        RationalField {}
    }
}

#[derive(Debug)]
pub struct RatPoly {
    inner: fmpq_poly::fmpq_poly_struct,
    var: Rc<RefCell<String>>,
}

impl<'a, T> Assign<T> for RatPoly
where
    T: Into<ValOrRef<'a, RatPoly>>,
{
    fn assign(&mut self, other: T) {
        unsafe {
            fmpq_poly::fmpq_poly_set(self.as_mut_ptr(), other.into().as_ptr());
        }
    }
}

impl Clone for RatPoly {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = self.parent().default();
        unsafe {
            fmpq_poly::fmpq_poly_set(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
}

impl Default for RatPoly {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpq_poly::fmpq_poly_init(z.as_mut_ptr());
            RatPoly {
                inner: z.assume_init(),
                var: Rc::new(RefCell::new("x".to_owned())),
            }
        }
    }
}

impl fmt::Display for RatPoly {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for RatPoly {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpq_poly::fmpq_poly_clear(self.as_mut_ptr()) }
    }
}

impl Hash for RatPoly {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent().hash(state);
        self.coefficients().hash(state);
    }
}

impl RatPoly {
    /// Returns a pointer to the inner [FLINT rational polynomial][fmpq_poly::fmpq_poly].
    #[inline]
    pub const fn as_ptr(&self) -> *const fmpq_poly::fmpq_poly_struct {
        &self.inner
    }

    /// Returns a mutable pointer to the inner [FLINT rational polynomial][fmpq_poly::fmpq_poly].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpq_poly::fmpq_poly_struct {
        &mut self.inner
    }

    /// Return the parent [ring of polynomials with rational coefficients][RatPolyRing].
    #[inline]
    pub fn parent(&self) -> RatPolyRing {
        RatPolyRing {
            var: Rc::clone(&self.var),
        }
    }

    pub fn base_ring(&self) -> RationalField {
        RationalField {}
    }

    /// Return the variable of the polynomial as a string.
    #[inline]
    pub fn var(&self) -> String {
        self.var.borrow().to_string()
    }

    /// Change the variable of the polynomial.
    #[inline]
    pub fn set_var<T: AsRef<String>>(&self, var: T) {
        self.var.replace(var.as_ref().to_string());
    }

    /// Return a pretty-printed string representation of a rational polynomial.
    pub fn get_str_pretty(&self) -> String {
        let v = CString::new(self.var()).unwrap();
        unsafe {
            let s = fmpq_poly::fmpq_poly_get_str_pretty(self.as_ptr(), v.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!"),
            }
        }
    }

    #[inline]
    pub fn len(&self) -> i64 {
        unsafe { fmpq_poly::fmpq_poly_length(self.as_ptr()) }
    }

    #[inline]
    pub fn degree(&self) -> i64 {
        unsafe { fmpq_poly::fmpq_poly_degree(self.as_ptr()) }
    }

    #[inline]
    pub fn get_coeff(&self, i: i64) -> Rational {
        let mut res = Rational::default();
        unsafe {
            fmpq_poly::fmpq_poly_get_coeff_fmpq(res.as_mut_ptr(), self.as_ptr(), i);
        }
        res
    }

    #[inline]
    pub fn set_coeff<'a, T>(&mut self, i: i64, coeff: T)
    where
        T: Into<ValOrRef<'a, Rational>>,
    {
        unsafe {
            fmpq_poly::fmpq_poly_set_coeff_fmpq(self.as_mut_ptr(), i, coeff.into().as_ptr());
        }
    }

    #[inline]
    pub fn coefficients(&self) -> Vec<Rational> {
        let len = self.len();

        let mut vec = Vec::with_capacity(usize::try_from(len).ok().unwrap());
        for i in 0..len {
            vec.push(self.get_coeff(i));
        }
        vec
    }
}

impl Serialize for RatPoly {
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

struct RatPolyVisitor {}

impl RatPolyVisitor {
    fn new() -> Self {
        RatPolyVisitor {}
    }
}

impl<'de> Visitor<'de> for RatPolyVisitor {
    type Value = RatPoly;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a RatPoly")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut coeffs: Vec<Integer> = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(x) = access.next_element()? {
            coeffs.push(x);
        }

        Ok(RatPoly::from(coeffs))
    }
}

impl<'de> Deserialize<'de> for RatPoly {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(RatPolyVisitor::new())
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::RatPoly;

    #[test]
    fn serde() {
        let x = RatPoly::from(vec![1, 0, 0, 2, 1]);
        let ser = bincode::serialize(&x).unwrap();
        let y: RatPoly = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}*/
