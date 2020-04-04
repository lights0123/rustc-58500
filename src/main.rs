#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use core::alloc::{GlobalAlloc, Layout};

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

struct DummyAlloc;

unsafe impl GlobalAlloc for DummyAlloc {
	unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
		unimplemented!()
	}

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
		unimplemented!()
	}
}

#[global_allocator]
static A: DummyAlloc = DummyAlloc;

#[alloc_error_handler]
fn on_oom(_layout: core::alloc::Layout) -> ! { loop {} }

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }

#[no_mangle]
fn main() {}
