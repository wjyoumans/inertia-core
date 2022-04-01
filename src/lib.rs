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

#![allow(unused_macros)]
//! Inertia is a (WIP) computational mathematics library for Rust. 
//!
//! Inertia-core contains the core functionality of the 
//! [Inertia](https://github.com/wjyoumans/inertia) crate, providing high-level wrappers for the 
//! [FLINT](https://flintlib.org/doc/), [Arb](https://arblib.org/), and 
//! [Antic](https://github.com/wbhart/antic) C libraries.


#[macro_use]
pub mod macros;

pub mod integer;
pub mod rational;
pub mod intmod;
pub mod intpoly;
pub mod ratpoly;
pub mod intmodpoly;
//pub mod intmpoly;
pub mod intmat;
pub mod finfld;


/// Enum holding either a value or borrow of type T.
pub enum ValOrRef<'a, T> {
    Val(T),
    Ref(&'a T),
}

/// Dereference a `ValOrRef<T>` to get a borrow of type T.
impl<'a, T> std::ops::Deref for ValOrRef<'a, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            ValOrRef::Val(x) => x,
            ValOrRef::Ref(x) => x,
        }
    }
}

/// Blanket implementation.
impl<'a, T> From<&'a T> for ValOrRef<'a, T> {
    fn from(x: &'a T) -> ValOrRef<'a, T> {
        ValOrRef::Ref(&x)
    }
}

pub mod util {
    #[must_use]
    #[inline]
    pub fn is_digit(c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }
}

/// Expand on the operations provided in `std::ops`.
pub mod ops {
    pub trait Assign<T = Self> {
        fn assign(&mut self, other: T);
    }

    /// Inverse as a unary operation.
    pub trait Inv {
        type Output;
        fn inv(self) -> Self::Output;
    }
    
    /// Inverse with assignment.
    pub trait InvAssign {
        fn inv_assign(&mut self);
    }

    /// Negation with assignment.
    pub trait NegAssign {
        fn neg_assign(&mut self);
    }
    
    /// Complement with assignment.
    pub trait NotAssign {
        fn not_assign(&mut self);
    }
    
    /// Bitwise `and` with assignment to the rhs operand.
    pub trait BitAndFrom<Lhs = Self> {
        fn bitand_from(&mut self, lhs: Lhs);
    }

    /// Bitwise `and` with assignment into a third argument.
    pub trait AssignBitAnd<Lhs = Self, Rhs = Self> {
        fn assign_bitand(&mut self, lhs: Lhs, rhs: Rhs);
    }
    
    /// Bitwise `or` with assignment to the rhs operand.
    pub trait BitOrFrom<Lhs = Self> {
        fn bitor_from(&mut self, lhs: Lhs);
    }

    /// Bitwise `or` with assignment into a third argument.
    pub trait AssignBitOr<T, U> {
        fn assign_bitor(&mut self, lhs: T, rhs: U);
    }
    
    /// Bitwise `xor` with assignment to the rhs operand.
    pub trait BitXorFrom<Lhs = Self> {
        fn bitxor_from(&mut self, lhs: Lhs);
    }

    /// Bitwise `xor` with assignment into a third argument.
    pub trait AssignBitXor<T, U> {
        fn assign_bitxor(&mut self, lhs: T, rhs: U);
    }
    
    /// Addition with assignment to the rhs operand.
    pub trait AddFrom<Lhs = Self> {
        fn add_from(&mut self, lhs: Lhs);
    }

    /// Addition with assignment into a third argument.
    pub trait AssignAdd<T, U> {
        fn assign_add(&mut self, lhs: T, rhs: U);
    }
    
    /// Subtraction with assignment to the rhs operand.
    pub trait SubFrom<Lhs = Self> {
        fn sub_from(&mut self, lhs: Lhs);
    }

    /// Subtraction with assignment into a third argument.
    pub trait AssignSub<T, U> {
        fn assign_sub(&mut self, lhs: T, rhs: U);
    }
    
    /// Multiplication with assignment to the rhs operand.
    pub trait MulFrom<Lhs = Self> {
        fn mul_from(&mut self, lhs: Lhs);
    }

    /// Multiplication with assignment into a third argument.
    pub trait AssignMul<T, U> {
        fn assign_mul(&mut self, lhs: T, rhs: U);
    }
    
    /// Division with assignment to the rhs operand.
    pub trait DivFrom<Lhs = Self> {
        fn div_from(&mut self, lhs: Lhs);
    }

    /// Division with assignment into a third argument.
    pub trait AssignDiv<T, U> {
        fn assign_div(&mut self, lhs: T, rhs: U);
    }
    
    /// Exponentiation.
    pub trait Pow<Rhs = Self> {
        type Output;
        fn pow(self, rhs: Rhs) -> Self::Output;
    }
    
    /// Exponentiation with assignment.
    pub trait PowAssign<Rhs = Self> {
        fn pow_assign(&mut self, rhs: Rhs);
    }
    
    /// Exponentiation with assignment to the rhs operand.
    pub trait PowFrom<Lhs = Self> {
        fn pow_from(&mut self, lhs: Lhs);
    }

    /// Exponentiation with assignment into a third argument.
    pub trait AssignPow<T, U> {
        fn assign_pow(&mut self, lhs: T, rhs: U);
    }
    
    /// Remainder with assignment to the rhs operand.
    pub trait RemFrom<Lhs = Self> {
        fn rem_from(&mut self, lhs: Lhs);
    }

    /// Remainder with assignment into a third argument.
    pub trait AssignRem<T, U> {
        fn assign_rem(&mut self, lhs: T, rhs: U);
    }

    /// Evaluation of an expression at `x`.
    pub trait Evaluate<T> {
        type Output;
        fn evaluate(&self, x: T) -> Self::Output;
    }

    /// Modular evaluation of an expression at `x`.
    pub trait EvaluateMod<T, U> {
        type Output;
        fn evaluate_mod(&self, x: T, modulus: U) -> Self::Output;
    }
}

pub use ops::*;
pub use integer::*;
pub use rational::*;
pub use intmod::*;
pub use intpoly::*;
pub use ratpoly::*;
pub use intmodpoly::*;
//pub use intmpoly::*;
pub use intmat::*;
pub use finfld::*;
