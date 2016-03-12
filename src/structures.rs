use std;

use libc;

use ffi;

pub type SampleMask = ffi::SampleMask;
pub type Bool32 = ffi::Bool32;
pub type Flags = ffi::Flags;
pub type DeviceSize = ffi::DeviceSize;
pub type FramebufferCreateFlags = ffi::FramebufferCreateFlags;
pub type QueryPoolCreateFlags = ffi::QueryPoolCreateFlags;
pub type RenderPassCreateFlags = ffi::RenderPassCreateFlags;
pub type SamplerCreateFlags = ffi::SamplerCreateFlags;
pub type PipelineLayoutCreateFlags = ffi::PipelineLayoutCreateFlags;
pub type PipelineCacheCreateFlags = ffi::PipelineCacheCreateFlags;
pub type PipelineDepthStencilStateCreateFlags = ffi::PipelineDepthStencilStateCreateFlags;
pub type PipelineDynamicStateCreateFlags = ffi::PipelineDynamicStateCreateFlags;
pub type PipelineColorBlendStateCreateFlags = ffi::PipelineColorBlendStateCreateFlags;
pub type PipelineMultisampleStateCreateFlags = ffi::PipelineMultisampleStateCreateFlags;
pub type PipelineRasterizationStateCreateFlags = ffi::PipelineRasterizationStateCreateFlags;
pub type PipelineViewportStateCreateFlags = ffi::PipelineViewportStateCreateFlags;
pub type PipelineTessellationStateCreateFlags = ffi::PipelineTessellationStateCreateFlags;
pub type PipelineInputAssemblyStateCreateFlags = ffi::PipelineInputAssemblyStateCreateFlags;
pub type PipelineVertexInputStateCreateFlags = ffi::PipelineVertexInputStateCreateFlags;
pub type PipelineShaderStageCreateFlags = ffi::PipelineShaderStageCreateFlags;
pub type DescriptorSetLayoutCreateFlags = ffi::DescriptorSetLayoutCreateFlags;
pub type BufferViewCreateFlags = ffi::BufferViewCreateFlags;
pub type InstanceCreateFlags = ffi::InstanceCreateFlags;
pub type DeviceCreateFlags = ffi::DeviceCreateFlags;
pub type DeviceQueueCreateFlags = ffi::DeviceQueueCreateFlags;
pub type QueueFlags = ffi::QueueFlags;
pub type MemoryPropertyFlags = ffi::MemoryPropertyFlags;
pub type MemoryHeapFlags = ffi::MemoryHeapFlags;
pub type AccessFlags = ffi::AccessFlags;
pub type BufferUsageFlags = ffi::BufferUsageFlags;
pub type BufferCreateFlags = ffi::BufferCreateFlags;
pub type ShaderStageFlags = ffi::ShaderStageFlags;
pub type ImageUsageFlags = ffi::ImageUsageFlags;
pub type ImageCreateFlags = ffi::ImageCreateFlags;
pub type ImageViewCreateFlags = ffi::ImageViewCreateFlags;
pub type PipelineCreateFlags = ffi::PipelineCreateFlags;
pub type ColorComponentFlags = ffi::ColorComponentFlags;
pub type FenceCreateFlags = ffi::FenceCreateFlags;
pub type SemaphoreCreateFlags = ffi::SemaphoreCreateFlags;
pub type FormatFeatureFlags = ffi::FormatFeatureFlags;
pub type QueryControlFlags = ffi::QueryControlFlags;
pub type QueryResultFlags = ffi::QueryResultFlags;
pub type ShaderModuleCreateFlags = ffi::ShaderModuleCreateFlags;
pub type EventCreateFlags = ffi::EventCreateFlags;
pub type CommandPoolCreateFlags = ffi::CommandPoolCreateFlags;
pub type CommandPoolResetFlags = ffi::CommandPoolResetFlags;
pub type CommandBufferResetFlags = ffi::CommandBufferResetFlags;
pub type CommandBufferUsageFlags = ffi::CommandBufferUsageFlags;
pub type QueryPipelineStatisticFlags = ffi::QueryPipelineStatisticFlags;
pub type MemoryMapFlags = ffi::MemoryMapFlags;
pub type ImageAspectFlags = ffi::ImageAspectFlags;
pub type SparseMemoryBindFlags = ffi::SparseMemoryBindFlags;
pub type SparseImageFormatFlags = ffi::SparseImageFormatFlags;
pub type SubpassDescriptionFlags = ffi::SubpassDescriptionFlags;
pub type PipelineStageFlags = ffi::PipelineStageFlags;
pub type SampleCountFlags = ffi::SampleCountFlags;
pub type AttachmentDescriptionFlags = ffi::AttachmentDescriptionFlags;
pub type StencilFaceFlags = ffi::StencilFaceFlags;
pub type CullModeFlags = ffi::CullModeFlags;
pub type DescriptorPoolCreateFlags = ffi::DescriptorPoolCreateFlags;
pub type DescriptorPoolResetFlags = ffi::DescriptorPoolResetFlags;
pub type DependencyFlags = ffi::DependencyFlags;
pub type CompositeAlphaFlagsKHR = ffi::CompositeAlphaFlagsKHR;
pub type DisplayPlaneAlphaFlagsKHR = ffi::DisplayPlaneAlphaFlagsKHR;
pub type SurfaceTransformFlagsKHR = ffi::SurfaceTransformFlagsKHR;
pub type SwapchainCreateFlagsKHR = ffi::SwapchainCreateFlagsKHR;
pub type DisplayModeCreateFlagsKHR = ffi::DisplayModeCreateFlagsKHR;
pub type DisplaySurfaceCreateFlagsKHR = ffi::DisplaySurfaceCreateFlagsKHR;
pub type AndroidSurfaceCreateFlagsKHR = ffi::AndroidSurfaceCreateFlagsKHR;
pub type MirSurfaceCreateFlagsKHR = ffi::MirSurfaceCreateFlagsKHR;
pub type WaylandSurfaceCreateFlagsKHR = ffi::WaylandSurfaceCreateFlagsKHR;
pub type Win32SurfaceCreateFlagsKHR = ffi::Win32SurfaceCreateFlagsKHR;
pub type XlibSurfaceCreateFlagsKHR = ffi::XlibSurfaceCreateFlagsKHR;
pub type XcbSurfaceCreateFlagsKHR = ffi::XcbSurfaceCreateFlagsKHR;
pub type DebugReportFlagsEXT = ffi::DebugReportFlagsEXT;
pub type ImageLayout = ffi::ImageLayout;
pub type AttachmentLoadOp = ffi::AttachmentLoadOp;
pub type AttachmentStoreOp = ffi::AttachmentStoreOp;
pub type ImageType = ffi::ImageType;
pub type ImageTiling = ffi::ImageTiling;
pub type ImageViewType = ffi::ImageViewType;
pub type CommandBufferLevel = ffi::CommandBufferLevel;
pub type ComponentSwizzle = ffi::ComponentSwizzle;
pub type DescriptorType = ffi::DescriptorType;
pub type QueryType = ffi::QueryType;
pub type BorderColor = ffi::BorderColor;
pub type PipelineBindPoint = ffi::PipelineBindPoint;
pub type PipelineCacheHeaderVersion = ffi::PipelineCacheHeaderVersion;
pub type PrimitiveTopology = ffi::PrimitiveTopology;
pub type SharingMode = ffi::SharingMode;
pub type IndexType = ffi::IndexType;
pub type Filter = ffi::Filter;
pub type SamplerMipmapMode = ffi::SamplerMipmapMode;
pub type SamplerAddressMode = ffi::SamplerAddressMode;
pub type CompareOp = ffi::CompareOp;
pub type PolygonMode = ffi::PolygonMode;
pub type FrontFace = ffi::FrontFace;
pub type BlendFactor = ffi::BlendFactor;
pub type BlendOp = ffi::BlendOp;
pub type StencilOp = ffi::StencilOp;
pub type LogicOp = ffi::LogicOp;
pub type InternalAllocationType = ffi::InternalAllocationType;
pub type SystemAllocationScope = ffi::SystemAllocationScope;
pub type PhysicalDeviceType = ffi::PhysicalDeviceType;
pub type VertexInputRate = ffi::VertexInputRate;
pub type Format = ffi::Format;
pub type StructureType = ffi::StructureType;
pub type SubpassContents = ffi::SubpassContents;
pub type Result = ffi::Result;
pub type DynamicState = ffi::DynamicState;
pub type PresentModeKHR = ffi::PresentModeKHR;
pub type ColorSpaceKHR = ffi::ColorSpaceKHR;
#[cfg(vk_ext_debug_report)]
pub type DebugReportObjectTypeEXT = ffi::DebugReportObjectTypeEXT;
#[cfg(vk_ext_debug_report)]
pub type DebugReportErrorEXT = ffi::DebugReportErrorEXT;
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = ffi::MAX_PHYSICAL_DEVICE_NAME_SIZE as usize;
pub const UUID_SIZE: usize = ffi::UUID_SIZE as usize;
pub const MAX_EXTENSION_NAME_SIZE: usize = ffi::MAX_EXTENSION_NAME_SIZE as usize;
pub const MAX_DESCRIPTION_SIZE: usize = ffi::MAX_DESCRIPTION_SIZE as usize;
pub const MAX_MEMORY_TYPES: usize = ffi::MAX_MEMORY_TYPES as usize;
pub const MAX_MEMORY_HEAPS: usize = ffi::MAX_MEMORY_HEAPS as usize;
pub const LOD_CLAMP_NONE: f32 = ffi::LOD_CLAMP_NONE as f32;
pub const REMAINING_MIP_LEVELS: usize = ffi::REMAINING_MIP_LEVELS as usize;
pub const REMAINING_ARRAY_LAYERS: usize = ffi::REMAINING_ARRAY_LAYERS as usize;
pub const WHOLE_SIZE: usize = ffi::WHOLE_SIZE as usize;
pub const ATTACHMENT_UNUSED: usize = ffi::ATTACHMENT_UNUSED as usize;
pub const TRUE: usize = ffi::TRUE as usize;
pub const FALSE: usize = ffi::FALSE as usize;
pub const QUEUE_FAMILY_IGNORED: usize = ffi::QUEUE_FAMILY_IGNORED as usize;
pub const SUBPASS_EXTERNAL: usize = ffi::SUBPASS_EXTERNAL as usize;

pub struct Instance {
	pub handle: ffi::Instance
}

impl Instance {
	pub fn from_ffi(raw: ffi::Instance) -> Instance {
		return Instance { handle: raw };
	}

