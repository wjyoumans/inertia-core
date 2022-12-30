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

// constructors

pub trait New<T> {
    fn new(src: T) -> Self;
}

/*
impl<T, S: Into<T>> New<S> for T {
    #[inline]
    fn new(src: S) -> T {
        src.into()
    }
}

impl<T: Clone> New<&T> for T {
    #[inline]
    fn new(src: &T) -> T {
        src.clone()
    }
}
*/

pub trait NewCtx<T, Ctx> {
    fn new(src: T, ctx: &Ctx) -> Self;
}

pub trait NewMatrix<T> {
    fn new(src: T, nrows: i64, ncols: i64) -> Self;
}

pub use inertia_algebra::ops::*;

pub use integer::*;
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

pub use ratfunc::*;

//pub use intmpoly::*;
