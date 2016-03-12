extern crate vulkan;

#[test]
fn it_creates_an_instance_using_ffi() {
    use std::ptr;

    use vulkan::ffi::*;

    let ai = ApplicationInfo {
        structure_ty: StructureType::ApplicationInfo,
        next: ptr::null(),
        application_name: b"Rust Vulkan Demo\0".as_ptr() as *const i8,
        application_version: 1,
        engine_name: b"vulkan.rs\0".as_ptr() as *const i8,
        engine_version: 1,
        api_version: 0
    };

    let ici = InstanceCreateInfo {
        structure_ty: StructureType::ApplicationInfo,
        next: ptr::null(),
        flags: 0,
        application_info: &ai,
        enabled_layer_count: 0,
        enabled_layer_names: ptr::null(),
        enabled_extension_count: 0,
        enabled_extension_names: ptr::null()
    };

    let mut instance = 0;
    let result = unsafe { vkCreateInstance(&ici, ptr::null(), &mut instance) };

    if result != Result::Success { panic!("Failed to create Vulkan instance") };

    unsafe { vkDestroyInstance(instance, ptr::null()) };
}

#[test]
fn it_creates_a_rust_instance() {
    use vulkan::*;

    // let ai = ApplicationInfo::new(None, "Rust Vulkan Demo", 1, "vulkan.rs", 1, 0);
    // let ici = InstanceCreateInfo::new(None, 0, &ai, vec![], vec![]);

    // let _ = Instance::new(&ici, None).unwrap();
}
