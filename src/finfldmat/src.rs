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

use flint_sys::fq_default as fq;
use flint_sys::fq_default_mat as fq_mat;
use std::ffi::CString;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::rc::Rc;
//use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
//use serde::ser::{Serialize, SerializeSeq, Serializer};
use crate::{
    ops::Assign, FinFldElem, FiniteField, FqCtx, IntModPoly, IntModPolyRing, IntPoly, Integer,
    ValOrRef,
};

#[derive(Clone, Debug)]
pub struct FinFldMatSpace {
    nrows: i64,
    ncols: i64,
    ctx: Rc<FqCtx>,
}

impl Eq for FinFldMatSpace {}

impl PartialEq for FinFldMatSpace {
    fn eq(&self, other: &FinFldMatSpace) -> bool {
        self.nrows() == other.nrows()
            && self.ncols() == other.ncols()
            && self.base_ring() == other.base_ring()
    }
}

impl fmt::Display for FinFldMatSpace {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Space of {} by {} matrices over {}",
            self.nrows,
            self.ncols,
            self.base_ring()
        )
    }
}

impl Hash for FinFldMatSpace {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nrows().hash(state);
        self.ncols().hash(state);
    }
}

impl FinFldMatSpace {
    /// Returns a pointer to the [FLINT context][fmpz_mod::fmpz_mod_ctx_struct].
    #[inline]
    pub fn ctx_as_ptr(&self) -> &fq::fq_default_ctx_struct {
        &self.ctx.0
    }

    /// Initialize the space of matrices with the given number of rows and columns.
    #[inline]
    pub fn init<'a, P, K>(p: P, k: K, nrows: i64, ncols: i64) -> Self
    where
        P: Into<ValOrRef<'a, Integer>>,
        K: TryInto<i64>,
    {
        let p = &*p.into();
        match k.try_into() {
            Ok(k) => {
                assert!(p.is_prime());
                assert!(k > 0);

                Self::init_unchecked(p, k, nrows, ncols)
            }
            Err(_) => panic!("Input cannot be converted into a signed long!"),
        }
    }

    pub fn init_unchecked<'a, P, K>(p: P, k: K, nrows: i64, ncols: i64) -> Self
    where
        P: Into<ValOrRef<'a, Integer>>,
        K: TryInto<i64>,
    {
        match k.try_into() {
            Ok(k) => {
                let var = CString::new("o").unwrap();
                let mut ctx = MaybeUninit::uninit();
                unsafe {
                    fq::fq_default_ctx_init(ctx.as_mut_ptr(), p.into().as_ptr(), k, var.as_ptr());
                    FinFldMatSpace {
                        nrows,
                        ncols,
                        ctx: Rc::new(FqCtx(ctx.assume_init())),
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
    pub fn default(&self) -> FinFldMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fq_mat::fq_default_mat_init(
                z.as_mut_ptr(),
                self.nrows(),
                self.ncols(),
                self.ctx_as_ptr(),
            );
            FinFldMat {
                inner: z.assume_init(),
                ctx: Rc::clone(&self.ctx),
            }
        }
    }

    #[inline]
    pub fn new<'a, T: 'a>(&self, entries: &'a [T]) -> FinFldMat
    where
        &'a T: Into<ValOrRef<'a, IntPoly>>,
    {
        let nrows = self.nrows() as usize;
        let ncols = self.ncols() as usize;
        assert_eq!(entries.len(), nrows * ncols);

        let mut row = 0;
        let mut col;
        let mut res = self.default();
        for (i, x) in entries.iter().enumerate() {
            col = (i % ncols) as i64;
            if col == 0 && i != 0 {
                row += 1;
            }
            res.set_entry(row, col, x);
        }
        res
    }

    #[inline]
    pub fn nrows(&self) -> i64 {
        self.nrows
    }

    #[inline]
    pub fn ncols(&self) -> i64 {
        self.ncols
    }

    #[inline]
    pub fn base_ring(&self) -> FiniteField {
        FiniteField {
            ctx: Rc::clone(&self.ctx),
        }
    }
}

#[derive(Debug)]
pub struct FinFldMat {
    inner: fq_mat::fq_default_mat_struct,
    ctx: Rc<FqCtx>,
}

impl<'a, T> Assign<T> for FinFldMat
where
    T: Into<ValOrRef<'a, FinFldMat>>,
{
    fn assign(&mut self, other: T) {
        let x = other.into();
        assert_eq!(self.parent(), x.parent());
        unsafe {
            fq_mat::fq_default_mat_set(self.as_mut_ptr(), x.as_ptr(), self.ctx_as_ptr());
        }
    }
}

impl Clone for FinFldMat {
    #[inline]
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fq_mat::fq_default_mat_init_set(z.as_mut_ptr(), self.as_ptr(), self.ctx_as_ptr());
            FinFldMat {
                inner: z.assume_init(),
                ctx: Rc::clone(&self.ctx),
            }
        }
    }
}

impl fmt::Display for FinFldMat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for FinFldMat {
    #[inline]
    fn drop(&mut self) {
        unsafe { fq_mat::fq_default_mat_clear(self.as_mut_ptr(), self.ctx_as_ptr()) }
    }
}

impl Hash for FinFldMat {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entries().hash(state);
    }
}

impl FinFldMat {
    /// Returns a pointer to the inner [fq_mat::fq_default_mat_struct].
    #[inline]
    pub const fn as_ptr(&self) -> *const fq_mat::fq_default_mat_struct {
        &self.inner
    }

