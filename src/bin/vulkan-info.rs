extern crate vulkan;

use vulkan::ffi;

fn main() {
    let extension_properties = vulkan::enumerate_instance_global_extension_properties().unwrap();

    for extension_property in extension_properties {
        println!("{}", extension_property.extension_name());
        println!("\tspec version: {:?}", extension_property.spec_version());
    }

    for layer_property in vulkan::enumerate_instance_layer_properties().unwrap() {
        println!("{}", layer_property.layer_name());
        println!("\tspec version: {:?}", layer_property.spec_version());
        println!("\timplementation version: {:?}", layer_property.implementation_version());
        println!("\tdescription: {}", layer_property.description());

        let extension_properties = vulkan::enumerate_instance_layer_extension_properties(layer_property.layer_name()).unwrap();

        println!("\textension_count: {}", extension_properties.len());

        for extension_property in extension_properties {
            println!("\t{}", extension_property.extension_name());
            println!("\t\tspec version: {:?}", extension_property.spec_version());
        }
    }
}
