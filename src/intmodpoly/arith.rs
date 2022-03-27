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

use std::ops::*;
use flint_sys::fmpz_mod_poly;
use crate::IntModPoly;
use crate::ops::*;

impl_cmp! {
    eq
    IntModPoly
    {
        fn eq(&self, rhs: &IntModPoly) -> bool {
            assert_eq!(self.parent(), rhs.parent());
            unsafe { 
                fmpz_mod_poly::fmpz_mod_poly_equal(
                    self.as_ptr(), 
                    rhs.as_ptr(), 
                    self.ctx_as_ptr()
                ) != 0 
            }
        }
    }
}

impl_unop_unsafe! {
    ctx
    IntModPoly
    Neg {neg}
    NegAssign {neg_assign}
    fmpz_mod_poly::fmpz_mod_poly_neg
}

impl_binop_unsafe! {
    ctx
    IntModPoly, IntModPoly, IntModPoly
    
    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod_poly::fmpz_mod_poly_add;
    
    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod_poly::fmpz_mod_poly_sub;
    
    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod_poly::fmpz_mod_poly_mul;
}

/*
impl_binop_unsafe! {
    ctx
    op_assign
    IntModPoly, Integer, IntModPoly
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod_poly::fmpz_mod_poly_add_fmpz;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod_poly::fmpz_mod_poly_sub_fmpz;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod_poly::fmpz_mod_poly_scalar_mul_fmpz;
}
*/

/*
impl_binop_unsafe! {
    None
    op_from
    Integer, IntPoly, IntPoly
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_poly_fmpz_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_poly::fmpz_poly_fmpz_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_poly_fmpz_scalar_mul;
    
    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz_poly_fmpz_scalar_mod;
}

impl_binop_unsafe! {
    None
    op_assign
    IntPoly, u64 {u64 u32 u16 u8}, IntPoly
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_poly_add_ui;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_poly_sub_ui;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_poly::fmpz_poly_scalar_mul_ui;
    
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpz_poly_scalar_mod_ui;
}

impl_binop_unsafe! {
    None
    op_assign
    IntPoly, i64 {i64 i32 i16 i8}, IntPoly
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_poly::fmpz_poly_add_si;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_poly::fmpz_poly_sub_si;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_poly::fmpz_poly_scalar_mul_si;
    
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpz_poly_scalar_mod_si;
}

impl_binop_unsafe! {
    None
    op_from
    u64 {u64 u32 u16 u8}, IntPoly, IntPoly
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_poly_ui_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_poly_ui_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_poly_ui_scalar_mul;
    
    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz_poly_ui_scalar_mod;
}

impl_binop_unsafe! {
    None
    op_from
    i64 {i64 i32 i16 i8}, IntPoly, IntPoly
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_poly_si_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_poly_si_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_poly_si_scalar_mul;
    
    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz_poly_si_scalar_mod;
}

#[inline]
unsafe fn fmpz_poly_add_ui(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: *const fmpz_poly::fmpz_poly_struct,
    x: c_ulong,
    )
{
    fmpz_poly::fmpz_poly_set_ui(res, x);
    fmpz_poly::fmpz_poly_add(res, f, res);
}

#[inline]
unsafe fn fmpz_poly_sub_ui(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: *const fmpz_poly::fmpz_poly_struct,
    x: c_ulong,
    )
{
    fmpz_poly::fmpz_poly_set_ui(res, x);
    fmpz_poly::fmpz_poly_sub(res, f, res);
}

#[inline]
unsafe fn fmpz_poly_scalar_mod_ui(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: *const fmpz_poly::fmpz_poly_struct,
    x: c_ulong,
    )
{
    fmpz_poly::fmpz_poly_set_ui(res, x);
    fmpz_poly::fmpz_poly_rem(res, f, res);
}

#[inline]
unsafe fn fmpz_poly_scalar_mod_si(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: *const fmpz_poly::fmpz_poly_struct,
    x: c_long,
    )
{
    fmpz_poly::fmpz_poly_set_si(res, x);
    fmpz_poly::fmpz_poly_rem(res, f, res);
}

#[inline]
unsafe fn fmpz_poly_fmpz_add(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: *const fmpz::fmpz,
    g: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpz_poly::fmpz_poly_add_fmpz(res, g, f);
}

#[inline]
unsafe fn fmpz_poly_fmpz_scalar_mul(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: *const fmpz::fmpz,
    g: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpz_poly::fmpz_poly_scalar_mul_fmpz(res, g, f);
}

#[inline]
unsafe fn fmpz_poly_fmpz_scalar_mod(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: *const fmpz::fmpz,
    g: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpz_poly::fmpz_poly_set_fmpz(res, f);
    fmpz_poly::fmpz_poly_rem(res, res, g);
}

#[inline]
unsafe fn fmpz_poly_ui_add(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: c_ulong,
    g: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpz_poly::fmpz_poly_set_ui(res, f);
    fmpz_poly::fmpz_poly_add(res, res, g);
}

#[inline]
unsafe fn fmpz_poly_ui_sub(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: c_ulong,
    g: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpz_poly::fmpz_poly_set_ui(res, f);
    fmpz_poly::fmpz_poly_sub(res, res, g);
}

#[inline]
unsafe fn fmpz_poly_ui_scalar_mul(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: c_ulong,
    g: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpz_poly::fmpz_poly_scalar_mul_ui(res, g, f);
}

#[inline]
unsafe fn fmpz_poly_ui_scalar_mod(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: c_ulong,
    g: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpz_poly::fmpz_poly_set_ui(res, f);
    fmpz_poly::fmpz_poly_rem(res, res, g);
}

#[inline]
unsafe fn fmpz_poly_si_add(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: c_long,
    g: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpz_poly::fmpz_poly_add_si(res, g, f);
}

#[inline]
unsafe fn fmpz_poly_si_sub(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: c_long,
    g: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpz_poly::fmpz_poly_sub_si(res, g, f);
    fmpz_poly::fmpz_poly_neg(res, res);
}

#[inline]
unsafe fn fmpz_poly_si_scalar_mul(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: c_long,
    g: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpz_poly::fmpz_poly_scalar_mul_si(res, g, f);
}

#[inline]
unsafe fn fmpz_poly_si_scalar_mod(
    res: *mut fmpz_poly::fmpz_poly_struct,
    f: c_long,
    g: *const fmpz_poly::fmpz_poly_struct,
    )
{
    fmpz_poly::fmpz_poly_set_si(res, f);
    fmpz_poly::fmpz_poly_rem(res, res, g);
}*/
