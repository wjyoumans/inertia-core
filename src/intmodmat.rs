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

mod arith;
mod conv;

use crate::*;
use flint_sys::{fmpz_mat, fmpz_mod, fmpz_mod_mat};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::rc::Rc;
//use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
//use serde::ser::{Serialize, SerializeSeq, Serializer};

#[derive(Clone, Debug)]
pub struct IntModMatSpace {
    nrows: i64,
    ncols: i64,
    ctx: Rc<FmpzModCtx>,
}

impl Eq for IntModMatSpace {}

impl PartialEq for IntModMatSpace {
    fn eq(&self, other: &IntModMatSpace) -> bool {
        self.nrows() == other.nrows()
            && self.ncols() == other.ncols()
            && self.base_ring() == other.base_ring()
    }
}

impl fmt::Display for IntModMatSpace {
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

impl Hash for IntModMatSpace {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nrows().hash(state);
        self.ncols().hash(state);
    }
}

impl IntModMatSpace {
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

    /// Initialize the space of matrices with the given number of rows and columns.
    #[inline]
    pub fn init<'a, T>(nrows: i64, ncols: i64, n: T) -> Self
    where
        T: AsRef<Integer>,
    {
        let mut ctx = MaybeUninit::uninit();
        unsafe {
            fmpz_mod::fmpz_mod_ctx_init(ctx.as_mut_ptr(), n.as_ref().as_ptr());
            IntModMatSpace {
                nrows,
                ncols,
                ctx: Rc::new(FmpzModCtx(ctx.assume_init())),
            }
        }
    }

    #[inline]
    pub fn default(&self) -> IntModMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_mod_mat::fmpz_mod_mat_init(
                z.as_mut_ptr(),
                self.nrows(),
                self.ncols(),
                self.modulus().as_ptr(),
            );
            IntModMat {
                inner: z.assume_init(),
                ctx: Rc::clone(&self.ctx),
            }
        }
    }

    #[inline]
    pub fn new<'a, T: 'a>(&self, entries: &'a [T]) -> IntModMat
    where
        &'a T: Into<Integer>,
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
            res.set_entry(row, col, x.into());
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
    pub fn base_ring(&self) -> IntModRing {
        IntModRing {
            ctx: Rc::clone(&self.ctx),
        }
    }
}

#[derive(Debug)]
pub struct IntModMat {
    inner: fmpz_mod_mat::fmpz_mod_mat_struct,
    ctx: Rc<FmpzModCtx>,
}

impl AsRef<IntModMat> for IntModMat {
    fn as_ref(&self) -> &IntModMat {
        self
    }
}

impl<'a, T> Assign<T> for IntModMat
where
    T: AsRef<IntModMat>,
{
    fn assign(&mut self, other: T) {
        let other = other.as_ref();
        assert_eq!(self.parent(), other.parent());
        unsafe {
            fmpz_mod_mat::fmpz_mod_mat_set(self.as_mut_ptr(), other.as_ptr());
        }
    }
}

impl Clone for IntModMat {
    #[inline]
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_mod_mat::fmpz_mod_mat_init_set(z.as_mut_ptr(), self.as_ptr());
            IntModMat {
                inner: z.assume_init(),
                ctx: Rc::clone(&self.ctx),
            }
        }
    }
}

impl fmt::Display for IntModMat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for IntModMat {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpz_mod_mat::fmpz_mod_mat_clear(self.as_mut_ptr()) }
    }
}

impl Hash for IntModMat {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entries().hash(state);
    }
}

impl IntModMat {
    /// Returns a pointer to the inner [fmpz_mod_mat::fmpz_mod_mat].
    #[inline]
    pub const fn as_ptr(&self) -> *const fmpz_mod_mat::fmpz_mod_mat_struct {
        &self.inner
    }

