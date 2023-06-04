use crate::wintype::*;

#[link(name = "ntoskrnl")]
extern "system" {
    pub fn MmIsAddressValid(VirtualAddress: PVOID) -> bool;
}

pub fn is_addr_valid(addr: usize) -> bool {
    unsafe { MmIsAddressValid(addr as _) }
}
