#[allow(dead_code)]
pub fn debug_break() {
    unsafe {
        core::arch::asm!("int3");
    }
}