    /// Returns a mutable pointer to the inner [fmpz_mod_mat::fmpz_mod_mat].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpz_mod_mat::fmpz_mod_mat_struct {
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
        unsafe { Integer::from_raw(*fmpz_mod::fmpz_mod_ctx_modulus(self.ctx_as_ptr())) }
    }

    #[inline]
    pub fn default<'a, T>(nrows: i64, ncols: i64, n: T) -> IntModMat
    where
        T: AsRef<Integer>,
    {
        let mut ctx = MaybeUninit::uninit();
        let mut z = MaybeUninit::uninit();
        let n = n.as_ref();
        unsafe {
            fmpz_mod::fmpz_mod_ctx_init(ctx.as_mut_ptr(), n.as_ptr());
            fmpz_mod_mat::fmpz_mod_mat_init(z.as_mut_ptr(), nrows, ncols, n.as_ptr());
            IntModMat {
                inner: z.assume_init(),
                ctx: Rc::new(FmpzModCtx(ctx.assume_init())),
            }
        }
    }

    #[inline]
    pub fn parent(&self) -> IntModMatSpace {
        IntModMatSpace {
            nrows: self.nrows(),
            ncols: self.ncols(),
            ctx: Rc::clone(&self.ctx),
        }
    }

    #[inline]
    pub fn base_ring(&self) -> IntModRing {
        IntModRing {
            ctx: Rc::clone(&self.ctx),
        }
    }

    /// Return the number of rows of the matrix.
    #[inline]
    pub fn nrows(&self) -> i64 {
        unsafe { fmpz_mod_mat::fmpz_mod_mat_nrows(self.as_ptr()) }
    }

    /// Return the number of columns of the matrix.
    #[inline]
    pub fn ncols(&self) -> i64 {
        unsafe { fmpz_mod_mat::fmpz_mod_mat_ncols(self.as_ptr()) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { fmpz_mod_mat::fmpz_mod_mat_is_empty(self.as_ptr()) != 0 }
    }

    #[inline]
    pub fn is_square(&self) -> bool {
        unsafe { fmpz_mod_mat::fmpz_mod_mat_is_square(self.as_ptr()) != 0 }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { fmpz_mod_mat::fmpz_mod_mat_is_zero(self.as_ptr()) != 0 }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { fmpz_mat::fmpz_mat_is_one(&(*self.as_ptr()).mat[0]) != 0 }
    }

    /// Get the `(i, j)`-th entry of the matrix.
    #[inline]
    pub fn get_entry(&self, i: i64, j: i64) -> IntMod {
        unsafe {
            let n = *fmpz_mod_mat::fmpz_mod_mat_entry(self.as_ptr(), i, j);
            IntMod {
                inner: n,
                ctx: Rc::clone(&self.ctx),
            }
        }
    }

    /// Set the `(i, j)`-th entry of the matrix.
    #[inline]
    pub fn set_entry<'a, T>(&mut self, i: i64, j: i64, e: T)
    where
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz_mod_mat::fmpz_mod_mat_set_entry(self.as_mut_ptr(), i, j, e.as_ref().as_ptr());
        }
    }

    /// Get a vector of the entries of the matrix.
    pub fn entries(&self) -> Vec<IntMod> {
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
}

/*
impl Serialize for IntModMat {
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

struct IntModMatVisitor {}

impl IntModMatVisitor {
    fn new() -> Self {
        IntModMatVisitor {}
    }
}

impl<'de> Visitor<'de> for IntModMatVisitor {
    type Value = IntModMat;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a IntModMat")
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

        let zm = IntModMatSpace::init(nrows, ncols);
        Ok(zm.new(&entries))
    }
}

impl<'de> Deserialize<'de> for IntModMat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(IntModMatVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::IntModMat;

    #[test]
    fn serde() {
        let x = IntModMat::from(vec![vec![1, 0], vec![0, 2]]);
        let ser = bincode::serialize(&x).unwrap();
        let y: IntModMat = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}*/
