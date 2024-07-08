#![no_std]
#![no_main]

use aya_ebpf::{
    macros::uprobe,
    programs::ProbeContext,
};
use aya_log_ebpf::info;

#[uprobe]
pub fn ru_uprocess_check(ctx: ProbeContext) -> u32 {
    match try_ru_uprocess_check(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_ru_uprocess_check(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "function getaddrinfo called by libc");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