	pub unsafe fn destroy_instance(&self, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_instance(self.handle, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn enumerate_physical_devices(&self, physical_device_count: *mut u32, physical_devices: *mut libc::size_t) -> ffi::Result {
		return ffi::enumerate_physical_devices(self.handle, physical_device_count as *mut u32, physical_devices as *mut libc::size_t);
	}

	pub unsafe fn get_instance_proc_addr(&self, name: *const libc::c_char) -> ffi::VoidFunctionCallback {
		return ffi::get_instance_proc_addr(self.handle, name as *const libc::c_char);
	}

	#[cfg(vk_khr_android_surface)]
	pub unsafe fn create_android_surface_khr(&self, create_info: *const ffi::AndroidSurfaceCreateInfoKHR, allocator: *const ffi::AllocationCallbacks, surface: *mut u64) -> ffi::Result {
		return ffi::create_android_surface_khr(self.handle, create_info as *const ffi::AndroidSurfaceCreateInfoKHR, allocator as *const ffi::AllocationCallbacks, surface as *mut u64);
	}

	#[cfg(vk_khr_display)]
	pub unsafe fn create_display_plane_surface_khr(&self, create_info: *const ffi::DisplaySurfaceCreateInfoKHR, allocator: *const ffi::AllocationCallbacks, surface: *mut u64) -> ffi::Result {
		return ffi::create_display_plane_surface_khr(self.handle, create_info as *const ffi::DisplaySurfaceCreateInfoKHR, allocator as *const ffi::AllocationCallbacks, surface as *mut u64);
	}

	#[cfg(vk_khr_mir_surface)]
	pub unsafe fn create_mir_surface_khr(&self, create_info: *const ffi::MirSurfaceCreateInfoKHR, allocator: *const ffi::AllocationCallbacks, surface: *mut u64) -> ffi::Result {
		return ffi::create_mir_surface_khr(self.handle, create_info as *const ffi::MirSurfaceCreateInfoKHR, allocator as *const ffi::AllocationCallbacks, surface as *mut u64);
	}

	#[cfg(vk_khr_surface)]
	pub unsafe fn destroy_surface_khr(&self, surface: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_surface_khr(self.handle, surface as u64, allocator as *const ffi::AllocationCallbacks);
	}

	#[cfg(vk_khr_wayland_surface)]
	pub unsafe fn create_wayland_surface_khr(&self, create_info: *const ffi::WaylandSurfaceCreateInfoKHR, allocator: *const ffi::AllocationCallbacks, surface: *mut u64) -> ffi::Result {
		return ffi::create_wayland_surface_khr(self.handle, create_info as *const ffi::WaylandSurfaceCreateInfoKHR, allocator as *const ffi::AllocationCallbacks, surface as *mut u64);
	}

	#[cfg(vk_khr_win32_surface)]
	pub unsafe fn create_win32_surface_khr(&self, create_info: *const ffi::Win32SurfaceCreateInfoKHR, allocator: *const ffi::AllocationCallbacks, surface: *mut u64) -> ffi::Result {
		return ffi::create_win32_surface_khr(self.handle, create_info as *const ffi::Win32SurfaceCreateInfoKHR, allocator as *const ffi::AllocationCallbacks, surface as *mut u64);
	}

	#[cfg(vk_khr_xlib_surface)]
	pub unsafe fn create_xlib_surface_khr(&self, create_info: *const ffi::XlibSurfaceCreateInfoKHR, allocator: *const ffi::AllocationCallbacks, surface: *mut u64) -> ffi::Result {
		return ffi::create_xlib_surface_khr(self.handle, create_info as *const ffi::XlibSurfaceCreateInfoKHR, allocator as *const ffi::AllocationCallbacks, surface as *mut u64);
	}

	#[cfg(vk_khr_xcb_surface)]
	pub unsafe fn create_xcb_surface_khr(&self, create_info: *const ffi::XcbSurfaceCreateInfoKHR, allocator: *const ffi::AllocationCallbacks, surface: *mut u64) -> ffi::Result {
		return ffi::create_xcb_surface_khr(self.handle, create_info as *const ffi::XcbSurfaceCreateInfoKHR, allocator as *const ffi::AllocationCallbacks, surface as *mut u64);
	}

	#[cfg(vk_ext_debug_report)]
	pub unsafe fn create_debug_report_callback_ext(&self, create_info: *const ffi::DebugReportCallbackCreateInfoEXT, allocator: *const ffi::AllocationCallbacks, callback: *mut u64) -> ffi::Result {
		return ffi::create_debug_report_callback_ext(self.handle, create_info as *const ffi::DebugReportCallbackCreateInfoEXT, allocator as *const ffi::AllocationCallbacks, callback as *mut u64);
	}

	#[cfg(vk_ext_debug_report)]
	pub unsafe fn destroy_debug_report_callback_ext(&self, callback: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_debug_report_callback_ext(self.handle, callback as u64, allocator as *const ffi::AllocationCallbacks);
	}

	#[cfg(vk_ext_debug_report)]
	pub unsafe fn debug_report_message_ext(&self, flags: ffi::DebugReportFlagsEXT, object_type: ffi::DebugReportObjectTypeEXT, object: u64, location: libc::size_t, message_code: i32, layer_prefix: *const libc::c_char, message: *const libc::c_char) {
		return ffi::debug_report_message_ext(self.handle, flags as ffi::DebugReportFlagsEXT, object_type as ffi::DebugReportObjectTypeEXT, object as u64, location as libc::size_t, message_code as i32, layer_prefix as *const libc::c_char, message as *const libc::c_char);
	}
}

pub struct PhysicalDevice {
	pub handle: ffi::PhysicalDevice
}

impl PhysicalDevice {
	pub fn from_ffi(raw: ffi::PhysicalDevice) -> PhysicalDevice {
		return PhysicalDevice { handle: raw };
	}

	pub unsafe fn get_physical_device_properties(&self, properties: *mut ffi::PhysicalDeviceProperties) {
		return ffi::get_physical_device_properties(self.handle, properties as *mut ffi::PhysicalDeviceProperties);
	}

	pub unsafe fn get_physical_device_queue_family_properties(&self, queue_family_property_count: *mut u32, queue_family_properties: *mut ffi::QueueFamilyProperties) {
		return ffi::get_physical_device_queue_family_properties(self.handle, queue_family_property_count as *mut u32, queue_family_properties as *mut ffi::QueueFamilyProperties);
	}

	pub unsafe fn get_physical_device_memory_properties(&self, memory_properties: *mut ffi::PhysicalDeviceMemoryProperties) {
		return ffi::get_physical_device_memory_properties(self.handle, memory_properties as *mut ffi::PhysicalDeviceMemoryProperties);
	}

	pub unsafe fn get_physical_device_features(&self, features: *mut ffi::PhysicalDeviceFeatures) {
		return ffi::get_physical_device_features(self.handle, features as *mut ffi::PhysicalDeviceFeatures);
	}

	pub unsafe fn get_physical_device_format_properties(&self, format: ffi::Format, format_properties: *mut ffi::FormatProperties) {
		return ffi::get_physical_device_format_properties(self.handle, format as ffi::Format, format_properties as *mut ffi::FormatProperties);
	}

	pub unsafe fn get_physical_device_image_format_properties(&self, format: ffi::Format, ty: ffi::ImageType, tiling: ffi::ImageTiling, usage: ffi::ImageUsageFlags, flags: ffi::ImageCreateFlags, image_format_properties: *mut ffi::ImageFormatProperties) -> ffi::Result {
		return ffi::get_physical_device_image_format_properties(self.handle, format as ffi::Format, ty as ffi::ImageType, tiling as ffi::ImageTiling, usage as ffi::ImageUsageFlags, flags as ffi::ImageCreateFlags, image_format_properties as *mut ffi::ImageFormatProperties);
	}

	pub unsafe fn create_device(&self, create_info: *const ffi::DeviceCreateInfo, allocator: *const ffi::AllocationCallbacks, device: *mut libc::size_t) -> ffi::Result {
		return ffi::create_device(self.handle, create_info as *const ffi::DeviceCreateInfo, allocator as *const ffi::AllocationCallbacks, device as *mut libc::size_t);
	}

	pub unsafe fn enumerate_device_layer_properties(&self, property_count: *mut u32, properties: *mut ffi::LayerProperties) -> ffi::Result {
		return ffi::enumerate_device_layer_properties(self.handle, property_count as *mut u32, properties as *mut ffi::LayerProperties);
	}

	pub unsafe fn enumerate_device_extension_properties(&self, layer_name: *const libc::c_char, property_count: *mut u32, properties: *mut ffi::ExtensionProperties) -> ffi::Result {
		return ffi::enumerate_device_extension_properties(self.handle, layer_name as *const libc::c_char, property_count as *mut u32, properties as *mut ffi::ExtensionProperties);
	}

	pub unsafe fn get_physical_device_sparse_image_format_properties(&self, format: ffi::Format, ty: ffi::ImageType, samples: ffi::SampleCountFlags, usage: ffi::ImageUsageFlags, tiling: ffi::ImageTiling, property_count: *mut u32, properties: *mut ffi::SparseImageFormatProperties) {
		return ffi::get_physical_device_sparse_image_format_properties(self.handle, format as ffi::Format, ty as ffi::ImageType, samples as ffi::SampleCountFlags, usage as ffi::ImageUsageFlags, tiling as ffi::ImageTiling, property_count as *mut u32, properties as *mut ffi::SparseImageFormatProperties);
	}

	#[cfg(vk_khr_display)]
	pub unsafe fn get_physical_device_display_properties_khr(&self, property_count: *mut u32, properties: *mut ffi::DisplayPropertiesKHR) -> ffi::Result {
		return ffi::get_physical_device_display_properties_khr(self.handle, property_count as *mut u32, properties as *mut ffi::DisplayPropertiesKHR);
	}

	#[cfg(vk_khr_display)]
	pub unsafe fn get_physical_device_display_plane_properties_khr(&self, property_count: *mut u32, properties: *mut ffi::DisplayPlanePropertiesKHR) -> ffi::Result {
		return ffi::get_physical_device_display_plane_properties_khr(self.handle, property_count as *mut u32, properties as *mut ffi::DisplayPlanePropertiesKHR);
	}

	#[cfg(vk_khr_display)]
	pub unsafe fn get_display_plane_supported_displays_khr(&self, plane_index: u32, display_count: *mut u32, displays: *mut u64) -> ffi::Result {
		return ffi::get_display_plane_supported_displays_khr(self.handle, plane_index as u32, display_count as *mut u32, displays as *mut u64);
	}

	#[cfg(vk_khr_display)]
	pub unsafe fn get_display_mode_properties_khr(&self, display: u64, property_count: *mut u32, properties: *mut ffi::DisplayModePropertiesKHR) -> ffi::Result {
		return ffi::get_display_mode_properties_khr(self.handle, display as u64, property_count as *mut u32, properties as *mut ffi::DisplayModePropertiesKHR);
	}

	#[cfg(vk_khr_display)]
	pub unsafe fn create_display_mode_khr(&self, display: u64, create_info: *const ffi::DisplayModeCreateInfoKHR, allocator: *const ffi::AllocationCallbacks, mode: *mut u64) -> ffi::Result {
		return ffi::create_display_mode_khr(self.handle, display as u64, create_info as *const ffi::DisplayModeCreateInfoKHR, allocator as *const ffi::AllocationCallbacks, mode as *mut u64);
	}

	#[cfg(vk_khr_display)]
	pub unsafe fn get_display_plane_capabilities_khr(&self, mode: u64, plane_index: u32, capabilities: *mut ffi::DisplayPlaneCapabilitiesKHR) -> ffi::Result {
		return ffi::get_display_plane_capabilities_khr(self.handle, mode as u64, plane_index as u32, capabilities as *mut ffi::DisplayPlaneCapabilitiesKHR);
	}

	#[cfg(vk_khr_mir_surface)]
	pub unsafe fn get_physical_device_mir_presentation_support_khr(&self, queue_family_index: u32, connection: libc::size_t) -> ffi::Bool32 {
		return ffi::get_physical_device_mir_presentation_support_khr(self.handle, queue_family_index as u32, connection as libc::size_t);
	}

	#[cfg(vk_khr_surface)]
	pub unsafe fn get_physical_device_surface_support_khr(&self, queue_family_index: u32, surface: u64, supported: *mut ffi::Bool32) -> ffi::Result {
		return ffi::get_physical_device_surface_support_khr(self.handle, queue_family_index as u32, surface as u64, supported as *mut ffi::Bool32);
	}

	#[cfg(vk_khr_surface)]
	pub unsafe fn get_physical_device_surface_capabilities_khr(&self, surface: u64, surface_capabilities: *mut ffi::SurfaceCapabilitiesKHR) -> ffi::Result {
		return ffi::get_physical_device_surface_capabilities_khr(self.handle, surface as u64, surface_capabilities as *mut ffi::SurfaceCapabilitiesKHR);
	}

	#[cfg(vk_khr_surface)]
	pub unsafe fn get_physical_device_surface_formats_khr(&self, surface: u64, surface_format_count: *mut u32, surface_formats: *mut ffi::SurfaceFormatKHR) -> ffi::Result {
		return ffi::get_physical_device_surface_formats_khr(self.handle, surface as u64, surface_format_count as *mut u32, surface_formats as *mut ffi::SurfaceFormatKHR);
	}

	#[cfg(vk_khr_surface)]
	pub unsafe fn get_physical_device_surface_present_modes_khr(&self, surface: u64, present_mode_count: *mut u32, present_modes: *mut ffi::PresentModeKHR) -> ffi::Result {
		return ffi::get_physical_device_surface_present_modes_khr(self.handle, surface as u64, present_mode_count as *mut u32, present_modes as *mut ffi::PresentModeKHR);
	}

	#[cfg(vk_khr_wayland_surface)]
	pub unsafe fn get_physical_device_wayland_presentation_support_khr(&self, queue_family_index: u32, display: libc::size_t) -> ffi::Bool32 {
		return ffi::get_physical_device_wayland_presentation_support_khr(self.handle, queue_family_index as u32, display as libc::size_t);
	}

	#[cfg(vk_khr_win32_surface)]
	pub unsafe fn get_physical_device_win32_presentation_support_khr(&self, queue_family_index: u32) -> ffi::Bool32 {
		return ffi::get_physical_device_win32_presentation_support_khr(self.handle, queue_family_index as u32);
	}

	#[cfg(vk_khr_xlib_surface)]
	pub unsafe fn get_physical_device_xlib_presentation_support_khr(&self, queue_family_index: u32, dpy: libc::size_t, visual_id: libc::size_t) -> ffi::Bool32 {
		return ffi::get_physical_device_xlib_presentation_support_khr(self.handle, queue_family_index as u32, dpy as libc::size_t, visual_id as libc::size_t);
	}

	#[cfg(vk_khr_xcb_surface)]
	pub unsafe fn get_physical_device_xcb_presentation_support_khr(&self, queue_family_index: u32, connection: libc::size_t, visual_id: libc::size_t) -> ffi::Bool32 {
		return ffi::get_physical_device_xcb_presentation_support_khr(self.handle, queue_family_index as u32, connection as libc::size_t, visual_id as libc::size_t);
	}
}

pub struct Device {
	pub handle: ffi::Device
}

impl Device {
	pub fn from_ffi(raw: ffi::Device) -> Device {
		return Device { handle: raw };
	}

	pub unsafe fn get_device_proc_addr(&self, name: *const libc::c_char) -> ffi::VoidFunctionCallback {
		return ffi::get_device_proc_addr(self.handle, name as *const libc::c_char);
	}

	pub unsafe fn destroy_device(&self, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_device(self.handle, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn get_device_queue(&self, queue_family_index: u32, queue_index: u32, queue: *mut libc::size_t) {
		return ffi::get_device_queue(self.handle, queue_family_index as u32, queue_index as u32, queue as *mut libc::size_t);
	}

	pub unsafe fn device_wait_idle(&self, ) -> ffi::Result {
		return ffi::device_wait_idle(self.handle, );
	}

	pub unsafe fn allocate_memory(&self, allocate_info: *const ffi::MemoryAllocateInfo, allocator: *const ffi::AllocationCallbacks, memory: *mut u64) -> ffi::Result {
		return ffi::allocate_memory(self.handle, allocate_info as *const ffi::MemoryAllocateInfo, allocator as *const ffi::AllocationCallbacks, memory as *mut u64);
	}

	pub unsafe fn free_memory(&self, memory: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::free_memory(self.handle, memory as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn map_memory(&self, memory: u64, offset: ffi::DeviceSize, size: ffi::DeviceSize, flags: ffi::MemoryMapFlags, data: *const *const libc::c_void) -> ffi::Result {
		return ffi::map_memory(self.handle, memory as u64, offset as ffi::DeviceSize, size as ffi::DeviceSize, flags as ffi::MemoryMapFlags, data as *const *const libc::c_void);
	}

	pub unsafe fn unmap_memory(&self, memory: u64) {
		return ffi::unmap_memory(self.handle, memory as u64);
	}

	pub unsafe fn flush_mapped_memory_ranges(&self, memory_range_count: u32, memory_ranges: *const ffi::MappedMemoryRange) -> ffi::Result {
		return ffi::flush_mapped_memory_ranges(self.handle, memory_range_count as u32, memory_ranges as *const ffi::MappedMemoryRange);
	}

	pub unsafe fn invalidate_mapped_memory_ranges(&self, memory_range_count: u32, memory_ranges: *const ffi::MappedMemoryRange) -> ffi::Result {
		return ffi::invalidate_mapped_memory_ranges(self.handle, memory_range_count as u32, memory_ranges as *const ffi::MappedMemoryRange);
	}

	pub unsafe fn get_device_memory_commitment(&self, memory: u64, committed_memory_in_bytes: *mut ffi::DeviceSize) {
		return ffi::get_device_memory_commitment(self.handle, memory as u64, committed_memory_in_bytes as *mut ffi::DeviceSize);
	}

	pub unsafe fn get_buffer_memory_requirements(&self, buffer: u64, memory_requirements: *mut ffi::MemoryRequirements) {
		return ffi::get_buffer_memory_requirements(self.handle, buffer as u64, memory_requirements as *mut ffi::MemoryRequirements);
	}

	pub unsafe fn bind_buffer_memory(&self, buffer: u64, memory: u64, memory_offset: ffi::DeviceSize) -> ffi::Result {
		return ffi::bind_buffer_memory(self.handle, buffer as u64, memory as u64, memory_offset as ffi::DeviceSize);
	}

	pub unsafe fn get_image_memory_requirements(&self, image: u64, memory_requirements: *mut ffi::MemoryRequirements) {
		return ffi::get_image_memory_requirements(self.handle, image as u64, memory_requirements as *mut ffi::MemoryRequirements);
	}

	pub unsafe fn bind_image_memory(&self, image: u64, memory: u64, memory_offset: ffi::DeviceSize) -> ffi::Result {
		return ffi::bind_image_memory(self.handle, image as u64, memory as u64, memory_offset as ffi::DeviceSize);
	}

	pub unsafe fn get_image_sparse_memory_requirements(&self, image: u64, sparse_memory_requirement_count: *mut u32, sparse_memory_requirements: *mut ffi::SparseImageMemoryRequirements) {
		return ffi::get_image_sparse_memory_requirements(self.handle, image as u64, sparse_memory_requirement_count as *mut u32, sparse_memory_requirements as *mut ffi::SparseImageMemoryRequirements);
	}

	pub unsafe fn create_fence(&self, create_info: *const ffi::FenceCreateInfo, allocator: *const ffi::AllocationCallbacks, fence: *mut u64) -> ffi::Result {
		return ffi::create_fence(self.handle, create_info as *const ffi::FenceCreateInfo, allocator as *const ffi::AllocationCallbacks, fence as *mut u64);
	}

	pub unsafe fn destroy_fence(&self, fence: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_fence(self.handle, fence as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn reset_fences(&self, fence_count: u32, fences: *const u64) -> ffi::Result {
		return ffi::reset_fences(self.handle, fence_count as u32, fences as *const u64);
	}

	pub unsafe fn get_fence_status(&self, fence: u64) -> ffi::Result {
		return ffi::get_fence_status(self.handle, fence as u64);
	}

	pub unsafe fn wait_for_fences(&self, fence_count: u32, fences: *const u64, wait_all: ffi::Bool32, timeout: u64) -> ffi::Result {
		return ffi::wait_for_fences(self.handle, fence_count as u32, fences as *const u64, wait_all as ffi::Bool32, timeout as u64);
	}

	pub unsafe fn create_semaphore(&self, create_info: *const ffi::SemaphoreCreateInfo, allocator: *const ffi::AllocationCallbacks, semaphore: *mut u64) -> ffi::Result {
		return ffi::create_semaphore(self.handle, create_info as *const ffi::SemaphoreCreateInfo, allocator as *const ffi::AllocationCallbacks, semaphore as *mut u64);
	}

	pub unsafe fn destroy_semaphore(&self, semaphore: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_semaphore(self.handle, semaphore as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn create_event(&self, create_info: *const ffi::EventCreateInfo, allocator: *const ffi::AllocationCallbacks, event: *mut u64) -> ffi::Result {
		return ffi::create_event(self.handle, create_info as *const ffi::EventCreateInfo, allocator as *const ffi::AllocationCallbacks, event as *mut u64);
	}

	pub unsafe fn destroy_event(&self, event: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_event(self.handle, event as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn get_event_status(&self, event: u64) -> ffi::Result {
		return ffi::get_event_status(self.handle, event as u64);
	}

	pub unsafe fn set_event(&self, event: u64) -> ffi::Result {
		return ffi::set_event(self.handle, event as u64);
	}

	pub unsafe fn reset_event(&self, event: u64) -> ffi::Result {
		return ffi::reset_event(self.handle, event as u64);
	}

	pub unsafe fn create_query_pool(&self, create_info: *const ffi::QueryPoolCreateInfo, allocator: *const ffi::AllocationCallbacks, query_pool: *mut u64) -> ffi::Result {
		return ffi::create_query_pool(self.handle, create_info as *const ffi::QueryPoolCreateInfo, allocator as *const ffi::AllocationCallbacks, query_pool as *mut u64);
	}

	pub unsafe fn destroy_query_pool(&self, query_pool: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_query_pool(self.handle, query_pool as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn get_query_pool_results(&self, query_pool: u64, first_query: u32, query_count: u32, data_size: libc::size_t, data: *mut libc::c_void, stride: ffi::DeviceSize, flags: ffi::QueryResultFlags) -> ffi::Result {
		return ffi::get_query_pool_results(self.handle, query_pool as u64, first_query as u32, query_count as u32, data_size as libc::size_t, data as *mut libc::c_void, stride as ffi::DeviceSize, flags as ffi::QueryResultFlags);
	}

	pub unsafe fn create_buffer(&self, create_info: *const ffi::BufferCreateInfo, allocator: *const ffi::AllocationCallbacks, buffer: *mut u64) -> ffi::Result {
		return ffi::create_buffer(self.handle, create_info as *const ffi::BufferCreateInfo, allocator as *const ffi::AllocationCallbacks, buffer as *mut u64);
	}

	pub unsafe fn destroy_buffer(&self, buffer: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_buffer(self.handle, buffer as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn create_buffer_view(&self, create_info: *const ffi::BufferViewCreateInfo, allocator: *const ffi::AllocationCallbacks, view: *mut u64) -> ffi::Result {
		return ffi::create_buffer_view(self.handle, create_info as *const ffi::BufferViewCreateInfo, allocator as *const ffi::AllocationCallbacks, view as *mut u64);
	}

	pub unsafe fn destroy_buffer_view(&self, buffer_view: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_buffer_view(self.handle, buffer_view as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn create_image(&self, create_info: *const ffi::ImageCreateInfo, allocator: *const ffi::AllocationCallbacks, image: *mut u64) -> ffi::Result {
		return ffi::create_image(self.handle, create_info as *const ffi::ImageCreateInfo, allocator as *const ffi::AllocationCallbacks, image as *mut u64);
	}

	pub unsafe fn destroy_image(&self, image: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_image(self.handle, image as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn get_image_subresource_layout(&self, image: u64, subresource: *const ffi::ImageSubresource, layout: *mut ffi::SubresourceLayout) {
		return ffi::get_image_subresource_layout(self.handle, image as u64, subresource as *const ffi::ImageSubresource, layout as *mut ffi::SubresourceLayout);
	}

	pub unsafe fn create_image_view(&self, create_info: *const ffi::ImageViewCreateInfo, allocator: *const ffi::AllocationCallbacks, view: *mut u64) -> ffi::Result {
		return ffi::create_image_view(self.handle, create_info as *const ffi::ImageViewCreateInfo, allocator as *const ffi::AllocationCallbacks, view as *mut u64);
	}

	pub unsafe fn destroy_image_view(&self, image_view: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_image_view(self.handle, image_view as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn create_shader_module(&self, create_info: *const ffi::ShaderModuleCreateInfo, allocator: *const ffi::AllocationCallbacks, shader_module: *mut u64) -> ffi::Result {
		return ffi::create_shader_module(self.handle, create_info as *const ffi::ShaderModuleCreateInfo, allocator as *const ffi::AllocationCallbacks, shader_module as *mut u64);
	}

	pub unsafe fn destroy_shader_module(&self, shader_module: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_shader_module(self.handle, shader_module as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn create_pipeline_cache(&self, create_info: *const ffi::PipelineCacheCreateInfo, allocator: *const ffi::AllocationCallbacks, pipeline_cache: *mut u64) -> ffi::Result {
		return ffi::create_pipeline_cache(self.handle, create_info as *const ffi::PipelineCacheCreateInfo, allocator as *const ffi::AllocationCallbacks, pipeline_cache as *mut u64);
	}

	pub unsafe fn destroy_pipeline_cache(&self, pipeline_cache: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_pipeline_cache(self.handle, pipeline_cache as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn get_pipeline_cache_data(&self, pipeline_cache: u64, data_size: *mut libc::size_t, data: *mut libc::c_void) -> ffi::Result {
		return ffi::get_pipeline_cache_data(self.handle, pipeline_cache as u64, data_size as *mut libc::size_t, data as *mut libc::c_void);
	}

	pub unsafe fn merge_pipeline_caches(&self, dst_cache: u64, src_cache_count: u32, src_caches: *const u64) -> ffi::Result {
		return ffi::merge_pipeline_caches(self.handle, dst_cache as u64, src_cache_count as u32, src_caches as *const u64);
	}

	pub unsafe fn create_graphics_pipelines(&self, pipeline_cache: u64, create_info_count: u32, create_infos: *const ffi::GraphicsPipelineCreateInfo, allocator: *const ffi::AllocationCallbacks, pipelines: *mut u64) -> ffi::Result {
		return ffi::create_graphics_pipelines(self.handle, pipeline_cache as u64, create_info_count as u32, create_infos as *const ffi::GraphicsPipelineCreateInfo, allocator as *const ffi::AllocationCallbacks, pipelines as *mut u64);
	}

	pub unsafe fn create_compute_pipelines(&self, pipeline_cache: u64, create_info_count: u32, create_infos: *const ffi::ComputePipelineCreateInfo, allocator: *const ffi::AllocationCallbacks, pipelines: *mut u64) -> ffi::Result {
		return ffi::create_compute_pipelines(self.handle, pipeline_cache as u64, create_info_count as u32, create_infos as *const ffi::ComputePipelineCreateInfo, allocator as *const ffi::AllocationCallbacks, pipelines as *mut u64);
	}

	pub unsafe fn destroy_pipeline(&self, pipeline: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_pipeline(self.handle, pipeline as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn create_pipeline_layout(&self, create_info: *const ffi::PipelineLayoutCreateInfo, allocator: *const ffi::AllocationCallbacks, pipeline_layout: *mut u64) -> ffi::Result {
		return ffi::create_pipeline_layout(self.handle, create_info as *const ffi::PipelineLayoutCreateInfo, allocator as *const ffi::AllocationCallbacks, pipeline_layout as *mut u64);
	}

	pub unsafe fn destroy_pipeline_layout(&self, pipeline_layout: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_pipeline_layout(self.handle, pipeline_layout as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn create_sampler(&self, create_info: *const ffi::SamplerCreateInfo, allocator: *const ffi::AllocationCallbacks, sampler: *mut u64) -> ffi::Result {
		return ffi::create_sampler(self.handle, create_info as *const ffi::SamplerCreateInfo, allocator as *const ffi::AllocationCallbacks, sampler as *mut u64);
	}

	pub unsafe fn destroy_sampler(&self, sampler: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_sampler(self.handle, sampler as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn create_descriptor_set_layout(&self, create_info: *const ffi::DescriptorSetLayoutCreateInfo, allocator: *const ffi::AllocationCallbacks, set_layout: *mut u64) -> ffi::Result {
		return ffi::create_descriptor_set_layout(self.handle, create_info as *const ffi::DescriptorSetLayoutCreateInfo, allocator as *const ffi::AllocationCallbacks, set_layout as *mut u64);
	}

	pub unsafe fn destroy_descriptor_set_layout(&self, descriptor_set_layout: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_descriptor_set_layout(self.handle, descriptor_set_layout as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn create_descriptor_pool(&self, create_info: *const ffi::DescriptorPoolCreateInfo, allocator: *const ffi::AllocationCallbacks, descriptor_pool: *mut u64) -> ffi::Result {
		return ffi::create_descriptor_pool(self.handle, create_info as *const ffi::DescriptorPoolCreateInfo, allocator as *const ffi::AllocationCallbacks, descriptor_pool as *mut u64);
	}

	pub unsafe fn destroy_descriptor_pool(&self, descriptor_pool: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_descriptor_pool(self.handle, descriptor_pool as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn reset_descriptor_pool(&self, descriptor_pool: u64, flags: ffi::DescriptorPoolResetFlags) -> ffi::Result {
		return ffi::reset_descriptor_pool(self.handle, descriptor_pool as u64, flags as ffi::DescriptorPoolResetFlags);
	}

	pub unsafe fn allocate_descriptor_sets(&self, allocate_info: *const ffi::DescriptorSetAllocateInfo, descriptor_sets: *mut u64) -> ffi::Result {
		return ffi::allocate_descriptor_sets(self.handle, allocate_info as *const ffi::DescriptorSetAllocateInfo, descriptor_sets as *mut u64);
	}

	pub unsafe fn free_descriptor_sets(&self, descriptor_pool: u64, descriptor_set_count: u32, descriptor_sets: *const u64) -> ffi::Result {
		return ffi::free_descriptor_sets(self.handle, descriptor_pool as u64, descriptor_set_count as u32, descriptor_sets as *const u64);
	}

	pub unsafe fn update_descriptor_sets(&self, descriptor_write_count: u32, descriptor_writes: *const ffi::WriteDescriptorSet, descriptor_copy_count: u32, descriptor_copies: *const ffi::CopyDescriptorSet) {
		return ffi::update_descriptor_sets(self.handle, descriptor_write_count as u32, descriptor_writes as *const ffi::WriteDescriptorSet, descriptor_copy_count as u32, descriptor_copies as *const ffi::CopyDescriptorSet);
	}

	pub unsafe fn create_framebuffer(&self, create_info: *const ffi::FramebufferCreateInfo, allocator: *const ffi::AllocationCallbacks, framebuffer: *mut u64) -> ffi::Result {
		return ffi::create_framebuffer(self.handle, create_info as *const ffi::FramebufferCreateInfo, allocator as *const ffi::AllocationCallbacks, framebuffer as *mut u64);
	}

	pub unsafe fn destroy_framebuffer(&self, framebuffer: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_framebuffer(self.handle, framebuffer as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn create_render_pass(&self, create_info: *const ffi::RenderPassCreateInfo, allocator: *const ffi::AllocationCallbacks, render_pass: *mut u64) -> ffi::Result {
		return ffi::create_render_pass(self.handle, create_info as *const ffi::RenderPassCreateInfo, allocator as *const ffi::AllocationCallbacks, render_pass as *mut u64);
	}

	pub unsafe fn destroy_render_pass(&self, render_pass: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_render_pass(self.handle, render_pass as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn get_render_area_granularity(&self, render_pass: u64, granularity: *mut ffi::Extent2D) {
		return ffi::get_render_area_granularity(self.handle, render_pass as u64, granularity as *mut ffi::Extent2D);
	}

	pub unsafe fn create_command_pool(&self, create_info: *const ffi::CommandPoolCreateInfo, allocator: *const ffi::AllocationCallbacks, command_pool: *mut u64) -> ffi::Result {
		return ffi::create_command_pool(self.handle, create_info as *const ffi::CommandPoolCreateInfo, allocator as *const ffi::AllocationCallbacks, command_pool as *mut u64);
	}

	pub unsafe fn destroy_command_pool(&self, command_pool: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_command_pool(self.handle, command_pool as u64, allocator as *const ffi::AllocationCallbacks);
	}

	pub unsafe fn reset_command_pool(&self, command_pool: u64, flags: ffi::CommandPoolResetFlags) -> ffi::Result {
		return ffi::reset_command_pool(self.handle, command_pool as u64, flags as ffi::CommandPoolResetFlags);
	}

	pub unsafe fn allocate_command_buffers(&self, allocate_info: *const ffi::CommandBufferAllocateInfo, command_buffers: *mut libc::size_t) -> ffi::Result {
		return ffi::allocate_command_buffers(self.handle, allocate_info as *const ffi::CommandBufferAllocateInfo, command_buffers as *mut libc::size_t);
	}

	pub unsafe fn free_command_buffers(&self, command_pool: u64, command_buffer_count: u32, command_buffers: *const libc::size_t) {
		return ffi::free_command_buffers(self.handle, command_pool as u64, command_buffer_count as u32, command_buffers as *const libc::size_t);
	}

	#[cfg(vk_khr_display_swapchain)]
	pub unsafe fn create_shared_swapchains_khr(&self, swapchain_count: u32, create_infos: *const ffi::SwapchainCreateInfoKHR, allocator: *const ffi::AllocationCallbacks, swapchains: *mut u64) -> ffi::Result {
		return ffi::create_shared_swapchains_khr(self.handle, swapchain_count as u32, create_infos as *const ffi::SwapchainCreateInfoKHR, allocator as *const ffi::AllocationCallbacks, swapchains as *mut u64);
	}

	#[cfg(vk_khr_swapchain)]
	pub unsafe fn create_swapchain_khr(&self, create_info: *const ffi::SwapchainCreateInfoKHR, allocator: *const ffi::AllocationCallbacks, swapchain: *mut u64) -> ffi::Result {
		return ffi::create_swapchain_khr(self.handle, create_info as *const ffi::SwapchainCreateInfoKHR, allocator as *const ffi::AllocationCallbacks, swapchain as *mut u64);
	}

	#[cfg(vk_khr_swapchain)]
	pub unsafe fn destroy_swapchain_khr(&self, swapchain: u64, allocator: *const ffi::AllocationCallbacks) {
		return ffi::destroy_swapchain_khr(self.handle, swapchain as u64, allocator as *const ffi::AllocationCallbacks);
	}

	#[cfg(vk_khr_swapchain)]
	pub unsafe fn get_swapchain_images_khr(&self, swapchain: u64, swapchain_image_count: *mut u32, swapchain_images: *mut u64) -> ffi::Result {
		return ffi::get_swapchain_images_khr(self.handle, swapchain as u64, swapchain_image_count as *mut u32, swapchain_images as *mut u64);
	}

	#[cfg(vk_khr_swapchain)]
	pub unsafe fn acquire_next_image_khr(&self, swapchain: u64, timeout: u64, semaphore: u64, fence: u64, image_index: *mut u32) -> ffi::Result {
		return ffi::acquire_next_image_khr(self.handle, swapchain as u64, timeout as u64, semaphore as u64, fence as u64, image_index as *mut u32);
	}
}

pub struct Queue {
	pub handle: ffi::Queue
}

impl Queue {
	pub fn from_ffi(raw: ffi::Queue) -> Queue {
		return Queue { handle: raw };
	}

	pub unsafe fn queue_submit(&self, submit_count: u32, submits: *const ffi::SubmitInfo, fence: u64) -> ffi::Result {
		return ffi::queue_submit(self.handle, submit_count as u32, submits as *const ffi::SubmitInfo, fence as u64);
	}

	pub unsafe fn queue_wait_idle(&self, ) -> ffi::Result {
		return ffi::queue_wait_idle(self.handle, );
	}

	pub unsafe fn queue_bind_sparse(&self, bind_info_count: u32, bind_info: *const ffi::BindSparseInfo, fence: u64) -> ffi::Result {
		return ffi::queue_bind_sparse(self.handle, bind_info_count as u32, bind_info as *const ffi::BindSparseInfo, fence as u64);
	}

	#[cfg(vk_khr_swapchain)]
	pub unsafe fn queue_present_khr(&self, present_info: *const ffi::PresentInfoKHR) -> ffi::Result {
		return ffi::queue_present_khr(self.handle, present_info as *const ffi::PresentInfoKHR);
	}
}

pub struct CommandBuffer {
	pub handle: ffi::CommandBuffer
}

impl CommandBuffer {
	pub fn from_ffi(raw: ffi::CommandBuffer) -> CommandBuffer {
		return CommandBuffer { handle: raw };
	}

	pub unsafe fn begin_command_buffer(&self, begin_info: *const ffi::CommandBufferBeginInfo) -> ffi::Result {
		return ffi::begin_command_buffer(self.handle, begin_info as *const ffi::CommandBufferBeginInfo);
	}

	pub unsafe fn end_command_buffer(&self, ) -> ffi::Result {
		return ffi::end_command_buffer(self.handle, );
	}

	pub unsafe fn reset_command_buffer(&self, flags: ffi::CommandBufferResetFlags) -> ffi::Result {
		return ffi::reset_command_buffer(self.handle, flags as ffi::CommandBufferResetFlags);
	}

	pub unsafe fn cmd_bind_pipeline(&self, pipeline_bind_point: ffi::PipelineBindPoint, pipeline: u64) {
		return ffi::cmd_bind_pipeline(self.handle, pipeline_bind_point as ffi::PipelineBindPoint, pipeline as u64);
	}

	pub unsafe fn cmd_set_viewport(&self, first_viewport: u32, viewport_count: u32, viewports: *const ffi::Viewport) {
		return ffi::cmd_set_viewport(self.handle, first_viewport as u32, viewport_count as u32, viewports as *const ffi::Viewport);
	}

	pub unsafe fn cmd_set_scissor(&self, first_scissor: u32, scissor_count: u32, scissors: *const ffi::Rect2D) {
		return ffi::cmd_set_scissor(self.handle, first_scissor as u32, scissor_count as u32, scissors as *const ffi::Rect2D);
	}

	pub unsafe fn cmd_set_line_width(&self, line_width: f32) {
		return ffi::cmd_set_line_width(self.handle, line_width as f32);
	}

	pub unsafe fn cmd_set_depth_bias(&self, depth_bias_constant_factor: f32, depth_bias_clamp: f32, depth_bias_slope_factor: f32) {
		return ffi::cmd_set_depth_bias(self.handle, depth_bias_constant_factor as f32, depth_bias_clamp as f32, depth_bias_slope_factor as f32);
	}

	pub unsafe fn cmd_set_blend_constants(&self, blend_constants: f32) {
		return ffi::cmd_set_blend_constants(self.handle, blend_constants as f32);
	}

	pub unsafe fn cmd_set_depth_bounds(&self, min_depth_bounds: f32, max_depth_bounds: f32) {
		return ffi::cmd_set_depth_bounds(self.handle, min_depth_bounds as f32, max_depth_bounds as f32);
	}

	pub unsafe fn cmd_set_stencil_compare_mask(&self, face_mask: ffi::StencilFaceFlags, compare_mask: u32) {
		return ffi::cmd_set_stencil_compare_mask(self.handle, face_mask as ffi::StencilFaceFlags, compare_mask as u32);
	}

	pub unsafe fn cmd_set_stencil_write_mask(&self, face_mask: ffi::StencilFaceFlags, write_mask: u32) {
		return ffi::cmd_set_stencil_write_mask(self.handle, face_mask as ffi::StencilFaceFlags, write_mask as u32);
	}

	pub unsafe fn cmd_set_stencil_reference(&self, face_mask: ffi::StencilFaceFlags, reference: u32) {
		return ffi::cmd_set_stencil_reference(self.handle, face_mask as ffi::StencilFaceFlags, reference as u32);
	}

	pub unsafe fn cmd_bind_descriptor_sets(&self, pipeline_bind_point: ffi::PipelineBindPoint, layout: u64, first_set: u32, descriptor_set_count: u32, descriptor_sets: *const u64, dynamic_offset_count: u32, dynamic_offsets: *const u32) {
		return ffi::cmd_bind_descriptor_sets(self.handle, pipeline_bind_point as ffi::PipelineBindPoint, layout as u64, first_set as u32, descriptor_set_count as u32, descriptor_sets as *const u64, dynamic_offset_count as u32, dynamic_offsets as *const u32);
	}

	pub unsafe fn cmd_bind_index_buffer(&self, buffer: u64, offset: ffi::DeviceSize, index_type: ffi::IndexType) {
		return ffi::cmd_bind_index_buffer(self.handle, buffer as u64, offset as ffi::DeviceSize, index_type as ffi::IndexType);
	}

	pub unsafe fn cmd_bind_vertex_buffers(&self, first_binding: u32, binding_count: u32, buffers: *const u64, offsets: *const ffi::DeviceSize) {
		return ffi::cmd_bind_vertex_buffers(self.handle, first_binding as u32, binding_count as u32, buffers as *const u64, offsets as *const ffi::DeviceSize);
	}

	pub unsafe fn cmd_draw(&self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) {
		return ffi::cmd_draw(self.handle, vertex_count as u32, instance_count as u32, first_vertex as u32, first_instance as u32);
	}

	pub unsafe fn cmd_draw_indexed(&self, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) {
		return ffi::cmd_draw_indexed(self.handle, index_count as u32, instance_count as u32, first_index as u32, vertex_offset as i32, first_instance as u32);
	}

	pub unsafe fn cmd_draw_indirect(&self, buffer: u64, offset: ffi::DeviceSize, draw_count: u32, stride: u32) {
		return ffi::cmd_draw_indirect(self.handle, buffer as u64, offset as ffi::DeviceSize, draw_count as u32, stride as u32);
	}

	pub unsafe fn cmd_draw_indexed_indirect(&self, buffer: u64, offset: ffi::DeviceSize, draw_count: u32, stride: u32) {
		return ffi::cmd_draw_indexed_indirect(self.handle, buffer as u64, offset as ffi::DeviceSize, draw_count as u32, stride as u32);
	}

	pub unsafe fn cmd_dispatch(&self, x: u32, y: u32, z: u32) {
		return ffi::cmd_dispatch(self.handle, x as u32, y as u32, z as u32);
	}

	pub unsafe fn cmd_dispatch_indirect(&self, buffer: u64, offset: ffi::DeviceSize) {
		return ffi::cmd_dispatch_indirect(self.handle, buffer as u64, offset as ffi::DeviceSize);
	}

	pub unsafe fn cmd_copy_buffer(&self, src_buffer: u64, dst_buffer: u64, region_count: u32, regions: *const ffi::BufferCopy) {
		return ffi::cmd_copy_buffer(self.handle, src_buffer as u64, dst_buffer as u64, region_count as u32, regions as *const ffi::BufferCopy);
	}

	pub unsafe fn cmd_copy_image(&self, src_image: u64, src_image_layout: ffi::ImageLayout, dst_image: u64, dst_image_layout: ffi::ImageLayout, region_count: u32, regions: *const ffi::ImageCopy) {
		return ffi::cmd_copy_image(self.handle, src_image as u64, src_image_layout as ffi::ImageLayout, dst_image as u64, dst_image_layout as ffi::ImageLayout, region_count as u32, regions as *const ffi::ImageCopy);
	}

	pub unsafe fn cmd_blit_image(&self, src_image: u64, src_image_layout: ffi::ImageLayout, dst_image: u64, dst_image_layout: ffi::ImageLayout, region_count: u32, regions: *const ffi::ImageBlit, filter: ffi::Filter) {
		return ffi::cmd_blit_image(self.handle, src_image as u64, src_image_layout as ffi::ImageLayout, dst_image as u64, dst_image_layout as ffi::ImageLayout, region_count as u32, regions as *const ffi::ImageBlit, filter as ffi::Filter);
	}

	pub unsafe fn cmd_copy_buffer_to_image(&self, src_buffer: u64, dst_image: u64, dst_image_layout: ffi::ImageLayout, region_count: u32, regions: *const ffi::BufferImageCopy) {
		return ffi::cmd_copy_buffer_to_image(self.handle, src_buffer as u64, dst_image as u64, dst_image_layout as ffi::ImageLayout, region_count as u32, regions as *const ffi::BufferImageCopy);
	}

	pub unsafe fn cmd_copy_image_to_buffer(&self, src_image: u64, src_image_layout: ffi::ImageLayout, dst_buffer: u64, region_count: u32, regions: *const ffi::BufferImageCopy) {
		return ffi::cmd_copy_image_to_buffer(self.handle, src_image as u64, src_image_layout as ffi::ImageLayout, dst_buffer as u64, region_count as u32, regions as *const ffi::BufferImageCopy);
	}

	pub unsafe fn cmd_update_buffer(&self, dst_buffer: u64, dst_offset: ffi::DeviceSize, data_size: ffi::DeviceSize, data: *const u32) {
		return ffi::cmd_update_buffer(self.handle, dst_buffer as u64, dst_offset as ffi::DeviceSize, data_size as ffi::DeviceSize, data as *const u32);
	}

	pub unsafe fn cmd_fill_buffer(&self, dst_buffer: u64, dst_offset: ffi::DeviceSize, size: ffi::DeviceSize, data: u32) {
		return ffi::cmd_fill_buffer(self.handle, dst_buffer as u64, dst_offset as ffi::DeviceSize, size as ffi::DeviceSize, data as u32);
	}

	pub unsafe fn cmd_clear_color_image(&self, image: u64, image_layout: ffi::ImageLayout, color: *const libc::size_t, range_count: u32, ranges: *const ffi::ImageSubresourceRange) {
		return ffi::cmd_clear_color_image(self.handle, image as u64, image_layout as ffi::ImageLayout, color as *const libc::size_t, range_count as u32, ranges as *const ffi::ImageSubresourceRange);
	}

	pub unsafe fn cmd_clear_depth_stencil_image(&self, image: u64, image_layout: ffi::ImageLayout, depth_stencil: *const ffi::ClearDepthStencilValue, range_count: u32, ranges: *const ffi::ImageSubresourceRange) {
		return ffi::cmd_clear_depth_stencil_image(self.handle, image as u64, image_layout as ffi::ImageLayout, depth_stencil as *const ffi::ClearDepthStencilValue, range_count as u32, ranges as *const ffi::ImageSubresourceRange);
	}

	pub unsafe fn cmd_clear_attachments(&self, attachment_count: u32, attachments: *const ffi::ClearAttachment, rect_count: u32, rects: *const ffi::ClearRect) {
		return ffi::cmd_clear_attachments(self.handle, attachment_count as u32, attachments as *const ffi::ClearAttachment, rect_count as u32, rects as *const ffi::ClearRect);
	}

	pub unsafe fn cmd_resolve_image(&self, src_image: u64, src_image_layout: ffi::ImageLayout, dst_image: u64, dst_image_layout: ffi::ImageLayout, region_count: u32, regions: *const ffi::ImageResolve) {
		return ffi::cmd_resolve_image(self.handle, src_image as u64, src_image_layout as ffi::ImageLayout, dst_image as u64, dst_image_layout as ffi::ImageLayout, region_count as u32, regions as *const ffi::ImageResolve);
	}

	pub unsafe fn cmd_set_event(&self, event: u64, stage_mask: ffi::PipelineStageFlags) {
		return ffi::cmd_set_event(self.handle, event as u64, stage_mask as ffi::PipelineStageFlags);
	}

	pub unsafe fn cmd_reset_event(&self, event: u64, stage_mask: ffi::PipelineStageFlags) {
		return ffi::cmd_reset_event(self.handle, event as u64, stage_mask as ffi::PipelineStageFlags);
	}

	pub unsafe fn cmd_wait_events(&self, event_count: u32, events: *const u64, src_stage_mask: ffi::PipelineStageFlags, dst_stage_mask: ffi::PipelineStageFlags, memory_barrier_count: u32, memory_barriers: *const ffi::MemoryBarrier, buffer_memory_barrier_count: u32, buffer_memory_barriers: *const ffi::BufferMemoryBarrier, image_memory_barrier_count: u32, image_memory_barriers: *const ffi::ImageMemoryBarrier) {
		return ffi::cmd_wait_events(self.handle, event_count as u32, events as *const u64, src_stage_mask as ffi::PipelineStageFlags, dst_stage_mask as ffi::PipelineStageFlags, memory_barrier_count as u32, memory_barriers as *const ffi::MemoryBarrier, buffer_memory_barrier_count as u32, buffer_memory_barriers as *const ffi::BufferMemoryBarrier, image_memory_barrier_count as u32, image_memory_barriers as *const ffi::ImageMemoryBarrier);
	}

	pub unsafe fn cmd_pipeline_barrier(&self, src_stage_mask: ffi::PipelineStageFlags, dst_stage_mask: ffi::PipelineStageFlags, dependency_flags: ffi::DependencyFlags, memory_barrier_count: u32, memory_barriers: *const ffi::MemoryBarrier, buffer_memory_barrier_count: u32, buffer_memory_barriers: *const ffi::BufferMemoryBarrier, image_memory_barrier_count: u32, image_memory_barriers: *const ffi::ImageMemoryBarrier) {
		return ffi::cmd_pipeline_barrier(self.handle, src_stage_mask as ffi::PipelineStageFlags, dst_stage_mask as ffi::PipelineStageFlags, dependency_flags as ffi::DependencyFlags, memory_barrier_count as u32, memory_barriers as *const ffi::MemoryBarrier, buffer_memory_barrier_count as u32, buffer_memory_barriers as *const ffi::BufferMemoryBarrier, image_memory_barrier_count as u32, image_memory_barriers as *const ffi::ImageMemoryBarrier);
	}

	pub unsafe fn cmd_begin_query(&self, query_pool: u64, query: u32, flags: ffi::QueryControlFlags) {
		return ffi::cmd_begin_query(self.handle, query_pool as u64, query as u32, flags as ffi::QueryControlFlags);
	}

	pub unsafe fn cmd_end_query(&self, query_pool: u64, query: u32) {
		return ffi::cmd_end_query(self.handle, query_pool as u64, query as u32);
	}

	pub unsafe fn cmd_reset_query_pool(&self, query_pool: u64, first_query: u32, query_count: u32) {
		return ffi::cmd_reset_query_pool(self.handle, query_pool as u64, first_query as u32, query_count as u32);
	}

	pub unsafe fn cmd_write_timestamp(&self, pipeline_stage: ffi::PipelineStageFlags, query_pool: u64, query: u32) {
		return ffi::cmd_write_timestamp(self.handle, pipeline_stage as ffi::PipelineStageFlags, query_pool as u64, query as u32);
	}

	pub unsafe fn cmd_copy_query_pool_results(&self, query_pool: u64, first_query: u32, query_count: u32, dst_buffer: u64, dst_offset: ffi::DeviceSize, stride: ffi::DeviceSize, flags: ffi::QueryResultFlags) {
		return ffi::cmd_copy_query_pool_results(self.handle, query_pool as u64, first_query as u32, query_count as u32, dst_buffer as u64, dst_offset as ffi::DeviceSize, stride as ffi::DeviceSize, flags as ffi::QueryResultFlags);
	}

	pub unsafe fn cmd_push_constants(&self, layout: u64, stage_flags: ffi::ShaderStageFlags, offset: u32, size: u32, values: *const libc::c_void) {
		return ffi::cmd_push_constants(self.handle, layout as u64, stage_flags as ffi::ShaderStageFlags, offset as u32, size as u32, values as *const libc::c_void);
	}

	pub unsafe fn cmd_begin_render_pass(&self, render_pass_begin: *const ffi::RenderPassBeginInfo, contents: ffi::SubpassContents) {
		return ffi::cmd_begin_render_pass(self.handle, render_pass_begin as *const ffi::RenderPassBeginInfo, contents as ffi::SubpassContents);
	}

	pub unsafe fn cmd_next_subpass(&self, contents: ffi::SubpassContents) {
		return ffi::cmd_next_subpass(self.handle, contents as ffi::SubpassContents);
	}

	pub unsafe fn cmd_end_render_pass(&self, ) {
		return ffi::cmd_end_render_pass(self.handle, );
	}

	pub unsafe fn cmd_execute_commands(&self, command_buffer_count: u32, command_buffers: *const libc::size_t) {
		return ffi::cmd_execute_commands(self.handle, command_buffer_count as u32, command_buffers as *const libc::size_t);
	}
}

pub struct DeviceMemory {
	pub handle: ffi::DeviceMemory
}

impl DeviceMemory {
	pub fn from_ffi(raw: ffi::DeviceMemory) -> DeviceMemory {
		return DeviceMemory { handle: raw };
	}
}

pub struct CommandPool {
	pub handle: ffi::CommandPool
}

impl CommandPool {
	pub fn from_ffi(raw: ffi::CommandPool) -> CommandPool {
		return CommandPool { handle: raw };
	}
}

pub struct Buffer {
	pub handle: ffi::Buffer
}

impl Buffer {
	pub fn from_ffi(raw: ffi::Buffer) -> Buffer {
		return Buffer { handle: raw };
	}
}

pub struct BufferView {
	pub handle: ffi::BufferView
}

impl BufferView {
	pub fn from_ffi(raw: ffi::BufferView) -> BufferView {
		return BufferView { handle: raw };
	}
}

pub struct Image {
	pub handle: ffi::Image
}

impl Image {
	pub fn from_ffi(raw: ffi::Image) -> Image {
		return Image { handle: raw };
	}
}

pub struct ImageView {
	pub handle: ffi::ImageView
}

impl ImageView {
	pub fn from_ffi(raw: ffi::ImageView) -> ImageView {
		return ImageView { handle: raw };
	}
}

pub struct ShaderModule {
	pub handle: ffi::ShaderModule
}

impl ShaderModule {
	pub fn from_ffi(raw: ffi::ShaderModule) -> ShaderModule {
		return ShaderModule { handle: raw };
	}
}

pub struct Pipeline {
	pub handle: ffi::Pipeline
}

impl Pipeline {
	pub fn from_ffi(raw: ffi::Pipeline) -> Pipeline {
		return Pipeline { handle: raw };
	}
}

pub struct PipelineLayout {
	pub handle: ffi::PipelineLayout
}

impl PipelineLayout {
	pub fn from_ffi(raw: ffi::PipelineLayout) -> PipelineLayout {
		return PipelineLayout { handle: raw };
	}
}

pub struct Sampler {
	pub handle: ffi::Sampler
}

impl Sampler {
	pub fn from_ffi(raw: ffi::Sampler) -> Sampler {
		return Sampler { handle: raw };
	}
}

pub struct DescriptorSet {
	pub handle: ffi::DescriptorSet
}

impl DescriptorSet {
	pub fn from_ffi(raw: ffi::DescriptorSet) -> DescriptorSet {
		return DescriptorSet { handle: raw };
	}
}

pub struct DescriptorSetLayout {
	pub handle: ffi::DescriptorSetLayout
}

impl DescriptorSetLayout {
	pub fn from_ffi(raw: ffi::DescriptorSetLayout) -> DescriptorSetLayout {
		return DescriptorSetLayout { handle: raw };
	}
}

pub struct DescriptorPool {
	pub handle: ffi::DescriptorPool
}

impl DescriptorPool {
	pub fn from_ffi(raw: ffi::DescriptorPool) -> DescriptorPool {
		return DescriptorPool { handle: raw };
	}
}

pub struct Fence {
	pub handle: ffi::Fence
}

impl Fence {
	pub fn from_ffi(raw: ffi::Fence) -> Fence {
		return Fence { handle: raw };
	}
}

pub struct Semaphore {
	pub handle: ffi::Semaphore
}

impl Semaphore {
	pub fn from_ffi(raw: ffi::Semaphore) -> Semaphore {
		return Semaphore { handle: raw };
	}
}

pub struct Event {
	pub handle: ffi::Event
}

impl Event {
	pub fn from_ffi(raw: ffi::Event) -> Event {
		return Event { handle: raw };
	}
}

pub struct QueryPool {
	pub handle: ffi::QueryPool
}

impl QueryPool {
	pub fn from_ffi(raw: ffi::QueryPool) -> QueryPool {
		return QueryPool { handle: raw };
	}
}

pub struct Framebuffer {
	pub handle: ffi::Framebuffer
}

impl Framebuffer {
	pub fn from_ffi(raw: ffi::Framebuffer) -> Framebuffer {
		return Framebuffer { handle: raw };
	}
}

pub struct RenderPass {
	pub handle: ffi::RenderPass
}

impl RenderPass {
	pub fn from_ffi(raw: ffi::RenderPass) -> RenderPass {
		return RenderPass { handle: raw };
	}
}

pub struct PipelineCache {
	pub handle: ffi::PipelineCache
}

impl PipelineCache {
	pub fn from_ffi(raw: ffi::PipelineCache) -> PipelineCache {
		return PipelineCache { handle: raw };
	}
}

pub struct DisplayKHR {
	pub handle: ffi::DisplayKHR
}

impl DisplayKHR {
	pub fn from_ffi(raw: ffi::DisplayKHR) -> DisplayKHR {
		return DisplayKHR { handle: raw };
	}
}

pub struct DisplayModeKHR {
	pub handle: ffi::DisplayModeKHR
}

impl DisplayModeKHR {
	pub fn from_ffi(raw: ffi::DisplayModeKHR) -> DisplayModeKHR {
		return DisplayModeKHR { handle: raw };
	}
}

pub struct SurfaceKHR {
	pub handle: ffi::SurfaceKHR
}

impl SurfaceKHR {
	pub fn from_ffi(raw: ffi::SurfaceKHR) -> SurfaceKHR {
		return SurfaceKHR { handle: raw };
	}
}

pub struct SwapchainKHR {
	pub handle: ffi::SwapchainKHR
}

impl SwapchainKHR {
	pub fn from_ffi(raw: ffi::SwapchainKHR) -> SwapchainKHR {
		return SwapchainKHR { handle: raw };
	}
}

pub struct DebugReportCallbackEXT {
	pub handle: ffi::DebugReportCallbackEXT
}

impl DebugReportCallbackEXT {
	pub fn from_ffi(raw: ffi::DebugReportCallbackEXT) -> DebugReportCallbackEXT {
		return DebugReportCallbackEXT { handle: raw };
	}
}

pub struct Offset2D {
	pub raw: ffi::Offset2D
}

impl Offset2D {
	pub unsafe fn from_ffi(raw: ffi::Offset2D) -> Offset2D {
		return Offset2D { raw: raw };
	}

	pub fn x(&self) -> i32 {
		return self.raw.x as i32;
	}

	pub fn y(&self) -> i32 {
		return self.raw.y as i32;
	}
}

pub struct Offset3D {
	pub raw: ffi::Offset3D
}

impl Offset3D {
	pub unsafe fn from_ffi(raw: ffi::Offset3D) -> Offset3D {
		return Offset3D { raw: raw };
	}

	pub fn x(&self) -> i32 {
		return self.raw.x as i32;
	}

	pub fn y(&self) -> i32 {
		return self.raw.y as i32;
	}

	pub fn z(&self) -> i32 {
		return self.raw.z as i32;
	}
}

pub struct Extent2D {
	pub raw: ffi::Extent2D
}

impl Extent2D {
	pub unsafe fn from_ffi(raw: ffi::Extent2D) -> Extent2D {
		return Extent2D { raw: raw };
	}

	pub fn width(&self) -> u32 {
		return self.raw.width as u32;
	}

	pub fn height(&self) -> u32 {
		return self.raw.height as u32;
	}
}

pub struct Extent3D {
	pub raw: ffi::Extent3D
}

impl Extent3D {
	pub unsafe fn from_ffi(raw: ffi::Extent3D) -> Extent3D {
		return Extent3D { raw: raw };
	}

	pub fn width(&self) -> u32 {
		return self.raw.width as u32;
	}

	pub fn height(&self) -> u32 {
		return self.raw.height as u32;
	}

	pub fn depth(&self) -> u32 {
		return self.raw.depth as u32;
	}
}

pub struct Viewport {
	pub raw: ffi::Viewport
}

impl Viewport {
	pub unsafe fn from_ffi(raw: ffi::Viewport) -> Viewport {
		return Viewport { raw: raw };
	}

	pub fn x(&self) -> f32 {
		return self.raw.x as f32;
	}

	pub fn y(&self) -> f32 {
		return self.raw.y as f32;
	}

	pub fn width(&self) -> f32 {
		return self.raw.width as f32;
	}

	pub fn height(&self) -> f32 {
		return self.raw.height as f32;
	}

	pub fn min_depth(&self) -> f32 {
		return self.raw.min_depth as f32;
	}

	pub fn max_depth(&self) -> f32 {
		return self.raw.max_depth as f32;
	}
}

pub struct Rect2D {
	pub raw: ffi::Rect2D
}

impl Rect2D {
	pub unsafe fn from_ffi(raw: ffi::Rect2D) -> Rect2D {
		return Rect2D { raw: raw };
	}

	pub fn offset(&self) -> &ffi::Offset2D {
		return &self.raw.offset;
	}

	pub fn extent(&self) -> &ffi::Extent2D {
		return &self.raw.extent;
	}
}

pub struct Rect3D {
	pub raw: ffi::Rect3D
}

impl Rect3D {
	pub unsafe fn from_ffi(raw: ffi::Rect3D) -> Rect3D {
		return Rect3D { raw: raw };
	}

	pub fn offset(&self) -> &ffi::Offset3D {
		return &self.raw.offset;
	}

	pub fn extent(&self) -> &ffi::Extent3D {
		return &self.raw.extent;
	}
}

pub struct ClearRect {
	pub raw: ffi::ClearRect
}

impl ClearRect {
	pub unsafe fn from_ffi(raw: ffi::ClearRect) -> ClearRect {
		return ClearRect { raw: raw };
	}

	pub fn rect(&self) -> &ffi::Rect2D {
		return &self.raw.rect;
	}

	pub fn base_array_layer(&self) -> u32 {
		return self.raw.base_array_layer as u32;
	}

	pub fn layer_count(&self) -> u32 {
		return self.raw.layer_count as u32;
	}
}

pub struct ComponentMapping {
	pub raw: ffi::ComponentMapping
}

impl ComponentMapping {
	pub unsafe fn from_ffi(raw: ffi::ComponentMapping) -> ComponentMapping {
		return ComponentMapping { raw: raw };
	}

	pub fn r(&self) -> ffi::ComponentSwizzle {
		return self.raw.r as ffi::ComponentSwizzle;
	}

	pub fn g(&self) -> ffi::ComponentSwizzle {
		return self.raw.g as ffi::ComponentSwizzle;
	}

	pub fn b(&self) -> ffi::ComponentSwizzle {
		return self.raw.b as ffi::ComponentSwizzle;
	}

	pub fn a(&self) -> ffi::ComponentSwizzle {
		return self.raw.a as ffi::ComponentSwizzle;
	}
}

pub struct PhysicalDeviceProperties {
	pub raw: ffi::PhysicalDeviceProperties
}

impl PhysicalDeviceProperties {
	pub unsafe fn from_ffi(raw: ffi::PhysicalDeviceProperties) -> PhysicalDeviceProperties {
		return PhysicalDeviceProperties { raw: raw };
	}

	pub fn api_version(&self) -> u32 {
		return self.raw.api_version as u32;
	}

	pub fn driver_version(&self) -> u32 {
		return self.raw.driver_version as u32;
	}

	pub fn vendor_id(&self) -> u32 {
		return self.raw.vendor_id as u32;
	}

	pub fn device_id(&self) -> u32 {
		return self.raw.device_id as u32;
	}

	pub fn device_type(&self) -> ffi::PhysicalDeviceType {
		return self.raw.device_type as ffi::PhysicalDeviceType;
	}

	pub fn device_name(&self) -> &str {
		return unsafe { std::ffi::CStr::from_ptr(self.raw.device_name.as_ptr()).to_str().unwrap() };
	}

	pub fn pipeline_cache_uuid(&self) -> &[u8; UUID_SIZE] {
		return &self.raw.pipeline_cache_uuid;
	}

	pub fn limits(&self) -> &ffi::PhysicalDeviceLimits {
		return &self.raw.limits;
	}

	pub fn sparse_properties(&self) -> &ffi::PhysicalDeviceSparseProperties {
		return &self.raw.sparse_properties;
	}
}

pub struct ExtensionProperties {
	pub raw: ffi::ExtensionProperties
}

impl ExtensionProperties {
	pub unsafe fn from_ffi(raw: ffi::ExtensionProperties) -> ExtensionProperties {
		return ExtensionProperties { raw: raw };
	}

	pub fn extension_name(&self) -> &str {
		return unsafe { std::ffi::CStr::from_ptr(self.raw.extension_name.as_ptr()).to_str().unwrap() };
	}

	pub fn spec_version(&self) -> u32 {
		return self.raw.spec_version as u32;
	}
}

pub struct LayerProperties {
	pub raw: ffi::LayerProperties
}

impl LayerProperties {
	pub unsafe fn from_ffi(raw: ffi::LayerProperties) -> LayerProperties {
		return LayerProperties { raw: raw };
	}

	pub fn layer_name(&self) -> &str {
		return unsafe { std::ffi::CStr::from_ptr(self.raw.layer_name.as_ptr()).to_str().unwrap() };
	}

	pub fn spec_version(&self) -> u32 {
		return self.raw.spec_version as u32;
	}

	pub fn implementation_version(&self) -> u32 {
		return self.raw.implementation_version as u32;
	}

	pub fn description(&self) -> &str {
		return unsafe { std::ffi::CStr::from_ptr(self.raw.description.as_ptr()).to_str().unwrap() };
	}
}

pub struct ApplicationInfo {
	pub raw: ffi::ApplicationInfo
}

impl ApplicationInfo {
	pub unsafe fn from_ffi(raw: ffi::ApplicationInfo) -> ApplicationInfo {
		return ApplicationInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn application_name(&self) -> &str {
		return unsafe { std::ffi::CStr::from_ptr(self.raw.application_name).to_str().unwrap() };
	}

	pub fn application_version(&self) -> u32 {
		return self.raw.application_version as u32;
	}

	pub fn engine_name(&self) -> &str {
		return unsafe { std::ffi::CStr::from_ptr(self.raw.engine_name).to_str().unwrap() };
	}

	pub fn engine_version(&self) -> u32 {
		return self.raw.engine_version as u32;
	}

	pub fn api_version(&self) -> u32 {
		return self.raw.api_version as u32;
	}
}

pub struct AllocationCallbacks {
	pub raw: ffi::AllocationCallbacks
}

impl AllocationCallbacks {
	pub unsafe fn from_ffi(raw: ffi::AllocationCallbacks) -> AllocationCallbacks {
		return AllocationCallbacks { raw: raw };
	}

	pub fn user_data(&self) -> &libc::c_void {
		return unsafe { &*self.raw.user_data as &libc::c_void };
	}

	pub fn allocation(&self) -> &ffi::AllocationFunctionCallback {
		return unsafe { &*self.raw.allocation as &ffi::AllocationFunctionCallback };
	}

	pub fn reallocation(&self) -> &ffi::ReallocationFunctionCallback {
		return unsafe { &*self.raw.reallocation as &ffi::ReallocationFunctionCallback };
	}

	pub fn free(&self) -> &ffi::FreeFunctionCallback {
		return unsafe { &*self.raw.free as &ffi::FreeFunctionCallback };
	}

	pub fn internal_allocation(&self) -> &ffi::InternalAllocationNotificationCallback {
		return unsafe { &*self.raw.internal_allocation as &ffi::InternalAllocationNotificationCallback };
	}

	pub fn internal_free(&self) -> &ffi::InternalFreeNotificationCallback {
		return unsafe { &*self.raw.internal_free as &ffi::InternalFreeNotificationCallback };
	}
}

pub struct DeviceQueueCreateInfo {
	pub raw: ffi::DeviceQueueCreateInfo
}

impl DeviceQueueCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::DeviceQueueCreateInfo) -> DeviceQueueCreateInfo {
		return DeviceQueueCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::DeviceQueueCreateFlags {
		return self.raw.flags as ffi::DeviceQueueCreateFlags;
	}

	pub fn queue_family_index(&self) -> u32 {
		return self.raw.queue_family_index as u32;
	}

	pub fn queue_count(&self) -> u32 {
		return self.raw.queue_count as u32;
	}

	pub fn queue_priorities(&self) -> &[f32] {
		return unsafe { std::slice::from_raw_parts(self.raw.queue_priorities, self.queue_count() as usize) };
	}
}

pub struct DeviceCreateInfo {
	pub raw: ffi::DeviceCreateInfo
}

impl DeviceCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::DeviceCreateInfo) -> DeviceCreateInfo {
		return DeviceCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::DeviceCreateFlags {
		return self.raw.flags as ffi::DeviceCreateFlags;
	}

	pub fn queue_create_info_count(&self) -> u32 {
		return self.raw.queue_create_info_count as u32;
	}

	pub fn queue_create_infos(&self) -> &[ffi::DeviceQueueCreateInfo] {
		return unsafe { std::slice::from_raw_parts(self.raw.queue_create_infos, self.queue_create_info_count() as usize) };
	}

	pub fn enabled_layer_count(&self) -> u32 {
		return self.raw.enabled_layer_count as u32;
	}

	pub fn enabled_layer_names(&self) -> Vec<&str> {
		return unsafe { let v = Vec::with_capacity(self.enabled_layer_count() as usize); v };
	}

	pub fn enabled_extension_count(&self) -> u32 {
		return self.raw.enabled_extension_count as u32;
	}

	pub fn enabled_extension_names(&self) -> Vec<&str> {
		return unsafe { let v = Vec::with_capacity(self.enabled_extension_count() as usize); v };
	}

	pub fn enabled_features(&self) -> &ffi::PhysicalDeviceFeatures {
		return unsafe { &*self.raw.enabled_features as &ffi::PhysicalDeviceFeatures };
	}
}

pub struct InstanceCreateInfo {
	pub raw: ffi::InstanceCreateInfo
}

impl InstanceCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::InstanceCreateInfo) -> InstanceCreateInfo {
		return InstanceCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::InstanceCreateFlags {
		return self.raw.flags as ffi::InstanceCreateFlags;
	}

	pub fn application_info(&self) -> &ffi::ApplicationInfo {
		return unsafe { &*self.raw.application_info as &ffi::ApplicationInfo };
	}

	pub fn enabled_layer_count(&self) -> u32 {
		return self.raw.enabled_layer_count as u32;
	}

	pub fn enabled_layer_names(&self) -> Vec<&str> {
		return unsafe { let v = Vec::with_capacity(self.enabled_layer_count() as usize); v };
	}

	pub fn enabled_extension_count(&self) -> u32 {
		return self.raw.enabled_extension_count as u32;
	}

	pub fn enabled_extension_names(&self) -> Vec<&str> {
		return unsafe { let v = Vec::with_capacity(self.enabled_extension_count() as usize); v };
	}
}

pub struct QueueFamilyProperties {
	pub raw: ffi::QueueFamilyProperties
}

impl QueueFamilyProperties {
	pub unsafe fn from_ffi(raw: ffi::QueueFamilyProperties) -> QueueFamilyProperties {
		return QueueFamilyProperties { raw: raw };
	}

	pub fn queue_flags(&self) -> ffi::QueueFlags {
		return self.raw.queue_flags as ffi::QueueFlags;
	}

	pub fn queue_count(&self) -> u32 {
		return self.raw.queue_count as u32;
	}

	pub fn timestamp_valid_bits(&self) -> u32 {
		return self.raw.timestamp_valid_bits as u32;
	}

	pub fn min_image_transfer_granularity(&self) -> &ffi::Extent3D {
		return &self.raw.min_image_transfer_granularity;
	}
}

pub struct PhysicalDeviceMemoryProperties {
	pub raw: ffi::PhysicalDeviceMemoryProperties
}

impl PhysicalDeviceMemoryProperties {
	pub unsafe fn from_ffi(raw: ffi::PhysicalDeviceMemoryProperties) -> PhysicalDeviceMemoryProperties {
		return PhysicalDeviceMemoryProperties { raw: raw };
	}

	pub fn memory_type_count(&self) -> u32 {
		return self.raw.memory_type_count as u32;
	}

	pub fn memory_types(&self) -> &[ffi::MemoryType; MAX_MEMORY_TYPES] {
		return &self.raw.memory_types;
	}

	pub fn memory_heap_count(&self) -> u32 {
		return self.raw.memory_heap_count as u32;
	}

	pub fn memory_heaps(&self) -> &[ffi::MemoryHeap; MAX_MEMORY_HEAPS] {
		return &self.raw.memory_heaps;
	}
}

pub struct MemoryAllocateInfo {
	pub raw: ffi::MemoryAllocateInfo
}

impl MemoryAllocateInfo {
	pub unsafe fn from_ffi(raw: ffi::MemoryAllocateInfo) -> MemoryAllocateInfo {
		return MemoryAllocateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn allocation_size(&self) -> ffi::DeviceSize {
		return self.raw.allocation_size as ffi::DeviceSize;
	}

	pub fn memory_type_index(&self) -> u32 {
		return self.raw.memory_type_index as u32;
	}
}

pub struct MemoryRequirements {
	pub raw: ffi::MemoryRequirements
}

impl MemoryRequirements {
	pub unsafe fn from_ffi(raw: ffi::MemoryRequirements) -> MemoryRequirements {
		return MemoryRequirements { raw: raw };
	}

	pub fn size(&self) -> ffi::DeviceSize {
		return self.raw.size as ffi::DeviceSize;
	}

	pub fn alignment(&self) -> ffi::DeviceSize {
		return self.raw.alignment as ffi::DeviceSize;
	}

	pub fn memory_type_bits(&self) -> u32 {
		return self.raw.memory_type_bits as u32;
	}
}

pub struct SparseImageFormatProperties {
	pub raw: ffi::SparseImageFormatProperties
}

impl SparseImageFormatProperties {
	pub unsafe fn from_ffi(raw: ffi::SparseImageFormatProperties) -> SparseImageFormatProperties {
		return SparseImageFormatProperties { raw: raw };
	}

	pub fn aspect_mask(&self) -> ffi::ImageAspectFlags {
		return self.raw.aspect_mask as ffi::ImageAspectFlags;
	}

	pub fn image_granularity(&self) -> &ffi::Extent3D {
		return &self.raw.image_granularity;
	}

	pub fn flags(&self) -> ffi::SparseImageFormatFlags {
		return self.raw.flags as ffi::SparseImageFormatFlags;
	}
}

pub struct SparseImageMemoryRequirements {
	pub raw: ffi::SparseImageMemoryRequirements
}

impl SparseImageMemoryRequirements {
	pub unsafe fn from_ffi(raw: ffi::SparseImageMemoryRequirements) -> SparseImageMemoryRequirements {
		return SparseImageMemoryRequirements { raw: raw };
	}

	pub fn format_properties(&self) -> &ffi::SparseImageFormatProperties {
		return &self.raw.format_properties;
	}

	pub fn image_mip_tail_first_lod(&self) -> u32 {
		return self.raw.image_mip_tail_first_lod as u32;
	}

	pub fn image_mip_tail_size(&self) -> ffi::DeviceSize {
		return self.raw.image_mip_tail_size as ffi::DeviceSize;
	}

	pub fn image_mip_tail_offset(&self) -> ffi::DeviceSize {
		return self.raw.image_mip_tail_offset as ffi::DeviceSize;
	}

	pub fn image_mip_tail_stride(&self) -> ffi::DeviceSize {
		return self.raw.image_mip_tail_stride as ffi::DeviceSize;
	}
}

pub struct MemoryType {
	pub raw: ffi::MemoryType
}

impl MemoryType {
	pub unsafe fn from_ffi(raw: ffi::MemoryType) -> MemoryType {
		return MemoryType { raw: raw };
	}

	pub fn property_flags(&self) -> ffi::MemoryPropertyFlags {
		return self.raw.property_flags as ffi::MemoryPropertyFlags;
	}

	pub fn heap_index(&self) -> u32 {
		return self.raw.heap_index as u32;
	}
}

pub struct MemoryHeap {
	pub raw: ffi::MemoryHeap
}

impl MemoryHeap {
	pub unsafe fn from_ffi(raw: ffi::MemoryHeap) -> MemoryHeap {
		return MemoryHeap { raw: raw };
	}

	pub fn size(&self) -> ffi::DeviceSize {
		return self.raw.size as ffi::DeviceSize;
	}

	pub fn flags(&self) -> ffi::MemoryHeapFlags {
		return self.raw.flags as ffi::MemoryHeapFlags;
	}
}

pub struct MappedMemoryRange {
	pub raw: ffi::MappedMemoryRange
}

impl MappedMemoryRange {
	pub unsafe fn from_ffi(raw: ffi::MappedMemoryRange) -> MappedMemoryRange {
		return MappedMemoryRange { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn memory(&self) -> u64 {
		return self.raw.memory as u64;
	}

	pub fn offset(&self) -> ffi::DeviceSize {
		return self.raw.offset as ffi::DeviceSize;
	}

	pub fn size(&self) -> ffi::DeviceSize {
		return self.raw.size as ffi::DeviceSize;
	}
}

pub struct FormatProperties {
	pub raw: ffi::FormatProperties
}

impl FormatProperties {
	pub unsafe fn from_ffi(raw: ffi::FormatProperties) -> FormatProperties {
		return FormatProperties { raw: raw };
	}

	pub fn linear_tiling_features(&self) -> ffi::FormatFeatureFlags {
		return self.raw.linear_tiling_features as ffi::FormatFeatureFlags;
	}

	pub fn optimal_tiling_features(&self) -> ffi::FormatFeatureFlags {
		return self.raw.optimal_tiling_features as ffi::FormatFeatureFlags;
	}

	pub fn buffer_features(&self) -> ffi::FormatFeatureFlags {
		return self.raw.buffer_features as ffi::FormatFeatureFlags;
	}
}

pub struct ImageFormatProperties {
	pub raw: ffi::ImageFormatProperties
}

impl ImageFormatProperties {
	pub unsafe fn from_ffi(raw: ffi::ImageFormatProperties) -> ImageFormatProperties {
		return ImageFormatProperties { raw: raw };
	}

	pub fn max_extent(&self) -> &ffi::Extent3D {
		return &self.raw.max_extent;
	}

	pub fn max_mip_levels(&self) -> u32 {
		return self.raw.max_mip_levels as u32;
	}

	pub fn max_array_layers(&self) -> u32 {
		return self.raw.max_array_layers as u32;
	}

	pub fn sample_counts(&self) -> ffi::SampleCountFlags {
		return self.raw.sample_counts as ffi::SampleCountFlags;
	}

	pub fn max_resource_size(&self) -> ffi::DeviceSize {
		return self.raw.max_resource_size as ffi::DeviceSize;
	}
}

pub struct DescriptorBufferInfo {
	pub raw: ffi::DescriptorBufferInfo
}

impl DescriptorBufferInfo {
	pub unsafe fn from_ffi(raw: ffi::DescriptorBufferInfo) -> DescriptorBufferInfo {
		return DescriptorBufferInfo { raw: raw };
	}

	pub fn buffer(&self) -> u64 {
		return self.raw.buffer as u64;
	}

	pub fn offset(&self) -> ffi::DeviceSize {
		return self.raw.offset as ffi::DeviceSize;
	}

	pub fn range(&self) -> ffi::DeviceSize {
		return self.raw.range as ffi::DeviceSize;
	}
}

pub struct DescriptorImageInfo {
	pub raw: ffi::DescriptorImageInfo
}

impl DescriptorImageInfo {
	pub unsafe fn from_ffi(raw: ffi::DescriptorImageInfo) -> DescriptorImageInfo {
		return DescriptorImageInfo { raw: raw };
	}

	pub fn sampler(&self) -> u64 {
		return self.raw.sampler as u64;
	}

	pub fn image_view(&self) -> u64 {
		return self.raw.image_view as u64;
	}

	pub fn image_layout(&self) -> ffi::ImageLayout {
		return self.raw.image_layout as ffi::ImageLayout;
	}
}

pub struct WriteDescriptorSet {
	pub raw: ffi::WriteDescriptorSet
}

impl WriteDescriptorSet {
	pub unsafe fn from_ffi(raw: ffi::WriteDescriptorSet) -> WriteDescriptorSet {
		return WriteDescriptorSet { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn dst_set(&self) -> u64 {
		return self.raw.dst_set as u64;
	}

	pub fn dst_binding(&self) -> u32 {
		return self.raw.dst_binding as u32;
	}

	pub fn dst_array_element(&self) -> u32 {
		return self.raw.dst_array_element as u32;
	}

	pub fn descriptor_count(&self) -> u32 {
		return self.raw.descriptor_count as u32;
	}

	pub fn descriptor_type(&self) -> ffi::DescriptorType {
		return self.raw.descriptor_type as ffi::DescriptorType;
	}

	pub fn image_info(&self) -> &[ffi::DescriptorImageInfo] {
		return unsafe { std::slice::from_raw_parts(self.raw.image_info, self.descriptor_count() as usize) };
	}

	pub fn buffer_info(&self) -> &[ffi::DescriptorBufferInfo] {
		return unsafe { std::slice::from_raw_parts(self.raw.buffer_info, self.descriptor_count() as usize) };
	}

	pub fn texel_buffer_view(&self) -> &[u64] {
		return unsafe { std::slice::from_raw_parts(self.raw.texel_buffer_view, self.descriptor_count() as usize) };
	}
}

pub struct CopyDescriptorSet {
	pub raw: ffi::CopyDescriptorSet
}

impl CopyDescriptorSet {
	pub unsafe fn from_ffi(raw: ffi::CopyDescriptorSet) -> CopyDescriptorSet {
		return CopyDescriptorSet { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn src_set(&self) -> u64 {
		return self.raw.src_set as u64;
	}

	pub fn src_binding(&self) -> u32 {
		return self.raw.src_binding as u32;
	}

	pub fn src_array_element(&self) -> u32 {
		return self.raw.src_array_element as u32;
	}

	pub fn dst_set(&self) -> u64 {
		return self.raw.dst_set as u64;
	}

	pub fn dst_binding(&self) -> u32 {
		return self.raw.dst_binding as u32;
	}

	pub fn dst_array_element(&self) -> u32 {
		return self.raw.dst_array_element as u32;
	}

	pub fn descriptor_count(&self) -> u32 {
		return self.raw.descriptor_count as u32;
	}
}

pub struct BufferCreateInfo {
	pub raw: ffi::BufferCreateInfo
}

impl BufferCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::BufferCreateInfo) -> BufferCreateInfo {
		return BufferCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::BufferCreateFlags {
		return self.raw.flags as ffi::BufferCreateFlags;
	}

	pub fn size(&self) -> ffi::DeviceSize {
		return self.raw.size as ffi::DeviceSize;
	}

	pub fn usage(&self) -> ffi::BufferUsageFlags {
		return self.raw.usage as ffi::BufferUsageFlags;
	}

	pub fn sharing_mode(&self) -> ffi::SharingMode {
		return self.raw.sharing_mode as ffi::SharingMode;
	}

	pub fn queue_family_index_count(&self) -> u32 {
		return self.raw.queue_family_index_count as u32;
	}

	pub fn queue_family_indices(&self) -> &[u32] {
		return unsafe { std::slice::from_raw_parts(self.raw.queue_family_indices, self.queue_family_index_count() as usize) };
	}
}

pub struct BufferViewCreateInfo {
	pub raw: ffi::BufferViewCreateInfo
}

impl BufferViewCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::BufferViewCreateInfo) -> BufferViewCreateInfo {
		return BufferViewCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::BufferViewCreateFlags {
		return self.raw.flags as ffi::BufferViewCreateFlags;
	}

	pub fn buffer(&self) -> u64 {
		return self.raw.buffer as u64;
	}

	pub fn format(&self) -> ffi::Format {
		return self.raw.format as ffi::Format;
	}

	pub fn offset(&self) -> ffi::DeviceSize {
		return self.raw.offset as ffi::DeviceSize;
	}

	pub fn range(&self) -> ffi::DeviceSize {
		return self.raw.range as ffi::DeviceSize;
	}
}

pub struct ImageSubresource {
	pub raw: ffi::ImageSubresource
}

impl ImageSubresource {
	pub unsafe fn from_ffi(raw: ffi::ImageSubresource) -> ImageSubresource {
		return ImageSubresource { raw: raw };
	}

	pub fn aspect_mask(&self) -> ffi::ImageAspectFlags {
		return self.raw.aspect_mask as ffi::ImageAspectFlags;
	}

	pub fn mip_level(&self) -> u32 {
		return self.raw.mip_level as u32;
	}

	pub fn array_layer(&self) -> u32 {
		return self.raw.array_layer as u32;
	}
}

pub struct ImageSubresourceLayers {
	pub raw: ffi::ImageSubresourceLayers
}

impl ImageSubresourceLayers {
	pub unsafe fn from_ffi(raw: ffi::ImageSubresourceLayers) -> ImageSubresourceLayers {
		return ImageSubresourceLayers { raw: raw };
	}

	pub fn aspect_mask(&self) -> ffi::ImageAspectFlags {
		return self.raw.aspect_mask as ffi::ImageAspectFlags;
	}

	pub fn mip_level(&self) -> u32 {
		return self.raw.mip_level as u32;
	}

	pub fn base_array_layer(&self) -> u32 {
		return self.raw.base_array_layer as u32;
	}

	pub fn layer_count(&self) -> u32 {
		return self.raw.layer_count as u32;
	}
}

pub struct ImageSubresourceRange {
	pub raw: ffi::ImageSubresourceRange
}

impl ImageSubresourceRange {
	pub unsafe fn from_ffi(raw: ffi::ImageSubresourceRange) -> ImageSubresourceRange {
		return ImageSubresourceRange { raw: raw };
	}

	pub fn aspect_mask(&self) -> ffi::ImageAspectFlags {
		return self.raw.aspect_mask as ffi::ImageAspectFlags;
	}

	pub fn base_mip_level(&self) -> u32 {
		return self.raw.base_mip_level as u32;
	}

	pub fn level_count(&self) -> u32 {
		return self.raw.level_count as u32;
	}

	pub fn base_array_layer(&self) -> u32 {
		return self.raw.base_array_layer as u32;
	}

	pub fn layer_count(&self) -> u32 {
		return self.raw.layer_count as u32;
	}
}

pub struct MemoryBarrier {
	pub raw: ffi::MemoryBarrier
}

impl MemoryBarrier {
	pub unsafe fn from_ffi(raw: ffi::MemoryBarrier) -> MemoryBarrier {
		return MemoryBarrier { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn src_access_mask(&self) -> ffi::AccessFlags {
		return self.raw.src_access_mask as ffi::AccessFlags;
	}

	pub fn dst_access_mask(&self) -> ffi::AccessFlags {
		return self.raw.dst_access_mask as ffi::AccessFlags;
	}
}

pub struct BufferMemoryBarrier {
	pub raw: ffi::BufferMemoryBarrier
}

impl BufferMemoryBarrier {
	pub unsafe fn from_ffi(raw: ffi::BufferMemoryBarrier) -> BufferMemoryBarrier {
		return BufferMemoryBarrier { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn src_access_mask(&self) -> ffi::AccessFlags {
		return self.raw.src_access_mask as ffi::AccessFlags;
	}

	pub fn dst_access_mask(&self) -> ffi::AccessFlags {
		return self.raw.dst_access_mask as ffi::AccessFlags;
	}

	pub fn src_queue_family_index(&self) -> u32 {
		return self.raw.src_queue_family_index as u32;
	}

	pub fn dst_queue_family_index(&self) -> u32 {
		return self.raw.dst_queue_family_index as u32;
	}

	pub fn buffer(&self) -> u64 {
		return self.raw.buffer as u64;
	}

	pub fn offset(&self) -> ffi::DeviceSize {
		return self.raw.offset as ffi::DeviceSize;
	}

	pub fn size(&self) -> ffi::DeviceSize {
		return self.raw.size as ffi::DeviceSize;
	}
}

pub struct ImageMemoryBarrier {
	pub raw: ffi::ImageMemoryBarrier
}

impl ImageMemoryBarrier {
	pub unsafe fn from_ffi(raw: ffi::ImageMemoryBarrier) -> ImageMemoryBarrier {
		return ImageMemoryBarrier { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn src_access_mask(&self) -> ffi::AccessFlags {
		return self.raw.src_access_mask as ffi::AccessFlags;
	}

	pub fn dst_access_mask(&self) -> ffi::AccessFlags {
		return self.raw.dst_access_mask as ffi::AccessFlags;
	}

	pub fn old_layout(&self) -> ffi::ImageLayout {
		return self.raw.old_layout as ffi::ImageLayout;
	}

	pub fn new_layout(&self) -> ffi::ImageLayout {
		return self.raw.new_layout as ffi::ImageLayout;
	}

	pub fn src_queue_family_index(&self) -> u32 {
		return self.raw.src_queue_family_index as u32;
	}

	pub fn dst_queue_family_index(&self) -> u32 {
		return self.raw.dst_queue_family_index as u32;
	}

	pub fn image(&self) -> u64 {
		return self.raw.image as u64;
	}

	pub fn subresource_range(&self) -> &ffi::ImageSubresourceRange {
		return &self.raw.subresource_range;
	}
}

pub struct ImageCreateInfo {
	pub raw: ffi::ImageCreateInfo
}

impl ImageCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::ImageCreateInfo) -> ImageCreateInfo {
		return ImageCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::ImageCreateFlags {
		return self.raw.flags as ffi::ImageCreateFlags;
	}

	pub fn image_type(&self) -> ffi::ImageType {
		return self.raw.image_type as ffi::ImageType;
	}

	pub fn format(&self) -> ffi::Format {
		return self.raw.format as ffi::Format;
	}

	pub fn extent(&self) -> &ffi::Extent3D {
		return &self.raw.extent;
	}

	pub fn mip_levels(&self) -> u32 {
		return self.raw.mip_levels as u32;
	}

	pub fn array_layers(&self) -> u32 {
		return self.raw.array_layers as u32;
	}

	pub fn samples(&self) -> ffi::SampleCountFlags {
		return self.raw.samples as ffi::SampleCountFlags;
	}

	pub fn tiling(&self) -> ffi::ImageTiling {
		return self.raw.tiling as ffi::ImageTiling;
	}

	pub fn usage(&self) -> ffi::ImageUsageFlags {
		return self.raw.usage as ffi::ImageUsageFlags;
	}

	pub fn sharing_mode(&self) -> ffi::SharingMode {
		return self.raw.sharing_mode as ffi::SharingMode;
	}

	pub fn queue_family_index_count(&self) -> u32 {
		return self.raw.queue_family_index_count as u32;
	}

	pub fn queue_family_indices(&self) -> &[u32] {
		return unsafe { std::slice::from_raw_parts(self.raw.queue_family_indices, self.queue_family_index_count() as usize) };
	}

	pub fn initial_layout(&self) -> ffi::ImageLayout {
		return self.raw.initial_layout as ffi::ImageLayout;
	}
}

pub struct SubresourceLayout {
	pub raw: ffi::SubresourceLayout
}

impl SubresourceLayout {
	pub unsafe fn from_ffi(raw: ffi::SubresourceLayout) -> SubresourceLayout {
		return SubresourceLayout { raw: raw };
	}

	pub fn offset(&self) -> ffi::DeviceSize {
		return self.raw.offset as ffi::DeviceSize;
	}

	pub fn size(&self) -> ffi::DeviceSize {
		return self.raw.size as ffi::DeviceSize;
	}

	pub fn row_pitch(&self) -> ffi::DeviceSize {
		return self.raw.row_pitch as ffi::DeviceSize;
	}

	pub fn array_pitch(&self) -> ffi::DeviceSize {
		return self.raw.array_pitch as ffi::DeviceSize;
	}

	pub fn depth_pitch(&self) -> ffi::DeviceSize {
		return self.raw.depth_pitch as ffi::DeviceSize;
	}
}

pub struct ImageViewCreateInfo {
	pub raw: ffi::ImageViewCreateInfo
}

impl ImageViewCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::ImageViewCreateInfo) -> ImageViewCreateInfo {
		return ImageViewCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::ImageViewCreateFlags {
		return self.raw.flags as ffi::ImageViewCreateFlags;
	}

	pub fn image(&self) -> u64 {
		return self.raw.image as u64;
	}

	pub fn view_type(&self) -> ffi::ImageViewType {
		return self.raw.view_type as ffi::ImageViewType;
	}

	pub fn format(&self) -> ffi::Format {
		return self.raw.format as ffi::Format;
	}

	pub fn components(&self) -> &ffi::ComponentMapping {
		return &self.raw.components;
	}

	pub fn subresource_range(&self) -> &ffi::ImageSubresourceRange {
		return &self.raw.subresource_range;
	}
}

pub struct BufferCopy {
	pub raw: ffi::BufferCopy
}

impl BufferCopy {
	pub unsafe fn from_ffi(raw: ffi::BufferCopy) -> BufferCopy {
		return BufferCopy { raw: raw };
	}

	pub fn src_offset(&self) -> ffi::DeviceSize {
		return self.raw.src_offset as ffi::DeviceSize;
	}

	pub fn dst_offset(&self) -> ffi::DeviceSize {
		return self.raw.dst_offset as ffi::DeviceSize;
	}

	pub fn size(&self) -> ffi::DeviceSize {
		return self.raw.size as ffi::DeviceSize;
	}
}

pub struct SparseMemoryBind {
	pub raw: ffi::SparseMemoryBind
}

impl SparseMemoryBind {
	pub unsafe fn from_ffi(raw: ffi::SparseMemoryBind) -> SparseMemoryBind {
		return SparseMemoryBind { raw: raw };
	}

	pub fn resource_offset(&self) -> ffi::DeviceSize {
		return self.raw.resource_offset as ffi::DeviceSize;
	}

	pub fn size(&self) -> ffi::DeviceSize {
		return self.raw.size as ffi::DeviceSize;
	}

	pub fn memory(&self) -> u64 {
		return self.raw.memory as u64;
	}

	pub fn memory_offset(&self) -> ffi::DeviceSize {
		return self.raw.memory_offset as ffi::DeviceSize;
	}

	pub fn flags(&self) -> ffi::SparseMemoryBindFlags {
		return self.raw.flags as ffi::SparseMemoryBindFlags;
	}
}

pub struct SparseImageMemoryBind {
	pub raw: ffi::SparseImageMemoryBind
}

impl SparseImageMemoryBind {
	pub unsafe fn from_ffi(raw: ffi::SparseImageMemoryBind) -> SparseImageMemoryBind {
		return SparseImageMemoryBind { raw: raw };
	}

	pub fn subresource(&self) -> &ffi::ImageSubresource {
		return &self.raw.subresource;
	}

	pub fn offset(&self) -> &ffi::Offset3D {
		return &self.raw.offset;
	}

	pub fn extent(&self) -> &ffi::Extent3D {
		return &self.raw.extent;
	}

	pub fn memory(&self) -> u64 {
		return self.raw.memory as u64;
	}

	pub fn memory_offset(&self) -> ffi::DeviceSize {
		return self.raw.memory_offset as ffi::DeviceSize;
	}

	pub fn flags(&self) -> ffi::SparseMemoryBindFlags {
		return self.raw.flags as ffi::SparseMemoryBindFlags;
	}
}

pub struct SparseBufferMemoryBindInfo {
	pub raw: ffi::SparseBufferMemoryBindInfo
}

impl SparseBufferMemoryBindInfo {
	pub unsafe fn from_ffi(raw: ffi::SparseBufferMemoryBindInfo) -> SparseBufferMemoryBindInfo {
		return SparseBufferMemoryBindInfo { raw: raw };
	}

	pub fn buffer(&self) -> u64 {
		return self.raw.buffer as u64;
	}

	pub fn bind_count(&self) -> u32 {
		return self.raw.bind_count as u32;
	}

	pub fn binds(&self) -> &[ffi::SparseMemoryBind] {
		return unsafe { std::slice::from_raw_parts(self.raw.binds, self.bind_count() as usize) };
	}
}

pub struct SparseImageOpaqueMemoryBindInfo {
	pub raw: ffi::SparseImageOpaqueMemoryBindInfo
}

impl SparseImageOpaqueMemoryBindInfo {
	pub unsafe fn from_ffi(raw: ffi::SparseImageOpaqueMemoryBindInfo) -> SparseImageOpaqueMemoryBindInfo {
		return SparseImageOpaqueMemoryBindInfo { raw: raw };
	}

	pub fn image(&self) -> u64 {
		return self.raw.image as u64;
	}

	pub fn bind_count(&self) -> u32 {
		return self.raw.bind_count as u32;
	}

	pub fn binds(&self) -> &[ffi::SparseMemoryBind] {
		return unsafe { std::slice::from_raw_parts(self.raw.binds, self.bind_count() as usize) };
	}
}

pub struct SparseImageMemoryBindInfo {
	pub raw: ffi::SparseImageMemoryBindInfo
}

impl SparseImageMemoryBindInfo {
	pub unsafe fn from_ffi(raw: ffi::SparseImageMemoryBindInfo) -> SparseImageMemoryBindInfo {
		return SparseImageMemoryBindInfo { raw: raw };
	}

	pub fn image(&self) -> u64 {
		return self.raw.image as u64;
	}

	pub fn bind_count(&self) -> u32 {
		return self.raw.bind_count as u32;
	}

	pub fn binds(&self) -> &[ffi::SparseImageMemoryBind] {
		return unsafe { std::slice::from_raw_parts(self.raw.binds, self.bind_count() as usize) };
	}
}

pub struct BindSparseInfo {
	pub raw: ffi::BindSparseInfo
}

impl BindSparseInfo {
	pub unsafe fn from_ffi(raw: ffi::BindSparseInfo) -> BindSparseInfo {
		return BindSparseInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn wait_semaphore_count(&self) -> u32 {
		return self.raw.wait_semaphore_count as u32;
	}

	pub fn wait_semaphores(&self) -> &[u64] {
		return unsafe { std::slice::from_raw_parts(self.raw.wait_semaphores, self.wait_semaphore_count() as usize) };
	}

	pub fn buffer_bind_count(&self) -> u32 {
		return self.raw.buffer_bind_count as u32;
	}

	pub fn buffer_binds(&self) -> &[ffi::SparseBufferMemoryBindInfo] {
		return unsafe { std::slice::from_raw_parts(self.raw.buffer_binds, self.buffer_bind_count() as usize) };
	}

	pub fn image_opaque_bind_count(&self) -> u32 {
		return self.raw.image_opaque_bind_count as u32;
	}

	pub fn image_opaque_binds(&self) -> &[ffi::SparseImageOpaqueMemoryBindInfo] {
		return unsafe { std::slice::from_raw_parts(self.raw.image_opaque_binds, self.image_opaque_bind_count() as usize) };
	}

	pub fn image_bind_count(&self) -> u32 {
		return self.raw.image_bind_count as u32;
	}

	pub fn image_binds(&self) -> &[ffi::SparseImageMemoryBindInfo] {
		return unsafe { std::slice::from_raw_parts(self.raw.image_binds, self.image_bind_count() as usize) };
	}

	pub fn signal_semaphore_count(&self) -> u32 {
		return self.raw.signal_semaphore_count as u32;
	}

	pub fn signal_semaphores(&self) -> &[u64] {
		return unsafe { std::slice::from_raw_parts(self.raw.signal_semaphores, self.signal_semaphore_count() as usize) };
	}
}

pub struct ImageCopy {
	pub raw: ffi::ImageCopy
}

impl ImageCopy {
	pub unsafe fn from_ffi(raw: ffi::ImageCopy) -> ImageCopy {
		return ImageCopy { raw: raw };
	}

	pub fn src_subresource(&self) -> &ffi::ImageSubresourceLayers {
		return &self.raw.src_subresource;
	}

	pub fn src_offset(&self) -> &ffi::Offset3D {
		return &self.raw.src_offset;
	}

	pub fn dst_subresource(&self) -> &ffi::ImageSubresourceLayers {
		return &self.raw.dst_subresource;
	}

	pub fn dst_offset(&self) -> &ffi::Offset3D {
		return &self.raw.dst_offset;
	}

	pub fn extent(&self) -> &ffi::Extent3D {
		return &self.raw.extent;
	}
}

pub struct ImageBlit {
	pub raw: ffi::ImageBlit
}

impl ImageBlit {
	pub unsafe fn from_ffi(raw: ffi::ImageBlit) -> ImageBlit {
		return ImageBlit { raw: raw };
	}

	pub fn src_subresource(&self) -> &ffi::ImageSubresourceLayers {
		return &self.raw.src_subresource;
	}

	pub fn src_offsets(&self) -> &[ffi::Offset3D; 2] {
		return &self.raw.src_offsets;
	}

	pub fn dst_subresource(&self) -> &ffi::ImageSubresourceLayers {
		return &self.raw.dst_subresource;
	}

	pub fn dst_offsets(&self) -> &[ffi::Offset3D; 2] {
		return &self.raw.dst_offsets;
	}
}

pub struct BufferImageCopy {
	pub raw: ffi::BufferImageCopy
}

impl BufferImageCopy {
	pub unsafe fn from_ffi(raw: ffi::BufferImageCopy) -> BufferImageCopy {
		return BufferImageCopy { raw: raw };
	}

	pub fn buffer_offset(&self) -> ffi::DeviceSize {
		return self.raw.buffer_offset as ffi::DeviceSize;
	}

	pub fn buffer_row_length(&self) -> u32 {
		return self.raw.buffer_row_length as u32;
	}

	pub fn buffer_image_height(&self) -> u32 {
		return self.raw.buffer_image_height as u32;
	}

	pub fn image_subresource(&self) -> &ffi::ImageSubresourceLayers {
		return &self.raw.image_subresource;
	}

	pub fn image_offset(&self) -> &ffi::Offset3D {
		return &self.raw.image_offset;
	}

	pub fn image_extent(&self) -> &ffi::Extent3D {
		return &self.raw.image_extent;
	}
}

pub struct ImageResolve {
	pub raw: ffi::ImageResolve
}

impl ImageResolve {
	pub unsafe fn from_ffi(raw: ffi::ImageResolve) -> ImageResolve {
		return ImageResolve { raw: raw };
	}

	pub fn src_subresource(&self) -> &ffi::ImageSubresourceLayers {
		return &self.raw.src_subresource;
	}

	pub fn src_offset(&self) -> &ffi::Offset3D {
		return &self.raw.src_offset;
	}

	pub fn dst_subresource(&self) -> &ffi::ImageSubresourceLayers {
		return &self.raw.dst_subresource;
	}

	pub fn dst_offset(&self) -> &ffi::Offset3D {
		return &self.raw.dst_offset;
	}

	pub fn extent(&self) -> &ffi::Extent3D {
		return &self.raw.extent;
	}
}

pub struct ShaderModuleCreateInfo {
	pub raw: ffi::ShaderModuleCreateInfo
}

impl ShaderModuleCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::ShaderModuleCreateInfo) -> ShaderModuleCreateInfo {
		return ShaderModuleCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::ShaderModuleCreateFlags {
		return self.raw.flags as ffi::ShaderModuleCreateFlags;
	}

	pub fn code_size(&self) -> libc::size_t {
		return self.raw.code_size as libc::size_t;
	}

	pub fn code(&self) -> &[u32] {
		return unsafe { std::slice::from_raw_parts(self.raw.code, self.code_size() / 4 as usize) };
	}
}

pub struct DescriptorSetLayoutBinding {
	pub raw: ffi::DescriptorSetLayoutBinding
}

impl DescriptorSetLayoutBinding {
	pub unsafe fn from_ffi(raw: ffi::DescriptorSetLayoutBinding) -> DescriptorSetLayoutBinding {
		return DescriptorSetLayoutBinding { raw: raw };
	}

	pub fn binding(&self) -> u32 {
		return self.raw.binding as u32;
	}

	pub fn descriptor_type(&self) -> ffi::DescriptorType {
		return self.raw.descriptor_type as ffi::DescriptorType;
	}

	pub fn descriptor_count(&self) -> u32 {
		return self.raw.descriptor_count as u32;
	}

	pub fn stage_flags(&self) -> ffi::ShaderStageFlags {
		return self.raw.stage_flags as ffi::ShaderStageFlags;
	}

	pub fn immutable_samplers(&self) -> &[u64] {
		return unsafe { std::slice::from_raw_parts(self.raw.immutable_samplers, self.descriptor_count() as usize) };
	}
}

pub struct DescriptorSetLayoutCreateInfo {
	pub raw: ffi::DescriptorSetLayoutCreateInfo
}

impl DescriptorSetLayoutCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::DescriptorSetLayoutCreateInfo) -> DescriptorSetLayoutCreateInfo {
		return DescriptorSetLayoutCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::DescriptorSetLayoutCreateFlags {
		return self.raw.flags as ffi::DescriptorSetLayoutCreateFlags;
	}

	pub fn binding_count(&self) -> u32 {
		return self.raw.binding_count as u32;
	}

	pub fn bindings(&self) -> &[ffi::DescriptorSetLayoutBinding] {
		return unsafe { std::slice::from_raw_parts(self.raw.bindings, self.binding_count() as usize) };
	}
}

pub struct DescriptorPoolSize {
	pub raw: ffi::DescriptorPoolSize
}

impl DescriptorPoolSize {
	pub unsafe fn from_ffi(raw: ffi::DescriptorPoolSize) -> DescriptorPoolSize {
		return DescriptorPoolSize { raw: raw };
	}

	pub fn ty(&self) -> ffi::DescriptorType {
		return self.raw.ty as ffi::DescriptorType;
	}

	pub fn descriptor_count(&self) -> u32 {
		return self.raw.descriptor_count as u32;
	}
}

pub struct DescriptorPoolCreateInfo {
	pub raw: ffi::DescriptorPoolCreateInfo
}

impl DescriptorPoolCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::DescriptorPoolCreateInfo) -> DescriptorPoolCreateInfo {
		return DescriptorPoolCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::DescriptorPoolCreateFlags {
		return self.raw.flags as ffi::DescriptorPoolCreateFlags;
	}

	pub fn max_sets(&self) -> u32 {
		return self.raw.max_sets as u32;
	}

	pub fn pool_size_count(&self) -> u32 {
		return self.raw.pool_size_count as u32;
	}

	pub fn pool_sizes(&self) -> &[ffi::DescriptorPoolSize] {
		return unsafe { std::slice::from_raw_parts(self.raw.pool_sizes, self.pool_size_count() as usize) };
	}
}

pub struct DescriptorSetAllocateInfo {
	pub raw: ffi::DescriptorSetAllocateInfo
}

impl DescriptorSetAllocateInfo {
	pub unsafe fn from_ffi(raw: ffi::DescriptorSetAllocateInfo) -> DescriptorSetAllocateInfo {
		return DescriptorSetAllocateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn descriptor_pool(&self) -> u64 {
		return self.raw.descriptor_pool as u64;
	}

	pub fn descriptor_set_count(&self) -> u32 {
		return self.raw.descriptor_set_count as u32;
	}

	pub fn set_layouts(&self) -> &[u64] {
		return unsafe { std::slice::from_raw_parts(self.raw.set_layouts, self.descriptor_set_count() as usize) };
	}
}

pub struct SpecializationMapEntry {
	pub raw: ffi::SpecializationMapEntry
}

impl SpecializationMapEntry {
	pub unsafe fn from_ffi(raw: ffi::SpecializationMapEntry) -> SpecializationMapEntry {
		return SpecializationMapEntry { raw: raw };
	}

	pub fn constant_id(&self) -> u32 {
		return self.raw.constant_id as u32;
	}

	pub fn offset(&self) -> u32 {
		return self.raw.offset as u32;
	}

	pub fn size(&self) -> libc::size_t {
		return self.raw.size as libc::size_t;
	}
}

pub struct SpecializationInfo {
	pub raw: ffi::SpecializationInfo
}

impl SpecializationInfo {
	pub unsafe fn from_ffi(raw: ffi::SpecializationInfo) -> SpecializationInfo {
		return SpecializationInfo { raw: raw };
	}

	pub fn map_entry_count(&self) -> u32 {
		return self.raw.map_entry_count as u32;
	}

	pub fn ma(&self) -> &[ffi::SpecializationMapEntry] {
		return unsafe { std::slice::from_raw_parts(self.raw.ma, self.map_entry_count() as usize) };
	}

	pub fn data_size(&self) -> libc::size_t {
		return self.raw.data_size as libc::size_t;
	}

	pub fn data(&self) -> &[libc::c_void] {
		return unsafe { std::slice::from_raw_parts(self.raw.data, self.data_size() as usize) };
	}
}

pub struct PipelineShaderStageCreateInfo {
	pub raw: ffi::PipelineShaderStageCreateInfo
}

impl PipelineShaderStageCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::PipelineShaderStageCreateInfo) -> PipelineShaderStageCreateInfo {
		return PipelineShaderStageCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::PipelineShaderStageCreateFlags {
		return self.raw.flags as ffi::PipelineShaderStageCreateFlags;
	}

	pub fn stage(&self) -> ffi::ShaderStageFlags {
		return self.raw.stage as ffi::ShaderStageFlags;
	}

	pub fn module(&self) -> u64 {
		return self.raw.module as u64;
	}

	pub fn name(&self) -> &str {
		return unsafe { std::ffi::CStr::from_ptr(self.raw.name).to_str().unwrap() };
	}

	pub fn specialization_info(&self) -> &ffi::SpecializationInfo {
		return unsafe { &*self.raw.specialization_info as &ffi::SpecializationInfo };
	}
}

pub struct ComputePipelineCreateInfo {
	pub raw: ffi::ComputePipelineCreateInfo
}

impl ComputePipelineCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::ComputePipelineCreateInfo) -> ComputePipelineCreateInfo {
		return ComputePipelineCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::PipelineCreateFlags {
		return self.raw.flags as ffi::PipelineCreateFlags;
	}

	pub fn stage(&self) -> &ffi::PipelineShaderStageCreateInfo {
		return &self.raw.stage;
	}

	pub fn layout(&self) -> u64 {
		return self.raw.layout as u64;
	}

	pub fn base_pipeline_handle(&self) -> u64 {
		return self.raw.base_pipeline_handle as u64;
	}

	pub fn base_pipeline_index(&self) -> i32 {
		return self.raw.base_pipeline_index as i32;
	}
}

pub struct VertexInputBindingDescription {
	pub raw: ffi::VertexInputBindingDescription
}

impl VertexInputBindingDescription {
	pub unsafe fn from_ffi(raw: ffi::VertexInputBindingDescription) -> VertexInputBindingDescription {
		return VertexInputBindingDescription { raw: raw };
	}

	pub fn binding(&self) -> u32 {
		return self.raw.binding as u32;
	}

	pub fn stride(&self) -> u32 {
		return self.raw.stride as u32;
	}

	pub fn input_rate(&self) -> ffi::VertexInputRate {
		return self.raw.input_rate as ffi::VertexInputRate;
	}
}

pub struct VertexInputAttributeDescription {
	pub raw: ffi::VertexInputAttributeDescription
}

impl VertexInputAttributeDescription {
	pub unsafe fn from_ffi(raw: ffi::VertexInputAttributeDescription) -> VertexInputAttributeDescription {
		return VertexInputAttributeDescription { raw: raw };
	}

	pub fn location(&self) -> u32 {
		return self.raw.location as u32;
	}

	pub fn binding(&self) -> u32 {
		return self.raw.binding as u32;
	}

	pub fn format(&self) -> ffi::Format {
		return self.raw.format as ffi::Format;
	}

	pub fn offset(&self) -> u32 {
		return self.raw.offset as u32;
	}
}

pub struct PipelineVertexInputStateCreateInfo {
	pub raw: ffi::PipelineVertexInputStateCreateInfo
}

impl PipelineVertexInputStateCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::PipelineVertexInputStateCreateInfo) -> PipelineVertexInputStateCreateInfo {
		return PipelineVertexInputStateCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::PipelineVertexInputStateCreateFlags {
		return self.raw.flags as ffi::PipelineVertexInputStateCreateFlags;
	}

	pub fn vertex_binding_description_count(&self) -> u32 {
		return self.raw.vertex_binding_description_count as u32;
	}

	pub fn vertex_binding_descriptions(&self) -> &[ffi::VertexInputBindingDescription] {
		return unsafe { std::slice::from_raw_parts(self.raw.vertex_binding_descriptions, self.vertex_binding_description_count() as usize) };
	}

	pub fn vertex_attribute_description_count(&self) -> u32 {
		return self.raw.vertex_attribute_description_count as u32;
	}

	pub fn vertex_attribute_descriptions(&self) -> &[ffi::VertexInputAttributeDescription] {
		return unsafe { std::slice::from_raw_parts(self.raw.vertex_attribute_descriptions, self.vertex_attribute_description_count() as usize) };
	}
}

pub struct PipelineInputAssemblyStateCreateInfo {
	pub raw: ffi::PipelineInputAssemblyStateCreateInfo
}

impl PipelineInputAssemblyStateCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::PipelineInputAssemblyStateCreateInfo) -> PipelineInputAssemblyStateCreateInfo {
		return PipelineInputAssemblyStateCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::PipelineInputAssemblyStateCreateFlags {
		return self.raw.flags as ffi::PipelineInputAssemblyStateCreateFlags;
	}

	pub fn topology(&self) -> ffi::PrimitiveTopology {
		return self.raw.topology as ffi::PrimitiveTopology;
	}

	pub fn primitive_restart_enable(&self) -> ffi::Bool32 {
		return self.raw.primitive_restart_enable as ffi::Bool32;
	}
}

pub struct PipelineTessellationStateCreateInfo {
	pub raw: ffi::PipelineTessellationStateCreateInfo
}

impl PipelineTessellationStateCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::PipelineTessellationStateCreateInfo) -> PipelineTessellationStateCreateInfo {
		return PipelineTessellationStateCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::PipelineTessellationStateCreateFlags {
		return self.raw.flags as ffi::PipelineTessellationStateCreateFlags;
	}

	pub fn patch_control_points(&self) -> u32 {
		return self.raw.patch_control_points as u32;
	}
}

pub struct PipelineViewportStateCreateInfo {
	pub raw: ffi::PipelineViewportStateCreateInfo
}

impl PipelineViewportStateCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::PipelineViewportStateCreateInfo) -> PipelineViewportStateCreateInfo {
		return PipelineViewportStateCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::PipelineViewportStateCreateFlags {
		return self.raw.flags as ffi::PipelineViewportStateCreateFlags;
	}

	pub fn viewport_count(&self) -> u32 {
		return self.raw.viewport_count as u32;
	}

	pub fn viewports(&self) -> &[ffi::Viewport] {
		return unsafe { std::slice::from_raw_parts(self.raw.viewports, self.viewport_count() as usize) };
	}

	pub fn scissor_count(&self) -> u32 {
		return self.raw.scissor_count as u32;
	}

	pub fn scissors(&self) -> &[ffi::Rect2D] {
		return unsafe { std::slice::from_raw_parts(self.raw.scissors, self.scissor_count() as usize) };
	}
}

pub struct PipelineRasterizationStateCreateInfo {
	pub raw: ffi::PipelineRasterizationStateCreateInfo
}

impl PipelineRasterizationStateCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::PipelineRasterizationStateCreateInfo) -> PipelineRasterizationStateCreateInfo {
		return PipelineRasterizationStateCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::PipelineRasterizationStateCreateFlags {
		return self.raw.flags as ffi::PipelineRasterizationStateCreateFlags;
	}

	pub fn depth_clamp_enable(&self) -> ffi::Bool32 {
		return self.raw.depth_clamp_enable as ffi::Bool32;
	}

	pub fn rasterizer_discard_enable(&self) -> ffi::Bool32 {
		return self.raw.rasterizer_discard_enable as ffi::Bool32;
	}

	pub fn polygon_mode(&self) -> ffi::PolygonMode {
		return self.raw.polygon_mode as ffi::PolygonMode;
	}

	pub fn cull_mode(&self) -> ffi::CullModeFlags {
		return self.raw.cull_mode as ffi::CullModeFlags;
	}

	pub fn front_face(&self) -> ffi::FrontFace {
		return self.raw.front_face as ffi::FrontFace;
	}

	pub fn depth_bias_enable(&self) -> ffi::Bool32 {
		return self.raw.depth_bias_enable as ffi::Bool32;
	}

	pub fn depth_bias_constant_factor(&self) -> f32 {
		return self.raw.depth_bias_constant_factor as f32;
	}

	pub fn depth_bias_clamp(&self) -> f32 {
		return self.raw.depth_bias_clamp as f32;
	}

	pub fn depth_bias_slope_factor(&self) -> f32 {
		return self.raw.depth_bias_slope_factor as f32;
	}

	pub fn line_width(&self) -> f32 {
		return self.raw.line_width as f32;
	}
}

pub struct PipelineMultisampleStateCreateInfo {
	pub raw: ffi::PipelineMultisampleStateCreateInfo
}

impl PipelineMultisampleStateCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::PipelineMultisampleStateCreateInfo) -> PipelineMultisampleStateCreateInfo {
		return PipelineMultisampleStateCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::PipelineMultisampleStateCreateFlags {
		return self.raw.flags as ffi::PipelineMultisampleStateCreateFlags;
	}

	pub fn rasterization_samples(&self) -> ffi::SampleCountFlags {
		return self.raw.rasterization_samples as ffi::SampleCountFlags;
	}

	pub fn sample_shading_enable(&self) -> ffi::Bool32 {
		return self.raw.sample_shading_enable as ffi::Bool32;
	}

	pub fn min_sample_shading(&self) -> f32 {
		return self.raw.min_sample_shading as f32;
	}

	pub fn sample_mask(&self) -> &[ffi::SampleMask] {
		return unsafe { std::slice::from_raw_parts(self.raw.sample_mask, (self.rasterization_samples().bits() as usize + 32 - 1) / 32 as usize) };
	}

	pub fn alpha_to_coverage_enable(&self) -> ffi::Bool32 {
		return self.raw.alpha_to_coverage_enable as ffi::Bool32;
	}

	pub fn alpha_to_one_enable(&self) -> ffi::Bool32 {
		return self.raw.alpha_to_one_enable as ffi::Bool32;
	}
}

pub struct PipelineColorBlendAttachmentState {
	pub raw: ffi::PipelineColorBlendAttachmentState
}

impl PipelineColorBlendAttachmentState {
	pub unsafe fn from_ffi(raw: ffi::PipelineColorBlendAttachmentState) -> PipelineColorBlendAttachmentState {
		return PipelineColorBlendAttachmentState { raw: raw };
	}

	pub fn blend_enable(&self) -> ffi::Bool32 {
		return self.raw.blend_enable as ffi::Bool32;
	}

	pub fn src_color_blend_factor(&self) -> ffi::BlendFactor {
		return self.raw.src_color_blend_factor as ffi::BlendFactor;
	}

	pub fn dst_color_blend_factor(&self) -> ffi::BlendFactor {
		return self.raw.dst_color_blend_factor as ffi::BlendFactor;
	}

	pub fn color_blend_op(&self) -> ffi::BlendOp {
		return self.raw.color_blend_op as ffi::BlendOp;
	}

	pub fn src_alpha_blend_factor(&self) -> ffi::BlendFactor {
		return self.raw.src_alpha_blend_factor as ffi::BlendFactor;
	}

	pub fn dst_alpha_blend_factor(&self) -> ffi::BlendFactor {
		return self.raw.dst_alpha_blend_factor as ffi::BlendFactor;
	}

	pub fn alpha_blend_op(&self) -> ffi::BlendOp {
		return self.raw.alpha_blend_op as ffi::BlendOp;
	}

	pub fn color_write_mask(&self) -> ffi::ColorComponentFlags {
		return self.raw.color_write_mask as ffi::ColorComponentFlags;
	}
}

pub struct PipelineColorBlendStateCreateInfo {
	pub raw: ffi::PipelineColorBlendStateCreateInfo
}

impl PipelineColorBlendStateCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::PipelineColorBlendStateCreateInfo) -> PipelineColorBlendStateCreateInfo {
		return PipelineColorBlendStateCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::PipelineColorBlendStateCreateFlags {
		return self.raw.flags as ffi::PipelineColorBlendStateCreateFlags;
	}

	pub fn logic_op_enable(&self) -> ffi::Bool32 {
		return self.raw.logic_op_enable as ffi::Bool32;
	}

	pub fn logic_op(&self) -> ffi::LogicOp {
		return self.raw.logic_op as ffi::LogicOp;
	}

	pub fn attachment_count(&self) -> u32 {
		return self.raw.attachment_count as u32;
	}

	pub fn attachments(&self) -> &[ffi::PipelineColorBlendAttachmentState] {
		return unsafe { std::slice::from_raw_parts(self.raw.attachments, self.attachment_count() as usize) };
	}

	pub fn blend_constants(&self) -> f32 {
		return self.raw.blend_constants as f32;
	}
}

pub struct PipelineDynamicStateCreateInfo {
	pub raw: ffi::PipelineDynamicStateCreateInfo
}

impl PipelineDynamicStateCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::PipelineDynamicStateCreateInfo) -> PipelineDynamicStateCreateInfo {
		return PipelineDynamicStateCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::PipelineDynamicStateCreateFlags {
		return self.raw.flags as ffi::PipelineDynamicStateCreateFlags;
	}

	pub fn dynamic_state_count(&self) -> u32 {
		return self.raw.dynamic_state_count as u32;
	}

	pub fn dynamic_states(&self) -> &[ffi::DynamicState] {
		return unsafe { std::slice::from_raw_parts(self.raw.dynamic_states, self.dynamic_state_count() as usize) };
	}
}

pub struct StencilOpState {
	pub raw: ffi::StencilOpState
}

impl StencilOpState {
	pub unsafe fn from_ffi(raw: ffi::StencilOpState) -> StencilOpState {
		return StencilOpState { raw: raw };
	}

	pub fn fail_op(&self) -> ffi::StencilOp {
		return self.raw.fail_op as ffi::StencilOp;
	}

	pub fn pass_op(&self) -> ffi::StencilOp {
		return self.raw.pass_op as ffi::StencilOp;
	}

	pub fn depth_fail_op(&self) -> ffi::StencilOp {
		return self.raw.depth_fail_op as ffi::StencilOp;
	}

	pub fn compare_op(&self) -> ffi::CompareOp {
		return self.raw.compare_op as ffi::CompareOp;
	}

	pub fn compare_mask(&self) -> u32 {
		return self.raw.compare_mask as u32;
	}

	pub fn write_mask(&self) -> u32 {
		return self.raw.write_mask as u32;
	}

	pub fn reference(&self) -> u32 {
		return self.raw.reference as u32;
	}
}

pub struct PipelineDepthStencilStateCreateInfo {
	pub raw: ffi::PipelineDepthStencilStateCreateInfo
}

impl PipelineDepthStencilStateCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::PipelineDepthStencilStateCreateInfo) -> PipelineDepthStencilStateCreateInfo {
		return PipelineDepthStencilStateCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::PipelineDepthStencilStateCreateFlags {
		return self.raw.flags as ffi::PipelineDepthStencilStateCreateFlags;
	}

	pub fn depth_test_enable(&self) -> ffi::Bool32 {
		return self.raw.depth_test_enable as ffi::Bool32;
	}

	pub fn depth_write_enable(&self) -> ffi::Bool32 {
		return self.raw.depth_write_enable as ffi::Bool32;
	}

	pub fn depth_compare_op(&self) -> ffi::CompareOp {
		return self.raw.depth_compare_op as ffi::CompareOp;
	}

	pub fn depth_bounds_test_enable(&self) -> ffi::Bool32 {
		return self.raw.depth_bounds_test_enable as ffi::Bool32;
	}

	pub fn stencil_test_enable(&self) -> ffi::Bool32 {
		return self.raw.stencil_test_enable as ffi::Bool32;
	}

	pub fn front(&self) -> &ffi::StencilOpState {
		return &self.raw.front;
	}

	pub fn back(&self) -> &ffi::StencilOpState {
		return &self.raw.back;
	}

	pub fn min_depth_bounds(&self) -> f32 {
		return self.raw.min_depth_bounds as f32;
	}

	pub fn max_depth_bounds(&self) -> f32 {
		return self.raw.max_depth_bounds as f32;
	}
}

pub struct GraphicsPipelineCreateInfo {
	pub raw: ffi::GraphicsPipelineCreateInfo
}

impl GraphicsPipelineCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::GraphicsPipelineCreateInfo) -> GraphicsPipelineCreateInfo {
		return GraphicsPipelineCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::PipelineCreateFlags {
		return self.raw.flags as ffi::PipelineCreateFlags;
	}

	pub fn stage_count(&self) -> u32 {
		return self.raw.stage_count as u32;
	}

	pub fn stages(&self) -> &[ffi::PipelineShaderStageCreateInfo] {
		return unsafe { std::slice::from_raw_parts(self.raw.stages, self.stage_count() as usize) };
	}

	pub fn vertex_input_state(&self) -> &ffi::PipelineVertexInputStateCreateInfo {
		return unsafe { &*self.raw.vertex_input_state as &ffi::PipelineVertexInputStateCreateInfo };
	}

	pub fn input_assembly_state(&self) -> &ffi::PipelineInputAssemblyStateCreateInfo {
		return unsafe { &*self.raw.input_assembly_state as &ffi::PipelineInputAssemblyStateCreateInfo };
	}

	pub fn tessellation_state(&self) -> &ffi::PipelineTessellationStateCreateInfo {
		return unsafe { &*self.raw.tessellation_state as &ffi::PipelineTessellationStateCreateInfo };
	}

	pub fn viewport_state(&self) -> &ffi::PipelineViewportStateCreateInfo {
		return unsafe { &*self.raw.viewport_state as &ffi::PipelineViewportStateCreateInfo };
	}

	pub fn rasterization_state(&self) -> &ffi::PipelineRasterizationStateCreateInfo {
		return unsafe { &*self.raw.rasterization_state as &ffi::PipelineRasterizationStateCreateInfo };
	}

	pub fn multisample_state(&self) -> &ffi::PipelineMultisampleStateCreateInfo {
		return unsafe { &*self.raw.multisample_state as &ffi::PipelineMultisampleStateCreateInfo };
	}

	pub fn depth_stencil_state(&self) -> &ffi::PipelineDepthStencilStateCreateInfo {
		return unsafe { &*self.raw.depth_stencil_state as &ffi::PipelineDepthStencilStateCreateInfo };
	}

	pub fn color_blend_state(&self) -> &ffi::PipelineColorBlendStateCreateInfo {
		return unsafe { &*self.raw.color_blend_state as &ffi::PipelineColorBlendStateCreateInfo };
	}

	pub fn dynamic_state(&self) -> &ffi::PipelineDynamicStateCreateInfo {
		return unsafe { &*self.raw.dynamic_state as &ffi::PipelineDynamicStateCreateInfo };
	}

	pub fn layout(&self) -> u64 {
		return self.raw.layout as u64;
	}

	pub fn render_pass(&self) -> u64 {
		return self.raw.render_pass as u64;
	}

	pub fn subpass(&self) -> u32 {
		return self.raw.subpass as u32;
	}

	pub fn base_pipeline_handle(&self) -> u64 {
		return self.raw.base_pipeline_handle as u64;
	}

	pub fn base_pipeline_index(&self) -> i32 {
		return self.raw.base_pipeline_index as i32;
	}
}

pub struct PipelineCacheCreateInfo {
	pub raw: ffi::PipelineCacheCreateInfo
}

impl PipelineCacheCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::PipelineCacheCreateInfo) -> PipelineCacheCreateInfo {
		return PipelineCacheCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::PipelineCacheCreateFlags {
		return self.raw.flags as ffi::PipelineCacheCreateFlags;
	}

	pub fn initial_data_size(&self) -> libc::size_t {
		return self.raw.initial_data_size as libc::size_t;
	}

	pub fn initial_data(&self) -> &[libc::c_void] {
		return unsafe { std::slice::from_raw_parts(self.raw.initial_data, self.initial_data_size() as usize) };
	}
}

pub struct PushConstantRange {
	pub raw: ffi::PushConstantRange
}

impl PushConstantRange {
	pub unsafe fn from_ffi(raw: ffi::PushConstantRange) -> PushConstantRange {
		return PushConstantRange { raw: raw };
	}

	pub fn stage_flags(&self) -> ffi::ShaderStageFlags {
		return self.raw.stage_flags as ffi::ShaderStageFlags;
	}

	pub fn offset(&self) -> u32 {
		return self.raw.offset as u32;
	}

	pub fn size(&self) -> u32 {
		return self.raw.size as u32;
	}
}

pub struct PipelineLayoutCreateInfo {
	pub raw: ffi::PipelineLayoutCreateInfo
}

impl PipelineLayoutCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::PipelineLayoutCreateInfo) -> PipelineLayoutCreateInfo {
		return PipelineLayoutCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::PipelineLayoutCreateFlags {
		return self.raw.flags as ffi::PipelineLayoutCreateFlags;
	}

	pub fn set_layout_count(&self) -> u32 {
		return self.raw.set_layout_count as u32;
	}

	pub fn set_layouts(&self) -> &[u64] {
		return unsafe { std::slice::from_raw_parts(self.raw.set_layouts, self.set_layout_count() as usize) };
	}

	pub fn push_constant_range_count(&self) -> u32 {
		return self.raw.push_constant_range_count as u32;
	}

	pub fn push_constant_ranges(&self) -> &[ffi::PushConstantRange] {
		return unsafe { std::slice::from_raw_parts(self.raw.push_constant_ranges, self.push_constant_range_count() as usize) };
	}
}

pub struct SamplerCreateInfo {
	pub raw: ffi::SamplerCreateInfo
}

impl SamplerCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::SamplerCreateInfo) -> SamplerCreateInfo {
		return SamplerCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::SamplerCreateFlags {
		return self.raw.flags as ffi::SamplerCreateFlags;
	}

	pub fn mag_filter(&self) -> ffi::Filter {
		return self.raw.mag_filter as ffi::Filter;
	}

	pub fn min_filter(&self) -> ffi::Filter {
		return self.raw.min_filter as ffi::Filter;
	}

	pub fn mipmap_mode(&self) -> ffi::SamplerMipmapMode {
		return self.raw.mipmap_mode as ffi::SamplerMipmapMode;
	}

	pub fn address_mode_u(&self) -> ffi::SamplerAddressMode {
		return self.raw.address_mode_u as ffi::SamplerAddressMode;
	}

	pub fn address_mode_v(&self) -> ffi::SamplerAddressMode {
		return self.raw.address_mode_v as ffi::SamplerAddressMode;
	}

	pub fn address_mode_w(&self) -> ffi::SamplerAddressMode {
		return self.raw.address_mode_w as ffi::SamplerAddressMode;
	}

	pub fn mip_lod_bias(&self) -> f32 {
		return self.raw.mip_lod_bias as f32;
	}

	pub fn anisotropy_enable(&self) -> ffi::Bool32 {
		return self.raw.anisotropy_enable as ffi::Bool32;
	}

	pub fn max_anisotropy(&self) -> f32 {
		return self.raw.max_anisotropy as f32;
	}

	pub fn compare_enable(&self) -> ffi::Bool32 {
		return self.raw.compare_enable as ffi::Bool32;
	}

	pub fn compare_op(&self) -> ffi::CompareOp {
		return self.raw.compare_op as ffi::CompareOp;
	}

	pub fn min_lod(&self) -> f32 {
		return self.raw.min_lod as f32;
	}

	pub fn max_lod(&self) -> f32 {
		return self.raw.max_lod as f32;
	}

	pub fn border_color(&self) -> ffi::BorderColor {
		return self.raw.border_color as ffi::BorderColor;
	}

	pub fn unnormalized_coordinates(&self) -> ffi::Bool32 {
		return self.raw.unnormalized_coordinates as ffi::Bool32;
	}
}

pub struct CommandPoolCreateInfo {
	pub raw: ffi::CommandPoolCreateInfo
}

impl CommandPoolCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::CommandPoolCreateInfo) -> CommandPoolCreateInfo {
		return CommandPoolCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::CommandPoolCreateFlags {
		return self.raw.flags as ffi::CommandPoolCreateFlags;
	}

	pub fn queue_family_index(&self) -> u32 {
		return self.raw.queue_family_index as u32;
	}
}

pub struct CommandBufferAllocateInfo {
	pub raw: ffi::CommandBufferAllocateInfo
}

impl CommandBufferAllocateInfo {
	pub unsafe fn from_ffi(raw: ffi::CommandBufferAllocateInfo) -> CommandBufferAllocateInfo {
		return CommandBufferAllocateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn command_pool(&self) -> u64 {
		return self.raw.command_pool as u64;
	}

	pub fn level(&self) -> ffi::CommandBufferLevel {
		return self.raw.level as ffi::CommandBufferLevel;
	}

	pub fn command_buffer_count(&self) -> u32 {
		return self.raw.command_buffer_count as u32;
	}
}

pub struct CommandBufferInheritanceInfo {
	pub raw: ffi::CommandBufferInheritanceInfo
}

impl CommandBufferInheritanceInfo {
	pub unsafe fn from_ffi(raw: ffi::CommandBufferInheritanceInfo) -> CommandBufferInheritanceInfo {
		return CommandBufferInheritanceInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn render_pass(&self) -> u64 {
		return self.raw.render_pass as u64;
	}

	pub fn subpass(&self) -> u32 {
		return self.raw.subpass as u32;
	}

	pub fn framebuffer(&self) -> u64 {
		return self.raw.framebuffer as u64;
	}

	pub fn occlusion_query_enable(&self) -> ffi::Bool32 {
		return self.raw.occlusion_query_enable as ffi::Bool32;
	}

	pub fn query_flags(&self) -> ffi::QueryControlFlags {
		return self.raw.query_flags as ffi::QueryControlFlags;
	}

	pub fn pipeline_statistics(&self) -> ffi::QueryPipelineStatisticFlags {
		return self.raw.pipeline_statistics as ffi::QueryPipelineStatisticFlags;
	}
}

pub struct CommandBufferBeginInfo {
	pub raw: ffi::CommandBufferBeginInfo
}

impl CommandBufferBeginInfo {
	pub unsafe fn from_ffi(raw: ffi::CommandBufferBeginInfo) -> CommandBufferBeginInfo {
		return CommandBufferBeginInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::CommandBufferUsageFlags {
		return self.raw.flags as ffi::CommandBufferUsageFlags;
	}

	pub fn inheritance_info(&self) -> &ffi::CommandBufferInheritanceInfo {
		return unsafe { &*self.raw.inheritance_info as &ffi::CommandBufferInheritanceInfo };
	}
}

pub struct RenderPassBeginInfo {
	pub raw: ffi::RenderPassBeginInfo
}

impl RenderPassBeginInfo {
	pub unsafe fn from_ffi(raw: ffi::RenderPassBeginInfo) -> RenderPassBeginInfo {
		return RenderPassBeginInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn render_pass(&self) -> u64 {
		return self.raw.render_pass as u64;
	}

	pub fn framebuffer(&self) -> u64 {
		return self.raw.framebuffer as u64;
	}

	pub fn render_area(&self) -> &ffi::Rect2D {
		return &self.raw.render_area;
	}

	pub fn clear_value_count(&self) -> u32 {
		return self.raw.clear_value_count as u32;
	}

	pub fn clear_values(&self) -> &[libc::size_t] {
		return unsafe { std::slice::from_raw_parts(self.raw.clear_values, self.clear_value_count() as usize) };
	}
}

pub struct ClearDepthStencilValue {
	pub raw: ffi::ClearDepthStencilValue
}

impl ClearDepthStencilValue {
	pub unsafe fn from_ffi(raw: ffi::ClearDepthStencilValue) -> ClearDepthStencilValue {
		return ClearDepthStencilValue { raw: raw };
	}

	pub fn depth(&self) -> f32 {
		return self.raw.depth as f32;
	}

	pub fn stencil(&self) -> u32 {
		return self.raw.stencil as u32;
	}
}

pub struct ClearAttachment {
	pub raw: ffi::ClearAttachment
}

impl ClearAttachment {
	pub unsafe fn from_ffi(raw: ffi::ClearAttachment) -> ClearAttachment {
		return ClearAttachment { raw: raw };
	}

	pub fn aspect_mask(&self) -> ffi::ImageAspectFlags {
		return self.raw.aspect_mask as ffi::ImageAspectFlags;
	}

	pub fn color_attachment(&self) -> u32 {
		return self.raw.color_attachment as u32;
	}

	pub fn clear_value(&self) -> libc::size_t {
		return self.raw.clear_value as libc::size_t;
	}
}

pub struct AttachmentDescription {
	pub raw: ffi::AttachmentDescription
}

impl AttachmentDescription {
	pub unsafe fn from_ffi(raw: ffi::AttachmentDescription) -> AttachmentDescription {
		return AttachmentDescription { raw: raw };
	}

	pub fn flags(&self) -> ffi::AttachmentDescriptionFlags {
		return self.raw.flags as ffi::AttachmentDescriptionFlags;
	}

	pub fn format(&self) -> ffi::Format {
		return self.raw.format as ffi::Format;
	}

	pub fn samples(&self) -> ffi::SampleCountFlags {
		return self.raw.samples as ffi::SampleCountFlags;
	}

	pub fn load_op(&self) -> ffi::AttachmentLoadOp {
		return self.raw.load_op as ffi::AttachmentLoadOp;
	}

	pub fn store_op(&self) -> ffi::AttachmentStoreOp {
		return self.raw.store_op as ffi::AttachmentStoreOp;
	}

	pub fn stencil_load_op(&self) -> ffi::AttachmentLoadOp {
		return self.raw.stencil_load_op as ffi::AttachmentLoadOp;
	}

	pub fn stencil_store_op(&self) -> ffi::AttachmentStoreOp {
		return self.raw.stencil_store_op as ffi::AttachmentStoreOp;
	}

	pub fn initial_layout(&self) -> ffi::ImageLayout {
		return self.raw.initial_layout as ffi::ImageLayout;
	}

	pub fn final_layout(&self) -> ffi::ImageLayout {
		return self.raw.final_layout as ffi::ImageLayout;
	}
}

pub struct AttachmentReference {
	pub raw: ffi::AttachmentReference
}

impl AttachmentReference {
	pub unsafe fn from_ffi(raw: ffi::AttachmentReference) -> AttachmentReference {
		return AttachmentReference { raw: raw };
	}

	pub fn attachment(&self) -> u32 {
		return self.raw.attachment as u32;
	}

	pub fn layout(&self) -> ffi::ImageLayout {
		return self.raw.layout as ffi::ImageLayout;
	}
}

pub struct SubpassDescription {
	pub raw: ffi::SubpassDescription
}

impl SubpassDescription {
	pub unsafe fn from_ffi(raw: ffi::SubpassDescription) -> SubpassDescription {
		return SubpassDescription { raw: raw };
	}

	pub fn flags(&self) -> ffi::SubpassDescriptionFlags {
		return self.raw.flags as ffi::SubpassDescriptionFlags;
	}

	pub fn pipeline_bind_point(&self) -> ffi::PipelineBindPoint {
		return self.raw.pipeline_bind_point as ffi::PipelineBindPoint;
	}

	pub fn input_attachment_count(&self) -> u32 {
		return self.raw.input_attachment_count as u32;
	}

	pub fn input_attachments(&self) -> &[ffi::AttachmentReference] {
		return unsafe { std::slice::from_raw_parts(self.raw.input_attachments, self.input_attachment_count() as usize) };
	}

	pub fn color_attachment_count(&self) -> u32 {
		return self.raw.color_attachment_count as u32;
	}

	pub fn color_attachments(&self) -> &[ffi::AttachmentReference] {
		return unsafe { std::slice::from_raw_parts(self.raw.color_attachments, self.color_attachment_count() as usize) };
	}

	pub fn resolve_attachments(&self) -> &[ffi::AttachmentReference] {
		return unsafe { std::slice::from_raw_parts(self.raw.resolve_attachments, self.color_attachment_count() as usize) };
	}

	pub fn depth_stencil_attachment(&self) -> &ffi::AttachmentReference {
		return unsafe { &*self.raw.depth_stencil_attachment as &ffi::AttachmentReference };
	}

	pub fn preserve_attachment_count(&self) -> u32 {
		return self.raw.preserve_attachment_count as u32;
	}

	pub fn preserve_attachments(&self) -> &[u32] {
		return unsafe { std::slice::from_raw_parts(self.raw.preserve_attachments, self.preserve_attachment_count() as usize) };
	}
}

pub struct SubpassDependency {
	pub raw: ffi::SubpassDependency
}

impl SubpassDependency {
	pub unsafe fn from_ffi(raw: ffi::SubpassDependency) -> SubpassDependency {
		return SubpassDependency { raw: raw };
	}

	pub fn src_subpass(&self) -> u32 {
		return self.raw.src_subpass as u32;
	}

	pub fn dst_subpass(&self) -> u32 {
		return self.raw.dst_subpass as u32;
	}

	pub fn src_stage_mask(&self) -> ffi::PipelineStageFlags {
		return self.raw.src_stage_mask as ffi::PipelineStageFlags;
	}

	pub fn dst_stage_mask(&self) -> ffi::PipelineStageFlags {
		return self.raw.dst_stage_mask as ffi::PipelineStageFlags;
	}

	pub fn src_access_mask(&self) -> ffi::AccessFlags {
		return self.raw.src_access_mask as ffi::AccessFlags;
	}

	pub fn dst_access_mask(&self) -> ffi::AccessFlags {
		return self.raw.dst_access_mask as ffi::AccessFlags;
	}

	pub fn dependency_flags(&self) -> ffi::DependencyFlags {
		return self.raw.dependency_flags as ffi::DependencyFlags;
	}
}

pub struct RenderPassCreateInfo {
	pub raw: ffi::RenderPassCreateInfo
}

impl RenderPassCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::RenderPassCreateInfo) -> RenderPassCreateInfo {
		return RenderPassCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::RenderPassCreateFlags {
		return self.raw.flags as ffi::RenderPassCreateFlags;
	}

	pub fn attachment_count(&self) -> u32 {
		return self.raw.attachment_count as u32;
	}

	pub fn attachments(&self) -> &[ffi::AttachmentDescription] {
		return unsafe { std::slice::from_raw_parts(self.raw.attachments, self.attachment_count() as usize) };
	}

	pub fn subpass_count(&self) -> u32 {
		return self.raw.subpass_count as u32;
	}

	pub fn subpasses(&self) -> &[ffi::SubpassDescription] {
		return unsafe { std::slice::from_raw_parts(self.raw.subpasses, self.subpass_count() as usize) };
	}

	pub fn dependency_count(&self) -> u32 {
		return self.raw.dependency_count as u32;
	}

	pub fn dependencies(&self) -> &[ffi::SubpassDependency] {
		return unsafe { std::slice::from_raw_parts(self.raw.dependencies, self.dependency_count() as usize) };
	}
}

pub struct EventCreateInfo {
	pub raw: ffi::EventCreateInfo
}

impl EventCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::EventCreateInfo) -> EventCreateInfo {
		return EventCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::EventCreateFlags {
		return self.raw.flags as ffi::EventCreateFlags;
	}
}

pub struct FenceCreateInfo {
	pub raw: ffi::FenceCreateInfo
}

impl FenceCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::FenceCreateInfo) -> FenceCreateInfo {
		return FenceCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::FenceCreateFlags {
		return self.raw.flags as ffi::FenceCreateFlags;
	}
}

pub struct PhysicalDeviceFeatures {
	pub raw: ffi::PhysicalDeviceFeatures
}

impl PhysicalDeviceFeatures {
	pub unsafe fn from_ffi(raw: ffi::PhysicalDeviceFeatures) -> PhysicalDeviceFeatures {
		return PhysicalDeviceFeatures { raw: raw };
	}

	pub fn robust_buffer_access(&self) -> ffi::Bool32 {
		return self.raw.robust_buffer_access as ffi::Bool32;
	}

	pub fn full_draw_index_uint32(&self) -> ffi::Bool32 {
		return self.raw.full_draw_index_uint32 as ffi::Bool32;
	}

	pub fn image_cube_array(&self) -> ffi::Bool32 {
		return self.raw.image_cube_array as ffi::Bool32;
	}

	pub fn independent_blend(&self) -> ffi::Bool32 {
		return self.raw.independent_blend as ffi::Bool32;
	}

	pub fn geometry_shader(&self) -> ffi::Bool32 {
		return self.raw.geometry_shader as ffi::Bool32;
	}

	pub fn tessellation_shader(&self) -> ffi::Bool32 {
		return self.raw.tessellation_shader as ffi::Bool32;
	}

	pub fn sample_rate_shading(&self) -> ffi::Bool32 {
		return self.raw.sample_rate_shading as ffi::Bool32;
	}

	pub fn dual_src_blend(&self) -> ffi::Bool32 {
		return self.raw.dual_src_blend as ffi::Bool32;
	}

	pub fn logic_op(&self) -> ffi::Bool32 {
		return self.raw.logic_op as ffi::Bool32;
	}

	pub fn multi_draw_indirect(&self) -> ffi::Bool32 {
		return self.raw.multi_draw_indirect as ffi::Bool32;
	}

	pub fn draw_indirect_first_instance(&self) -> ffi::Bool32 {
		return self.raw.draw_indirect_first_instance as ffi::Bool32;
	}

	pub fn depth_clamp(&self) -> ffi::Bool32 {
		return self.raw.depth_clamp as ffi::Bool32;
	}

	pub fn depth_bias_clamp(&self) -> ffi::Bool32 {
		return self.raw.depth_bias_clamp as ffi::Bool32;
	}

	pub fn fill_mode_non_solid(&self) -> ffi::Bool32 {
		return self.raw.fill_mode_non_solid as ffi::Bool32;
	}

	pub fn depth_bounds(&self) -> ffi::Bool32 {
		return self.raw.depth_bounds as ffi::Bool32;
	}

	pub fn wide_lines(&self) -> ffi::Bool32 {
		return self.raw.wide_lines as ffi::Bool32;
	}

	pub fn large_points(&self) -> ffi::Bool32 {
		return self.raw.large_points as ffi::Bool32;
	}

	pub fn alpha_to_one(&self) -> ffi::Bool32 {
		return self.raw.alpha_to_one as ffi::Bool32;
	}

	pub fn multi_viewport(&self) -> ffi::Bool32 {
		return self.raw.multi_viewport as ffi::Bool32;
	}

	pub fn sampler_anisotropy(&self) -> ffi::Bool32 {
		return self.raw.sampler_anisotropy as ffi::Bool32;
	}

	pub fn texture_compression_etc2(&self) -> ffi::Bool32 {
		return self.raw.texture_compression_etc2 as ffi::Bool32;
	}

	pub fn texture_compression_astc_ldr(&self) -> ffi::Bool32 {
		return self.raw.texture_compression_astc_ldr as ffi::Bool32;
	}

	pub fn texture_compression_bc(&self) -> ffi::Bool32 {
		return self.raw.texture_compression_bc as ffi::Bool32;
	}

	pub fn occlusion_query_precise(&self) -> ffi::Bool32 {
		return self.raw.occlusion_query_precise as ffi::Bool32;
	}

	pub fn pipeline_statistics_query(&self) -> ffi::Bool32 {
		return self.raw.pipeline_statistics_query as ffi::Bool32;
	}

	pub fn vertex_pipeline_stores_and_atomics(&self) -> ffi::Bool32 {
		return self.raw.vertex_pipeline_stores_and_atomics as ffi::Bool32;
	}

	pub fn fragment_stores_and_atomics(&self) -> ffi::Bool32 {
		return self.raw.fragment_stores_and_atomics as ffi::Bool32;
	}

	pub fn shader_tessellation_and_geometry_point_size(&self) -> ffi::Bool32 {
		return self.raw.shader_tessellation_and_geometry_point_size as ffi::Bool32;
	}

	pub fn shader_image_gather_extended(&self) -> ffi::Bool32 {
		return self.raw.shader_image_gather_extended as ffi::Bool32;
	}

	pub fn shader_storage_image_extended_formats(&self) -> ffi::Bool32 {
		return self.raw.shader_storage_image_extended_formats as ffi::Bool32;
	}

	pub fn shader_storage_image_multisample(&self) -> ffi::Bool32 {
		return self.raw.shader_storage_image_multisample as ffi::Bool32;
	}

	pub fn shader_storage_image_read_without_format(&self) -> ffi::Bool32 {
		return self.raw.shader_storage_image_read_without_format as ffi::Bool32;
	}

	pub fn shader_storage_image_write_without_format(&self) -> ffi::Bool32 {
		return self.raw.shader_storage_image_write_without_format as ffi::Bool32;
	}

	pub fn shader_uniform_buffer_array_dynamic_indexing(&self) -> ffi::Bool32 {
		return self.raw.shader_uniform_buffer_array_dynamic_indexing as ffi::Bool32;
	}

	pub fn shader_sampled_image_array_dynamic_indexing(&self) -> ffi::Bool32 {
		return self.raw.shader_sampled_image_array_dynamic_indexing as ffi::Bool32;
	}

	pub fn shader_storage_buffer_array_dynamic_indexing(&self) -> ffi::Bool32 {
		return self.raw.shader_storage_buffer_array_dynamic_indexing as ffi::Bool32;
	}

	pub fn shader_storage_image_array_dynamic_indexing(&self) -> ffi::Bool32 {
		return self.raw.shader_storage_image_array_dynamic_indexing as ffi::Bool32;
	}

	pub fn shader_clip_distance(&self) -> ffi::Bool32 {
		return self.raw.shader_clip_distance as ffi::Bool32;
	}

	pub fn shader_cull_distance(&self) -> ffi::Bool32 {
		return self.raw.shader_cull_distance as ffi::Bool32;
	}

	pub fn shader_float64(&self) -> ffi::Bool32 {
		return self.raw.shader_float64 as ffi::Bool32;
	}

	pub fn shader_int64(&self) -> ffi::Bool32 {
		return self.raw.shader_int64 as ffi::Bool32;
	}

	pub fn shader_int16(&self) -> ffi::Bool32 {
		return self.raw.shader_int16 as ffi::Bool32;
	}

	pub fn shader_resource_residency(&self) -> ffi::Bool32 {
		return self.raw.shader_resource_residency as ffi::Bool32;
	}

	pub fn shader_resource_min_lod(&self) -> ffi::Bool32 {
		return self.raw.shader_resource_min_lod as ffi::Bool32;
	}

	pub fn sparse_binding(&self) -> ffi::Bool32 {
		return self.raw.sparse_binding as ffi::Bool32;
	}

	pub fn sparse_residency_buffer(&self) -> ffi::Bool32 {
		return self.raw.sparse_residency_buffer as ffi::Bool32;
	}

	pub fn sparse_residency_image2_d(&self) -> ffi::Bool32 {
		return self.raw.sparse_residency_image2_d as ffi::Bool32;
	}

	pub fn sparse_residency_image3_d(&self) -> ffi::Bool32 {
		return self.raw.sparse_residency_image3_d as ffi::Bool32;
	}

	pub fn sparse_residency2_samples(&self) -> ffi::Bool32 {
		return self.raw.sparse_residency2_samples as ffi::Bool32;
	}

	pub fn sparse_residency4_samples(&self) -> ffi::Bool32 {
		return self.raw.sparse_residency4_samples as ffi::Bool32;
	}

	pub fn sparse_residency8_samples(&self) -> ffi::Bool32 {
		return self.raw.sparse_residency8_samples as ffi::Bool32;
	}

	pub fn sparse_residency16_samples(&self) -> ffi::Bool32 {
		return self.raw.sparse_residency16_samples as ffi::Bool32;
	}

	pub fn sparse_residency_aliased(&self) -> ffi::Bool32 {
		return self.raw.sparse_residency_aliased as ffi::Bool32;
	}

	pub fn variable_multisample_rate(&self) -> ffi::Bool32 {
		return self.raw.variable_multisample_rate as ffi::Bool32;
	}

	pub fn inherited_queries(&self) -> ffi::Bool32 {
		return self.raw.inherited_queries as ffi::Bool32;
	}
}

pub struct PhysicalDeviceSparseProperties {
	pub raw: ffi::PhysicalDeviceSparseProperties
}

impl PhysicalDeviceSparseProperties {
	pub unsafe fn from_ffi(raw: ffi::PhysicalDeviceSparseProperties) -> PhysicalDeviceSparseProperties {
		return PhysicalDeviceSparseProperties { raw: raw };
	}

	pub fn residency_standard2_d_block_shape(&self) -> ffi::Bool32 {
		return self.raw.residency_standard2_d_block_shape as ffi::Bool32;
	}

	pub fn residency_standard2_d_multisample_block_shape(&self) -> ffi::Bool32 {
		return self.raw.residency_standard2_d_multisample_block_shape as ffi::Bool32;
	}

	pub fn residency_standard3_d_block_shape(&self) -> ffi::Bool32 {
		return self.raw.residency_standard3_d_block_shape as ffi::Bool32;
	}

	pub fn residency_aligned_mip_size(&self) -> ffi::Bool32 {
		return self.raw.residency_aligned_mip_size as ffi::Bool32;
	}

	pub fn residency_non_resident_strict(&self) -> ffi::Bool32 {
		return self.raw.residency_non_resident_strict as ffi::Bool32;
	}
}

pub struct PhysicalDeviceLimits {
	pub raw: ffi::PhysicalDeviceLimits
}

impl PhysicalDeviceLimits {
	pub unsafe fn from_ffi(raw: ffi::PhysicalDeviceLimits) -> PhysicalDeviceLimits {
		return PhysicalDeviceLimits { raw: raw };
	}

	pub fn max_image_dimension1_d(&self) -> u32 {
		return self.raw.max_image_dimension1_d as u32;
	}

	pub fn max_image_dimension2_d(&self) -> u32 {
		return self.raw.max_image_dimension2_d as u32;
	}

	pub fn max_image_dimension3_d(&self) -> u32 {
		return self.raw.max_image_dimension3_d as u32;
	}

	pub fn max_image_dimension_cube(&self) -> u32 {
		return self.raw.max_image_dimension_cube as u32;
	}

	pub fn max_image_array_layers(&self) -> u32 {
		return self.raw.max_image_array_layers as u32;
	}

	pub fn max_texel_buffer_elements(&self) -> u32 {
		return self.raw.max_texel_buffer_elements as u32;
	}

	pub fn max_uniform_buffer_range(&self) -> u32 {
		return self.raw.max_uniform_buffer_range as u32;
	}

	pub fn max_storage_buffer_range(&self) -> u32 {
		return self.raw.max_storage_buffer_range as u32;
	}

	pub fn max_push_constants_size(&self) -> u32 {
		return self.raw.max_push_constants_size as u32;
	}

	pub fn max_memory_allocation_count(&self) -> u32 {
		return self.raw.max_memory_allocation_count as u32;
	}

	pub fn max_sampler_allocation_count(&self) -> u32 {
		return self.raw.max_sampler_allocation_count as u32;
	}

	pub fn buffer_image_granularity(&self) -> ffi::DeviceSize {
		return self.raw.buffer_image_granularity as ffi::DeviceSize;
	}

	pub fn sparse_address_space_size(&self) -> ffi::DeviceSize {
		return self.raw.sparse_address_space_size as ffi::DeviceSize;
	}

	pub fn max_bound_descriptor_sets(&self) -> u32 {
		return self.raw.max_bound_descriptor_sets as u32;
	}

	pub fn max_per_stage_descriptor_samplers(&self) -> u32 {
		return self.raw.max_per_stage_descriptor_samplers as u32;
	}

	pub fn max_per_stage_descriptor_uniform_buffers(&self) -> u32 {
		return self.raw.max_per_stage_descriptor_uniform_buffers as u32;
	}

	pub fn max_per_stage_descriptor_storage_buffers(&self) -> u32 {
		return self.raw.max_per_stage_descriptor_storage_buffers as u32;
	}

	pub fn max_per_stage_descriptor_sampled_images(&self) -> u32 {
		return self.raw.max_per_stage_descriptor_sampled_images as u32;
	}

	pub fn max_per_stage_descriptor_storage_images(&self) -> u32 {
		return self.raw.max_per_stage_descriptor_storage_images as u32;
	}

	pub fn max_per_stage_descriptor_input_attachments(&self) -> u32 {
		return self.raw.max_per_stage_descriptor_input_attachments as u32;
	}

	pub fn max_per_stage_resources(&self) -> u32 {
		return self.raw.max_per_stage_resources as u32;
	}

	pub fn max_descriptor_set_samplers(&self) -> u32 {
		return self.raw.max_descriptor_set_samplers as u32;
	}

	pub fn max_descriptor_set_uniform_buffers(&self) -> u32 {
		return self.raw.max_descriptor_set_uniform_buffers as u32;
	}

	pub fn max_descriptor_set_uniform_buffers_dynamic(&self) -> u32 {
		return self.raw.max_descriptor_set_uniform_buffers_dynamic as u32;
	}

	pub fn max_descriptor_set_storage_buffers(&self) -> u32 {
		return self.raw.max_descriptor_set_storage_buffers as u32;
	}

	pub fn max_descriptor_set_storage_buffers_dynamic(&self) -> u32 {
		return self.raw.max_descriptor_set_storage_buffers_dynamic as u32;
	}

	pub fn max_descriptor_set_sampled_images(&self) -> u32 {
		return self.raw.max_descriptor_set_sampled_images as u32;
	}

	pub fn max_descriptor_set_storage_images(&self) -> u32 {
		return self.raw.max_descriptor_set_storage_images as u32;
	}

	pub fn max_descriptor_set_input_attachments(&self) -> u32 {
		return self.raw.max_descriptor_set_input_attachments as u32;
	}

	pub fn max_vertex_input_attributes(&self) -> u32 {
		return self.raw.max_vertex_input_attributes as u32;
	}

	pub fn max_vertex_input_bindings(&self) -> u32 {
		return self.raw.max_vertex_input_bindings as u32;
	}

	pub fn max_vertex_input_attribute_offset(&self) -> u32 {
		return self.raw.max_vertex_input_attribute_offset as u32;
	}

	pub fn max_vertex_input_binding_stride(&self) -> u32 {
		return self.raw.max_vertex_input_binding_stride as u32;
	}

	pub fn max_vertex_output_components(&self) -> u32 {
		return self.raw.max_vertex_output_components as u32;
	}

	pub fn max_tessellation_generation_level(&self) -> u32 {
		return self.raw.max_tessellation_generation_level as u32;
	}

	pub fn max_tessellation_patch_size(&self) -> u32 {
		return self.raw.max_tessellation_patch_size as u32;
	}

	pub fn max_tessellation_control_per_vertex_input_components(&self) -> u32 {
		return self.raw.max_tessellation_control_per_vertex_input_components as u32;
	}

	pub fn max_tessellation_control_per_vertex_output_components(&self) -> u32 {
		return self.raw.max_tessellation_control_per_vertex_output_components as u32;
	}

	pub fn max_tessellation_control_per_patch_output_components(&self) -> u32 {
		return self.raw.max_tessellation_control_per_patch_output_components as u32;
	}

	pub fn max_tessellation_control_total_output_components(&self) -> u32 {
		return self.raw.max_tessellation_control_total_output_components as u32;
	}

	pub fn max_tessellation_evaluation_input_components(&self) -> u32 {
		return self.raw.max_tessellation_evaluation_input_components as u32;
	}

	pub fn max_tessellation_evaluation_output_components(&self) -> u32 {
		return self.raw.max_tessellation_evaluation_output_components as u32;
	}

	pub fn max_geometry_shader_invocations(&self) -> u32 {
		return self.raw.max_geometry_shader_invocations as u32;
	}

	pub fn max_geometry_input_components(&self) -> u32 {
		return self.raw.max_geometry_input_components as u32;
	}

	pub fn max_geometry_output_components(&self) -> u32 {
		return self.raw.max_geometry_output_components as u32;
	}

	pub fn max_geometry_output_vertices(&self) -> u32 {
		return self.raw.max_geometry_output_vertices as u32;
	}

	pub fn max_geometry_total_output_components(&self) -> u32 {
		return self.raw.max_geometry_total_output_components as u32;
	}

	pub fn max_fragment_input_components(&self) -> u32 {
		return self.raw.max_fragment_input_components as u32;
	}

	pub fn max_fragment_output_attachments(&self) -> u32 {
		return self.raw.max_fragment_output_attachments as u32;
	}

	pub fn max_fragment_dual_src_attachments(&self) -> u32 {
		return self.raw.max_fragment_dual_src_attachments as u32;
	}

	pub fn max_fragment_combined_output_resources(&self) -> u32 {
		return self.raw.max_fragment_combined_output_resources as u32;
	}

	pub fn max_compute_shared_memory_size(&self) -> u32 {
		return self.raw.max_compute_shared_memory_size as u32;
	}

	pub fn max_compute_work_group_count(&self) -> u32 {
		return self.raw.max_compute_work_group_count as u32;
	}

	pub fn max_compute_work_group_invocations(&self) -> u32 {
		return self.raw.max_compute_work_group_invocations as u32;
	}

	pub fn max_compute_work_group_size(&self) -> u32 {
		return self.raw.max_compute_work_group_size as u32;
	}

	pub fn sub_pixel_precision_bits(&self) -> u32 {
		return self.raw.sub_pixel_precision_bits as u32;
	}

	pub fn sub_texel_precision_bits(&self) -> u32 {
		return self.raw.sub_texel_precision_bits as u32;
	}

	pub fn mipmap_precision_bits(&self) -> u32 {
		return self.raw.mipmap_precision_bits as u32;
	}

	pub fn max_draw_indexed_index_value(&self) -> u32 {
		return self.raw.max_draw_indexed_index_value as u32;
	}

	pub fn max_draw_indirect_count(&self) -> u32 {
		return self.raw.max_draw_indirect_count as u32;
	}

	pub fn max_sampler_lod_bias(&self) -> f32 {
		return self.raw.max_sampler_lod_bias as f32;
	}

	pub fn max_sampler_anisotropy(&self) -> f32 {
		return self.raw.max_sampler_anisotropy as f32;
	}

	pub fn max_viewports(&self) -> u32 {
		return self.raw.max_viewports as u32;
	}

	pub fn max_viewport_dimensions(&self) -> u32 {
		return self.raw.max_viewport_dimensions as u32;
	}

	pub fn viewport_bounds_range(&self) -> f32 {
		return self.raw.viewport_bounds_range as f32;
	}

	pub fn viewport_sub_pixel_bits(&self) -> u32 {
		return self.raw.viewport_sub_pixel_bits as u32;
	}

	pub fn min_memory_map_alignment(&self) -> libc::size_t {
		return self.raw.min_memory_map_alignment as libc::size_t;
	}

	pub fn min_texel_buffer_offset_alignment(&self) -> ffi::DeviceSize {
		return self.raw.min_texel_buffer_offset_alignment as ffi::DeviceSize;
	}

	pub fn min_uniform_buffer_offset_alignment(&self) -> ffi::DeviceSize {
		return self.raw.min_uniform_buffer_offset_alignment as ffi::DeviceSize;
	}

	pub fn min_storage_buffer_offset_alignment(&self) -> ffi::DeviceSize {
		return self.raw.min_storage_buffer_offset_alignment as ffi::DeviceSize;
	}

	pub fn min_texel_offset(&self) -> i32 {
		return self.raw.min_texel_offset as i32;
	}

	pub fn max_texel_offset(&self) -> u32 {
		return self.raw.max_texel_offset as u32;
	}

	pub fn min_texel_gather_offset(&self) -> i32 {
		return self.raw.min_texel_gather_offset as i32;
	}

	pub fn max_texel_gather_offset(&self) -> u32 {
		return self.raw.max_texel_gather_offset as u32;
	}

	pub fn min_interpolation_offset(&self) -> f32 {
		return self.raw.min_interpolation_offset as f32;
	}

	pub fn max_interpolation_offset(&self) -> f32 {
		return self.raw.max_interpolation_offset as f32;
	}

	pub fn sub_pixel_interpolation_offset_bits(&self) -> u32 {
		return self.raw.sub_pixel_interpolation_offset_bits as u32;
	}

	pub fn max_framebuffer_width(&self) -> u32 {
		return self.raw.max_framebuffer_width as u32;
	}

	pub fn max_framebuffer_height(&self) -> u32 {
		return self.raw.max_framebuffer_height as u32;
	}

	pub fn max_framebuffer_layers(&self) -> u32 {
		return self.raw.max_framebuffer_layers as u32;
	}

	pub fn framebuffer_color_sample_counts(&self) -> ffi::SampleCountFlags {
		return self.raw.framebuffer_color_sample_counts as ffi::SampleCountFlags;
	}

	pub fn framebuffer_depth_sample_counts(&self) -> ffi::SampleCountFlags {
		return self.raw.framebuffer_depth_sample_counts as ffi::SampleCountFlags;
	}

	pub fn framebuffer_stencil_sample_counts(&self) -> ffi::SampleCountFlags {
		return self.raw.framebuffer_stencil_sample_counts as ffi::SampleCountFlags;
	}

	pub fn framebuffer_no_attachments_sample_counts(&self) -> ffi::SampleCountFlags {
		return self.raw.framebuffer_no_attachments_sample_counts as ffi::SampleCountFlags;
	}

	pub fn max_color_attachments(&self) -> u32 {
		return self.raw.max_color_attachments as u32;
	}

	pub fn sampled_image_color_sample_counts(&self) -> ffi::SampleCountFlags {
		return self.raw.sampled_image_color_sample_counts as ffi::SampleCountFlags;
	}

	pub fn sampled_image_integer_sample_counts(&self) -> ffi::SampleCountFlags {
		return self.raw.sampled_image_integer_sample_counts as ffi::SampleCountFlags;
	}

	pub fn sampled_image_depth_sample_counts(&self) -> ffi::SampleCountFlags {
		return self.raw.sampled_image_depth_sample_counts as ffi::SampleCountFlags;
	}

	pub fn sampled_image_stencil_sample_counts(&self) -> ffi::SampleCountFlags {
		return self.raw.sampled_image_stencil_sample_counts as ffi::SampleCountFlags;
	}

	pub fn storage_image_sample_counts(&self) -> ffi::SampleCountFlags {
		return self.raw.storage_image_sample_counts as ffi::SampleCountFlags;
	}

	pub fn max_sample_mask_words(&self) -> u32 {
		return self.raw.max_sample_mask_words as u32;
	}

	pub fn timestamp_compute_and_graphics(&self) -> ffi::Bool32 {
		return self.raw.timestamp_compute_and_graphics as ffi::Bool32;
	}

	pub fn timestamp_period(&self) -> f32 {
		return self.raw.timestamp_period as f32;
	}

	pub fn max_clip_distances(&self) -> u32 {
		return self.raw.max_clip_distances as u32;
	}

	pub fn max_cull_distances(&self) -> u32 {
		return self.raw.max_cull_distances as u32;
	}

	pub fn max_combined_clip_and_cull_distances(&self) -> u32 {
		return self.raw.max_combined_clip_and_cull_distances as u32;
	}

	pub fn discrete_queue_priorities(&self) -> u32 {
		return self.raw.discrete_queue_priorities as u32;
	}

	pub fn point_size_range(&self) -> f32 {
		return self.raw.point_size_range as f32;
	}

	pub fn line_width_range(&self) -> f32 {
		return self.raw.line_width_range as f32;
	}

	pub fn point_size_granularity(&self) -> f32 {
		return self.raw.point_size_granularity as f32;
	}

	pub fn line_width_granularity(&self) -> f32 {
		return self.raw.line_width_granularity as f32;
	}

	pub fn strict_lines(&self) -> ffi::Bool32 {
		return self.raw.strict_lines as ffi::Bool32;
	}

	pub fn standard_sample_locations(&self) -> ffi::Bool32 {
		return self.raw.standard_sample_locations as ffi::Bool32;
	}

	pub fn optimal_buffer_copy_offset_alignment(&self) -> ffi::DeviceSize {
		return self.raw.optimal_buffer_copy_offset_alignment as ffi::DeviceSize;
	}

	pub fn optimal_buffer_copy_row_pitch_alignment(&self) -> ffi::DeviceSize {
		return self.raw.optimal_buffer_copy_row_pitch_alignment as ffi::DeviceSize;
	}

	pub fn non_coherent_atom_size(&self) -> ffi::DeviceSize {
		return self.raw.non_coherent_atom_size as ffi::DeviceSize;
	}
}

pub struct SemaphoreCreateInfo {
	pub raw: ffi::SemaphoreCreateInfo
}

impl SemaphoreCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::SemaphoreCreateInfo) -> SemaphoreCreateInfo {
		return SemaphoreCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::SemaphoreCreateFlags {
		return self.raw.flags as ffi::SemaphoreCreateFlags;
	}
}

pub struct QueryPoolCreateInfo {
	pub raw: ffi::QueryPoolCreateInfo
}

impl QueryPoolCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::QueryPoolCreateInfo) -> QueryPoolCreateInfo {
		return QueryPoolCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::QueryPoolCreateFlags {
		return self.raw.flags as ffi::QueryPoolCreateFlags;
	}

	pub fn query_type(&self) -> ffi::QueryType {
		return self.raw.query_type as ffi::QueryType;
	}

	pub fn query_count(&self) -> u32 {
		return self.raw.query_count as u32;
	}

	pub fn pipeline_statistics(&self) -> ffi::QueryPipelineStatisticFlags {
		return self.raw.pipeline_statistics as ffi::QueryPipelineStatisticFlags;
	}
}

pub struct FramebufferCreateInfo {
	pub raw: ffi::FramebufferCreateInfo
}

impl FramebufferCreateInfo {
	pub unsafe fn from_ffi(raw: ffi::FramebufferCreateInfo) -> FramebufferCreateInfo {
		return FramebufferCreateInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::FramebufferCreateFlags {
		return self.raw.flags as ffi::FramebufferCreateFlags;
	}

	pub fn render_pass(&self) -> u64 {
		return self.raw.render_pass as u64;
	}

	pub fn attachment_count(&self) -> u32 {
		return self.raw.attachment_count as u32;
	}

	pub fn attachments(&self) -> &[u64] {
		return unsafe { std::slice::from_raw_parts(self.raw.attachments, self.attachment_count() as usize) };
	}

	pub fn width(&self) -> u32 {
		return self.raw.width as u32;
	}

	pub fn height(&self) -> u32 {
		return self.raw.height as u32;
	}

	pub fn layers(&self) -> u32 {
		return self.raw.layers as u32;
	}
}

pub struct DrawIndirectCommand {
	pub raw: ffi::DrawIndirectCommand
}

impl DrawIndirectCommand {
	pub unsafe fn from_ffi(raw: ffi::DrawIndirectCommand) -> DrawIndirectCommand {
		return DrawIndirectCommand { raw: raw };
	}

	pub fn vertex_count(&self) -> u32 {
		return self.raw.vertex_count as u32;
	}

	pub fn instance_count(&self) -> u32 {
		return self.raw.instance_count as u32;
	}

	pub fn first_vertex(&self) -> u32 {
		return self.raw.first_vertex as u32;
	}

	pub fn first_instance(&self) -> u32 {
		return self.raw.first_instance as u32;
	}
}

pub struct DrawIndexedIndirectCommand {
	pub raw: ffi::DrawIndexedIndirectCommand
}

impl DrawIndexedIndirectCommand {
	pub unsafe fn from_ffi(raw: ffi::DrawIndexedIndirectCommand) -> DrawIndexedIndirectCommand {
		return DrawIndexedIndirectCommand { raw: raw };
	}

	pub fn index_count(&self) -> u32 {
		return self.raw.index_count as u32;
	}

	pub fn instance_count(&self) -> u32 {
		return self.raw.instance_count as u32;
	}

	pub fn first_index(&self) -> u32 {
		return self.raw.first_index as u32;
	}

	pub fn vertex_offset(&self) -> i32 {
		return self.raw.vertex_offset as i32;
	}

	pub fn first_instance(&self) -> u32 {
		return self.raw.first_instance as u32;
	}
}

pub struct DispatchIndirectCommand {
	pub raw: ffi::DispatchIndirectCommand
}

impl DispatchIndirectCommand {
	pub unsafe fn from_ffi(raw: ffi::DispatchIndirectCommand) -> DispatchIndirectCommand {
		return DispatchIndirectCommand { raw: raw };
	}

	pub fn x(&self) -> u32 {
		return self.raw.x as u32;
	}

	pub fn y(&self) -> u32 {
		return self.raw.y as u32;
	}

	pub fn z(&self) -> u32 {
		return self.raw.z as u32;
	}
}

pub struct SubmitInfo {
	pub raw: ffi::SubmitInfo
}

impl SubmitInfo {
	pub unsafe fn from_ffi(raw: ffi::SubmitInfo) -> SubmitInfo {
		return SubmitInfo { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn wait_semaphore_count(&self) -> u32 {
		return self.raw.wait_semaphore_count as u32;
	}

	pub fn wait_semaphores(&self) -> &[u64] {
		return unsafe { std::slice::from_raw_parts(self.raw.wait_semaphores, self.wait_semaphore_count() as usize) };
	}

	pub fn wait_dst_stage_mask(&self) -> &[ffi::PipelineStageFlags] {
		return unsafe { std::slice::from_raw_parts(self.raw.wait_dst_stage_mask, self.wait_semaphore_count() as usize) };
	}

	pub fn command_buffer_count(&self) -> u32 {
		return self.raw.command_buffer_count as u32;
	}

	pub fn command_buffers(&self) -> &[libc::size_t] {
		return unsafe { std::slice::from_raw_parts(self.raw.command_buffers, self.command_buffer_count() as usize) };
	}

	pub fn signal_semaphore_count(&self) -> u32 {
		return self.raw.signal_semaphore_count as u32;
	}

	pub fn signal_semaphores(&self) -> &[u64] {
		return unsafe { std::slice::from_raw_parts(self.raw.signal_semaphores, self.signal_semaphore_count() as usize) };
	}
}

#[cfg(vk_khr_display)]
pub struct DisplayPropertiesKHR {
	pub raw: ffi::DisplayPropertiesKHR
}

#[cfg(vk_khr_display)]
impl DisplayPropertiesKHR {
	pub unsafe fn from_ffi(raw: ffi::DisplayPropertiesKHR) -> DisplayPropertiesKHR {
		return DisplayPropertiesKHR { raw: raw };
	}

	pub fn display(&self) -> u64 {
		return self.raw.display as u64;
	}

	pub fn display_name(&self) -> libc::c_char {
		return self.raw.display_name as libc::c_char;
	}

	pub fn physical_dimensions(&self) -> &ffi::Extent2D {
		return &self.raw.physical_dimensions;
	}

	pub fn physical_resolution(&self) -> &ffi::Extent2D {
		return &self.raw.physical_resolution;
	}

	pub fn supported_transforms(&self) -> ffi::SurfaceTransformFlagsKHR {
		return self.raw.supported_transforms as ffi::SurfaceTransformFlagsKHR;
	}

	pub fn plane_reorder_possible(&self) -> ffi::Bool32 {
		return self.raw.plane_reorder_possible as ffi::Bool32;
	}

	pub fn persistent_content(&self) -> ffi::Bool32 {
		return self.raw.persistent_content as ffi::Bool32;
	}
}

#[cfg(vk_khr_display)]
pub struct DisplayPlanePropertiesKHR {
	pub raw: ffi::DisplayPlanePropertiesKHR
}

#[cfg(vk_khr_display)]
impl DisplayPlanePropertiesKHR {
	pub unsafe fn from_ffi(raw: ffi::DisplayPlanePropertiesKHR) -> DisplayPlanePropertiesKHR {
		return DisplayPlanePropertiesKHR { raw: raw };
	}

	pub fn current_display(&self) -> u64 {
		return self.raw.current_display as u64;
	}

	pub fn current_stack_index(&self) -> u32 {
		return self.raw.current_stack_index as u32;
	}
}

#[cfg(vk_khr_display)]
pub struct DisplayModeParametersKHR {
	pub raw: ffi::DisplayModeParametersKHR
}

#[cfg(vk_khr_display)]
impl DisplayModeParametersKHR {
	pub unsafe fn from_ffi(raw: ffi::DisplayModeParametersKHR) -> DisplayModeParametersKHR {
		return DisplayModeParametersKHR { raw: raw };
	}

	pub fn visible_region(&self) -> &ffi::Extent2D {
		return &self.raw.visible_region;
	}

	pub fn refresh_rate(&self) -> u32 {
		return self.raw.refresh_rate as u32;
	}
}

#[cfg(vk_khr_display)]
pub struct DisplayModePropertiesKHR {
	pub raw: ffi::DisplayModePropertiesKHR
}

#[cfg(vk_khr_display)]
impl DisplayModePropertiesKHR {
	pub unsafe fn from_ffi(raw: ffi::DisplayModePropertiesKHR) -> DisplayModePropertiesKHR {
		return DisplayModePropertiesKHR { raw: raw };
	}

	pub fn display_mode(&self) -> u64 {
		return self.raw.display_mode as u64;
	}

	pub fn parameters(&self) -> &ffi::DisplayModeParametersKHR {
		return &self.raw.parameters;
	}
}

#[cfg(vk_khr_display)]
pub struct DisplayModeCreateInfoKHR {
	pub raw: ffi::DisplayModeCreateInfoKHR
}

#[cfg(vk_khr_display)]
impl DisplayModeCreateInfoKHR {
	pub unsafe fn from_ffi(raw: ffi::DisplayModeCreateInfoKHR) -> DisplayModeCreateInfoKHR {
		return DisplayModeCreateInfoKHR { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::DisplayModeCreateFlagsKHR {
		return self.raw.flags as ffi::DisplayModeCreateFlagsKHR;
	}

	pub fn parameters(&self) -> &ffi::DisplayModeParametersKHR {
		return &self.raw.parameters;
	}
}

#[cfg(vk_khr_display)]
pub struct DisplayPlaneCapabilitiesKHR {
	pub raw: ffi::DisplayPlaneCapabilitiesKHR
}

#[cfg(vk_khr_display)]
impl DisplayPlaneCapabilitiesKHR {
	pub unsafe fn from_ffi(raw: ffi::DisplayPlaneCapabilitiesKHR) -> DisplayPlaneCapabilitiesKHR {
		return DisplayPlaneCapabilitiesKHR { raw: raw };
	}

	pub fn supported_alpha(&self) -> ffi::DisplayPlaneAlphaFlagsKHR {
		return self.raw.supported_alpha as ffi::DisplayPlaneAlphaFlagsKHR;
	}

	pub fn min_src_position(&self) -> &ffi::Offset2D {
		return &self.raw.min_src_position;
	}

	pub fn max_src_position(&self) -> &ffi::Offset2D {
		return &self.raw.max_src_position;
	}

	pub fn min_src_extent(&self) -> &ffi::Extent2D {
		return &self.raw.min_src_extent;
	}

	pub fn max_src_extent(&self) -> &ffi::Extent2D {
		return &self.raw.max_src_extent;
	}

	pub fn min_dst_position(&self) -> &ffi::Offset2D {
		return &self.raw.min_dst_position;
	}

	pub fn max_dst_position(&self) -> &ffi::Offset2D {
		return &self.raw.max_dst_position;
	}

	pub fn min_dst_extent(&self) -> &ffi::Extent2D {
		return &self.raw.min_dst_extent;
	}

	pub fn max_dst_extent(&self) -> &ffi::Extent2D {
		return &self.raw.max_dst_extent;
	}
}

#[cfg(vk_khr_display)]
pub struct DisplaySurfaceCreateInfoKHR {
	pub raw: ffi::DisplaySurfaceCreateInfoKHR
}

#[cfg(vk_khr_display)]
impl DisplaySurfaceCreateInfoKHR {
	pub unsafe fn from_ffi(raw: ffi::DisplaySurfaceCreateInfoKHR) -> DisplaySurfaceCreateInfoKHR {
		return DisplaySurfaceCreateInfoKHR { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::DisplaySurfaceCreateFlagsKHR {
		return self.raw.flags as ffi::DisplaySurfaceCreateFlagsKHR;
	}

	pub fn display_mode(&self) -> u64 {
		return self.raw.display_mode as u64;
	}

	pub fn plane_index(&self) -> u32 {
		return self.raw.plane_index as u32;
	}

	pub fn plane_stack_index(&self) -> u32 {
		return self.raw.plane_stack_index as u32;
	}

	pub fn transform(&self) -> ffi::SurfaceTransformFlagsKHR {
		return self.raw.transform as ffi::SurfaceTransformFlagsKHR;
	}

	pub fn global_alpha(&self) -> f32 {
		return self.raw.global_alpha as f32;
	}

	pub fn alpha_mode(&self) -> ffi::DisplayPlaneAlphaFlagsKHR {
		return self.raw.alpha_mode as ffi::DisplayPlaneAlphaFlagsKHR;
	}

	pub fn image_extent(&self) -> &ffi::Extent2D {
		return &self.raw.image_extent;
	}
}

#[cfg(vk_khr_display_swapchain)]
pub struct DisplayPresentInfoKHR {
	pub raw: ffi::DisplayPresentInfoKHR
}

#[cfg(vk_khr_display_swapchain)]
impl DisplayPresentInfoKHR {
	pub unsafe fn from_ffi(raw: ffi::DisplayPresentInfoKHR) -> DisplayPresentInfoKHR {
		return DisplayPresentInfoKHR { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn src_rect(&self) -> &ffi::Rect2D {
		return &self.raw.src_rect;
	}

	pub fn dst_rect(&self) -> &ffi::Rect2D {
		return &self.raw.dst_rect;
	}

	pub fn persistent(&self) -> ffi::Bool32 {
		return self.raw.persistent as ffi::Bool32;
	}
}

pub struct SurfaceCapabilitiesKHR {
	pub raw: ffi::SurfaceCapabilitiesKHR
}

impl SurfaceCapabilitiesKHR {
	pub unsafe fn from_ffi(raw: ffi::SurfaceCapabilitiesKHR) -> SurfaceCapabilitiesKHR {
		return SurfaceCapabilitiesKHR { raw: raw };
	}

	pub fn min_image_count(&self) -> u32 {
		return self.raw.min_image_count as u32;
	}

	pub fn max_image_count(&self) -> u32 {
		return self.raw.max_image_count as u32;
	}

	pub fn current_extent(&self) -> &ffi::Extent2D {
		return &self.raw.current_extent;
	}

	pub fn min_image_extent(&self) -> &ffi::Extent2D {
		return &self.raw.min_image_extent;
	}

	pub fn max_image_extent(&self) -> &ffi::Extent2D {
		return &self.raw.max_image_extent;
	}

	pub fn max_image_array_layers(&self) -> u32 {
		return self.raw.max_image_array_layers as u32;
	}

	pub fn supported_transforms(&self) -> ffi::SurfaceTransformFlagsKHR {
		return self.raw.supported_transforms as ffi::SurfaceTransformFlagsKHR;
	}

	pub fn current_transform(&self) -> ffi::SurfaceTransformFlagsKHR {
		return self.raw.current_transform as ffi::SurfaceTransformFlagsKHR;
	}

	pub fn supported_composite_alpha(&self) -> ffi::CompositeAlphaFlagsKHR {
		return self.raw.supported_composite_alpha as ffi::CompositeAlphaFlagsKHR;
	}

	pub fn supported_usage_flags(&self) -> ffi::ImageUsageFlags {
		return self.raw.supported_usage_flags as ffi::ImageUsageFlags;
	}
}

#[cfg(vk_khr_android_surface)]
pub struct AndroidSurfaceCreateInfoKHR {
	pub raw: ffi::AndroidSurfaceCreateInfoKHR
}

#[cfg(vk_khr_android_surface)]
impl AndroidSurfaceCreateInfoKHR {
	pub unsafe fn from_ffi(raw: ffi::AndroidSurfaceCreateInfoKHR) -> AndroidSurfaceCreateInfoKHR {
		return AndroidSurfaceCreateInfoKHR { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::AndroidSurfaceCreateFlagsKHR {
		return self.raw.flags as ffi::AndroidSurfaceCreateFlagsKHR;
	}

	pub fn window(&self) -> libc::size_t {
		return self.raw.window as libc::size_t;
	}
}

#[cfg(vk_khr_mir_surface)]
pub struct MirSurfaceCreateInfoKHR {
	pub raw: ffi::MirSurfaceCreateInfoKHR
}

#[cfg(vk_khr_mir_surface)]
impl MirSurfaceCreateInfoKHR {
	pub unsafe fn from_ffi(raw: ffi::MirSurfaceCreateInfoKHR) -> MirSurfaceCreateInfoKHR {
		return MirSurfaceCreateInfoKHR { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::MirSurfaceCreateFlagsKHR {
		return self.raw.flags as ffi::MirSurfaceCreateFlagsKHR;
	}

	pub fn connection(&self) -> libc::size_t {
		return self.raw.connection as libc::size_t;
	}

	pub fn mir_surface(&self) -> libc::size_t {
		return self.raw.mir_surface as libc::size_t;
	}
}

#[cfg(vk_khr_wayland_surface)]
pub struct WaylandSurfaceCreateInfoKHR {
	pub raw: ffi::WaylandSurfaceCreateInfoKHR
}

#[cfg(vk_khr_wayland_surface)]
impl WaylandSurfaceCreateInfoKHR {
	pub unsafe fn from_ffi(raw: ffi::WaylandSurfaceCreateInfoKHR) -> WaylandSurfaceCreateInfoKHR {
		return WaylandSurfaceCreateInfoKHR { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::WaylandSurfaceCreateFlagsKHR {
		return self.raw.flags as ffi::WaylandSurfaceCreateFlagsKHR;
	}

	pub fn display(&self) -> libc::size_t {
		return self.raw.display as libc::size_t;
	}

	pub fn surface(&self) -> libc::size_t {
		return self.raw.surface as libc::size_t;
	}
}

#[cfg(vk_khr_win32_surface)]
pub struct Win32SurfaceCreateInfoKHR {
	pub raw: ffi::Win32SurfaceCreateInfoKHR
}

#[cfg(vk_khr_win32_surface)]
impl Win32SurfaceCreateInfoKHR {
	pub unsafe fn from_ffi(raw: ffi::Win32SurfaceCreateInfoKHR) -> Win32SurfaceCreateInfoKHR {
		return Win32SurfaceCreateInfoKHR { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::Win32SurfaceCreateFlagsKHR {
		return self.raw.flags as ffi::Win32SurfaceCreateFlagsKHR;
	}

	pub fn hinstance(&self) -> libc::size_t {
		return self.raw.hinstance as libc::size_t;
	}

	pub fn hwnd(&self) -> libc::size_t {
		return self.raw.hwnd as libc::size_t;
	}
}

#[cfg(vk_khr_xlib_surface)]
pub struct XlibSurfaceCreateInfoKHR {
	pub raw: ffi::XlibSurfaceCreateInfoKHR
}

#[cfg(vk_khr_xlib_surface)]
impl XlibSurfaceCreateInfoKHR {
	pub unsafe fn from_ffi(raw: ffi::XlibSurfaceCreateInfoKHR) -> XlibSurfaceCreateInfoKHR {
		return XlibSurfaceCreateInfoKHR { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::XlibSurfaceCreateFlagsKHR {
		return self.raw.flags as ffi::XlibSurfaceCreateFlagsKHR;
	}

	pub fn dpy(&self) -> libc::size_t {
		return self.raw.dpy as libc::size_t;
	}

	pub fn window(&self) -> libc::size_t {
		return self.raw.window as libc::size_t;
	}
}

#[cfg(vk_khr_xcb_surface)]
pub struct XcbSurfaceCreateInfoKHR {
	pub raw: ffi::XcbSurfaceCreateInfoKHR
}

#[cfg(vk_khr_xcb_surface)]
impl XcbSurfaceCreateInfoKHR {
	pub unsafe fn from_ffi(raw: ffi::XcbSurfaceCreateInfoKHR) -> XcbSurfaceCreateInfoKHR {
		return XcbSurfaceCreateInfoKHR { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::XcbSurfaceCreateFlagsKHR {
		return self.raw.flags as ffi::XcbSurfaceCreateFlagsKHR;
	}

	pub fn connection(&self) -> libc::size_t {
		return self.raw.connection as libc::size_t;
	}

	pub fn window(&self) -> libc::size_t {
		return self.raw.window as libc::size_t;
	}
}

pub struct SurfaceFormatKHR {
	pub raw: ffi::SurfaceFormatKHR
}

impl SurfaceFormatKHR {
	pub unsafe fn from_ffi(raw: ffi::SurfaceFormatKHR) -> SurfaceFormatKHR {
		return SurfaceFormatKHR { raw: raw };
	}

	pub fn format(&self) -> ffi::Format {
		return self.raw.format as ffi::Format;
	}

	pub fn color_space(&self) -> ffi::ColorSpaceKHR {
		return self.raw.color_space as ffi::ColorSpaceKHR;
	}
}

pub struct SwapchainCreateInfoKHR {
	pub raw: ffi::SwapchainCreateInfoKHR
}

impl SwapchainCreateInfoKHR {
	pub unsafe fn from_ffi(raw: ffi::SwapchainCreateInfoKHR) -> SwapchainCreateInfoKHR {
		return SwapchainCreateInfoKHR { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::SwapchainCreateFlagsKHR {
		return self.raw.flags as ffi::SwapchainCreateFlagsKHR;
	}

	pub fn surface(&self) -> u64 {
		return self.raw.surface as u64;
	}

	pub fn min_image_count(&self) -> u32 {
		return self.raw.min_image_count as u32;
	}

	pub fn image_format(&self) -> ffi::Format {
		return self.raw.image_format as ffi::Format;
	}

	pub fn image_color_space(&self) -> ffi::ColorSpaceKHR {
		return self.raw.image_color_space as ffi::ColorSpaceKHR;
	}

	pub fn image_extent(&self) -> &ffi::Extent2D {
		return &self.raw.image_extent;
	}

	pub fn image_array_layers(&self) -> u32 {
		return self.raw.image_array_layers as u32;
	}

	pub fn image_usage(&self) -> ffi::ImageUsageFlags {
		return self.raw.image_usage as ffi::ImageUsageFlags;
	}

	pub fn image_sharing_mode(&self) -> ffi::SharingMode {
		return self.raw.image_sharing_mode as ffi::SharingMode;
	}

	pub fn queue_family_index_count(&self) -> u32 {
		return self.raw.queue_family_index_count as u32;
	}

	pub fn queue_family_indices(&self) -> &[u32] {
		return unsafe { std::slice::from_raw_parts(self.raw.queue_family_indices, self.queue_family_index_count() as usize) };
	}

	pub fn pre_transform(&self) -> ffi::SurfaceTransformFlagsKHR {
		return self.raw.pre_transform as ffi::SurfaceTransformFlagsKHR;
	}

	pub fn composite_alpha(&self) -> ffi::CompositeAlphaFlagsKHR {
		return self.raw.composite_alpha as ffi::CompositeAlphaFlagsKHR;
	}

	pub fn present_mode(&self) -> ffi::PresentModeKHR {
		return self.raw.present_mode as ffi::PresentModeKHR;
	}

	pub fn clipped(&self) -> ffi::Bool32 {
		return self.raw.clipped as ffi::Bool32;
	}

	pub fn old_swapchain(&self) -> u64 {
		return self.raw.old_swapchain as u64;
	}
}

pub struct PresentInfoKHR {
	pub raw: ffi::PresentInfoKHR
}

impl PresentInfoKHR {
	pub unsafe fn from_ffi(raw: ffi::PresentInfoKHR) -> PresentInfoKHR {
		return PresentInfoKHR { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn wait_semaphore_count(&self) -> u32 {
		return self.raw.wait_semaphore_count as u32;
	}

	pub fn wait_semaphores(&self) -> &[u64] {
		return unsafe { std::slice::from_raw_parts(self.raw.wait_semaphores, self.wait_semaphore_count() as usize) };
	}

	pub fn swapchain_count(&self) -> u32 {
		return self.raw.swapchain_count as u32;
	}

	pub fn swapchains(&self) -> &[u64] {
		return unsafe { std::slice::from_raw_parts(self.raw.swapchains, self.swapchain_count() as usize) };
	}

	pub fn image_indices(&self) -> &[u32] {
		return unsafe { std::slice::from_raw_parts(self.raw.image_indices, self.swapchain_count() as usize) };
	}

	pub fn results(&self) -> &[ffi::Result] {
		return unsafe { std::slice::from_raw_parts(self.raw.results, self.swapchain_count() as usize) };
	}
}

pub struct DebugReportCallbackCreateInfoEXT {
	pub raw: ffi::DebugReportCallbackCreateInfoEXT
}

impl DebugReportCallbackCreateInfoEXT {
	pub unsafe fn from_ffi(raw: ffi::DebugReportCallbackCreateInfoEXT) -> DebugReportCallbackCreateInfoEXT {
		return DebugReportCallbackCreateInfoEXT { raw: raw };
	}

	pub fn structure_ty(&self) -> ffi::StructureType {
		return self.raw.structure_ty as ffi::StructureType;
	}

	pub fn next(&self) -> &libc::c_void {
		return unsafe { &*self.raw.next as &libc::c_void };
	}

	pub fn flags(&self) -> ffi::DebugReportFlagsEXT {
		return self.raw.flags as ffi::DebugReportFlagsEXT;
	}

	pub fn callback(&self) -> &ffi::DebugReportCallbackEXTCallback {
		return unsafe { &*self.raw.callback as &ffi::DebugReportCallbackEXTCallback };
	}

	pub fn user_data(&self) -> &libc::c_void {
		return unsafe { &*self.raw.user_data as &libc::c_void };
	}
}
