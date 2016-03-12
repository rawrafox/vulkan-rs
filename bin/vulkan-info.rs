extern crate vulkan;

use std::ptr;

use vulkan::ffi;

fn main() {
    unsafe {
        let mut n: u32 = 0;
        let result = ffi::vkEnumerateInstanceLayerProperties(&mut n, ptr::null_mut());
        println!("{}", n);
    }
}
