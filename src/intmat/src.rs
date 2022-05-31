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

use crate::{ops::Assign, Integer, IntegerRing, IntPoly, RatMat, ValOrRef};
use flint_sys::{fmpz, fmpz_mat, fmpq_mat};
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct IntMatSpace {
    nrows: i64,
    ncols: i64,
}

impl Eq for IntMatSpace {}

impl PartialEq for IntMatSpace {
    fn eq(&self, other: &IntMatSpace) -> bool {
        self.nrows() == other.nrows() && self.ncols() == other.ncols()
    }
}

impl fmt::Display for IntMatSpace {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Space of {} by {} matrices over Integer Ring",
            self.nrows, self.ncols
        )
    }
}

impl Hash for IntMatSpace {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nrows().hash(state);
        self.ncols().hash(state);
    }
}

impl IntMatSpace {
    /// Initialize the space of matrices with the given number of rows and columns.
    #[inline]
    pub fn init(nrows: i64, ncols: i64) -> Self where 
    {
        IntMatSpace { nrows, ncols }
    }

    #[inline]
    pub fn default(&self) -> IntMat {
        IntMat::default(self.nrows, self.ncols)
    }

    #[inline]
    pub fn new<'a, T: 'a>(&self, entries: &'a [T]) -> IntMat
    where
        &'a T: Into<ValOrRef<'a, Integer>>,
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
    pub fn base_ring(&self) -> IntegerRing {
        IntegerRing {}
    }
}

#[derive(Debug)]
pub struct IntMat {
    inner: fmpz_mat::fmpz_mat_struct,
}

impl<'a, T> Assign<T> for IntMat
where
    T: Into<ValOrRef<'a, IntMat>>,
{
    fn assign(&mut self, other: T) {
        let x = other.into();
        assert_eq!(self.nrows(), x.nrows());
        assert_eq!(self.ncols(), x.ncols());
        unsafe {
            fmpz_mat::fmpz_mat_set(self.as_mut_ptr(), x.as_ptr());
        }
    }
}

impl Clone for IntMat {
    #[inline]
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_mat::fmpz_mat_init_set(z.as_mut_ptr(), self.as_ptr());
            IntMat {
                inner: z.assume_init(),
            }
        }
    }
}

impl fmt::Display for IntMat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for IntMat {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpz_mat::fmpz_mat_clear(self.as_mut_ptr()) }
    }
}

impl Hash for IntMat {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entries().hash(state);
    }
}

impl IntMat {
    /// Returns a pointer to the inner [FLINT integer matrix][fmpz_mat::fmpz_mat].
    #[inline]
    pub const fn as_ptr(&self) -> *const fmpz_mat::fmpz_mat_struct {
        &self.inner
    }

