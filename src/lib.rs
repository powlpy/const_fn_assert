// Copyright 2019 const_fn_assert Developers
//
// Licensed under the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>. This file may not be
// copied, modified, or distributed except according to those terms.

#![no_std]
#![doc(html_root_url = "https://docs.rs/const_fn_assert")]

#![forbid(const_err)]

#[doc(hidden)]
#[allow(dead_code)]
pub const ASSERT: [(); 1] = [()];

#[macro_export]
macro_rules! cfn_assert {
    ($x:expr $(,)?) => {
        let _ = $crate::ASSERT[!{let cond: bool = $x; cond} as usize];
    }
}

#[macro_export]
macro_rules! cfn_assert_eq {
    ($x:expr, $y:expr $(,)?) => {
        $crate::cfn_assert!($x == $y)
    }
}

#[macro_export]
macro_rules! cfn_assert_ne {
    ($x:expr, $y:expr $(,)?) => {
        $crate::cfn_assert!($x != $y)
    }
}

