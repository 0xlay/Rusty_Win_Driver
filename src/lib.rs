#![no_std]
#![allow(non_snake_case)]

mod dbghelp;
mod mem;
mod ntstatus;
mod wintype;
mod log;

#[allow(unused_imports)]
use dbghelp::*;
use mem::*;
use ntstatus::*;
use wintype::*;
use log::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "system" fn driver_entry(_driver: PVOID, _path: PVOID) -> NTSTATUS {
    // debug_break();

    let is_valid = is_addr_valid(0xFFFF) as u64;
    log!("Is 0xFFFF valid address: %d", is_valid);

    STATUS_SUCCESS
}
