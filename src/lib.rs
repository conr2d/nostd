// Copyright (c) Jeeyong Um <conr2d@proton.me>, Jungyong Um <ian.jungyong.um@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc as alloc_;

/// The nostd prelude
///
/// This module is intended for users of nostd where linking to std is not possible or desirable.
pub mod prelude {
    #[cfg(feature = "alloc")]
    pub use alloc_::{
        borrow::ToOwned,
        boxed::Box,
        format,
        string::{String, ToString},
        vec,
        vec::Vec,
    };
}

#[cfg(feature = "alloc")]
pub use alloc_::*;
pub use core::*;

macro_rules! merge_exports {
    ($module:ident) => {
        pub mod $module {
            #[cfg(feature = "alloc")]
            pub use alloc_::$module::*;
            #[allow(unused_imports)]
            pub use core::$module::*;
        }
    };
}

merge_exports!(alloc);
merge_exports!(borrow);
merge_exports!(fmt);
merge_exports!(slice);
merge_exports!(str);
merge_exports!(sync);
merge_exports!(task);

pub mod ffi {
    #[cfg(feature = "alloc")]
    pub use alloc_::ffi::*;
    #[allow(unused_imports)]
    pub use core::ffi::*;
    // Suppress ambiguous_glob_reexports
    pub mod c_str {}
}

#[cfg(feature = "alloc")]
pub mod collections {
    pub use alloc_::collections::*;

    #[cfg(all(feature = "hashbrown", not(feature = "std")))]
    pub use hashbrown::{hash_map, hash_set, HashMap, HashSet};
    #[cfg(feature = "std")]
    pub use std::collections::{hash_map, hash_set, HashMap, HashSet};
}

#[cfg(all(feature = "io", not(feature = "std")))]
pub mod io;
#[cfg(feature = "std")]
pub use std::io;
