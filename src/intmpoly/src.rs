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
use flint_sys::fmpz_mpoly;
//use serde::ser::{Serialize, Serializer, SerializeSeq};
//use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess};
use crate::{Integer, IntegerRing, ValOrRef, IntoValOrRef};

const ORD_MPOLY: u32 = 0; // ORD_LEX = 0, ORD_DEGLEX = 1, ORD_DEGREVLEX = 2


#[derive(Debug)]
struct FmpzMPolyCtx(fmpz_mpoly::fmpz_mpoly_ctx_struct);

impl Drop for FmpzMPolyCtx {
    fn drop(&mut self) {
        unsafe { fmpz_mpoly::fmpz_mpoly_ctx_clear(&mut self.0); }
    }
}

#[derive(Clone, Debug)]
pub struct IntMPolyRing {
    ctx: Rc<FmpzMPolyCtx>,
    vars: Rc<RefCell<Vec<String>>>,
}

/*
impl fmt::Display for IntMPolyRing {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Univariate polynomial ring in {} over Integer Ring", self.var())
    }
}

impl Hash for IntPolyRing {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.var().hash(state);
    }
}
*/

impl IntMPolyRing {
    /// Returns a pointer to the [FLINT context][fmpz_mpoly::fmpz_mpoly_ctx_struct].
    #[inline]
    pub fn ctx_as_ptr(&self) -> &fmpz_mpoly::fmpz_mpoly_ctx_struct {
        &self.ctx.0
    }
    
    #[inline]
    pub fn init(nvars: i64) -> Self {

        let vars = Vec::with_capacity(usize::try_from(nvars).ok().unwrap());
        for i in 0..nvars {
            vars.push(format!("x{}", i));
        }

        let mut ctx = MaybeUninit::uninit();
        unsafe {
            fmpz_mpoly::fmpz_mpoly_ctx_init(ctx.as_mut_ptr(), nvars, ORD_MPOLY);
            IntMPolyRing {
                ctx: Rc::new(FmpzMPolyCtx(ctx.assume_init())),
                vars: Rc::new(RefCell::new(vars)),
            }
        }
    }

    #[inline]
    pub fn default(&self) -> IntMPoly {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_mpoly::fmpz_mpoly_init(z.as_mut_ptr(), self.ctx_as_ptr());
            IntMPoly {
                inner: z.assume_init(),
                ctx: Rc::clone(&self.ctx),
                vars: Rc::clone(&self.vars),
            }
        }
    }

    #[inline]
    pub fn new<T: Into<IntMPoly>>(&self, x: T) -> IntMPoly {
        x.into()
    }
   
    /*
    /// Return the variable of the polynomial as a `&str`.
    pub fn var(&self) -> String {
        self.var.read().unwrap().to_string()
    }
    
    /// Change the variable of the polynomial.
    pub fn set_var<T: AsRef<String>>(&self, var: T) {
        *self.var.write().unwrap() = var.as_ref().to_string()
    }
    */
    pub fn base_ring(&self) -> IntegerRing {
        IntegerRing {}
    }
}

#[derive(Debug)]
pub struct IntMPoly {
    inner: fmpz_mpoly::fmpz_mpoly_struct,
    ctx: Rc<FmpzMPolyCtx>,
    vars: Rc<RefCell<Vec<String>>>,
}

/*
impl Clone for IntMPoly {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = self.parent().default();
        unsafe { 
            fmpz_mpoly::fmpz_mpoly_set(res.as_mut_ptr(), self.as_ptr(), self.ctx_as_ptr()); 
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
                var: Arc::new(RwLock::new("x".to_owned())) 
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
*/

impl Drop for IntMPoly {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpz_mpoly::fmpz_mpoly_clear(self.as_mut_ptr(), self.ctx_as_ptr())}
    }
}

/*
impl Hash for IntPoly {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent().hash(state);
        self.coefficients().hash(state);
    }
}

impl<'a, T> IntoValOrRef<'a, IntPoly> for T where
    T: Into<IntPoly>
{
    #[inline]
    fn val_or_ref(self) -> ValOrRef<'a, IntPoly> {
        ValOrRef::Val(self.into())
    }
}
*/


impl IntMPoly {
    
    /// Returns a pointer to the inner 
    /// [FLINT multivariate integer polynomial][fmpz_mpoly::fmpz_mpoly_struct].
    #[inline]
    pub const fn as_ptr(&self) -> *const fmpz_mpoly::fmpz_mpoly_struct {
        &self.inner
    }

    /// Returns a mutable pointer to the inner 
    /// [FLINT multivariate integer polynomial][fmpz_mpoly::fmpz_mpoly_struct].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpz_mpoly::fmpz_mpoly_struct {
        &mut self.inner
    }
    
    /// Returns a pointer to the [FLINT context][fmpz_mpoly::fmpz_mpoly_ctx_struct].
    #[inline]
    pub fn ctx_as_ptr(&self) -> &fmpz_mpoly::fmpz_mpoly_ctx_struct {
        &self.ctx.0
    }
  
    /// Return the parent [multivariate polynomial ring][IntMPolyRing].
    #[inline]
    pub fn parent(&self) -> IntMPolyRing {
        IntMPolyRing {
            ctx: Rc::clone(&self.ctx),
            vars: Rc::clone(&self.vars),
        }
    }

    /// Return the variables of the polynomial as strings.
    #[inline]
    pub fn vars(&self) -> Vec<String> {
        self.vars.borrow().to_owned()
    }
    
    /*
    /// Change the variables of the polynomial.
    #[inline]
    pub fn set_vars<T: AsRef<String>>(&self, vars: &[T]) {
        *self.vars.write().unwrap() = var.as_ref().to_string()
    }
    
    /// Return a pretty-printed string representation of an integer polynomial.
    pub fn get_str_pretty(&self) -> String {
        let v = CString::new(self.var()).unwrap();
        unsafe {
            let s = fmpz_poly::fmpz_poly_get_str_pretty(self.as_ptr(), v.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    */

    /*
    /// Return the number of terms in the polynomial. If in canonical form, this will be the number
    /// of nonzero coefficients.
    #[inline]
    pub fn len(&self) -> i64 {
        unsafe { fmpz_mpoly::fmpz_mpoly_length(self.as_ptr(), self.ctx_as_ptr())}
    }

    // degree/degrees/total_degree
    #[inline]
    pub fn total_degree(&self) -> Integer {
        let mut res = MaybeUninit::uninit();
        unsafe { fmpz_mpoly::fmpz_mpoly_total_degree_fmpz(res.as_mut_ptr(), self.as_ptr()); }
        res
    }

    #[inline]
    pub fn get_coeff<'a, T>(&self, exp_vec: &[T]) -> Vec<Integer> 
        where T: IntoValOrRef<'a, Integer>
    {
        let mut out = Vec::default();
        unsafe {
            fmpz_mpoly::fmpz_mpoly_get_coeff_fmpz_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                i
            );
        }
        out
    }
    
    #[inline]
    pub fn set_coeff<'a, T>(&mut self, i: i64, coeff: T) where
        T: IntoValOrRef<'a, Integer>
    {
        unsafe {
            fmpz_poly::fmpz_poly_set_coeff_fmpz(self.as_mut_ptr(), i, coeff.val_or_ref().as_ptr());
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
    }*/
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
