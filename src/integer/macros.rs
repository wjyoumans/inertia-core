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

#[macro_export]
macro_rules! int {
    () => {
        Integer::zero()
    };
    ($arg:expr) => {
        Integer::new(&$arg)
    };
}

#[macro_export]
macro_rules! pow2 {
    ($arg:expr) => {
        match Integer::try_from(Integer::new(2).pow(Integer::new(&$arg))) {
            Ok(res) => res,
            Err(err) => panic!("{}", err),
        }
    };
}


pub use int;
pub use pow2;
