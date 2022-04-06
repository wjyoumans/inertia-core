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
use flint_sys::fmpz_poly;
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use crate::{ops::Assign, Integer, IntegerRing, ValOrRef};

#[derive(Clone, Debug)]
pub struct IntPolyRing {
    var: Rc<RefCell<String>>,
}

impl Eq for IntPolyRing {}

impl PartialEq for IntPolyRing {
    #[inline]
    fn eq(&self, _: &IntPolyRing) -> bool {
        true
    }
}

impl fmt::Display for IntPolyRing {
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

impl Hash for IntPolyRing {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nvars().hash(state);
    }
}

impl IntPolyRing {
    #[inline]
    pub fn init(var: &str) -> Self {
        IntPolyRing {
            var: Rc::new(RefCell::new(var.to_string())),
        }
    }

    #[inline]
    pub fn default(&self) -> IntPoly {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_poly::fmpz_poly_init(z.as_mut_ptr());
            IntPoly {
                inner: z.assume_init(),
                var: Rc::clone(&self.var),
            }
        }
    }

    #[inline]
    pub fn new<T: Into<IntPoly>>(&self, x: T) -> IntPoly {
        x.into()
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
    pub fn set_var<T: AsRef<String>>(&self, var: T) {
        self.var.replace(var.as_ref().to_string());
    }

    #[inline]
    pub fn base_ring(&self) -> IntegerRing {
        IntegerRing {}
    }
}

#[derive(Debug)]
pub struct IntPoly {
    inner: fmpz_poly::fmpz_poly_struct,
    var: Rc<RefCell<String>>,
}

impl<'a, T> Assign<T> for IntPoly
where
    T: Into<ValOrRef<'a, IntPoly>>,
{
    fn assign(&mut self, other: T) {
        unsafe {
            fmpz_poly::fmpz_poly_set(self.as_mut_ptr(), other.into().as_ptr());
        }
    }
}

impl Clone for IntPoly {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = self.parent().default();
        unsafe {
            fmpz_poly::fmpz_poly_set(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
}

impl Default for IntPoly {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_poly::fmpz_poly_init(z.as_mut_ptr());
            IntPoly {
                inner: z.assume_init(),
                var: Rc::new(RefCell::new("x".to_owned())),
            }
        }
    }
}

impl fmt::Display for IntPoly {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for IntPoly {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpz_poly::fmpz_poly_clear(self.as_mut_ptr()) }
    }
}

impl Hash for IntPoly {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent().hash(state);
        self.coefficients().hash(state);
    }
}

impl IntPoly {
    /// Returns a pointer to the inner [FLINT integer polynomial][fmpz_poly::fmpz_poly].
    #[inline]
    pub const fn as_ptr(&self) -> *const fmpz_poly::fmpz_poly_struct {
        &self.inner
    }

    /// Returns a mutable pointer to the inner [FLINT integer polynomial][fmpz_poly::fmpz_poly].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpz_poly::fmpz_poly_struct {
        &mut self.inner
    }

    /// Return the parent [ring of polynomials with integer coefficients][IntPolyRing].
    #[inline]
    pub fn parent(&self) -> IntPolyRing {
        IntPolyRing {
            var: Rc::clone(&self.var),
        }
    }

    pub fn base_ring(&self) -> IntegerRing {
        IntegerRing {}
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

    /// Return a pretty-printed string representation of an integer polynomial.
    pub fn get_str_pretty(&self) -> String {
        let v = CString::new(self.var()).unwrap();
        unsafe {
            let s = fmpz_poly::fmpz_poly_get_str_pretty(self.as_ptr(), v.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!"),
            }
        }
    }

    #[inline]
    pub fn len(&self) -> i64 {
        unsafe { fmpz_poly::fmpz_poly_length(self.as_ptr()) }
    }

    #[inline]
    pub fn degree(&self) -> i64 {
        unsafe { fmpz_poly::fmpz_poly_degree(self.as_ptr()) }
    }

    #[inline]
    pub fn get_coeff(&self, i: i64) -> Integer {
        let mut res = Integer::default();
        unsafe {
            fmpz_poly::fmpz_poly_get_coeff_fmpz(res.as_mut_ptr(), self.as_ptr(), i);
        }
        res
    }

    #[inline]
    pub fn set_coeff<'a, T>(&mut self, i: i64, coeff: T)
    where
        T: Into<ValOrRef<'a, Integer>>,
    {
        unsafe {
            fmpz_poly::fmpz_poly_set_coeff_fmpz(self.as_mut_ptr(), i, coeff.into().as_ptr());
        }
    }

    #[inline]
    pub fn coefficients(&self) -> Vec<Integer> {
        let len = self.len();

        let mut vec = Vec::<Integer>::with_capacity(usize::try_from(len).ok().unwrap());
        for i in 0..len {
            vec.push(self.get_coeff(i));
        }
        vec
    }
}

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
}
