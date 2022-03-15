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

use crate::{Integer, IntMod};
use crate::ops::*;

use std::ops::*;
use std::sync::Arc;

use flint_sys::{fmpz, fmpz_mod};
use libc::{c_long, c_ulong};


// TODO: consume lhs/rhs to avoid allocation
macro_rules! impl_binop_option {
    (
        $lhs:ident, $rhs:ident, $out:ident
        $op:ident {$meth:ident}
        {
            $($code:tt)*
        }
    ) => {
        impl $op<&$rhs> for &$lhs {
            type Output = Option<$out>;
            #[inline]
            $($code)*
        }
        
        impl $op<$rhs> for &$lhs {
            type Output = Option<$out>;
            #[inline]
            fn $meth(self, rhs: $rhs) -> Option<$out> {
                self.$meth(&rhs)
            }
        }
        
        impl $op<&$rhs> for $lhs {
            type Output = Option<$out>;
            #[inline]
            $($code)*
        }
        
        impl $op<$rhs> for $lhs {
            type Output = Option<$out>;
            #[inline]
            fn $meth(self, rhs: $rhs) -> Option<$out> {
                self.$meth(&rhs)
            }
        }
    };
}

impl_unop_unsafe! {
    ctx
    IntMod
    Neg {neg}
    NegAssign {neg_assign}
    fmpz_mod::fmpz_mod_neg
}

impl_unop! {
    IntMod, Option<IntMod>
    Inv {inv}
    {
        fn inv(self) -> Option<IntMod> {
            let mut res = self.parent().default();
            unsafe {
                let b = fmpz::fmpz_invmod(
                    res.as_mut_ptr(), 
                    self.as_ptr(), 
                    self.modulus().as_ptr()
                );
                if b == 0 {
                    None
                } else {
                    Some(res)
                }
            }
        }
    }
}

impl_binop_option! {
    IntMod, Integer, IntMod
    Pow {pow}
    {
        fn pow(self, pow: &Integer) -> Option<IntMod> {
            let mut res = self.parent().default();
            unsafe {
                let b = fmpz_mod::fmpz_mod_pow_fmpz(
                    res.as_mut_ptr(), 
                    self.as_ptr(),
                    pow.as_ptr(),
                    self.ctx_as_ptr()
                );
                if b == 0 {
                    None
                } else {
                    Some(res)
                }
            }
        }
    }
}

macro_rules! impl_pow_prim {
    ($lhs:ident, $($rhs:ident)*) => ($(
        impl_binop_option! {
            $lhs, $rhs, IntMod
            Pow {pow}
            {
                fn pow(self, rhs: &$rhs) -> Option<IntMod> {
                    if rhs < &0 {
                        if let Some(x) = self.inv() {
                            Some(x.pow(rhs.abs() as u64))
                        } else {
                            None
                        }
                    } else {
                        Some(self.pow(*rhs as u64))
                    }
                }
            }
        }
    )*);
}
impl_pow_prim!(IntMod, i8 i16 i32 i64);

macro_rules! impl_div {
    ($lhs:ident, $($rhs:ident)*) => ($(
        impl_binop_option! {
            $lhs, $rhs, IntMod
            Div {div}
            {
                fn div(self, rhs: &$rhs) -> Option<IntMod> {
                    if let Some(x) = Integer::from(rhs).invmod(self.modulus()) {
                        Some(self * x)
                    } else {
                        None
                    }
                }
            }
        }
    )*);
    ($($lhs:ident)*, IntMod) => ($(
        impl_binop_option! {
            $lhs, IntMod, IntMod
            Div {div}
            {
                fn div(self, rhs: &IntMod) -> Option<IntMod> {
                    if let Some(x) = rhs.inv() {
                        Some(self * x)
                    } else {
                        None
                    }
                }
            }
        }
    )*)
}

impl_div!(u8 u16 u32 u64 i8 i16 i32 i64 Integer IntMod, IntMod);
impl_div!(IntMod, u8 u16 u32 u64 i8 i16 i32 i64);

impl_binop_option! {
    IntMod, Integer, IntMod
    Div {div}
    {
        fn div(self, rhs: &Integer) -> Option<IntMod> {
            if let Some(x) = rhs.invmod(self.modulus()) {
                Some(self * x)
            } else {
                None
            }
        }
    }
}

impl_binop_unsafe! {
    ctx
    IntMod, IntMod, IntMod
    
    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add;
    
    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub;
    
    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    IntMod, Integer, IntMod
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add_fmpz;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub_fmpz;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul_fmpz;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    Integer, IntMod, IntMod
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add_fmpz;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub_fmpz;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul_fmpz;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    IntMod, u64 {u64 u32 u16 u8}, IntMod
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add_ui;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub_ui;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul_ui;
    
    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpz_mod::fmpz_mod_pow_ui;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    IntMod, i64 {i64 i32 i16 i8}, IntMod
   
    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add_si;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub_si;
    
    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul_si;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    u64 {u64 u32 u16 u8}, IntMod, IntMod
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod_ui_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_ui_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod_ui_mul;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    i64 {i64 i32 i16 i8}, IntMod, IntMod
   
    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod_si_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_si_sub;
    
    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod_si_mul;
}


#[inline]
unsafe fn fmpz_mod_ui_add(
    res: *mut fmpz::fmpz,
    f: c_ulong,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx)
{
    fmpz_mod::fmpz_mod_add_ui(res, g, f, ctx);
}

#[inline]
unsafe fn fmpz_mod_si_add(
    res: *mut fmpz::fmpz,
    f: c_long,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx)
{
    fmpz_mod::fmpz_mod_add_si(res, g, f, ctx);
}

#[inline]
unsafe fn fmpz_mod_ui_mul(
    res: *mut fmpz::fmpz,
    f: c_ulong,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx)
{
    fmpz_mod::fmpz_mod_mul_ui(res, g, f, ctx);
}

#[inline]
unsafe fn fmpz_mod_si_mul(
    res: *mut fmpz::fmpz,
    f: c_long,
    g: *const fmpz::fmpz,
    ctx: *const fmpz_mod::fmpz_mod_ctx)
{
    fmpz_mod::fmpz_mod_mul_si(res, g, f, ctx);
}
