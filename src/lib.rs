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

#[macro_use]
mod macros;
mod error;

mod integer;
mod intpoly;
mod intmat;

mod rational;
mod ratpoly;
mod ratmat;

mod intmod;
mod intmodpoly;
mod intmodmat;

mod finfld;
mod finfldpoly;
mod finfldmat;

//mod intmpoly;
pub mod ratfunc;

mod real;
mod complex;

pub mod binquad;
pub mod numfld;

mod util {
    #[must_use]
    #[inline]
    pub fn is_digit(c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }
}

pub use error::{Error, Result};
pub use inertia_algebra::ops::*;

pub use integer::*;
pub use integer::macros::*;

pub use intpoly::*;
pub use intmat::*;

pub use rational::*;
pub use ratpoly::*;
pub use ratmat::*;

pub use intmod::*;
pub use intmodpoly::*;
pub use intmodmat::*;

pub use finfld::*;
pub use finfldpoly::*;
pub use finfldmat::*;

//pub use intmpoly::*;
pub use ratfunc::*;

pub use real::*;
pub use complex::*;

pub use binquad::*;
pub use numfld::*;

