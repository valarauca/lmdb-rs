/*
 * Copyright 2017 William Cody Laeder
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 *     Unless required by applicable law or agreed to in writing, software
 *     distributed under the License is distributed on an "AS IS" BASIS,
 *     WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *     See the License for the specific language governing permissions and
 *     limitations under the License.
 */

#![allow(trivial_casts)]
#![allow(trivial_numeric_casts)]

extern crate libc;

#[macro_use] extern crate bitflags;
#[macro_use] extern crate log;

extern crate liblmdb_sys as ffi;

pub use libc::c_int;
pub use ffi::{mdb_filehandle_t, MDB_stat, MDB_envinfo, MDB_val};
pub use core::{EnvBuilder, Environment, EnvFlags, EnvCreateFlags};
pub use core::{Database, DbFlags, DbHandle};
pub use core::{Transaction, ReadonlyTransaction, MdbError, MdbValue};
pub use core::{Cursor, CursorValue, CursorIter, CursorKeyRangeIter};
pub use traits::{FromMdbValue, ToMdbValue};

pub mod core;
pub mod traits;
mod utils;

#[cfg(test)]
mod tests;
