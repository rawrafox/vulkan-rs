#![feature(const_fn, libc)]

extern crate libc;
#[macro_use] extern crate bitflags;

pub mod ffi;
mod structures;

pub use structures::*;

pub fn create_instance(create_info: &InstanceCreateInfo, allocator: Option<&AllocationCallbacks>) -> std::result::Result<Instance, ffi::Result> {
    let create_info = &create_info.raw as *const ffi::InstanceCreateInfo;
    let allocator = allocator.map_or(std::ptr::null(), |x| { &x.raw as *const ffi::AllocationCallbacks });

	unsafe {
		let mut instance: Instance = std::mem::uninitialized();
		let result = ffi::create_instance(create_info, allocator, &mut instance.handle as *mut libc::size_t);
		if result != ffi::Result::Success { return Err(result) };
		return Ok(instance);
	}
}

pub fn enumerate_instance_layer_properties() -> std::result::Result<Vec<LayerProperties>, ffi::Result> {
    let mut property_count = 0u32;

	unsafe {
		let result = ffi::enumerate_instance_layer_properties(&mut property_count, std::ptr::null_mut());
		if result != ffi::Result::Success { return Err(result) };
    }

    let result = unsafe {
		let mut properties: Vec<ffi::LayerProperties> = Vec::with_capacity(property_count as usize);
		let result = ffi::enumerate_instance_layer_properties(&mut property_count, properties.as_mut_ptr());
		if result != ffi::Result::Success { return Err(result) };
		properties.set_len(property_count as usize);
        properties
	};

    let result: Vec<LayerProperties> = result.into_iter().map(|x: ffi::LayerProperties| unsafe {
        LayerProperties::from_ffi(x)
    }).collect();

    return Ok(result);
}

pub fn enumerate_instance_global_extension_properties() -> std::result::Result<Vec<ExtensionProperties>, ffi::Result> {
    let mut property_count = 0u32;

    unsafe {
        let result = ffi::enumerate_instance_extension_properties(std::ptr::null(), &mut property_count, std::ptr::null_mut());
        if result != ffi::Result::Success { return Err(result) };
    }

    let result = unsafe {
        let mut properties: Vec<ffi::ExtensionProperties> = Vec::with_capacity(property_count as usize);
        let result = ffi::enumerate_instance_extension_properties(std::ptr::null(), &mut property_count, properties.as_mut_ptr());
        if result != ffi::Result::Success { return Err(result) };
        properties.set_len(property_count as usize);
        properties
    };

    let result: Vec<ExtensionProperties> = result.into_iter().map(|x: ffi::ExtensionProperties| unsafe {
        ExtensionProperties::from_ffi(x)
    }).collect();

    return Ok(result);
}

pub fn enumerate_instance_layer_extension_properties(layer_name: &str) -> std::result::Result<Vec<ExtensionProperties>, ffi::Result> {
    let mut property_count = 0u32;
    let layer_name = std::ffi::CString::new(layer_name).unwrap();

    unsafe {
        let result = ffi::enumerate_instance_extension_properties(layer_name.as_ptr(), &mut property_count, std::ptr::null_mut());
        if result != ffi::Result::Success { return Err(result) };
    }

    let result = unsafe {
        let mut properties: Vec<ffi::ExtensionProperties> = Vec::with_capacity(property_count as usize);
        let result = ffi::enumerate_instance_extension_properties(layer_name.as_ptr(), &mut property_count, properties.as_mut_ptr());
        if result != ffi::Result::Success { return Err(result) };
        properties.set_len(property_count as usize);
        properties
    };

    let result: Vec<ExtensionProperties> = result.into_iter().map(|x: ffi::ExtensionProperties| unsafe {
        ExtensionProperties::from_ffi(x)
    }).collect();

    return Ok(result);
}
