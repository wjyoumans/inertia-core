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

mod ops;
mod conv;

//#[cfg(feature = "serde")]
//mod serde;

use crate::*;
use flint_sys::fmpz_mod_mat::*;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;


#[derive(Debug)]
pub struct IntModMat {
    inner: fmpz_mod_mat_struct,
    ctx: IntModCtx
}

impl AsRef<IntModMat> for IntModMat {
    fn as_ref(&self) -> &IntModMat {
        self
    }
}

impl Clone for IntModMat {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_mod_mat_init_set(z.as_mut_ptr(), self.as_ptr());
            IntModMat::from_raw(z.assume_init(), self.context().clone())
        }
    }
}

impl fmt::Display for IntModMat {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", IntMat::from(self))
        //write!(f, "{}", IntMat::from(self) % x.modulus())
    }
}

impl Drop for IntModMat {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpz_mod_mat_clear(self.as_mut_ptr()) }
    }
}

// TODO: avoid IntMat allocation
impl Hash for IntModMat {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.context().hash(state);
        IntMat::from(self).hash(state);
    }
}

/*
impl<const CAP: usize> NewMatrix<[&Integer; CAP]> for IntMat {
    fn new(src: [&Integer; CAP], nrows: i64, ncols: i64) -> Self {
        let nrows_ui: usize = nrows.try_into().expect(
            "Cannot convert signed long to usize.");
        let ncols_ui: usize = ncols.try_into().expect(
            "Cannot convert signed long to usize.");
        
        assert_eq!(src.len(), nrows_ui * ncols_ui);
        let mut res = IntMat::zero(nrows, ncols);

        let mut col;
        let mut row = 0usize;
        for (i, x) in src.into_iter().enumerate() {
            col = i % ncols_ui;
            if col == 0 && i != 0 {
                row += 1;
            }
            res.set_entry(row, col, x);
        }
        res
    }
}

impl<T, const CAP: usize> NewMatrix<[T; CAP]> for IntMat 
where
    T: Into<Integer>
{
    fn new(src: [T; CAP], nrows: i64, ncols: i64) -> Self {
        let nrows_ui: usize = nrows.try_into().expect(
            "Cannot convert signed long to usize.");
        let ncols_ui: usize = ncols.try_into().expect(
            "Cannot convert signed long to usize.");
        
        assert_eq!(src.len(), nrows_ui * ncols_ui);
        let mut res = IntMat::zero(nrows, ncols);

        let mut col;
        let mut row = 0usize;
        for (i, x) in src.into_iter().enumerate() {
            col = i % ncols_ui;
            if col == 0 && i != 0 {
                row += 1;
            }
            res.set_entry(row, col, x.into());
        }
        res
    }
}

impl NewMatrix<&[Integer]> for IntMat {
    fn new(src: &[Integer], nrows: i64, ncols: i64) -> Self {
        let nrows_ui: usize = nrows.try_into().expect(
            "Cannot convert signed long to usize.");
        let ncols_ui: usize = ncols.try_into().expect(
            "Cannot convert signed long to usize.");
        
        assert_eq!(src.len(), nrows_ui * ncols_ui);
        let mut res = IntMat::zero(nrows, ncols);

        let mut col;
        let mut row = 0usize;
        for (i, x) in src.iter().enumerate() {
            col = i % ncols_ui;
            if col == 0 && i != 0 {
                row += 1;
            }
            res.set_entry(row, col, x);
        }
        res
    }
}

impl<'a, T> NewMatrix<&'a [T]> for IntMat
where
    &'a T: Into<Integer>
{
    fn new(src: &'a [T], nrows: i64, ncols: i64) -> Self {
        let nrows_ui: usize = nrows.try_into().expect(
            "Cannot convert signed long to usize.");
        let ncols_ui: usize = ncols.try_into().expect(
            "Cannot convert signed long to usize.");
        
        assert_eq!(src.len(), nrows_ui * ncols_ui);
        let mut res = IntMat::zero(nrows, ncols);

        let mut col;
        let mut row = 0usize;
        for (i, x) in src.iter().enumerate() {
            col = i % ncols_ui;
            if col == 0 && i != 0 {
                row += 1;
            }
            res.set_entry(row, col, x.into());
        }
        res
    }
}
*/