    /// Returns a mutable pointer to the inner [fq_mat::fq_default_mat_struct].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fq_mat::fq_default_mat_struct {
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

    #[inline]
    pub fn parent(&self) -> FinFldMatSpace {
        FinFldMatSpace {
            nrows: self.nrows(),
            ncols: self.ncols(),
            ctx: Rc::clone(&self.ctx),
        }
    }

    #[inline]
    pub fn base_ring(&self) -> FiniteField {
        FiniteField {
            ctx: Rc::clone(&self.ctx),
        }
    }

    /// Return the number of rows of the matrix.
    #[inline]
    pub fn nrows(&self) -> i64 {
        unsafe { fq_mat::fq_default_mat_nrows(self.as_ptr(), self.ctx_as_ptr()) }
    }

    /// Return the number of columns of the matrix.
    #[inline]
    pub fn ncols(&self) -> i64 {
        unsafe { fq_mat::fq_default_mat_ncols(self.as_ptr(), self.ctx_as_ptr()) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { fq_mat::fq_default_mat_is_empty(self.as_ptr(), self.ctx_as_ptr()) != 0 }
    }

    #[inline]
    pub fn is_square(&self) -> bool {
        unsafe { fq_mat::fq_default_mat_is_square(self.as_ptr(), self.ctx_as_ptr()) != 0 }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { fq_mat::fq_default_mat_is_zero(self.as_ptr(), self.ctx_as_ptr()) != 0 }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { fq_mat::fq_default_mat_is_one(self.as_ptr(), self.ctx_as_ptr()) != 0 }
    }

    /*
    /// Get a shallow copy of the `(i, j)`-th entry of the matrix. Mutating this modifies
    /// the entry of the matrix.
    #[inline]
    pub fn entry_copy(&self, i: i64, j: i64) -> ManuallyDrop<IntMod> {
        unsafe {
            let x = fmpz_mod_mat::fmpz_mod_mat_entry(self.as_ptr(), i, j);
            //fmpz::fmpz_mod(x, x, self.modulus_copy().as_ptr());
            ManuallyDrop::new(IntMod { inner: *x, ctx: Rc::downgrade(&Rc::clone(&self.ctx)) })
        }
    }*/

    /// Get a deep copy of the `(i, j)`-th entry of the matrix.
    #[inline]
    pub fn get_entry(&self, i: i64, j: i64) -> FinFldElem {
        let mut res = self.base_ring().default();
        unsafe {
            fq_mat::fq_default_mat_entry(res.as_mut_ptr(), self.as_ptr(), i, j, self.ctx_as_ptr());
        }
        res
    }

    /// Set the `(i, j)`-th entry of the matrix.
    #[inline]
    pub fn set_entry<'a, T>(&mut self, i: i64, j: i64, e: T)
    where
        T: Into<ValOrRef<'a, IntPoly>>,
    {
        let x = self.base_ring().new(e);
        unsafe {
            fq_mat::fq_default_mat_entry_set(
                self.as_mut_ptr(),
                i,
                j,
                x.as_ptr(),
                self.ctx_as_ptr(),
            );
        }
    }

    /// Get a deep copy of the entries of the matrix.
    pub fn entries(&self) -> Vec<FinFldElem> {
        let r = self.nrows();
        let c = self.ncols();
        let mut out = Vec::with_capacity(usize::try_from(r * c).ok().unwrap());

        for i in 0..r {
            for j in 0..c {
                out.push(self.get_entry(i, j));
            }
        }
        out
    }

    /*
    /// Get a shallow copy of the entries of the matrix.
    pub fn entries_copy(&self) -> Vec<ManuallyDrop<IntMod>> {
        let r = self.nrows();
        let c = self.ncols();
        let mut out = Vec::with_capacity(usize::try_from(r * c).ok().unwrap());

        for i in 0..r {
            for j in 0..c {
                out.push(self.entry_copy(i, j));
            }
        }
        out
    }*/
}

/*
impl Serialize for FinFldMat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entries = self.entries();
        let mut seq = serializer.serialize_seq(Some(entries.len() + 2))?;

        seq.serialize_element(&self.nrows())?;
        seq.serialize_element(&self.ncols())?;
        for e in entries.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

struct FinFldMatVisitor {}

impl FinFldMatVisitor {
    fn new() -> Self {
        FinFldMatVisitor {}
    }
}

impl<'de> Visitor<'de> for FinFldMatVisitor {
    type Value = FinFldMat;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a FinFldMat")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut entries: Vec<Integer> = Vec::with_capacity(access.size_hint().unwrap_or(0));
        let nrows = access.next_element()?.unwrap();
        let ncols = access.next_element()?.unwrap();

        while let Some(x) = access.next_element()? {
            entries.push(x);
        }

        let zm = FinFldMatSpace::init(nrows, ncols);
        Ok(zm.new(&entries))
    }
}

impl<'de> Deserialize<'de> for FinFldMat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(FinFldMatVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::FinFldMat;

    #[test]
    fn serde() {
        let x = FinFldMat::from(vec![vec![1, 0], vec![0, 2]]);
        let ser = bincode::serialize(&x).unwrap();
        let y: FinFldMat = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}*/
