// SPDX-License-Identifier: MIT OR Apache-2.0
#![no_std]
#![deny(unsafe_code)]
#![feature(allocator_api)]

extern crate alloc;

use alloc::alloc::Global;
use arctypon_core::Mailbox;
use ostd::prelude::*;

#[ostd::main]
fn kernel_main() {
    let mut mb: Mailbox<u32, Global> = Mailbox::new_in(8, Global);
    mb.try_send(42).ok();
    println!(
        "arctypon: hello from the actor kernel! recv = {:?}",
        mb.try_recv()
    );
}