    /// Returns a mutable pointer to the inner 
    /// [FLINT integer matrix][fmpz_mat::fmpz_mat].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpz_mat::fmpz_mat_struct {
        &mut self.inner
    }

    /// Instantiate an integer matrix from a 
    /// [FLINT integer matrix][fmpz_mat::fmpz_mat_struct].
    #[inline]
    pub fn from_raw(raw: fmpz_mat::fmpz_mat_struct) -> IntMat {
        IntMat { inner: raw }
    }

    #[inline]
    pub fn default(nrows: i64, ncols: i64) -> IntMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_mat::fmpz_mat_init(z.as_mut_ptr(), nrows, ncols);
            IntMat {
                inner: z.assume_init(),
            }
        }
    }

    #[inline]
    pub fn parent(&self) -> IntMatSpace {
        IntMatSpace {
            nrows: self.nrows(),
            ncols: self.ncols(),
        }
    }

    #[inline]
    pub fn base_ring(&self) -> IntegerRing {
        IntegerRing {}
    }

    /// Return the number of rows of the integer matrix.
    #[inline]
    pub fn nrows(&self) -> i64 {
        unsafe { fmpz_mat::fmpz_mat_nrows(self.as_ptr()) }
    }

    /// Return the number of columns of the integer matrix.
    #[inline]
    pub fn ncols(&self) -> i64 {
        unsafe { fmpz_mat::fmpz_mat_ncols(self.as_ptr()) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { fmpz_mat::fmpz_mat_is_empty(self.as_ptr()) != 0 }
    }

    #[inline]
    pub fn is_square(&self) -> bool {
        unsafe { fmpz_mat::fmpz_mat_is_square(self.as_ptr()) != 0 }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { fmpz_mat::fmpz_mat_is_zero(self.as_ptr()) != 0 }
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { fmpz_mat::fmpz_mat_is_one(self.as_ptr()) != 0 }
    }

    /// Get the `(i, j)`-th entry of the matrix.
    #[inline]
    pub fn get_entry<S>(&self, i: S, j: S) -> Integer where 
        S: TryInto<i64>,
        <S as TryInto<i64>>::Error: fmt::Debug
    {
        let i = i.try_into().unwrap();
        let j = j.try_into().unwrap();
        unsafe { 
            Integer::from_raw(*fmpz_mat::fmpz_mat_entry(self.as_ptr(), i, j)) 
        }
    }
    
    /// Get the `(i, j)`-th entry of an integer matrix and assign it to `out`. 
    /// Avoids unnecessary allocation.
    #[inline]
    pub fn get_entry_assign<S>(&self, i: S, j: S, out: &mut Integer) where
        S: TryInto<i64>,
        <S as TryInto<i64>>::Error: fmt::Debug
    {
        unsafe {
            let x = fmpz_mat::fmpz_mat_entry(
                self.as_ptr(), 
                i.try_into().unwrap(), 
                j.try_into().unwrap()
            );
            fmpz::fmpz_set(out.as_mut_ptr(), x);
        }
    }

    /// Set the `(i, j)`-th entry of the matrix.
    #[inline]
    pub fn set_entry<'a, T>(&mut self, i: i64, j: i64, e: T)
    where
        T: Into<ValOrRef<'a, Integer>>,
    {
        unsafe {
            let x = fmpz_mat::fmpz_mat_entry(self.as_ptr(), i, j);
            fmpz::fmpz_set(x, e.into().as_ptr());
        }
    }

    /// Get a vector with all of the entries of the matrix.
    pub fn entries(&self) -> Vec<Integer> {
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

    /// Swap two integer matrices. The dimensions are allowed to be different.
    #[inline]
    pub fn swap(&mut self, other: &mut IntMat) {
        unsafe { 
            fmpz_mat::fmpz_mat_swap(self.as_mut_ptr(), other.as_mut_ptr()); 
        }
    }

    /// Swap the rows `r` and `s` of an integer matrix. 
    pub fn swap_rows<S>(&mut self, r: S, s: S) where 
        S: TryInto<i64>,
        <S as TryInto<i64>>::Error: fmt::Debug
    {
        let r = r.try_into().unwrap();
        let s = s.try_into().unwrap();
        assert!(r < self.nrows());
        assert!(s < self.nrows());

        unsafe { 
            fmpz_mat::fmpz_mat_swap_rows(
                self.as_mut_ptr(), 
                std::ptr::null(),
                r,
                s
            ); 
        }
    }
    
    /// Swap the columns `r` and `s` of an integer matrix. 
    pub fn swap_cols<S>(&mut self, r: S, s: S) where 
        S: TryInto<i64>,
        <S as TryInto<i64>>::Error: fmt::Debug
    {
        let r = r.try_into().unwrap();
        let s = s.try_into().unwrap();
        assert!(r < self.ncols());
        assert!(s < self.ncols());

        unsafe { 
            fmpz_mat::fmpz_mat_swap_rows(
                self.as_mut_ptr(), 
                std::ptr::null(),
                r,
                s
            ); 
        }
    }
    
    /// Swap row `i` and `r - i` for `0 <= i < r/2` where `r` is the number 
    /// of rows of the input matrix.
    #[inline]
    pub fn invert_rows(&mut self) {
        unsafe { 
            fmpz_mat::fmpz_mat_invert_rows(
                self.as_mut_ptr(), 
                std::ptr::null()
            ); 
        }
    }
    
    /// Swap columns `i` and `c - i` for `0 <= i < c/2` where `c` is the number
    /// of columns of the input matrix.
    #[inline]
    pub fn invert_columns(&mut self) {
        unsafe { 
            fmpz_mat::fmpz_mat_invert_cols(
                self.as_mut_ptr(), 
                std::ptr::null()
            ); 
        }
    }
   
    /* TODO: function missing from bindings
    /// Swap two integer matrices by swapping the individual entries rather 
    /// than swapping the contents of their structs.
    #[inline]
    pub fn swap_entrywise(&mut self, other: &mut IntMat) {
        unsafe { 
            fmpz_mat::fmpz_mat_swap_entrywise(
                self.as_mut_ptr(), 
                other.as_mut_ptr()
            ); 
        }
    }
    */

    /// Set `self` to the zero matrix.
    #[inline]
    pub fn zero(&mut self) {
        unsafe {
            fmpz_mat::fmpz_mat_zero(self.as_mut_ptr());
        }
    }

    /*
    /// Return true if the matrix is invertible.
    #[inline]
    pub fn is_invertible(&self) -> bool {
        self.is_square() && !self.det().is_zero()
    }*/

    /// Return true if row `i` is all zeros.
    pub fn is_zero_row<S>(&self, i: S) -> bool where 
        S: TryInto<i64>,
        <S as TryInto<i64>>::Error: fmt::Debug
    {
        unsafe {
            fmpz_mat::fmpz_mat_is_zero_row(
                self.as_ptr(), 
                i.try_into().unwrap()
            ) != 0
        }
    }

    /*
    /// Return true if column `i` is all zeros.
    // TODO: Does an additional allocation compared to `is_zero_row`.
    #[inline]
    pub fn is_zero_col(&self, i: usize) -> bool {
        self.col(i).is_zero()
    }*/


    /// Return the transpose of an integer matrix.
    #[inline]
    pub fn transpose(&self) -> IntMat {
        let mut res = self.parent().default();
        unsafe {
            fmpz_mat::fmpz_mat_transpose(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }

    /// Compute the transpose of a square integer matrix in place.
    #[inline]
    pub fn transpose_assign(&mut self) {
        assert!(self.is_square());
        unsafe { fmpz_mat::fmpz_mat_transpose(self.as_mut_ptr(), self.as_ptr()); }
    }
    
    /// Return the matrix obtained by horizontally concatenating `self` with 
    /// `other` in that order. The number of rows of both matrices must agree.
    pub fn hcat<'a, T>(&self, other: T) -> IntMat where
        T: Into<ValOrRef<'a, IntMat>>
    {
        let other = other.into();
        assert_eq!(self.nrows(), other.nrows());

        let mut res = MaybeUninit::uninit();
        unsafe {
            fmpz_mat::fmpz_mat_init(
                res.as_mut_ptr(), 
                self.nrows(), 
                self.ncols() + other.ncols()
            );
            fmpz_mat::fmpz_mat_concat_horizontal(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
            IntMat { inner: res.assume_init() }
        }
    }
    
    /// Return the matrix obtained by vertically concatenating `self` with 
    /// `other` in that order. The number of columns of both matrices must agree.
    pub fn vcat<'a, T>(&self, other: T) -> IntMat where
        T: Into<ValOrRef<'a, IntMat>>
    {
        let other = other.into();
        assert_eq!(self.ncols(), other.ncols());

        let mut res = MaybeUninit::uninit();
        unsafe {
            fmpz_mat::fmpz_mat_init(
                res.as_mut_ptr(), 
                self.nrows() + other.nrows(), 
                self.ncols()
            );
            fmpz_mat::fmpz_mat_concat_vertical(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
            IntMat { inner: res.assume_init() }
        }
    }
   
    /// Return a new matrix containing the `r2 - r1` by `c2 - c1` submatrix of 
    /// an integer matrix whose `(0, 0)` entry is the `(r1, c1)` entry of the input.
    pub fn submatrix<S>(&self, r1: S, c1: S, r2: S, c2: S) -> IntMat where 
        S: TryInto<i64>,
        <S as TryInto<i64>>::Error: fmt::Debug
    {
        let r1 = r1.try_into().unwrap();
        let r2 = r2.try_into().unwrap();
        let c1 = c1.try_into().unwrap();
        let c2 = c2.try_into().unwrap();

        assert!((r2+r1) <= self.nrows());
        assert!((c2+c1) <= self.ncols());

        let mut res = MaybeUninit::uninit();
        let mut win = MaybeUninit::uninit();
        unsafe {
            fmpz_mat::fmpz_mat_init(
                res.as_mut_ptr(), 
                r2 - r1, 
                c2 - c1
            );
            fmpz_mat::fmpz_mat_window_init(
                win.as_mut_ptr(), 
                self.as_ptr(),
                r1,
                c1,
                r2,
                c2
            );
            fmpz_mat::fmpz_mat_set(res.as_mut_ptr(), win.as_ptr());
            fmpz_mat::fmpz_mat_window_clear(win.as_mut_ptr());
            IntMat { inner: res.assume_init() }
        }
    }
    
    /// Return row `i` as an integer matrix.
    #[inline]
    pub fn row<S>(&self, i: i64) -> IntMat where 
        S: TryInto<i64>,
        <S as TryInto<i64>>::Error: fmt::Debug
    {
        let i = i.try_into().unwrap();
        self.submatrix(i, 0, i + 1, self.ncols())
    }
   
    /// Return column `j` as an integer matrix.
    #[inline]
    pub fn col<S>(&self, j: S) -> IntMat where
        S: TryInto<i64>,
        <S as TryInto<i64>>::Error: fmt::Debug
    {
        let j = j.try_into().unwrap();
        self.submatrix(0, j, self.nrows(), j + 1)
    }

    /// Return the square of an integer matrix. The matrix must be square.
    #[inline]
    pub fn square(&self) -> Self {
        assert!(self.is_square());
        let mut res = self.parent().default();
        unsafe { 
            fmpz_mat::fmpz_mat_sqr(res.as_mut_ptr(), self.as_ptr()) 
        }
        res
    }
    
    /// Return the square of an integer matrix. The matrix must be square.
    #[inline]
    pub fn square_assign(&mut self) {
        assert!(self.is_square());
        unsafe { 
            fmpz_mat::fmpz_mat_sqr(self.as_mut_ptr(), self.as_ptr());
        }
    }
    
    /// Return the kronecker product of two integer matrices.
    pub fn kronecker_product<'a, T>(&self, other: T) -> IntMat where 
        T: Into<ValOrRef<'a, IntMat>>
    {
        let other = other.into();
        let mut res = MaybeUninit::uninit();
        unsafe { 
            fmpz_mat::fmpz_mat_init(
                res.as_mut_ptr(), 
                self.nrows()*other.nrows(), 
                self.ncols()*other.ncols()
            );
            fmpz_mat::fmpz_mat_kronecker_product(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            ); 
            IntMat { inner: res.assume_init() }
        }
    }
    
    /// Compute the trace of a square integer matrix.
    #[inline]
    pub fn trace(&self) -> Integer {
        assert!(self.is_square());
        let mut res = Integer::default();
        unsafe { 
            fmpz_mat::fmpz_mat_trace(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }

    /// Return the content of an integer matrix, that is, the gcd of all its 
    /// entries. Returns zero if the matrix is empty.
    #[inline]
    pub fn content(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { 
            fmpz_mat::fmpz_mat_content(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    /// Compute the determinant of the matrix.
    #[inline]
    pub fn det(&self) -> Integer {
        assert!(self.is_square());
        let mut res = Integer::default();
        unsafe { 
            fmpz_mat::fmpz_mat_det(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    /// Return an absolute upper bound on the determinant of a square integer 
    /// matrix computed from the Hadamard inequality.
    #[inline]
    pub fn det_bound(&self) -> Integer {
        assert!(self.is_square());
        let mut res = Integer::default();
        unsafe { 
            fmpz_mat::fmpz_mat_det_bound(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    /// Return a positive divisor of the determinant of a square integer matrix. 
    /// If the determinant is zero this will always return zero.
    #[inline]
    pub fn det_divisor(&self) -> Integer {
        assert!(self.is_square());
        let mut res = Integer::default();
        unsafe { 
            fmpz_mat::fmpz_mat_det_divisor(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    /// Applies a similarity transform to an `n` by `n` integer matrix. If `P` 
    /// is the identity matrix whose zero entries in row `r` have been replaced 
    /// by `d`, this transform is equivalent to `P^-1 * M * P`. 
    pub fn similarity<'a, S, T>(&self, r: S, d: T) -> IntMat where 
        S: TryInto<i64>,
        <S as TryInto<i64>>::Error: fmt::Debug,
        T: Into<ValOrRef<'a, Integer>>
    {
        assert!(self.is_square());
        let mut res = self.clone();
        unsafe { 
            fmpz_mat::fmpz_mat_similarity(
                res.as_mut_ptr(), 
                r.try_into().unwrap(), 
                d.into().as_ptr()
            ); 
        }
        res
    }
    
    /// Applies a similarity transform to an `n` by `n` integer matrix in place.
    pub fn similarity_assign<'a, S, T>(&mut self, r: S, d: T) where 
        S: TryInto<i64>,
        <S as TryInto<i64>>::Error: fmt::Debug,
        T: Into<ValOrRef<'a, Integer>>
    {
        assert!(self.is_square());
        unsafe { 
            fmpz_mat::fmpz_mat_similarity(
                self.as_mut_ptr(), 
                r.try_into().unwrap(), 
                d.into().as_ptr()
            ); 
        }
    }
  
    /// Return the characteristic polynomial of a square integer matrix.
    #[inline]
    pub fn charpoly(&self) -> IntPoly {
        assert!(self.is_square());
        let mut res = IntPoly::default();
        unsafe { 
            fmpz_mat::fmpz_mat_charpoly(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    /// Return the minimal polynomial of a square integer matrix.
    #[inline]
    pub fn minpoly(&self) -> IntPoly {
        assert!(self.is_square());
        let mut res = IntPoly::default();
        unsafe { 
            fmpz_mat::fmpz_mat_minpoly(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }

    /// Return the rank of a matrix, that is, the number of linearly independent 
    /// columns (equivalently, rows) of an integer matrix. The rank is computed by 
    /// row reducing a copy of the input matrix.
    #[inline]
    pub fn rank(&self) -> i64 {
        unsafe { fmpz_mat::fmpz_mat_rank(self.as_ptr()) }
    }

    /// Solve `AX = B` for nonsingular `A`.
    pub fn solve<'a, T>(&self, rhs: T) -> Option<RatMat> where 
        T: Into<ValOrRef<'a, IntMat>>
    {
        let b = rhs.into();
        assert_eq!(self.nrows(), b.nrows());

        let mut res = MaybeUninit::uninit();
        unsafe { 
            fmpq_mat::fmpq_mat_init(
                res.as_mut_ptr(),
                self.ncols(),
                b.ncols()
            );
            let x = fmpq_mat::fmpq_mat_solve_fmpz_mat(
                res.as_mut_ptr(), 
                self.as_ptr(),
                b.as_ptr()
            );
            if x == 0 {
                None
            } else {
                Some(RatMat::from_raw(res.assume_init()))
            }
        }
    }

    /*
    pub fn solve_fraction_free<'a, T>(&self, B: &'a T) -> Option<RatMat> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());

        let mut res = RatMat::zero(self.ncols(), B.ncols());
        unsafe { 
            let x = flint_sys::fmpq_mat::fmpq_mat_solve_fmpz_mat_fraction_free(
                res.as_mut_ptr(), 
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 0 {
                None
            } else {
                Some(res)
            }
        }
    }
    
    pub fn solve_dixon<'a, T>(&self, B: &'a T) -> Option<RatMat> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());

        let mut res = RatMat::zero(self.ncols(), B.ncols());
        unsafe { 
            let x = flint_sys::fmpq_mat::fmpq_mat_solve_fmpz_mat_dixon(
                res.as_mut_ptr(), 
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 0 {
                None
            } else {
                Some(res)
            }
        }
    }
    
    pub fn solve_multi_mod<'a, T>(&self, B: &'a T) -> Option<RatMat> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());

        let mut res = RatMat::zero(self.ncols(), B.ncols());
        unsafe { 
            let x = flint_sys::fmpq_mat::fmpq_mat_solve_fmpz_mat_multi_mod(
                res.as_mut_ptr(), 
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 0 {
                None
            } else {
                Some(res)
            }
        }
    }
    
    pub fn solve_fflu<'a, T>(&self, B: &'a T) -> Option<RatMat> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());

        let mut res = IntMat<'a>::zero(self.ncols(), B.ncols());
        let mut den = Integer::default();
        unsafe { 
            let x = flint_sys::flint_sys::fmpz_mat::fmpz_mat_solve_fflu(
                res.as_mut_ptr(),
                den.as_mut_ptr(),
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 0 {
                None
            } else {
                Some(res/den)
            }
        }
    }
    
    pub fn solve_cramer<'a, T>(&self, B: &'a T) -> Option<RatMat> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());

        let mut res = IntMat<'a>::zero(self.ncols(), B.ncols());
        let mut den = Integer::default();
        unsafe { 
            let x = flint_sys::flint_sys::fmpz_mat::fmpz_mat_solve_cramer(
                res.as_mut_ptr(), 
                den.as_mut_ptr(),
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 0 {
                None
            } else {
                Some(res/den)
            }
        }
    }
    
    pub fn can_solve<'a, T>(&self, B: &'a T) -> Option<RatMat> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());
        
        let mut res = IntMat<'a>::zero(self.ncols(), 1);
        let mut den = Integer::default();
        unsafe { 
            let x = flint_sys::fmpz_mat::fmpz_mat_can_solve(
                res.as_mut_ptr(), 
                den.as_mut_ptr(),
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 1 {
                Some(res/den)
            } else {
                None
            }
        }
    }
    
    pub fn can_solve_fflu<'a, T>(&self, B: &'a T) -> Option<RatMat> where &'a T: Into<IntMat<'a>> {
        let B = B.into();
        assert_eq!(self.nrows(), B.nrows());
        
        let mut res = IntMat<'a>::zero(self.ncols(), 1);
        let mut den = Integer::default();
        unsafe { 
            let x = flint_sys::fmpz_mat::fmpz_mat_can_solve_fflu(
                res.as_mut_ptr(), 
                den.as_mut_ptr(),
                self.as_ptr(),
                B.as_ptr()
            );
            if x == 1 {
                Some(res/den)
            } else {
                None
            }
        }
    }

    pub fn solve_bound(&self, B: &IntMat<'a>) -> (Integer, Integer) {
        let mut N = Integer::default();
        let mut D = Integer::default();
        
        unsafe {
            flint_sys::fmpz_mat::fmpz_mat_solve_bound(
                N.as_mut_ptr(), 
                D.as_mut_ptr(), 
                self.as_ptr(), 
                B.as_ptr()
            );
        }
        (N, D)
    }
    */

    /// Return the rank and (A, den) a fraction-free LU decomposition of the input.
    pub fn fflu(&self) -> (i64, IntMat, Integer) {
        let mut res = self.parent().default();
        let mut den = Integer::default();

        unsafe {
            let rank = fmpz_mat::fmpz_mat_fflu(
                res.as_mut_ptr(), 
                den.as_mut_ptr(), 
                std::ptr::null(), 
                self.as_ptr(), 
                0
            );
            (rank, res, den)
        }
    }
   
    pub fn rref(&self) -> (i64, IntMat, Integer) {
        let mut res = self.parent().default();
        let mut den = Integer::default();

        unsafe {
            let rank = fmpz_mat::fmpz_mat_rref(
                res.as_mut_ptr(), 
                den.as_mut_ptr(), 
                self.as_ptr()
            );
            (rank, res, den)
        }
    }
    
    pub fn rref_mod<'a, T>(&self, modulus: T) -> (i64, IntMat) where 
        T: Into<ValOrRef<'a, Integer>> 
    {
        let mut res = self.parent().default();
        unsafe {
            let rank = fmpz_mat::fmpz_mat_rref_mod(
                std::ptr::null_mut(),
                res.as_mut_ptr(),
                modulus.into().as_ptr()
            );
            (rank, res)
        }
    }

    /*
    pub fn gram_schmidt(&self) -> RatMat {
        RatMat::from(self).gram_schmidt()
    }*/

    pub fn strong_echelon_form_mod<'a, T>(&self, modulus: T) -> IntMat where 
        T: Into<ValOrRef<'a, Integer>>
    {
        let mut res = self.parent().default();
        unsafe {
            fmpz_mat::fmpz_mat_strong_echelon_form_mod(
                res.as_mut_ptr(),
                modulus.into().as_ptr()
            );
        }
        res
    }
    
    pub fn howell_form_mod<'a, T>(&self, modulus: T) -> (i64, IntMat) where 
        T: Into<ValOrRef<'a, Integer>>
    {
        assert!(self.ncols() <= self.nrows());
        let mut res = self.parent().default();
        unsafe {
            let rank = fmpz_mat::fmpz_mat_howell_form_mod(
                res.as_mut_ptr(),
                modulus.into().as_ptr()
            );
            (rank, res)
        }
    }
  
    /*
    // TODO: get rows/cols of nullspace first
    // WHY SUBMATRIX
    pub fn nullspace(&self) -> IntMat {
        let mut res = MaybeUninit::uninit();

        unsafe {
            fmpz_mat::fmpz_mat_init(
                res.as_mut_ptr(),
                self.nrows(),
                self.ncols()
            );
            let rank = fmpz_mat::fmpz_mat_nullspace(
                res.as_mut_ptr(),
                self.as_ptr()
            );
            res.submatrix(0, 0, res.nrows(), rank)
        }
    }*/
    
    pub fn hnf(&self) -> IntMat {
        let mut res = self.parent().default();
        unsafe { 
            fmpz_mat::fmpz_mat_hnf(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    pub fn hnf_transform(&self) -> (IntMat, IntMat) {
        let mut h = self.parent().default();
        let mut u = self.parent().default();
        unsafe { 
            fmpz_mat::fmpz_mat_hnf_transform(
                h.as_mut_ptr(), 
                u.as_mut_ptr(), 
                self.as_ptr()
            ); 
        }
        (h, u)
    }
    
    pub fn is_hnf(&self) -> bool {
        unsafe { fmpz_mat::fmpz_mat_is_in_hnf(self.as_ptr()) == 1 }
    }
    
    pub fn snf(&self) -> IntMat {
        let mut res = self.parent().default();
        unsafe { fmpz_mat::fmpz_mat_snf(res.as_mut_ptr(), self.as_ptr()); }
        res
    }
    
    pub fn is_snf(&self) -> bool {
        unsafe { fmpz_mat::fmpz_mat_is_in_snf(self.as_ptr()) == 1 }
    }
    
    /*
    pub fn gram(&self) -> IntMat<'a> {
        let mut B = IntMat<'a>::zero(self.nrows(), self.ncols());
        unsafe { flint_sys::fmpz_mat::fmpz_mat_gram(B.as_mut_ptr(), self.as_ptr()); }
        B
    }

    pub fn is_hadamard(&self) -> bool {
        unsafe { flint_sys::fmpz_mat::fmpz_mat_is_hadamard(self.as_ptr()) != 0 }
    }

    pub fn hadamard(n: c_long) -> IntMat<'a> {
        let mut H = IntMat<'a>::zero(n, n);
        unsafe { flint_sys::fmpz_mat::fmpz_mat_hadamard(H.as_mut_ptr());}
        H
    }
   
    pub fn chol_d(&self) -> IntMat<'a> {
        assert!(self.is_symmetric());
        assert!(self.is_positive_definite());
        let mut R = IntMat<'a>::zero(?, ?);
        unsafe { flint_sys::fmpz_mat::fmpz_mat_chol_d(R.as_mut_ptr(), self.as_ptr());}
        R
    }
   
    // TODO: default delta/eta? 
    pub fn lll<'b, T>(&self, delta: &'b T, eta: &'b T) -> IntMat<'a> where &'b T: Into<Rational> {
        let mut B = self.clone();
        unsafe { 
            flint_sys::fmpz_mat::fmpz_mat_lll_storjohann(
                B.as_mut_ptr(), 
                delta.into().as_ptr(), 
                eta.into().as_ptr()
            );
        }
        B
    }
    
    pub fn lll_original<'b, T>(&self, delta: &'b T, eta: &'b T) -> IntMat<'a> where &'b T: Into<Rational> {
        let mut B = self.clone();
        unsafe { 
            flint_sys::fmpz_mat::fmpz_mat_lll_original(
                B.as_mut_ptr(), 
                delta.into().as_ptr(), 
                eta.into().as_ptr()
            );
        }
        B
    }

    pub fn rational_reconstruction<'a, T>(&self, modulus: &'a T) -> RatMat where &'a T: Into<Integer> {
        let mut res = RatMat::from(self);
        unsafe {
            flint_sys::fmpq_mat::fmpq_mat_set_fmpz_mat_mod_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                modulus.into().as_ptr()
            );
        }
        res
    }
    */
}

impl Serialize for IntMat {
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

struct IntMatVisitor {}

impl IntMatVisitor {
    fn new() -> Self {
        IntMatVisitor {}
    }
}

impl<'de> Visitor<'de> for IntMatVisitor {
    type Value = IntMat;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an IntMat")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut entries: Vec<Integer> = Vec::with_capacity(
            access.size_hint().unwrap_or(0));
        let nrows: i64 = access.next_element()?.unwrap();
        let ncols: i64 = access.next_element()?.unwrap();

        while let Some(x) = access.next_element()? {
            entries.push(x);
        }

        let zm = IntMatSpace::init(nrows, ncols);
        Ok(zm.new(&entries))
    }
}

impl<'de> Deserialize<'de> for IntMat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(IntMatVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::IntMat;

    #[test]
    fn serde() {
        let x = IntMat::from(vec![vec![1, 0], vec![0, 2]]);
        let ser = bincode::serialize(&x).unwrap();
        let y: IntMat = bincode::deserialize(&ser).unwrap();
        assert_eq!(x, y);
    }
}
