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

//! Conversion of data structures to and from MDB_val
//!
//! Since MDB_val is valid through whole transaction, it is kind of safe
//! to keep plain data, i.e. to keep raw pointers and transmute them back
//! and forward into corresponding data structures to avoid any unnecessary
//! copying.
//!
//! `MdbValue` is a simple wrapper with bounded lifetime which should help
//! keep it sane, i.e. provide compile errors when data retrieved outlives
//! transaction.
//!
//! It would be extremely helpful to create `compile-fail` tests to ensure
//! this, but unfortunately there is no way yet.


use std::{self, mem, slice};
use std::borrow::{Cow, ToOwned,Borrow};
use std::net::{Ipv4Addr,Ipv6Addr,IpAddr,SocketAddrV4,SocketAddrV6,SocketAddr};

use core::MdbValue;
use ffi::MDB_val;

/// `ToMdbValue` is supposed to convert a value to a memory
/// slice which `lmdb` uses to prevent multiple copying data
/// multiple times. May be unsafe.
pub trait ToMdbValue {
    fn to_mdb_value<'a>(&'a self) -> MdbValue<'a>;
}

/// `FromMdbValue` is supposed to reconstruct a value from
/// memory slice. It allows to use zero copy where it is
/// required.
pub trait FromMdbValue {
    fn from_mdb_value(value: &MdbValue) -> Self;
}

impl ToMdbValue for String {
    fn to_mdb_value<'a>(&'a self) -> MdbValue<'a> {
        unsafe {
            let t: &'a str = self;
            MdbValue::new(std::mem::transmute(t.as_ptr()), t.len())
        }
    }
}

impl<'a> ToMdbValue for &'a str {
    fn to_mdb_value<'b>(&'b self) -> MdbValue<'b> {
        unsafe {
            MdbValue::new(mem::transmute(self.as_ptr()),
                          self.len())
        }
    }
}

impl ToMdbValue for str {
    fn to_mdb_value<'a>(&'a self) -> MdbValue<'a> {
        unsafe {
            MdbValue::new(mem::transmute(self.as_ptr()),
                          self.len())
        }
    }
}

impl<'a> ToMdbValue for Cow<'a,str> {
    fn to_mdb_value<'b>(&'b self) -> MdbValue<'b> {
        match self {
            &Cow::Borrowed(ref x) => x.to_mdb_value(),
            &Cow::Owned(ref x) => x.as_str().to_mdb_value(),
        }
    }
}

impl ToMdbValue for MDB_val {
    fn to_mdb_value<'a>(&'a self) -> MdbValue<'a> {
        unsafe {
            MdbValue::from_raw(self)
        }
    }
}

impl<'a> ToMdbValue for MdbValue<'a> {
    fn to_mdb_value<'b>(&'b self) -> MdbValue<'b> {
        *self
    }
}

impl FromMdbValue for String {
    fn from_mdb_value(value: &MdbValue) -> String {
        unsafe {
            let ptr = mem::transmute(value.get_ref());
            let bytes = slice::from_raw_parts(ptr, value.get_size());
            match String::from_utf8_lossy(bytes) {
                Cow::Owned(x) => x,
                Cow::Borrowed(x) => x.to_string(),
            }
        }
    }
}

impl FromMdbValue for () {
    fn from_mdb_value(_: &MdbValue) {
    }
}

impl<'b> FromMdbValue for &'b str {
    fn from_mdb_value(value: &MdbValue) -> &'b str {
        unsafe {
            std::mem::transmute(slice::from_raw_parts(value.get_ref(), value.get_size()))
        }
    }
}
impl<'b> FromMdbValue for Cow<'b,str> {
    fn from_mdb_value(value: &MdbValue) -> Cow<'b,str> {
        Cow::Borrowed(unsafe {
            std::mem::transmute(slice::from_raw_parts(value.get_ref(), value.get_size()))
        })
    }
}


/*
 * Remove a lot of boilerplate
 */
