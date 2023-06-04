use crate::wintype::*;

#[link(name = "ntoskrnl")]
extern "C" {
    pub fn DbgPrint(format: PCSTR, ...) -> NTSTATUS;
}

#[macro_export]
macro_rules! log {
    ($string: expr) => {
        unsafe {
            $crate::DbgPrint(concat!("[RS_KERNEL_LOG] ", $string, "\0").as_ptr())
        }
    };

    ($string: expr, $($x:tt)*) => {
        unsafe {
            $crate::DbgPrint(concat!("[RS_KERNEL_LOG] ", $string, "\0").as_ptr(), $($x)*)
        }
    };
}
