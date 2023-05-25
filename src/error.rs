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

use thiserror::Error;

// Including backtrace seems to require nightly as of 5/23.

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to convert {val} of type {in_type} to type {out_type}.")]
    ConversionError {
        val: String,
        in_type: String,
        out_type: String,
    },
    #[error("Division error: {0}")]
    DivisionError(String),
    // A generic error message.
    #[error("{0}")]
    Msg(String)
}

pub type Result<T> = std::result::Result<T, Error>;
