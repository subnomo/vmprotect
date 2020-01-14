use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use real_c_string::{real_c_string, real_c_wstring};

pub mod internal;
#[cfg(feature = "licensing")]
pub mod licensing;
#[cfg(feature = "service")]
pub mod service;
#[cfg(feature = "strings")]
pub mod strings;

#[macro_export]
macro_rules! protected {
    ($x: expr; mutate; $code: expr) => {{
        let ret;
        unsafe {
            $crate::internal::VMProtectBeginMutation($crate::real_c_string!($x))
        };
        ret = $code;
        unsafe {
            $crate::internal::VMProtectEnd();
        };
        ret
    }};
    ($x: expr; virtualize false; $code: expr) => {{
        let ret;
        unsafe {
            $crate::internal::VMProtectBeginVirtualization($crate::real_c_string!(
                $x
            ))
        };
        ret = $code;
        unsafe {
            $crate::internal::VMProtectEnd();
        };
        ret
    }};
    ($x: expr; virtualize true; $code: expr) => {{
        let ret;
        unsafe {
            $crate::internal::VMProtectBeginVirtualizationLockByKey(
                $crate::real_c_string!($x),
            )
        };
        ret = $code;
        unsafe {
            $crate::internal::VMProtectEnd();
        };
        ret
    }};
    ($x: expr; ultra false; $code: expr) => {{
        let ret;
        unsafe { $crate::internal::VMProtectBeginUltra($crate::real_c_string!($x)) };
        ret = $code;
        unsafe {
            $crate::internal::VMProtectEnd();
        };
        ret
    }};
    ($x: expr; ultra true; $code: expr) => {{
        let ret;
        unsafe {
            $crate::internal::VMProtectBeginUltraLockByKey($crate::real_c_string!(
                $x
            ))
        };
        ret = $code;
        unsafe {
            $crate::internal::VMProtectEnd();
        };
        ret
    }};
    (A; $x: expr) => {
        $crate::strings::encrypted_a::EncryptedStringA(
            unsafe {
                $crate::internal::VMProtectDecryptStringA($crate::real_c_string!($x))
            },
            std::marker::PhantomData,
        ) as $crate::strings::encrypted_a::EncryptedStringA
    };
    (W; $x: expr) => {
        $crate::strings::encrypted_w::EncryptedStringW(
            unsafe {
                $crate::internal::VMProtectDecryptStringW($crate::real_c_wstring!(
                    $x
                ))
            },
            std::marker::PhantomData,
        ) as $crate::strings::encrypted_w::EncryptedStringW // To remove mut
    };
}