impl IntModMat {
    /*
    // private helper methods to convert usize indices to i64, emit consistent
    // messages on panic, and bounds check
    fn check_indices(&self, i: usize, j: usize) -> (i64, i64) {
        (self.check_row_index(i), self.check_col_index(j))
    }

    fn check_row_index(&self, i: usize) -> i64 {
        let i = i.try_into().expect("Cannot convert index to a signed long.");
        assert!(i < self.nrows_si());
        i
    }
    
    fn check_col_index(&self, j: usize) -> i64 {
        let j = j.try_into().expect("Cannot convert index to a signed long.");
        assert!(j < self.ncols_si());
        j
    }
    */
    
    /*
    #[inline]
    pub fn new<S>(src: S, nrows: i64, ncols: i64, ctx: &IntModCtx) -> IntModMat 
    where
        Self: NewMatrix<S>
    {
        <IntMat as NewMatrix<S>>::new(src, nrows, ncols)
    }
    */
    
    #[inline]
    pub fn zero(nrows: i64, ncols: i64, ctx: &IntModCtx) -> IntModMat {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz_mod_mat_init(z.as_mut_ptr(), nrows, ncols, ctx.modulus_as_ptr());
            IntModMat::from_raw(z.assume_init(), ctx.clone())
        }
    }
   
    /*
    #[inline]
    pub fn one(dim: i64) -> IntMat {
        let mut res = IntMat::zero(dim, dim);
        unsafe {
            fmpz_mat::fmpz_mat_one(res.as_mut_ptr());
        }
        res
    }*/

    #[inline]
    pub const fn as_ptr(&self) -> *const fmpz_mod_mat_struct {
        &self.inner
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut fmpz_mod_mat_struct {
        &mut self.inner
    }

    #[inline]
    pub fn from_raw(inner: fmpz_mod_mat_struct, ctx: IntModCtx) -> Self {
        IntModMat { inner, ctx }
    }
    
    #[inline]
    pub fn context(&self) -> &IntModCtx {
        &self.ctx
    }
    
    #[inline]
    pub fn modulus(&self) -> Integer {
        self.context().modulus()
    }

    /// Return the number of rows.
    #[inline]
    pub fn nrows(&self) -> usize {
        self.nrows_si().try_into().expect("Cannot convert signed long to usize.")
    }
    
    /// Return the number of rows.
    #[inline]
    pub fn nrows_si(&self) -> i64 {
        unsafe { fmpz_mod_mat_nrows(self.as_ptr())}
    }

    /// Return the number of columns.
    #[inline]
    pub fn ncols(&self) -> usize {
        self.ncols_si().try_into().expect("Cannot convert signed long to usize.")
    }
    
    /// Return the number of columns.
    #[inline]
    pub fn ncols_si(&self) -> i64 {
        unsafe { fmpz_mod_mat_ncols(self.as_ptr())}
    }
    /*

    /// Set `self` to the zero matrix.
    #[inline]
    pub fn zero_assign(&mut self) {
        unsafe {
            fmpz_mat::fmpz_mat_zero(self.as_mut_ptr());
        }
    }
    
    /// Set `self` to the identity matrix. Panics if the matrix is not square.
    #[inline]
    pub fn one_assign(&mut self) {
        assert!(self.is_square());
        unsafe {
            fmpz_mat::fmpz_mat_one(self.as_mut_ptr());
        }
    }

    /// Return the number of rows.
    #[inline]
    pub fn nrows(&self) -> usize {
        self.nrows_si().try_into().expect("Cannot convert signed long to usize.")
    }
    
    /// Return the number of rows.
    #[inline]
    pub fn nrows_si(&self) -> i64 {
        unsafe { fmpz_mat::fmpz_mat_nrows(self.as_ptr())}
    }

    /// Return the number of columns.
    #[inline]
    pub fn ncols(&self) -> usize {
        self.ncols_si().try_into().expect("Cannot convert signed long to usize.")
    }
    
    /// Return the number of columns.
    #[inline]
    pub fn ncols_si(&self) -> i64 {
        unsafe { fmpz_mat::fmpz_mat_ncols(self.as_ptr())}
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
    pub fn get_entry(&self, i: usize, j: usize) -> Integer {
        let mut res = Integer::zero();
        self.assign_entry(i, j, &mut res);
        res
    }
    
    // TODO: need consistent naming convention
    /// Get the `(i, j)`-th entry of an integer matrix and assign it to `out`. 
    /// Avoids unnecessary allocation.
    #[inline]
    pub fn assign_entry(&self, i: usize, j: usize, out: &mut Integer) {
        let (i, j) = self.check_indices(i, j);
        unsafe {
            let x = fmpz_mat::fmpz_mat_entry(self.as_ptr(), i, j);
            fmpz::fmpz_set(out.as_mut_ptr(), x);
        }
    }

    /// Set the `(i, j)`-th entry of the matrix.
    #[inline]
    pub fn set_entry<T: AsRef<Integer>>(&mut self, i: usize, j: usize, e: T) {
        let (i, j) = self.check_indices(i, j);
        unsafe {
            let x = fmpz_mat::fmpz_mat_entry(self.as_ptr(), i, j);
            fmpz::fmpz_set(x, e.as_ref().as_ptr());
        }
    }

    /// Get a vector with all of the entries of the matrix.
    pub fn get_entries(&self) -> Vec<Integer> {
        let r = self.nrows();
        let c = self.ncols();
        let mut out = Vec::with_capacity(r * c);

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

    /// Swap the rows `r1` and `r2` of an integer matrix. 
    pub fn swap_rows(&mut self, r1: usize, r2: usize) {
        let r1 = self.check_row_index(r1);
        let r2 = self.check_row_index(r2);
        unsafe { 
            fmpz_mat::fmpz_mat_swap_rows(
                self.as_mut_ptr(), 
                std::ptr::null(),
                r1,
                r2
            ); 
        }
    }
    
    /// Swap the columns `r` and `s` of an integer matrix. 
    pub fn swap_cols(&mut self, c1: usize, c2: usize) {
        let c1 = self.check_col_index(c1);
        let c2 = self.check_col_index(c2);
        unsafe { 
            fmpz_mat::fmpz_mat_swap_rows(
                self.as_mut_ptr(), 
                std::ptr::null(),
                c1,
                c2
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

    /*
    /// Return true if the matrix is invertible.
    #[inline]
    pub fn is_invertible(&self) -> bool {
        self.is_square() && !self.det().is_zero()
    }*/

    /// Return true if row `i` is all zeros.
    pub fn is_zero_row(&self, i: usize) -> bool { 
        let i = self.check_row_index(i);
        unsafe {
            fmpz_mat::fmpz_mat_is_zero_row(self.as_ptr(), i) != 0
        }
    }

    /// Return true if column `i` is all zeros.
    // TODO: Does an additional allocation compared to `is_zero_row`.
    #[inline]
    pub fn is_zero_col(&self, i: usize) -> bool {
        self.column(i).is_zero()
    }

    /// Return the transpose.
    #[inline]
    pub fn transpose(&self) -> IntMat {
        let mut res = IntMat::zero(self.nrows_si(), self.ncols_si());
        unsafe {
            fmpz_mat::fmpz_mat_transpose(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }

    /// Transpose the matrix in place. Panics if the matrix is not square.
    #[inline]
    pub fn transpose_assign(&mut self) {
        assert!(self.is_square());
        unsafe { fmpz_mat::fmpz_mat_transpose(self.as_mut_ptr(), self.as_ptr()); }
    }
    
    /// Horizontally concatenate two matrices. Panics if the number of rows of 
    /// both matrices do not agree.
    pub fn hcat<T>(&self, other: T) -> IntMat where
        T: AsRef<IntMat>
    {
        let other = other.as_ref();
        let nrows = self.nrows_si();
        assert_eq!(nrows, other.nrows_si());

        let mut res = IntMat::zero(nrows, self.ncols_si() + other.ncols_si());
        unsafe {
            fmpz_mat::fmpz_mat_concat_horizontal(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
        }
        res
    }
    
    /// Vertically concatenate two matrices. Panics if the number of columns of 
    /// both matrices do not agree.
    pub fn vcat<T>(&self, other: T) -> IntMat where
        T: AsRef<IntMat>
    {
        let other = other.as_ref();
        let ncols = self.ncols_si();
        assert_eq!(ncols, other.ncols_si());

        let mut res = IntMat::zero(self.nrows_si() + other.nrows_si(), ncols);
        unsafe {
            fmpz_mat::fmpz_mat_concat_horizontal(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
        }
        res
    }
   
    // TODO: 'window' version to avoid allocation
    /// Return a new matrix containing the `r2 - r1` by `c2 - c1` submatrix of 
    /// an integer matrix whose `(0, 0)` entry is the `(r1, c1)` entry of the input.
    pub fn submatrix(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> IntMat {
        if r1 == r2 || c1 == c2 {
            return IntMat::zero(0, 0)
        }
        
        assert!(r1 <= r2);
        assert!(c1 <= c2);
        let (r1, c1) = self.check_indices(r1, c1);
        let (r2, c2) = self.check_indices(r2, c2);

        let mut res = IntMat::zero(r2 - r1, c2 - c1);
        let mut win = MaybeUninit::uninit();
        unsafe {
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
        }
        res

    }
    
    /// Return row `i` as an integer matrix.
    #[inline]
    pub fn row(&self, i: usize) -> IntMat {
        self.submatrix(i, 0, i + 1, self.ncols())
    }
   
    /// Return column `j` as an integer matrix.
    #[inline]
    pub fn column(&self, j: usize) -> IntMat {
        self.submatrix(0, j, self.nrows(), j + 1)
    }

    /// Square an integer matrix. The matrix must be square.
    #[inline]
    pub fn square(&self) -> Self {
        assert!(self.is_square());
        let mut res = IntMat::zero(self.nrows_si(), self.ncols_si());
        unsafe {
            fmpz_mat::fmpz_mat_sqr(res.as_mut_ptr(), self.as_ptr()) 
        }
        res
    }
    
    /// Square an integer matrix in place. The matrix must be square.
    #[inline]
    pub fn square_assign(&mut self) {
        assert!(self.is_square());
        unsafe { 
            fmpz_mat::fmpz_mat_sqr(self.as_mut_ptr(), self.as_ptr());
        }
    }
    
    /// Return the kronecker product of two integer matrices.
    pub fn kronecker_product<T>(&self, other: T) -> IntMat where 
        T: AsRef<IntMat>
    {
        let other = other.as_ref();
        let mut res = IntMat::zero(
            self.nrows_si() * other.nrows_si(),
            self.ncols_si() * other.ncols_si()
        );
        unsafe { 
            fmpz_mat::fmpz_mat_kronecker_product(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            ); 
        }
        res
    }
    
    /// Compute the trace of a square integer matrix.
    #[inline]
    pub fn trace(&self) -> Integer {
        assert!(self.is_square());
        let mut res = Integer::zero();
        unsafe { 
            fmpz_mat::fmpz_mat_trace(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }

    /// Return the content of an integer matrix, that is, the gcd of all its 
    /// entries. Returns zero if the matrix is empty.
    #[inline]
    pub fn content(&self) -> Integer {
        let mut res = Integer::zero();
        unsafe { 
            fmpz_mat::fmpz_mat_content(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    /// Compute the determinant of the matrix.
    #[inline]
    pub fn det(&self) -> Integer {
        assert!(self.is_square());
        let mut res = Integer::zero();
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
        let mut res = Integer::zero();
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
        let mut res = Integer::zero();
        unsafe { 
            fmpz_mat::fmpz_mat_det_divisor(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    /// Applies a similarity transform to an `n` by `n` integer matrix. If `P` 
    /// is the identity matrix whose zero entries in row `r` have been replaced 
    /// by `d`, this transform is equivalent to `P^-1 * M * P`. 
    #[inline]
    pub fn similarity<T>(&self, r: usize, d: T) -> IntMat where 
        T: AsRef<Integer>
    {
        let mut res = self.clone();
        res.similarity_assign(r, d);
        res
    }
    
    /// Applies a similarity transform to an `n` by `n` integer matrix in place.
    pub fn similarity_assign<T>(&mut self, r: usize, d: T) where 
        T: AsRef<Integer>
    {
        let r = self.check_row_index(r);
        assert!(self.is_square());
        unsafe { 
            fmpz_mat::fmpz_mat_similarity(
                self.as_mut_ptr(), 
                r.into(),
                d.as_ref().as_ptr()
            ); 
        }
    }
  
    /// Return the characteristic polynomial of a square integer matrix.
    #[inline]
    pub fn charpoly(&self) -> IntPoly {
        assert!(self.is_square());
        let mut res = IntPoly::zero();
        unsafe { 
            fmpz_mat::fmpz_mat_charpoly(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    /// Return the minimal polynomial of a square integer matrix.
    #[inline]
    pub fn minpoly(&self) -> IntPoly {
        assert!(self.is_square());
        let mut res = IntPoly::zero();
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

    /*
    /// Solve `AX = B` for nonsingular `A`.
    pub fn solve<T>(&self, rhs: T) -> Option<RatMat> where 
        T: AsRef<IntMat>
    {
        let b = rhs.as_ref();
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
    }*/

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
        let mut res = IntMat::zero(self.nrows_si(), self.ncols_si());
        let mut den = Integer::zero();

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
        let mut res = IntMat::zero(self.nrows_si(), self.ncols_si());
        let mut den = Integer::zero();

        unsafe {
            let rank = fmpz_mat::fmpz_mat_rref(
                res.as_mut_ptr(), 
                den.as_mut_ptr(), 
                self.as_ptr()
            );
            (rank, res, den)
        }
    }
    
    pub fn rref_mod<T>(&self, modulus: T) -> (i64, IntMat) where 
        T: AsRef<Integer> 
    {
        let mut res = IntMat::zero(self.nrows_si(), self.ncols_si());
        unsafe {
            let rank = fmpz_mat::fmpz_mat_rref_mod(
                std::ptr::null_mut(),
                res.as_mut_ptr(),
                modulus.as_ref().as_ptr()
            );
            (rank, res)
        }
    }

    /*
    pub fn gram_schmidt(&self) -> RatMat {
        RatMat::from(self).gram_schmidt()
    }*/

    pub fn strong_echelon_form_mod<T>(&self, modulus: T) -> IntMat where 
        T: AsRef<Integer>
    {
        let mut res = IntMat::zero(self.nrows_si(), self.ncols_si());
        unsafe {
            fmpz_mat::fmpz_mat_strong_echelon_form_mod(
                res.as_mut_ptr(),
                modulus.as_ref().as_ptr()
            );
        }
        res
    }
    
    pub fn howell_form_mod<T>(&self, modulus: T) -> (i64, IntMat) where 
        T: AsRef<Integer>
    {
        assert!(self.ncols() <= self.nrows());
        let mut res = IntMat::zero(self.nrows_si(), self.ncols_si());
        unsafe {
            let rank = fmpz_mat::fmpz_mat_howell_form_mod(
                res.as_mut_ptr(),
                modulus.as_ref().as_ptr()
            );
            (rank, res)
        }
    }
 
    /*
    // TODO: get rows/cols of nullspace first
    // left or right?
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

    // FIXME: aliasing allowed? then do hnf_assign
    pub fn hnf(&self) -> IntMat {
        let mut res = IntMat::zero(self.nrows_si(), self.ncols_si());
        unsafe { 
            fmpz_mat::fmpz_mat_hnf(res.as_mut_ptr(), self.as_ptr()); 
        }
        res
    }
    
    pub fn hnf_transform(&self) -> (IntMat, IntMat) {
        let mut h = IntMat::zero(self.nrows_si(), self.ncols_si());
        let mut u = IntMat::zero(self.nrows_si(), self.ncols_si());
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
        let mut res = IntMat::zero(self.nrows_si(), self.ncols_si());
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
    */
}