macro_rules! mdb_for {

    // standard entry point
    //      triggers interior macros which do other stuff
    ($t:ty) => {
        mdb_for!(@T $t);
        mdb_for!(@T [$t;1]);
        mdb_for!(@T [$t;2]);
        mdb_for!(@T [$t;3]);
        mdb_for!(@T [$t;4]);
        mdb_for!(@T [$t;5]);
        mdb_for!(@T [$t;6]);
        mdb_for!(@T [$t;7]);
        mdb_for!(@T [$t;8]);
        mdb_for!(@T [$t;9]);
        mdb_for!(@T [$t;10]);
        mdb_for!(@T [$t;11]);
        mdb_for!(@T [$t;12]);
        mdb_for!(@T [$t;13]);
        mdb_for!(@T [$t;14]);
        mdb_for!(@T [$t;15]);
        mdb_for!(@T [$t;16]);
    };

    // Type level details
    (@T $t: ty) => {
        impl ToMdbValue for $t {
            fn to_mdb_value<'a>(&'a self) -> MdbValue<'a> {
                MdbValue::new_from_sized(self)
            }
        }

        impl<'a> ToMdbValue for &'a $t {
            fn to_mdb_value<'b>(&'b self) -> MdbValue<'b> {
                MdbValue::new_from_sized(*self)
            }
        }
        
        impl FromMdbValue for $t {
            fn from_mdb_value(value: &MdbValue) -> $t {
                unsafe {
                    let t: *const $t = mem::transmute(value.get_ref());
                    return ::std::ptr::read_unaligned::<$t>(t);
                }
            }
        }

        impl<'a> FromMdbValue for &'a $t {
            fn from_mdb_value(value: &MdbValue) -> &'a $t {
                unsafe { mem::transmute(value.get_ref()) }
            }
        }
   
        // trigger reset
        mdb_for!(@COL $t);
        mdb_for!(@OPT $t);
        mdb_for!(@COL Option<$t>);
    };

    // Type with option
    (@OPT $t: ty) => {
        impl ToMdbValue for Option<$t> {
            fn to_mdb_value<'a>(&'a self) -> MdbValue<'a> {
                MdbValue::new_from_sized(self)
            }
        }

        impl<'a> ToMdbValue for &'a Option<$t> {
            fn to_mdb_value<'b>(&'b self) -> MdbValue<'b> {
                MdbValue::new_from_sized(*self)
            }
        }

        impl FromMdbValue for Option<$t> {
            fn from_mdb_value(value: &MdbValue) -> Option<$t> {
                unsafe {
                    let t: *const Option<$t> = mem::transmute(value.get_ref());
                    return ::std::ptr::read_unaligned::<Option<$t>>(t);
                }
            }
        }

        impl<'a> FromMdbValue for &'a Option<$t> {
            fn from_mdb_value(value: &MdbValue) -> &'a Option<$t> {
                unsafe {
                    mem::transmute(value.get_ref())
                }
            }
        }
    };
    (@COL $t: ty) => {

        impl ToMdbValue for Vec<$t> {
            fn to_mdb_value<'b>(&'b self) -> MdbValue<'b> {
                self.as_slice().to_mdb_value()
            }
        }

        impl<'a> ToMdbValue for &'a Vec<$t> {
            fn to_mdb_value<'b>(&'b self) -> MdbValue<'b> {
                self.as_slice().to_mdb_value()
            }
        }

        impl<'a> ToMdbValue for &'a [$t] {
            fn to_mdb_value<'b>(&'b self) -> MdbValue<'b> {
                unsafe {
                    MdbValue::new(mem::transmute(self.as_ptr()),
                        mem::size_of::<$t>() * self.len(),)
                }
            }
        }

        impl<'a> ToMdbValue for Cow<'a, [$t]> {
            fn to_mdb_value<'b>(&'b self) -> MdbValue<'b> {
                match self {
                    &Cow::Borrowed(ref x) => x.to_mdb_value(),
                    &Cow::Owned(ref x) => x.as_slice().to_mdb_value(),
                }
            }
        }

        impl ToMdbValue for [$t] {
            fn to_mdb_value<'b>(&'b self) -> MdbValue<'b> {
                unsafe {
                    MdbValue::new(mem::transmute(self.as_ptr()),
                        mem::size_of::<$t>() * self.len(),)
                }
            }
        }

        impl<'a> FromMdbValue for &'a [$t] {
            fn from_mdb_value(value: &MdbValue) -> &'a [$t] {
                unsafe {
                    let t: *const $t = mem::transmute(value.get_ref());
                    let len = value.get_size() / mem::size_of::<$t>();
                    slice::from_raw_parts(t,len)
                }
            }
        }

        impl<'a> FromMdbValue for Cow<'a,[$t]> {
            fn from_mdb_value(value: &MdbValue) -> Cow<'a, [$t]> {
                Cow::Borrowed(unsafe {
                    let t: *const $t = mem::transmute(value.get_ref());
                    let len = value.get_size() / mem::size_of::<$t>();
                    slice::from_raw_parts(t,len)
                })
            }
        }

        impl FromMdbValue for Vec<$t> {
            fn from_mdb_value(value: &MdbValue) -> Vec<$t> {
                unsafe {
                    let t: *const $t = mem::transmute(value.get_ref());
                    let len = value.get_size() / mem::size_of::<$t>();
                    slice::from_raw_parts(t,len)
                }.to_vec()
            }
        }
    };
}


mdb_for!(SocketAddr);
mdb_for!(SocketAddrV6);
mdb_for!(SocketAddrV4);
mdb_for!(IpAddr);
mdb_for!(Ipv6Addr);
mdb_for!(Ipv4Addr);
mdb_for!(u8);
mdb_for!(i8);
mdb_for!(u16);
mdb_for!(i16);
mdb_for!(u32);
mdb_for!(i32);
mdb_for!(u64);
mdb_for!(i64);
mdb_for!(f32);
mdb_for!(f64);
mdb_for!(bool);
mdb_for!(char);



