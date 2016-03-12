use libc;

pub type SampleMask = u32;
pub type Bool32 = u32;
pub type Flags = u32;
pub type DeviceSize = u64;
pub type FramebufferCreateFlags = Flags;
pub type QueryPoolCreateFlags = Flags;
pub type RenderPassCreateFlags = Flags;
pub type SamplerCreateFlags = Flags;
pub type PipelineLayoutCreateFlags = Flags;
pub type PipelineCacheCreateFlags = Flags;
pub type PipelineDepthStencilStateCreateFlags = Flags;
pub type PipelineDynamicStateCreateFlags = Flags;
pub type PipelineColorBlendStateCreateFlags = Flags;
pub type PipelineMultisampleStateCreateFlags = Flags;
pub type PipelineRasterizationStateCreateFlags = Flags;
pub type PipelineViewportStateCreateFlags = Flags;
pub type PipelineTessellationStateCreateFlags = Flags;
pub type PipelineInputAssemblyStateCreateFlags = Flags;
pub type PipelineVertexInputStateCreateFlags = Flags;
pub type PipelineShaderStageCreateFlags = Flags;
pub type DescriptorSetLayoutCreateFlags = Flags;
pub type BufferViewCreateFlags = Flags;
pub type InstanceCreateFlags = Flags;
pub type DeviceCreateFlags = Flags;
pub type DeviceQueueCreateFlags = Flags;
pub type ImageViewCreateFlags = Flags;
pub type SemaphoreCreateFlags = Flags;
pub type ShaderModuleCreateFlags = Flags;
pub type EventCreateFlags = Flags;
pub type MemoryMapFlags = Flags;
pub type SubpassDescriptionFlags = Flags;
pub type DescriptorPoolResetFlags = Flags;
pub type SwapchainCreateFlagsKHR = Flags;
pub type DisplayModeCreateFlagsKHR = Flags;
pub type DisplaySurfaceCreateFlagsKHR = Flags;
pub type AndroidSurfaceCreateFlagsKHR = Flags;
pub type MirSurfaceCreateFlagsKHR = Flags;
pub type WaylandSurfaceCreateFlagsKHR = Flags;
pub type Win32SurfaceCreateFlagsKHR = Flags;
pub type XlibSurfaceCreateFlagsKHR = Flags;
pub type XcbSurfaceCreateFlagsKHR = Flags;
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: libc::size_t = 256;
pub const UUID_SIZE: libc::size_t = 16;
pub const MAX_EXTENSION_NAME_SIZE: libc::size_t = 256;
pub const MAX_DESCRIPTION_SIZE: libc::size_t = 256;
pub const MAX_MEMORY_TYPES: libc::size_t = 32;
pub const MAX_MEMORY_HEAPS: libc::size_t = 16;
pub const LOD_CLAMP_NONE: f32 = 1000.0f32;
pub const REMAINING_MIP_LEVELS: libc::size_t = usize::max_value();
pub const REMAINING_ARRAY_LAYERS: libc::size_t = usize::max_value();
pub const WHOLE_SIZE: libc::size_t = usize::max_value();
pub const ATTACHMENT_UNUSED: libc::size_t = usize::max_value();
pub const TRUE: libc::size_t = 1;
pub const FALSE: libc::size_t = 0;
pub const QUEUE_FAMILY_IGNORED: libc::size_t = usize::max_value();
pub const SUBPASS_EXTERNAL: libc::size_t = usize::max_value();
pub type InternalAllocationNotificationCallback = libc::size_t;
pub type InternalFreeNotificationCallback = libc::size_t;
pub type ReallocationFunctionCallback = libc::size_t;
pub type AllocationFunctionCallback = libc::size_t;
pub type FreeFunctionCallback = libc::size_t;
pub type VoidFunctionCallback = libc::size_t;
pub type DebugReportCallbackEXTCallback = libc::size_t;
pub type Instance = libc::size_t;
pub type PhysicalDevice = libc::size_t;
pub type Device = libc::size_t;
pub type Queue = libc::size_t;
pub type CommandBuffer = libc::size_t;
pub type DeviceMemory = u64;
pub type CommandPool = u64;
pub type Buffer = u64;
pub type BufferView = u64;
pub type Image = u64;
pub type ImageView = u64;
pub type ShaderModule = u64;
pub type Pipeline = u64;
pub type PipelineLayout = u64;
pub type Sampler = u64;
pub type DescriptorSet = u64;
pub type DescriptorSetLayout = u64;
pub type DescriptorPool = u64;
pub type Fence = u64;
pub type Semaphore = u64;
pub type Event = u64;
pub type QueryPool = u64;
pub type Framebuffer = u64;
pub type RenderPass = u64;
pub type PipelineCache = u64;
pub type DisplayKHR = u64;
pub type DisplayModeKHR = u64;
pub type SurfaceKHR = u64;
pub type SwapchainKHR = u64;
pub type DebugReportCallbackEXT = u64;

bitflags! {
	#[repr(C)]
	flags QueueFlags: Flags {
		const QUEUE_GRAPHICS_BIT = 1, // Queue supports graphics operations
		const QUEUE_COMPUTE_BIT = 2, // Queue supports compute operations
		const QUEUE_TRANSFER_BIT = 4, // Queue supports transfer operations
		const QUEUE_SPARSE_BINDING_BIT = 8, // Queue supports sparse resource memory management operations
	}
}

bitflags! {
	#[repr(C)]
	flags MemoryPropertyFlags: Flags {
		const MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 1, // If otherwise stated, then allocate memory on device
		const MEMORY_PROPERTY_HOST_VISIBLE_BIT = 2, // Memory is mappable by host
		const MEMORY_PROPERTY_HOST_COHERENT_BIT = 4, // Memory will have i/o coherency. If not set, application may need to use vkFlushMappedMemoryRanges and vkInvalidateMappedMemoryRanges to flush/invalidate host cache
		const MEMORY_PROPERTY_HOST_CACHED_BIT = 8, // Memory will be cached by the host
		const MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 16, // Memory may be allocated by the driver when it is required
	}
}

bitflags! {
	#[repr(C)]
	flags MemoryHeapFlags: Flags {
		const MEMORY_HEAP_DEVICE_LOCAL_BIT = 1, // If set, heap represents device memory
	}
}

bitflags! {
	#[repr(C)]
	flags AccessFlags: Flags {
		const ACCESS_INDIRECT_COMMAND_READ_BIT = 1, // Controls coherency of indirect command reads
		const ACCESS_INDEX_READ_BIT = 2, // Controls coherency of index reads
		const ACCESS_VERTEX_ATTRIBUTE_READ_BIT = 4, // Controls coherency of vertex attribute reads
		const ACCESS_UNIFORM_READ_BIT = 8, // Controls coherency of uniform buffer reads
		const ACCESS_INPUT_ATTACHMENT_READ_BIT = 16, // Controls coherency of input attachment reads
		const ACCESS_SHADER_READ_BIT = 32, // Controls coherency of shader reads
		const ACCESS_SHADER_WRITE_BIT = 64, // Controls coherency of shader writes
		const ACCESS_COLOR_ATTACHMENT_READ_BIT = 128, // Controls coherency of color attachment reads
		const ACCESS_COLOR_ATTACHMENT_WRITE_BIT = 256, // Controls coherency of color attachment writes
		const ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT = 512, // Controls coherency of depth/stencil attachment reads
		const ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 1024, // Controls coherency of depth/stencil attachment writes
		const ACCESS_TRANSFER_READ_BIT = 2048, // Controls coherency of transfer reads
		const ACCESS_TRANSFER_WRITE_BIT = 4096, // Controls coherency of transfer writes
		const ACCESS_HOST_READ_BIT = 8192, // Controls coherency of host reads
		const ACCESS_HOST_WRITE_BIT = 16384, // Controls coherency of host writes
		const ACCESS_MEMORY_READ_BIT = 32768, // Controls coherency of memory reads
		const ACCESS_MEMORY_WRITE_BIT = 65536, // Controls coherency of memory writes
	}
}

bitflags! {
	#[repr(C)]
	flags BufferUsageFlags: Flags {
		const BUFFER_USAGE_TRANSFER_SRC_BIT = 1, // Can be used as a source of transfer operations
		const BUFFER_USAGE_TRANSFER_DST_BIT = 2, // Can be used as a destination of transfer operations
		const BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = 4, // Can be used as TBO
		const BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = 8, // Can be used as IBO
		const BUFFER_USAGE_UNIFORM_BUFFER_BIT = 16, // Can be used as UBO
		const BUFFER_USAGE_STORAGE_BUFFER_BIT = 32, // Can be used as SSBO
		const BUFFER_USAGE_INDEX_BUFFER_BIT = 64, // Can be used as source of fixed-function index fetch (index buffer)
		const BUFFER_USAGE_VERTEX_BUFFER_BIT = 128, // Can be used as source of fixed-function vertex fetch (VBO)
		const BUFFER_USAGE_INDIRECT_BUFFER_BIT = 256, // Can be the source of indirect parameters (e.g. indirect buffer, parameter buffer)
	}
}

bitflags! {
	#[repr(C)]
	flags BufferCreateFlags: Flags {
		const BUFFER_CREATE_SPARSE_BINDING_BIT = 1, // Buffer should support sparse backing
		const BUFFER_CREATE_SPARSE_RESIDENCY_BIT = 2, // Buffer should support sparse backing with partial residency
		const BUFFER_CREATE_SPARSE_ALIASED_BIT = 4, // Buffer should support constent data access to physical memory blocks mapped into multiple locations of sparse buffers
	}
}

bitflags! {
	#[repr(C)]
	flags ShaderStageFlags: Flags {
		const SHADER_STAGE_VERTEX_BIT = 1,
		const SHADER_STAGE_TESSELLATION_CONTROL_BIT = 2,
		const SHADER_STAGE_TESSELLATION_EVALUATION_BIT = 4,
		const SHADER_STAGE_GEOMETRY_BIT = 8,
		const SHADER_STAGE_FRAGMENT_BIT = 16,
		const SHADER_STAGE_COMPUTE_BIT = 32,
		const SHADER_STAGE_ALL_GRAPHICS = 0x0000001F,
		const SHADER_STAGE_ALL = 0x7FFFFFFF,
	}
}

bitflags! {
	#[repr(C)]
	flags ImageUsageFlags: Flags {
		const IMAGE_USAGE_TRANSFER_SRC_BIT = 1, // Can be used as a source of transfer operations
		const IMAGE_USAGE_TRANSFER_DST_BIT = 2, // Can be used as a destination of transfer operations
		const IMAGE_USAGE_SAMPLED_BIT = 4, // Can be sampled from (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)
		const IMAGE_USAGE_STORAGE_BIT = 8, // Can be used as storage image (STORAGE_IMAGE descriptor type)
		const IMAGE_USAGE_COLOR_ATTACHMENT_BIT = 16, // Can be used as framebuffer color attachment
		const IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = 32, // Can be used as framebuffer depth/stencil attachment
		const IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = 64, // Image data not needed outside of rendering
		const IMAGE_USAGE_INPUT_ATTACHMENT_BIT = 128, // Can be used as framebuffer input attachment
	}
}

bitflags! {
	#[repr(C)]
	flags ImageCreateFlags: Flags {
		const IMAGE_CREATE_SPARSE_BINDING_BIT = 1, // Image should support sparse backing
		const IMAGE_CREATE_SPARSE_RESIDENCY_BIT = 2, // Image should support sparse backing with partial residency
		const IMAGE_CREATE_SPARSE_ALIASED_BIT = 4, // Image should support constent data access to physical memory blocks mapped into multiple locations of sparse images
		const IMAGE_CREATE_MUTABLE_FORMAT_BIT = 8, // Allows image views to have different format than the base image
		const IMAGE_CREATE_CUBE_COMPATIBLE_BIT = 16, // Allows creating image views with cube type from the created image
	}
}

bitflags! {
	#[repr(C)]
	flags PipelineCreateFlags: Flags {
		const PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = 1,
		const PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = 2,
		const PIPELINE_CREATE_DERIVATIVE_BIT = 4,
	}
}

bitflags! {
	#[repr(C)]
	flags ColorComponentFlags: Flags {
		const COLOR_COMPONENT_R_BIT = 1,
		const COLOR_COMPONENT_G_BIT = 2,
		const COLOR_COMPONENT_B_BIT = 4,
		const COLOR_COMPONENT_A_BIT = 8,
	}
}

bitflags! {
	#[repr(C)]
	flags FenceCreateFlags: Flags {
		const FENCE_CREATE_SIGNALED_BIT = 1,
	}
}

bitflags! {
	#[repr(C)]
	flags FormatFeatureFlags: Flags {
		const FORMAT_FEATURE_SAMPLED_IMAGE_BIT = 1, // Format can be used for sampled images (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)
		const FORMAT_FEATURE_STORAGE_IMAGE_BIT = 2, // Format can be used for storage images (STORAGE_IMAGE descriptor type)
		const FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT = 4, // Format supports atomic operations in case it's used for storage images
		const FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT = 8, // Format can be used for uniform texel buffers (TBOs)
		const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT = 16, // Format can be used for storage texel buffers (IBOs)
		const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 32, // Format supports atomic operations in case it's used for storage texel buffers
		const FORMAT_FEATURE_VERTEX_BUFFER_BIT = 64, // Format can be used for vertex buffers (VBOs)
		const FORMAT_FEATURE_COLOR_ATTACHMENT_BIT = 128, // Format can be used for color attachment images
		const FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT = 256, // Format supports blending in case it's used for color attachment images
		const FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT = 512, // Format can be used for depth/stencil attachment images
		const FORMAT_FEATURE_BLIT_SRC_BIT = 1024, // Format can be used as the source image of blits with vkCmdBlitImage
		const FORMAT_FEATURE_BLIT_DST_BIT = 2048, // Format can be used as the destination image of blits with vkCmdBlitImage
		const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT = 4096, // Format can be filtered with VK_FILTER_LINEAR when being sampled
	}
}

bitflags! {
	#[repr(C)]
	flags QueryControlFlags: Flags {
		const QUERY_CONTROL_PRECISE_BIT = 1, // Require precise results to be collected by the query
	}
}

bitflags! {
	#[repr(C)]
	flags QueryResultFlags: Flags {
		const QUERY_RESULT_64_BIT = 1, // Results of the queries are written to the destination buffer as 64-bit values
		const QUERY_RESULT_WAIT_BIT = 2, // Results of the queries are waited on before proceeding with the result copy
		const QUERY_RESULT_WITH_AVAILABILITY_BIT = 4, // Besides the results of the query, the availability of the results is also written
		const QUERY_RESULT_PARTIAL_BIT = 8, // Copy the partial results of the query even if the final results aren't available
	}
}

bitflags! {
	#[repr(C)]
	flags CommandPoolCreateFlags: Flags {
		const COMMAND_POOL_CREATE_TRANSIENT_BIT = 1, // Command buffers have a short lifetime
		const COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = 2, // Command buffers may release their memory individually
	}
}

bitflags! {
	#[repr(C)]
	flags CommandPoolResetFlags: Flags {
		const COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = 1, // Release resources owned by the pool
	}
}

bitflags! {
	#[repr(C)]
	flags CommandBufferResetFlags: Flags {
		const COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = 1, // Release resources owned by the buffer
	}
}

bitflags! {
	#[repr(C)]
	flags CommandBufferUsageFlags: Flags {
		const COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = 1,
		const COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = 2,
		const COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = 4, // Command buffer may be submitted/executed more than once simultaneously
	}
}

bitflags! {
	#[repr(C)]
	flags QueryPipelineStatisticFlags: Flags {
		const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT = 1, // Optional
		const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT = 2, // Optional
		const QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT = 4, // Optional
		const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT = 8, // Optional
		const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT = 16, // Optional
		const QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT = 32, // Optional
		const QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT = 64, // Optional
		const QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT = 128, // Optional
		const QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 256, // Optional
		const QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 512, // Optional
		const QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT = 1024, // Optional
	}
}

bitflags! {
	#[repr(C)]
	flags ImageAspectFlags: Flags {
		const IMAGE_ASPECT_COLOR_BIT = 1,
		const IMAGE_ASPECT_DEPTH_BIT = 2,
		const IMAGE_ASPECT_STENCIL_BIT = 4,
		const IMAGE_ASPECT_METADATA_BIT = 8,
	}
}

bitflags! {
	#[repr(C)]
	flags SparseMemoryBindFlags: Flags {
		const SPARSE_MEMORY_BIND_METADATA_BIT = 1, // Operation binds resource metadata to memory
	}
}

bitflags! {
	#[repr(C)]
	flags SparseImageFormatFlags: Flags {
		const SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT = 1, // Image uses a single miptail region for all array layers
		const SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT = 2, // Image requires mip levels to be an exact multiple of the sparse image block size for non-miptail levels.
		const SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT = 4, // Image uses a non-standard sparse block size
	}
}

bitflags! {
	#[repr(C)]
	flags PipelineStageFlags: Flags {
		const PIPELINE_STAGE_TOP_OF_PIPE_BIT = 1, // Before subsequent commands are processed
		const PIPELINE_STAGE_DRAW_INDIRECT_BIT = 2, // Draw/DispatchIndirect command fetch
		const PIPELINE_STAGE_VERTEX_INPUT_BIT = 4, // Vertex/index fetch
		const PIPELINE_STAGE_VERTEX_SHADER_BIT = 8, // Vertex shading
		const PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT = 16, // Tessellation control shading
		const PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT = 32, // Tessellation evaluation shading
		const PIPELINE_STAGE_GEOMETRY_SHADER_BIT = 64, // Geometry shading
		const PIPELINE_STAGE_FRAGMENT_SHADER_BIT = 128, // Fragment shading
		const PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT = 256, // Early fragment (depth and stencil) tests
		const PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT = 512, // Late fragment (depth and stencil) tests
		const PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT = 1024, // Color attachment writes
		const PIPELINE_STAGE_COMPUTE_SHADER_BIT = 2048, // Compute shading
		const PIPELINE_STAGE_TRANSFER_BIT = 4096, // Transfer/copy operations
		const PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT = 8192, // After previous commands have completed
		const PIPELINE_STAGE_HOST_BIT = 16384, // Indicates host (CPU) is a source/sink of the dependency
		const PIPELINE_STAGE_ALL_GRAPHICS_BIT = 32768, // All stages of the graphics pipeline
		const PIPELINE_STAGE_ALL_COMMANDS_BIT = 65536, // All stages supported on the queue
	}
}

bitflags! {
	#[repr(C)]
	flags SampleCountFlags: Flags {
		const SAMPLE_COUNT_1_BIT = 1, // Sample count 1 supported
		const SAMPLE_COUNT_2_BIT = 2, // Sample count 2 supported
		const SAMPLE_COUNT_4_BIT = 4, // Sample count 4 supported
		const SAMPLE_COUNT_8_BIT = 8, // Sample count 8 supported
		const SAMPLE_COUNT_16_BIT = 16, // Sample count 16 supported
		const SAMPLE_COUNT_32_BIT = 32, // Sample count 32 supported
		const SAMPLE_COUNT_64_BIT = 64, // Sample count 64 supported
	}
}

bitflags! {
	#[repr(C)]
	flags AttachmentDescriptionFlags: Flags {
		const ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = 1, // The attachment may alias physical memory of another attachment in the same render pass
	}
}

bitflags! {
	#[repr(C)]
	flags StencilFaceFlags: Flags {
		const STENCIL_FACE_FRONT_BIT = 1, // Front face
		const STENCIL_FACE_BACK_BIT = 2, // Back face
		const STENCIL_FRONT_AND_BACK = 0x00000003, // Front and back faces
	}
}

bitflags! {
	#[repr(C)]
	flags CullModeFlags: Flags {
		const CULL_MODE_NONE = 0,
		const CULL_MODE_FRONT_BIT = 1,
		const CULL_MODE_BACK_BIT = 2,
		const CULL_MODE_FRONT_AND_BACK = 0x00000003,
	}
}

bitflags! {
	#[repr(C)]
	flags DescriptorPoolCreateFlags: Flags {
		const DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = 1, // Descriptor sets may be freed individually
	}
}

bitflags! {
	#[repr(C)]
	flags DependencyFlags: Flags {
		const DEPENDENCY_BY_REGION_BIT = 1, // Dependency is per pixel region 
	}
}

bitflags! {
	#[repr(C)]
	flags CompositeAlphaFlagsKHR: Flags {
		const COMPOSITE_ALPHA_OPAQUE_BIT_KHR = 1,
		const COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = 2,
		const COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = 4,
		const COMPOSITE_ALPHA_INHERIT_BIT_KHR = 8,
	}
}

bitflags! {
	#[repr(C)]
	flags DisplayPlaneAlphaFlagsKHR: Flags {
		const DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = 1,
		const DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = 2,
		const DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = 4,
		const DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = 8,
	}
}

bitflags! {
	#[repr(C)]
	flags SurfaceTransformFlagsKHR: Flags {
		const SURFACE_TRANSFORM_IDENTITY_BIT_KHR = 1,
		const SURFACE_TRANSFORM_ROTATE_90_BIT_KHR = 2,
		const SURFACE_TRANSFORM_ROTATE_180_BIT_KHR = 4,
		const SURFACE_TRANSFORM_ROTATE_270_BIT_KHR = 8,
		const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR = 16,
		const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 32,
		const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 64,
		const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 128,
		const SURFACE_TRANSFORM_INHERIT_BIT_KHR = 256,
	}
}

bitflags! {
	#[repr(C)]
	flags DebugReportFlagsEXT: Flags {
		const DEBUG_REPORT_INFORMATION_BIT_EXT = 1,
		const DEBUG_REPORT_WARNING_BIT_EXT = 2,
		const DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT = 4,
		const DEBUG_REPORT_ERROR_BIT_EXT = 8,
		const DEBUG_REPORT_DEBUG_BIT_EXT = 16,
	}
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageLayout {
	Undefined = 0, // Implicit layout an image is when its contents are undefined due to various reasons (e.g. right after creation)
	General = 1, // General layout when image can be used for any kind of access
	ColorAttachmentOptimal = 2, // Optimal layout when image is only used for color attachment read/write
	DepthStencilAttachmentOptimal = 3, // Optimal layout when image is only used for depth/stencil attachment read/write
	DepthStencilReadOnlyOptimal = 4, // Optimal layout when image is used for read only depth/stencil attachment and shader access
	ShaderReadOnlyOptimal = 5, // Optimal layout when image is used for read only shader access
	TransferSrcOptimal = 6, // Optimal layout when image is used only as source of transfer operations
	TransferDstOptimal = 7, // Optimal layout when image is used only as destination of transfer operations
	Preinitialized = 8, // Initial layout used when the data is populated by the CPU
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttachmentLoadOp {
	Load = 0,
	Clear = 1,
	DontCare = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttachmentStoreOp {
	Store = 0,
	DontCare = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageType {
	ImageType1d = 0,
	ImageType2d = 1,
	ImageType3d = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageTiling {
	Optimal = 0,
	Linear = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageViewType {
	ImageViewType1d = 0,
	ImageViewType2d = 1,
	ImageViewType3d = 2,
	ImageViewTypeCube = 3,
	ImageViewType1dArray = 4,
	ImageViewType2dArray = 5,
	ImageViewTypeCubeArray = 6,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CommandBufferLevel {
	Primary = 0,
	Secondary = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComponentSwizzle {
	Identity = 0,
	Zero = 1,
	One = 2,
	R = 3,
	G = 4,
	B = 5,
	A = 6,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DescriptorType {
	Sampler = 0,
	CombinedImageSampler = 1,
	SampledImage = 2,
	StorageImage = 3,
	UniformTexelBuffer = 4,
	StorageTexelBuffer = 5,
	UniformBuffer = 6,
	StorageBuffer = 7,
	UniformBufferDynamic = 8,
	StorageBufferDynamic = 9,
	InputAttachment = 10,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QueryType {
	Occlusion = 0,
	PipelineStatistics = 1, // Optional
	Timestamp = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BorderColor {
	FloatTransparentBlack = 0,
	IntTransparentBlack = 1,
	FloatOpaqueBlack = 2,
	IntOpaqueBlack = 3,
	FloatOpaqueWhite = 4,
	IntOpaqueWhite = 5,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PipelineBindPoint {
	Graphics = 0,
	Compute = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PipelineCacheHeaderVersion {
	One = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrimitiveTopology {
	PointList = 0,
	LineList = 1,
	LineStrip = 2,
	TriangleList = 3,
	TriangleStrip = 4,
	TriangleFan = 5,
	LineListWithAdjacency = 6,
	LineStripWithAdjacency = 7,
	TriangleListWithAdjacency = 8,
	TriangleStripWithAdjacency = 9,
	PatchList = 10,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SharingMode {
	Exclusive = 0,
	Concurrent = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IndexType {
	Uint16 = 0,
	Uint32 = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Filter {
	Nearest = 0,
	Linear = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SamplerMipmapMode {
	Nearest = 0, // Choose nearest mip level
	Linear = 1, // Linear filter between mip levels
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SamplerAddressMode {
	Repeat = 0,
	MirroredRepeat = 1,
	ClampToEdge = 2,
	ClampToBorder = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CompareOp {
	Never = 0,
	Less = 1,
	Equal = 2,
	LessOrEqual = 3,
	Greater = 4,
	NotEqual = 5,
	GreaterOrEqual = 6,
	Always = 7,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PolygonMode {
	Fill = 0,
	Line = 1,
	Point = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrontFace {
	CounterClockwise = 0,
	Clockwise = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlendFactor {
	Zero = 0,
	One = 1,
	SrcColor = 2,
	OneMinusSrcColor = 3,
	DstColor = 4,
	OneMinusDstColor = 5,
	SrcAlpha = 6,
	OneMinusSrcAlpha = 7,
	DstAlpha = 8,
	OneMinusDstAlpha = 9,
	ConstantColor = 10,
	OneMinusConstantColor = 11,
	ConstantAlpha = 12,
	OneMinusConstantAlpha = 13,
	SrcAlphaSaturate = 14,
	Src1Color = 15,
	OneMinusSrc1Color = 16,
	Src1Alpha = 17,
	OneMinusSrc1Alpha = 18,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlendOp {
	Add = 0,
	Subtract = 1,
	ReverseSubtract = 2,
	Min = 3,
	Max = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StencilOp {
	Keep = 0,
	Zero = 1,
	Replace = 2,
	IncrementAndClamp = 3,
	DecrementAndClamp = 4,
	Invert = 5,
	IncrementAndWrap = 6,
	DecrementAndWrap = 7,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogicOp {
	Clear = 0,
	And = 1,
	AndReverse = 2,
	Copy = 3,
	AndInverted = 4,
	NoOp = 5,
	Xor = 6,
	Or = 7,
	Nor = 8,
	Equivalent = 9,
	Invert = 10,
	OrReverse = 11,
	CopyInverted = 12,
	OrInverted = 13,
	Nand = 14,
	Set = 15,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InternalAllocationType {
	Executable = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SystemAllocationScope {
	Command = 0,
	Object = 1,
	Cache = 2,
	Device = 3,
	Instance = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PhysicalDeviceType {
	Other = 0,
	IntegratedGpu = 1,
	DiscreteGpu = 2,
	VirtualGpu = 3,
	Cpu = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VertexInputRate {
	Vertex = 0,
	Instance = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Format { // Vulkan format definitions
	Undefined = 0,
	R4g4UnormPack8 = 1,
	R4g4b4a4UnormPack16 = 2,
	B4g4r4a4UnormPack16 = 3,
	R5g6b5UnormPack16 = 4,
	B5g6r5UnormPack16 = 5,
	R5g5b5a1UnormPack16 = 6,
	B5g5r5a1UnormPack16 = 7,
	A1r5g5b5UnormPack16 = 8,
	R8Unorm = 9,
	R8Snorm = 10,
	R8Uscaled = 11,
	R8Sscaled = 12,
	R8Uint = 13,
	R8Sint = 14,
	R8Srgb = 15,
	R8g8Unorm = 16,
	R8g8Snorm = 17,
	R8g8Uscaled = 18,
	R8g8Sscaled = 19,
	R8g8Uint = 20,
	R8g8Sint = 21,
	R8g8Srgb = 22,
	R8g8b8Unorm = 23,
	R8g8b8Snorm = 24,
	R8g8b8Uscaled = 25,
	R8g8b8Sscaled = 26,
	R8g8b8Uint = 27,
	R8g8b8Sint = 28,
	R8g8b8Srgb = 29,
	B8g8r8Unorm = 30,
	B8g8r8Snorm = 31,
	B8g8r8Uscaled = 32,
	B8g8r8Sscaled = 33,
	B8g8r8Uint = 34,
	B8g8r8Sint = 35,
	B8g8r8Srgb = 36,
	R8g8b8a8Unorm = 37,
	R8g8b8a8Snorm = 38,
	R8g8b8a8Uscaled = 39,
	R8g8b8a8Sscaled = 40,
	R8g8b8a8Uint = 41,
	R8g8b8a8Sint = 42,
	R8g8b8a8Srgb = 43,
	B8g8r8a8Unorm = 44,
	B8g8r8a8Snorm = 45,
	B8g8r8a8Uscaled = 46,
	B8g8r8a8Sscaled = 47,
	B8g8r8a8Uint = 48,
	B8g8r8a8Sint = 49,
	B8g8r8a8Srgb = 50,
	A8b8g8r8UnormPack32 = 51,
	A8b8g8r8SnormPack32 = 52,
	A8b8g8r8UscaledPack32 = 53,
	A8b8g8r8SscaledPack32 = 54,
	A8b8g8r8UintPack32 = 55,
	A8b8g8r8SintPack32 = 56,
	A8b8g8r8SrgbPack32 = 57,
	A2r10g10b10UnormPack32 = 58,
	A2r10g10b10SnormPack32 = 59,
	A2r10g10b10UscaledPack32 = 60,
	A2r10g10b10SscaledPack32 = 61,
	A2r10g10b10UintPack32 = 62,
	A2r10g10b10SintPack32 = 63,
	A2b10g10r10UnormPack32 = 64,
	A2b10g10r10SnormPack32 = 65,
	A2b10g10r10UscaledPack32 = 66,
	A2b10g10r10SscaledPack32 = 67,
	A2b10g10r10UintPack32 = 68,
	A2b10g10r10SintPack32 = 69,
	R16Unorm = 70,
	R16Snorm = 71,
	R16Uscaled = 72,
	R16Sscaled = 73,
	R16Uint = 74,
	R16Sint = 75,
	R16Sfloat = 76,
	R16g16Unorm = 77,
	R16g16Snorm = 78,
	R16g16Uscaled = 79,
	R16g16Sscaled = 80,
	R16g16Uint = 81,
	R16g16Sint = 82,
	R16g16Sfloat = 83,
	R16g16b16Unorm = 84,
	R16g16b16Snorm = 85,
	R16g16b16Uscaled = 86,
	R16g16b16Sscaled = 87,
	R16g16b16Uint = 88,
	R16g16b16Sint = 89,
	R16g16b16Sfloat = 90,
	R16g16b16a16Unorm = 91,
	R16g16b16a16Snorm = 92,
	R16g16b16a16Uscaled = 93,
	R16g16b16a16Sscaled = 94,
	R16g16b16a16Uint = 95,
	R16g16b16a16Sint = 96,
	R16g16b16a16Sfloat = 97,
	R32Uint = 98,
	R32Sint = 99,
	R32Sfloat = 100,
	R32g32Uint = 101,
	R32g32Sint = 102,
	R32g32Sfloat = 103,
	R32g32b32Uint = 104,
	R32g32b32Sint = 105,
	R32g32b32Sfloat = 106,
	R32g32b32a32Uint = 107,
	R32g32b32a32Sint = 108,
	R32g32b32a32Sfloat = 109,
	R64Uint = 110,
	R64Sint = 111,
	R64Sfloat = 112,
	R64g64Uint = 113,
	R64g64Sint = 114,
	R64g64Sfloat = 115,
	R64g64b64Uint = 116,
	R64g64b64Sint = 117,
	R64g64b64Sfloat = 118,
	R64g64b64a64Uint = 119,
	R64g64b64a64Sint = 120,
	R64g64b64a64Sfloat = 121,
	B10g11r11UfloatPack32 = 122,
	E5b9g9r9UfloatPack32 = 123,
	D16Unorm = 124,
	X8D24UnormPack32 = 125,
	D32Sfloat = 126,
	S8Uint = 127,
	D16UnormS8Uint = 128,
	D24UnormS8Uint = 129,
	D32SfloatS8Uint = 130,
	Bc1RgbUnormBlock = 131,
	Bc1RgbSrgbBlock = 132,
	Bc1RgbaUnormBlock = 133,
	Bc1RgbaSrgbBlock = 134,
	Bc2UnormBlock = 135,
	Bc2SrgbBlock = 136,
	Bc3UnormBlock = 137,
	Bc3SrgbBlock = 138,
	Bc4UnormBlock = 139,
	Bc4SnormBlock = 140,
	Bc5UnormBlock = 141,
	Bc5SnormBlock = 142,
	Bc6hUfloatBlock = 143,
	Bc6hSfloatBlock = 144,
	Bc7UnormBlock = 145,
	Bc7SrgbBlock = 146,
	Etc2R8g8b8UnormBlock = 147,
	Etc2R8g8b8SrgbBlock = 148,
	Etc2R8g8b8a1UnormBlock = 149,
	Etc2R8g8b8a1SrgbBlock = 150,
	Etc2R8g8b8a8UnormBlock = 151,
	Etc2R8g8b8a8SrgbBlock = 152,
	EacR11UnormBlock = 153,
	EacR11SnormBlock = 154,
	EacR11g11UnormBlock = 155,
	EacR11g11SnormBlock = 156,
	Astc4x4UnormBlock = 157,
	Astc4x4SrgbBlock = 158,
	Astc5x4UnormBlock = 159,
	Astc5x4SrgbBlock = 160,
	Astc5x5UnormBlock = 161,
	Astc5x5SrgbBlock = 162,
	Astc6x5UnormBlock = 163,
	Astc6x5SrgbBlock = 164,
	Astc6x6UnormBlock = 165,
	Astc6x6SrgbBlock = 166,
	Astc8x5UnormBlock = 167,
	Astc8x5SrgbBlock = 168,
	Astc8x6UnormBlock = 169,
	Astc8x6SrgbBlock = 170,
	Astc8x8UnormBlock = 171,
	Astc8x8SrgbBlock = 172,
	Astc10x5UnormBlock = 173,
	Astc10x5SrgbBlock = 174,
	Astc10x6UnormBlock = 175,
	Astc10x6SrgbBlock = 176,
	Astc10x8UnormBlock = 177,
	Astc10x8SrgbBlock = 178,
	Astc10x10UnormBlock = 179,
	Astc10x10SrgbBlock = 180,
	Astc12x10UnormBlock = 181,
	Astc12x10SrgbBlock = 182,
	Astc12x12UnormBlock = 183,
	Astc12x12SrgbBlock = 184,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StructureType { // Structure type enumerant
	ApplicationInfo = 0,
	InstanceCreateInfo = 1,
	DeviceQueueCreateInfo = 2,
	DeviceCreateInfo = 3,
	SubmitInfo = 4,
	MemoryAllocateInfo = 5,
	MappedMemoryRange = 6,
	BindSparseInfo = 7,
	FenceCreateInfo = 8,
	SemaphoreCreateInfo = 9,
	EventCreateInfo = 10,
	QueryPoolCreateInfo = 11,
	BufferCreateInfo = 12,
	BufferViewCreateInfo = 13,
	ImageCreateInfo = 14,
	ImageViewCreateInfo = 15,
	ShaderModuleCreateInfo = 16,
	PipelineCacheCreateInfo = 17,
	PipelineShaderStageCreateInfo = 18,
	PipelineVertexInputStateCreateInfo = 19,
	PipelineInputAssemblyStateCreateInfo = 20,
	PipelineTessellationStateCreateInfo = 21,
	PipelineViewportStateCreateInfo = 22,
	PipelineRasterizationStateCreateInfo = 23,
	PipelineMultisampleStateCreateInfo = 24,
	PipelineDepthStencilStateCreateInfo = 25,
	PipelineColorBlendStateCreateInfo = 26,
	PipelineDynamicStateCreateInfo = 27,
	GraphicsPipelineCreateInfo = 28,
	ComputePipelineCreateInfo = 29,
	PipelineLayoutCreateInfo = 30,
	SamplerCreateInfo = 31,
	DescriptorSetLayoutCreateInfo = 32,
	DescriptorPoolCreateInfo = 33,
	DescriptorSetAllocateInfo = 34,
	WriteDescriptorSet = 35,
	CopyDescriptorSet = 36,
	FramebufferCreateInfo = 37,
	RenderPassCreateInfo = 38,
	CommandPoolCreateInfo = 39,
	CommandBufferAllocateInfo = 40,
	CommandBufferInheritanceInfo = 41,
	CommandBufferBeginInfo = 42,
	RenderPassBeginInfo = 43,
	BufferMemoryBarrier = 44,
	ImageMemoryBarrier = 45,
	MemoryBarrier = 46,
	LoaderInstanceCreateInfo = 47,
	LoaderDeviceCreateInfo = 48,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SubpassContents {
	Inline = 0,
	SecondaryCommandBuffers = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Result { // Error and return codes
	Success = 0, // Command completed successfully
	NotReady = 1, // A fence or query has not yet completed
	Timeout = 2, // A wait operation has not completed in the specified time
	EventSet = 3, // An event is signaled
	EventReset = 4, // An event is unsignalled
	Incomplete = 5, // A return array was too small for the resul
	ErrorOutOfHostMemory = -1, // A host memory allocation has failed
	ErrorOutOfDeviceMemory = -2, // A device memory allocation has failed
	ErrorInitializationFailed = -3, // The logical device has been lost. See <<devsandqueues-lost-device>>
	ErrorDeviceLost = -4, // Initialization of a object has failed
	ErrorMemoryMapFailed = -5, // Mapping of a memory object has failed
	ErrorLayerNotPresent = -6, // Layer specified does not exist
	ErrorExtensionNotPresent = -7, // Extension specified does not exist
	ErrorFeatureNotPresent = -8, // Requested feature is not available on this device
	ErrorIncompatibleDriver = -9, // Unable to find a Vulkan driver
	ErrorTooManyObjects = -10, // Too many objects of the type have already been created
	ErrorFormatNotSupported = -11, // Requested format is not supported on this device
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DynamicState {
	Viewport = 0,
	Scissor = 1,
	LineWidth = 2,
	DepthBias = 3,
	BlendConstants = 4,
	DepthBounds = 5,
	StencilCompareMask = 6,
	StencilWriteMask = 7,
	StencilReference = 8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PresentModeKHR {
	ImmediateKhr = 0,
	MailboxKhr = 1,
	FifoKhr = 2,
	FifoRelaxedKhr = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorSpaceKHR {
	SrgbNonlinearKhr = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg(vk_ext_debug_report)]
pub enum DebugReportObjectTypeEXT {
	DebugReportObjectTypeUnknownExt = 0,
	DebugReportObjectTypeInstanceExt = 1,
	DebugReportObjectTypePhysicalDeviceExt = 2,
	DebugReportObjectTypeDeviceExt = 3,
	DebugReportObjectTypeQueueExt = 4,
	DebugReportObjectTypeSemaphoreExt = 5,
	DebugReportObjectTypeCommandBufferExt = 6,
	DebugReportObjectTypeFenceExt = 7,
	DebugReportObjectTypeDeviceMemoryExt = 8,
	DebugReportObjectTypeBufferExt = 9,
	DebugReportObjectTypeImageExt = 10,
	DebugReportObjectTypeEventExt = 11,
	DebugReportObjectTypeQueryPoolExt = 12,
	DebugReportObjectTypeBufferViewExt = 13,
	DebugReportObjectTypeImageViewExt = 14,
	DebugReportObjectTypeShaderModuleExt = 15,
	DebugReportObjectTypePipelineCacheExt = 16,
	DebugReportObjectTypePipelineLayoutExt = 17,
	DebugReportObjectTypeRenderPassExt = 18,
	DebugReportObjectTypePipelineExt = 19,
	DebugReportObjectTypeDescriptorSetLayoutExt = 20,
	DebugReportObjectTypeSamplerExt = 21,
	DebugReportObjectTypeDescriptorPoolExt = 22,
	DebugReportObjectTypeDescriptorSetExt = 23,
	DebugReportObjectTypeFramebufferExt = 24,
	DebugReportObjectTypeCommandPoolExt = 25,
	DebugReportObjectTypeSurfaceKhrExt = 26,
	DebugReportObjectTypeSwapchainKhrExt = 27,
	DebugReportObjectTypeDebugReportExt = 28,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg(vk_ext_debug_report)]
pub enum DebugReportErrorEXT {
	DebugReportErrorNoneExt = 0,
	DebugReportErrorCallbackRefExt = 1,
}

#[repr(C)]
pub struct Offset2D {
	pub x: i32,
	pub y: i32,
}

#[repr(C)]
pub struct Offset3D {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}

#[repr(C)]
pub struct Extent2D {
	pub width: u32,
	pub height: u32,
}

#[repr(C)]
pub struct Extent3D {
	pub width: u32,
	pub height: u32,
	pub depth: u32,
}

#[repr(C)]
pub struct Viewport {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
	pub min_depth: f32,
	pub max_depth: f32,
}

#[repr(C)]
pub struct Rect2D {
	pub offset: Offset2D,
	pub extent: Extent2D,
}

#[repr(C)]
pub struct Rect3D {
	pub offset: Offset3D,
	pub extent: Extent3D,
}

#[repr(C)]
pub struct ClearRect {
	pub rect: Rect2D,
	pub base_array_layer: u32,
	pub layer_count: u32,
}

#[repr(C)]
pub struct ComponentMapping {
	pub r: ComponentSwizzle,
	pub g: ComponentSwizzle,
	pub b: ComponentSwizzle,
	pub a: ComponentSwizzle,
}

#[repr(C)]
pub struct PhysicalDeviceProperties {
	pub api_version: u32,
	pub driver_version: u32,
	pub vendor_id: u32,
	pub device_id: u32,
	pub device_type: PhysicalDeviceType,
	pub device_name: [libc::c_char; MAX_PHYSICAL_DEVICE_NAME_SIZE],
	pub pipeline_cache_uuid: [u8; UUID_SIZE],
	pub limits: PhysicalDeviceLimits,
	pub sparse_properties: PhysicalDeviceSparseProperties,
}

#[repr(C)]
pub struct ExtensionProperties {
	pub extension_name: [libc::c_char; MAX_EXTENSION_NAME_SIZE],
	pub spec_version: u32,
}

#[repr(C)]
pub struct LayerProperties {
	pub layer_name: [libc::c_char; MAX_EXTENSION_NAME_SIZE],
	pub spec_version: u32,
	pub implementation_version: u32,
	pub description: [libc::c_char; MAX_DESCRIPTION_SIZE],
}

#[repr(C)]
pub struct ApplicationInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub application_name: *const libc::c_char,
	pub application_version: u32,
	pub engine_name: *const libc::c_char,
	pub engine_version: u32,
	pub api_version: u32,
}

#[repr(C)]
pub struct AllocationCallbacks {
	pub user_data: *const libc::c_void,
	pub allocation: *const AllocationFunctionCallback,
	pub reallocation: *const ReallocationFunctionCallback,
	pub free: *const FreeFunctionCallback,
	pub internal_allocation: *const InternalAllocationNotificationCallback,
	pub internal_free: *const InternalFreeNotificationCallback,
}

#[repr(C)]
pub struct DeviceQueueCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: DeviceQueueCreateFlags,
	pub queue_family_index: u32,
	pub queue_count: u32,
	pub queue_priorities: *const f32,
}

#[repr(C)]
pub struct DeviceCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: DeviceCreateFlags,
	pub queue_create_info_count: u32,
	pub queue_create_infos: *const DeviceQueueCreateInfo,
	pub enabled_layer_count: u32,
	pub enabled_layer_names: *const *const libc::c_char,
	pub enabled_extension_count: u32,
	pub enabled_extension_names: *const *const libc::c_char,
	pub enabled_features: *const PhysicalDeviceFeatures,
}

#[repr(C)]
pub struct InstanceCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: InstanceCreateFlags,
	pub application_info: *const ApplicationInfo,
	pub enabled_layer_count: u32,
	pub enabled_layer_names: *const *const libc::c_char,
	pub enabled_extension_count: u32,
	pub enabled_extension_names: *const *const libc::c_char,
}

#[repr(C)]
pub struct QueueFamilyProperties {
	pub queue_flags: QueueFlags,
	pub queue_count: u32,
	pub timestamp_valid_bits: u32,
	pub min_image_transfer_granularity: Extent3D,
}

#[repr(C)]
pub struct PhysicalDeviceMemoryProperties {
	pub memory_type_count: u32,
	pub memory_types: [MemoryType; MAX_MEMORY_TYPES],
	pub memory_heap_count: u32,
	pub memory_heaps: [MemoryHeap; MAX_MEMORY_HEAPS],
}

#[repr(C)]
pub struct MemoryAllocateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub allocation_size: DeviceSize,
	pub memory_type_index: u32,
}

#[repr(C)]
pub struct MemoryRequirements {
	pub size: DeviceSize,
	pub alignment: DeviceSize,
	pub memory_type_bits: u32,
}

#[repr(C)]
pub struct SparseImageFormatProperties {
	pub aspect_mask: ImageAspectFlags,
	pub image_granularity: Extent3D,
	pub flags: SparseImageFormatFlags,
}

#[repr(C)]
pub struct SparseImageMemoryRequirements {
	pub format_properties: SparseImageFormatProperties,
	pub image_mip_tail_first_lod: u32,
	pub image_mip_tail_size: DeviceSize,
	pub image_mip_tail_offset: DeviceSize,
	pub image_mip_tail_stride: DeviceSize,
}

#[repr(C)]
pub struct MemoryType {
	pub property_flags: MemoryPropertyFlags,
	pub heap_index: u32,
}

#[repr(C)]
pub struct MemoryHeap {
	pub size: DeviceSize,
	pub flags: MemoryHeapFlags,
}

#[repr(C)]
pub struct MappedMemoryRange {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub memory: u64,
	pub offset: DeviceSize,
	pub size: DeviceSize,
}

#[repr(C)]
pub struct FormatProperties {
	pub linear_tiling_features: FormatFeatureFlags,
	pub optimal_tiling_features: FormatFeatureFlags,
	pub buffer_features: FormatFeatureFlags,
}

#[repr(C)]
pub struct ImageFormatProperties {
	pub max_extent: Extent3D,
	pub max_mip_levels: u32,
	pub max_array_layers: u32,
	pub sample_counts: SampleCountFlags,
	pub max_resource_size: DeviceSize,
}

#[repr(C)]
pub struct DescriptorBufferInfo {
	pub buffer: u64,
	pub offset: DeviceSize,
	pub range: DeviceSize,
}

#[repr(C)]
pub struct DescriptorImageInfo {
	pub sampler: u64,
	pub image_view: u64,
	pub image_layout: ImageLayout,
}

#[repr(C)]
pub struct WriteDescriptorSet {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub dst_set: u64,
	pub dst_binding: u32,
	pub dst_array_element: u32,
	pub descriptor_count: u32,
	pub descriptor_type: DescriptorType,
	pub image_info: *const DescriptorImageInfo,
	pub buffer_info: *const DescriptorBufferInfo,
	pub texel_buffer_view: *const u64,
}

#[repr(C)]
pub struct CopyDescriptorSet {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub src_set: u64,
	pub src_binding: u32,
	pub src_array_element: u32,
	pub dst_set: u64,
	pub dst_binding: u32,
	pub dst_array_element: u32,
	pub descriptor_count: u32,
}

#[repr(C)]
pub struct BufferCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: BufferCreateFlags,
	pub size: DeviceSize,
	pub usage: BufferUsageFlags,
	pub sharing_mode: SharingMode,
	pub queue_family_index_count: u32,
	pub queue_family_indices: *const u32,
}

#[repr(C)]
pub struct BufferViewCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: BufferViewCreateFlags,
	pub buffer: u64,
	pub format: Format,
	pub offset: DeviceSize,
	pub range: DeviceSize,
}

#[repr(C)]
pub struct ImageSubresource {
	pub aspect_mask: ImageAspectFlags,
	pub mip_level: u32,
	pub array_layer: u32,
}

#[repr(C)]
pub struct ImageSubresourceLayers {
	pub aspect_mask: ImageAspectFlags,
	pub mip_level: u32,
	pub base_array_layer: u32,
	pub layer_count: u32,
}

#[repr(C)]
pub struct ImageSubresourceRange {
	pub aspect_mask: ImageAspectFlags,
	pub base_mip_level: u32,
	pub level_count: u32,
	pub base_array_layer: u32,
	pub layer_count: u32,
}

#[repr(C)]
pub struct MemoryBarrier {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub src_access_mask: AccessFlags,
	pub dst_access_mask: AccessFlags,
}

#[repr(C)]
pub struct BufferMemoryBarrier {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub src_access_mask: AccessFlags,
	pub dst_access_mask: AccessFlags,
	pub src_queue_family_index: u32,
	pub dst_queue_family_index: u32,
	pub buffer: u64,
	pub offset: DeviceSize,
	pub size: DeviceSize,
}

#[repr(C)]
pub struct ImageMemoryBarrier {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub src_access_mask: AccessFlags,
	pub dst_access_mask: AccessFlags,
	pub old_layout: ImageLayout,
	pub new_layout: ImageLayout,
	pub src_queue_family_index: u32,
	pub dst_queue_family_index: u32,
	pub image: u64,
	pub subresource_range: ImageSubresourceRange,
}

#[repr(C)]
pub struct ImageCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: ImageCreateFlags,
	pub image_type: ImageType,
	pub format: Format,
	pub extent: Extent3D,
	pub mip_levels: u32,
	pub array_layers: u32,
	pub samples: SampleCountFlags,
	pub tiling: ImageTiling,
	pub usage: ImageUsageFlags,
	pub sharing_mode: SharingMode,
	pub queue_family_index_count: u32,
	pub queue_family_indices: *const u32,
	pub initial_layout: ImageLayout,
}

#[repr(C)]
pub struct SubresourceLayout {
	pub offset: DeviceSize,
	pub size: DeviceSize,
	pub row_pitch: DeviceSize,
	pub array_pitch: DeviceSize,
	pub depth_pitch: DeviceSize,
}

#[repr(C)]
pub struct ImageViewCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: ImageViewCreateFlags,
	pub image: u64,
	pub view_type: ImageViewType,
	pub format: Format,
	pub components: ComponentMapping,
	pub subresource_range: ImageSubresourceRange,
}

#[repr(C)]
pub struct BufferCopy {
	pub src_offset: DeviceSize,
	pub dst_offset: DeviceSize,
	pub size: DeviceSize,
}

#[repr(C)]
pub struct SparseMemoryBind {
	pub resource_offset: DeviceSize,
	pub size: DeviceSize,
	pub memory: u64,
	pub memory_offset: DeviceSize,
	pub flags: SparseMemoryBindFlags,
}

#[repr(C)]
pub struct SparseImageMemoryBind {
	pub subresource: ImageSubresource,
	pub offset: Offset3D,
	pub extent: Extent3D,
	pub memory: u64,
	pub memory_offset: DeviceSize,
	pub flags: SparseMemoryBindFlags,
}

#[repr(C)]
pub struct SparseBufferMemoryBindInfo {
	pub buffer: u64,
	pub bind_count: u32,
	pub binds: *const SparseMemoryBind,
}

#[repr(C)]
pub struct SparseImageOpaqueMemoryBindInfo {
	pub image: u64,
	pub bind_count: u32,
	pub binds: *const SparseMemoryBind,
}

#[repr(C)]
pub struct SparseImageMemoryBindInfo {
	pub image: u64,
	pub bind_count: u32,
	pub binds: *const SparseImageMemoryBind,
}

#[repr(C)]
pub struct BindSparseInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub wait_semaphore_count: u32,
	pub wait_semaphores: *const u64,
	pub buffer_bind_count: u32,
	pub buffer_binds: *const SparseBufferMemoryBindInfo,
	pub image_opaque_bind_count: u32,
	pub image_opaque_binds: *const SparseImageOpaqueMemoryBindInfo,
	pub image_bind_count: u32,
	pub image_binds: *const SparseImageMemoryBindInfo,
	pub signal_semaphore_count: u32,
	pub signal_semaphores: *const u64,
}

#[repr(C)]
pub struct ImageCopy {
	pub src_subresource: ImageSubresourceLayers,
	pub src_offset: Offset3D,
	pub dst_subresource: ImageSubresourceLayers,
	pub dst_offset: Offset3D,
	pub extent: Extent3D,
}

#[repr(C)]
pub struct ImageBlit {
	pub src_subresource: ImageSubresourceLayers,
	pub src_offsets: [Offset3D; 2],
	pub dst_subresource: ImageSubresourceLayers,
	pub dst_offsets: [Offset3D; 2],
}

#[repr(C)]
pub struct BufferImageCopy {
	pub buffer_offset: DeviceSize,
	pub buffer_row_length: u32,
	pub buffer_image_height: u32,
	pub image_subresource: ImageSubresourceLayers,
	pub image_offset: Offset3D,
	pub image_extent: Extent3D,
}

#[repr(C)]
pub struct ImageResolve {
	pub src_subresource: ImageSubresourceLayers,
	pub src_offset: Offset3D,
	pub dst_subresource: ImageSubresourceLayers,
	pub dst_offset: Offset3D,
	pub extent: Extent3D,
}

#[repr(C)]
pub struct ShaderModuleCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: ShaderModuleCreateFlags,
	pub code_size: libc::size_t,
	pub code: *const u32,
}

#[repr(C)]
pub struct DescriptorSetLayoutBinding {
	pub binding: u32,
	pub descriptor_type: DescriptorType,
	pub descriptor_count: u32,
	pub stage_flags: ShaderStageFlags,
	pub immutable_samplers: *const u64,
}

#[repr(C)]
pub struct DescriptorSetLayoutCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: DescriptorSetLayoutCreateFlags,
	pub binding_count: u32,
	pub bindings: *const DescriptorSetLayoutBinding,
}

#[repr(C)]
pub struct DescriptorPoolSize {
	pub ty: DescriptorType,
	pub descriptor_count: u32,
}

#[repr(C)]
pub struct DescriptorPoolCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: DescriptorPoolCreateFlags,
	pub max_sets: u32,
	pub pool_size_count: u32,
	pub pool_sizes: *const DescriptorPoolSize,
}

#[repr(C)]
pub struct DescriptorSetAllocateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub descriptor_pool: u64,
	pub descriptor_set_count: u32,
	pub set_layouts: *const u64,
}

#[repr(C)]
pub struct SpecializationMapEntry {
	pub constant_id: u32,
	pub offset: u32,
	pub size: libc::size_t,
}

#[repr(C)]
pub struct SpecializationInfo {
	pub map_entry_count: u32,
	pub ma: *const SpecializationMapEntry,
	pub data_size: libc::size_t,
	pub data: *const libc::c_void,
}

#[repr(C)]
pub struct PipelineShaderStageCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: PipelineShaderStageCreateFlags,
	pub stage: ShaderStageFlags,
	pub module: u64,
	pub name: *const libc::c_char,
	pub specialization_info: *const SpecializationInfo,
}

#[repr(C)]
pub struct ComputePipelineCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: PipelineCreateFlags,
	pub stage: PipelineShaderStageCreateInfo,
	pub layout: u64,
	pub base_pipeline_handle: u64,
	pub base_pipeline_index: i32,
}

#[repr(C)]
pub struct VertexInputBindingDescription {
	pub binding: u32,
	pub stride: u32,
	pub input_rate: VertexInputRate,
}

#[repr(C)]
pub struct VertexInputAttributeDescription {
	pub location: u32,
	pub binding: u32,
	pub format: Format,
	pub offset: u32,
}

#[repr(C)]
pub struct PipelineVertexInputStateCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: PipelineVertexInputStateCreateFlags,
	pub vertex_binding_description_count: u32,
	pub vertex_binding_descriptions: *const VertexInputBindingDescription,
	pub vertex_attribute_description_count: u32,
	pub vertex_attribute_descriptions: *const VertexInputAttributeDescription,
}

#[repr(C)]
pub struct PipelineInputAssemblyStateCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: PipelineInputAssemblyStateCreateFlags,
	pub topology: PrimitiveTopology,
	pub primitive_restart_enable: Bool32,
}

#[repr(C)]
pub struct PipelineTessellationStateCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: PipelineTessellationStateCreateFlags,
	pub patch_control_points: u32,
}

#[repr(C)]
pub struct PipelineViewportStateCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: PipelineViewportStateCreateFlags,
	pub viewport_count: u32,
	pub viewports: *const Viewport,
	pub scissor_count: u32,
	pub scissors: *const Rect2D,
}

#[repr(C)]
pub struct PipelineRasterizationStateCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: PipelineRasterizationStateCreateFlags,
	pub depth_clamp_enable: Bool32,
	pub rasterizer_discard_enable: Bool32,
	pub polygon_mode: PolygonMode,
	pub cull_mode: CullModeFlags,
	pub front_face: FrontFace,
	pub depth_bias_enable: Bool32,
	pub depth_bias_constant_factor: f32,
	pub depth_bias_clamp: f32,
	pub depth_bias_slope_factor: f32,
	pub line_width: f32,
}

#[repr(C)]
pub struct PipelineMultisampleStateCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: PipelineMultisampleStateCreateFlags,
	pub rasterization_samples: SampleCountFlags,
	pub sample_shading_enable: Bool32,
	pub min_sample_shading: f32,
	pub sample_mask: *const SampleMask,
	pub alpha_to_coverage_enable: Bool32,
	pub alpha_to_one_enable: Bool32,
}

#[repr(C)]
pub struct PipelineColorBlendAttachmentState {
	pub blend_enable: Bool32,
	pub src_color_blend_factor: BlendFactor,
	pub dst_color_blend_factor: BlendFactor,
	pub color_blend_op: BlendOp,
	pub src_alpha_blend_factor: BlendFactor,
	pub dst_alpha_blend_factor: BlendFactor,
	pub alpha_blend_op: BlendOp,
	pub color_write_mask: ColorComponentFlags,
}

#[repr(C)]
pub struct PipelineColorBlendStateCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: PipelineColorBlendStateCreateFlags,
	pub logic_op_enable: Bool32,
	pub logic_op: LogicOp,
	pub attachment_count: u32,
	pub attachments: *const PipelineColorBlendAttachmentState,
	pub blend_constants: f32,
}

#[repr(C)]
pub struct PipelineDynamicStateCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: PipelineDynamicStateCreateFlags,
	pub dynamic_state_count: u32,
	pub dynamic_states: *const DynamicState,
}

#[repr(C)]
pub struct StencilOpState {
	pub fail_op: StencilOp,
	pub pass_op: StencilOp,
	pub depth_fail_op: StencilOp,
	pub compare_op: CompareOp,
	pub compare_mask: u32,
	pub write_mask: u32,
	pub reference: u32,
}

#[repr(C)]
pub struct PipelineDepthStencilStateCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: PipelineDepthStencilStateCreateFlags,
	pub depth_test_enable: Bool32,
	pub depth_write_enable: Bool32,
	pub depth_compare_op: CompareOp,
	pub depth_bounds_test_enable: Bool32,
	pub stencil_test_enable: Bool32,
	pub front: StencilOpState,
	pub back: StencilOpState,
	pub min_depth_bounds: f32,
	pub max_depth_bounds: f32,
}

#[repr(C)]
pub struct GraphicsPipelineCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: PipelineCreateFlags,
	pub stage_count: u32,
	pub stages: *const PipelineShaderStageCreateInfo,
	pub vertex_input_state: *const PipelineVertexInputStateCreateInfo,
	pub input_assembly_state: *const PipelineInputAssemblyStateCreateInfo,
	pub tessellation_state: *const PipelineTessellationStateCreateInfo,
	pub viewport_state: *const PipelineViewportStateCreateInfo,
	pub rasterization_state: *const PipelineRasterizationStateCreateInfo,
	pub multisample_state: *const PipelineMultisampleStateCreateInfo,
	pub depth_stencil_state: *const PipelineDepthStencilStateCreateInfo,
	pub color_blend_state: *const PipelineColorBlendStateCreateInfo,
	pub dynamic_state: *const PipelineDynamicStateCreateInfo,
	pub layout: u64,
	pub render_pass: u64,
	pub subpass: u32,
	pub base_pipeline_handle: u64,
	pub base_pipeline_index: i32,
}

#[repr(C)]
pub struct PipelineCacheCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: PipelineCacheCreateFlags,
	pub initial_data_size: libc::size_t,
	pub initial_data: *const libc::c_void,
}

#[repr(C)]
pub struct PushConstantRange {
	pub stage_flags: ShaderStageFlags,
	pub offset: u32,
	pub size: u32,
}

#[repr(C)]
pub struct PipelineLayoutCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: PipelineLayoutCreateFlags,
	pub set_layout_count: u32,
	pub set_layouts: *const u64,
	pub push_constant_range_count: u32,
	pub push_constant_ranges: *const PushConstantRange,
}

#[repr(C)]
pub struct SamplerCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: SamplerCreateFlags,
	pub mag_filter: Filter,
	pub min_filter: Filter,
	pub mipmap_mode: SamplerMipmapMode,
	pub address_mode_u: SamplerAddressMode,
	pub address_mode_v: SamplerAddressMode,
	pub address_mode_w: SamplerAddressMode,
	pub mip_lod_bias: f32,
	pub anisotropy_enable: Bool32,
	pub max_anisotropy: f32,
	pub compare_enable: Bool32,
	pub compare_op: CompareOp,
	pub min_lod: f32,
	pub max_lod: f32,
	pub border_color: BorderColor,
	pub unnormalized_coordinates: Bool32,
}

#[repr(C)]
pub struct CommandPoolCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: CommandPoolCreateFlags,
	pub queue_family_index: u32,
}

#[repr(C)]
pub struct CommandBufferAllocateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub command_pool: u64,
	pub level: CommandBufferLevel,
	pub command_buffer_count: u32,
}

#[repr(C)]
pub struct CommandBufferInheritanceInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub render_pass: u64,
	pub subpass: u32,
	pub framebuffer: u64,
	pub occlusion_query_enable: Bool32,
	pub query_flags: QueryControlFlags,
	pub pipeline_statistics: QueryPipelineStatisticFlags,
}

#[repr(C)]
pub struct CommandBufferBeginInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: CommandBufferUsageFlags,
	pub inheritance_info: *const CommandBufferInheritanceInfo,
}

#[repr(C)]
pub struct RenderPassBeginInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub render_pass: u64,
	pub framebuffer: u64,
	pub render_area: Rect2D,
	pub clear_value_count: u32,
	pub clear_values: *const libc::size_t,
}

#[repr(C)]
pub struct ClearDepthStencilValue {
	pub depth: f32,
	pub stencil: u32,
}

#[repr(C)]
pub struct ClearAttachment {
	pub aspect_mask: ImageAspectFlags,
	pub color_attachment: u32,
	pub clear_value: libc::size_t,
}

#[repr(C)]
pub struct AttachmentDescription {
	pub flags: AttachmentDescriptionFlags,
	pub format: Format,
	pub samples: SampleCountFlags,
	pub load_op: AttachmentLoadOp,
	pub store_op: AttachmentStoreOp,
	pub stencil_load_op: AttachmentLoadOp,
	pub stencil_store_op: AttachmentStoreOp,
	pub initial_layout: ImageLayout,
	pub final_layout: ImageLayout,
}

#[repr(C)]
pub struct AttachmentReference {
	pub attachment: u32,
	pub layout: ImageLayout,
}

#[repr(C)]
pub struct SubpassDescription {
	pub flags: SubpassDescriptionFlags,
	pub pipeline_bind_point: PipelineBindPoint,
	pub input_attachment_count: u32,
	pub input_attachments: *const AttachmentReference,
	pub color_attachment_count: u32,
	pub color_attachments: *const AttachmentReference,
	pub resolve_attachments: *const AttachmentReference,
	pub depth_stencil_attachment: *const AttachmentReference,
	pub preserve_attachment_count: u32,
	pub preserve_attachments: *const u32,
}

#[repr(C)]
pub struct SubpassDependency {
	pub src_subpass: u32,
	pub dst_subpass: u32,
	pub src_stage_mask: PipelineStageFlags,
	pub dst_stage_mask: PipelineStageFlags,
	pub src_access_mask: AccessFlags,
	pub dst_access_mask: AccessFlags,
	pub dependency_flags: DependencyFlags,
}

#[repr(C)]
pub struct RenderPassCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: RenderPassCreateFlags,
	pub attachment_count: u32,
	pub attachments: *const AttachmentDescription,
	pub subpass_count: u32,
	pub subpasses: *const SubpassDescription,
	pub dependency_count: u32,
	pub dependencies: *const SubpassDependency,
}

#[repr(C)]
pub struct EventCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: EventCreateFlags,
}

#[repr(C)]
pub struct FenceCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: FenceCreateFlags,
}

#[repr(C)]
pub struct PhysicalDeviceFeatures {
	pub robust_buffer_access: Bool32,
	pub full_draw_index_uint32: Bool32,
	pub image_cube_array: Bool32,
	pub independent_blend: Bool32,
	pub geometry_shader: Bool32,
	pub tessellation_shader: Bool32,
	pub sample_rate_shading: Bool32,
	pub dual_src_blend: Bool32,
	pub logic_op: Bool32,
	pub multi_draw_indirect: Bool32,
	pub draw_indirect_first_instance: Bool32,
	pub depth_clamp: Bool32,
	pub depth_bias_clamp: Bool32,
	pub fill_mode_non_solid: Bool32,
	pub depth_bounds: Bool32,
	pub wide_lines: Bool32,
	pub large_points: Bool32,
	pub alpha_to_one: Bool32,
	pub multi_viewport: Bool32,
	pub sampler_anisotropy: Bool32,
	pub texture_compression_etc2: Bool32,
	pub texture_compression_astc_ldr: Bool32,
	pub texture_compression_bc: Bool32,
	pub occlusion_query_precise: Bool32,
	pub pipeline_statistics_query: Bool32,
	pub vertex_pipeline_stores_and_atomics: Bool32,
	pub fragment_stores_and_atomics: Bool32,
	pub shader_tessellation_and_geometry_point_size: Bool32,
	pub shader_image_gather_extended: Bool32,
	pub shader_storage_image_extended_formats: Bool32,
	pub shader_storage_image_multisample: Bool32,
	pub shader_storage_image_read_without_format: Bool32,
	pub shader_storage_image_write_without_format: Bool32,
	pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
	pub shader_sampled_image_array_dynamic_indexing: Bool32,
	pub shader_storage_buffer_array_dynamic_indexing: Bool32,
	pub shader_storage_image_array_dynamic_indexing: Bool32,
	pub shader_clip_distance: Bool32,
	pub shader_cull_distance: Bool32,
	pub shader_float64: Bool32,
	pub shader_int64: Bool32,
	pub shader_int16: Bool32,
	pub shader_resource_residency: Bool32,
	pub shader_resource_min_lod: Bool32,
	pub sparse_binding: Bool32,
	pub sparse_residency_buffer: Bool32,
	pub sparse_residency_image2_d: Bool32,
	pub sparse_residency_image3_d: Bool32,
	pub sparse_residency2_samples: Bool32,
	pub sparse_residency4_samples: Bool32,
	pub sparse_residency8_samples: Bool32,
	pub sparse_residency16_samples: Bool32,
	pub sparse_residency_aliased: Bool32,
	pub variable_multisample_rate: Bool32,
	pub inherited_queries: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceSparseProperties {
	pub residency_standard2_d_block_shape: Bool32,
	pub residency_standard2_d_multisample_block_shape: Bool32,
	pub residency_standard3_d_block_shape: Bool32,
	pub residency_aligned_mip_size: Bool32,
	pub residency_non_resident_strict: Bool32,
}

#[repr(C)]
pub struct PhysicalDeviceLimits {
	pub max_image_dimension1_d: u32,
	pub max_image_dimension2_d: u32,
	pub max_image_dimension3_d: u32,
	pub max_image_dimension_cube: u32,
	pub max_image_array_layers: u32,
	pub max_texel_buffer_elements: u32,
	pub max_uniform_buffer_range: u32,
	pub max_storage_buffer_range: u32,
	pub max_push_constants_size: u32,
	pub max_memory_allocation_count: u32,
	pub max_sampler_allocation_count: u32,
	pub buffer_image_granularity: DeviceSize,
	pub sparse_address_space_size: DeviceSize,
	pub max_bound_descriptor_sets: u32,
	pub max_per_stage_descriptor_samplers: u32,
	pub max_per_stage_descriptor_uniform_buffers: u32,
	pub max_per_stage_descriptor_storage_buffers: u32,
	pub max_per_stage_descriptor_sampled_images: u32,
	pub max_per_stage_descriptor_storage_images: u32,
	pub max_per_stage_descriptor_input_attachments: u32,
	pub max_per_stage_resources: u32,
	pub max_descriptor_set_samplers: u32,
	pub max_descriptor_set_uniform_buffers: u32,
	pub max_descriptor_set_uniform_buffers_dynamic: u32,
	pub max_descriptor_set_storage_buffers: u32,
	pub max_descriptor_set_storage_buffers_dynamic: u32,
	pub max_descriptor_set_sampled_images: u32,
	pub max_descriptor_set_storage_images: u32,
	pub max_descriptor_set_input_attachments: u32,
	pub max_vertex_input_attributes: u32,
	pub max_vertex_input_bindings: u32,
	pub max_vertex_input_attribute_offset: u32,
	pub max_vertex_input_binding_stride: u32,
	pub max_vertex_output_components: u32,
	pub max_tessellation_generation_level: u32,
	pub max_tessellation_patch_size: u32,
	pub max_tessellation_control_per_vertex_input_components: u32,
	pub max_tessellation_control_per_vertex_output_components: u32,
	pub max_tessellation_control_per_patch_output_components: u32,
	pub max_tessellation_control_total_output_components: u32,
	pub max_tessellation_evaluation_input_components: u32,
	pub max_tessellation_evaluation_output_components: u32,
	pub max_geometry_shader_invocations: u32,
	pub max_geometry_input_components: u32,
	pub max_geometry_output_components: u32,
	pub max_geometry_output_vertices: u32,
	pub max_geometry_total_output_components: u32,
	pub max_fragment_input_components: u32,
	pub max_fragment_output_attachments: u32,
	pub max_fragment_dual_src_attachments: u32,
	pub max_fragment_combined_output_resources: u32,
	pub max_compute_shared_memory_size: u32,
	pub max_compute_work_group_count: u32,
	pub max_compute_work_group_invocations: u32,
	pub max_compute_work_group_size: u32,
	pub sub_pixel_precision_bits: u32,
	pub sub_texel_precision_bits: u32,
	pub mipmap_precision_bits: u32,
	pub max_draw_indexed_index_value: u32,
	pub max_draw_indirect_count: u32,
	pub max_sampler_lod_bias: f32,
	pub max_sampler_anisotropy: f32,
	pub max_viewports: u32,
	pub max_viewport_dimensions: u32,
	pub viewport_bounds_range: f32,
	pub viewport_sub_pixel_bits: u32,
	pub min_memory_map_alignment: libc::size_t,
	pub min_texel_buffer_offset_alignment: DeviceSize,
	pub min_uniform_buffer_offset_alignment: DeviceSize,
	pub min_storage_buffer_offset_alignment: DeviceSize,
	pub min_texel_offset: i32,
	pub max_texel_offset: u32,
	pub min_texel_gather_offset: i32,
	pub max_texel_gather_offset: u32,
	pub min_interpolation_offset: f32,
	pub max_interpolation_offset: f32,
	pub sub_pixel_interpolation_offset_bits: u32,
	pub max_framebuffer_width: u32,
	pub max_framebuffer_height: u32,
	pub max_framebuffer_layers: u32,
	pub framebuffer_color_sample_counts: SampleCountFlags,
	pub framebuffer_depth_sample_counts: SampleCountFlags,
	pub framebuffer_stencil_sample_counts: SampleCountFlags,
	pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
	pub max_color_attachments: u32,
	pub sampled_image_color_sample_counts: SampleCountFlags,
	pub sampled_image_integer_sample_counts: SampleCountFlags,
	pub sampled_image_depth_sample_counts: SampleCountFlags,
	pub sampled_image_stencil_sample_counts: SampleCountFlags,
	pub storage_image_sample_counts: SampleCountFlags,
	pub max_sample_mask_words: u32,
	pub timestamp_compute_and_graphics: Bool32,
	pub timestamp_period: f32,
	pub max_clip_distances: u32,
	pub max_cull_distances: u32,
	pub max_combined_clip_and_cull_distances: u32,
	pub discrete_queue_priorities: u32,
	pub point_size_range: f32,
	pub line_width_range: f32,
	pub point_size_granularity: f32,
	pub line_width_granularity: f32,
	pub strict_lines: Bool32,
	pub standard_sample_locations: Bool32,
	pub optimal_buffer_copy_offset_alignment: DeviceSize,
	pub optimal_buffer_copy_row_pitch_alignment: DeviceSize,
	pub non_coherent_atom_size: DeviceSize,
}

#[repr(C)]
pub struct SemaphoreCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: SemaphoreCreateFlags,
}

#[repr(C)]
pub struct QueryPoolCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: QueryPoolCreateFlags,
	pub query_type: QueryType,
	pub query_count: u32,
	pub pipeline_statistics: QueryPipelineStatisticFlags,
}

#[repr(C)]
pub struct FramebufferCreateInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: FramebufferCreateFlags,
	pub render_pass: u64,
	pub attachment_count: u32,
	pub attachments: *const u64,
	pub width: u32,
	pub height: u32,
	pub layers: u32,
}

#[repr(C)]
pub struct DrawIndirectCommand {
	pub vertex_count: u32,
	pub instance_count: u32,
	pub first_vertex: u32,
	pub first_instance: u32,
}

#[repr(C)]
pub struct DrawIndexedIndirectCommand {
	pub index_count: u32,
	pub instance_count: u32,
	pub first_index: u32,
	pub vertex_offset: i32,
	pub first_instance: u32,
}

#[repr(C)]
pub struct DispatchIndirectCommand {
	pub x: u32,
	pub y: u32,
	pub z: u32,
}

#[repr(C)]
pub struct SubmitInfo {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub wait_semaphore_count: u32,
	pub wait_semaphores: *const u64,
	pub wait_dst_stage_mask: *const PipelineStageFlags,
	pub command_buffer_count: u32,
	pub command_buffers: *const libc::size_t,
	pub signal_semaphore_count: u32,
	pub signal_semaphores: *const u64,
}

#[repr(C)]
#[cfg(vk_khr_display)]
pub struct DisplayPropertiesKHR {
	pub display: u64,
	pub display_name: libc::c_char,
	pub physical_dimensions: Extent2D,
	pub physical_resolution: Extent2D,
	pub supported_transforms: SurfaceTransformFlagsKHR,
	pub plane_reorder_possible: Bool32,
	pub persistent_content: Bool32,
}

#[repr(C)]
#[cfg(vk_khr_display)]
pub struct DisplayPlanePropertiesKHR {
	pub current_display: u64,
	pub current_stack_index: u32,
}

#[repr(C)]
#[cfg(vk_khr_display)]
pub struct DisplayModeParametersKHR {
	pub visible_region: Extent2D,
	pub refresh_rate: u32,
}

#[repr(C)]
#[cfg(vk_khr_display)]
pub struct DisplayModePropertiesKHR {
	pub display_mode: u64,
	pub parameters: DisplayModeParametersKHR,
}

#[repr(C)]
#[cfg(vk_khr_display)]
pub struct DisplayModeCreateInfoKHR {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: DisplayModeCreateFlagsKHR,
	pub parameters: DisplayModeParametersKHR,
}

#[repr(C)]
#[cfg(vk_khr_display)]
pub struct DisplayPlaneCapabilitiesKHR {
	pub supported_alpha: DisplayPlaneAlphaFlagsKHR,
	pub min_src_position: Offset2D,
	pub max_src_position: Offset2D,
	pub min_src_extent: Extent2D,
	pub max_src_extent: Extent2D,
	pub min_dst_position: Offset2D,
	pub max_dst_position: Offset2D,
	pub min_dst_extent: Extent2D,
	pub max_dst_extent: Extent2D,
}

#[repr(C)]
#[cfg(vk_khr_display)]
pub struct DisplaySurfaceCreateInfoKHR {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: DisplaySurfaceCreateFlagsKHR,
	pub display_mode: u64,
	pub plane_index: u32,
	pub plane_stack_index: u32,
	pub transform: SurfaceTransformFlagsKHR,
	pub global_alpha: f32,
	pub alpha_mode: DisplayPlaneAlphaFlagsKHR,
	pub image_extent: Extent2D,
}

#[repr(C)]
#[cfg(vk_khr_display_swapchain)]
pub struct DisplayPresentInfoKHR {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub src_rect: Rect2D,
	pub dst_rect: Rect2D,
	pub persistent: Bool32,
}

#[repr(C)]
pub struct SurfaceCapabilitiesKHR {
	pub min_image_count: u32,
	pub max_image_count: u32,
	pub current_extent: Extent2D,
	pub min_image_extent: Extent2D,
	pub max_image_extent: Extent2D,
	pub max_image_array_layers: u32,
	pub supported_transforms: SurfaceTransformFlagsKHR,
	pub current_transform: SurfaceTransformFlagsKHR,
	pub supported_composite_alpha: CompositeAlphaFlagsKHR,
	pub supported_usage_flags: ImageUsageFlags,
}

#[repr(C)]
#[cfg(vk_khr_android_surface)]
pub struct AndroidSurfaceCreateInfoKHR {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: AndroidSurfaceCreateFlagsKHR,
	pub window: libc::size_t,
}

#[repr(C)]
#[cfg(vk_khr_mir_surface)]
pub struct MirSurfaceCreateInfoKHR {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: MirSurfaceCreateFlagsKHR,
	pub connection: libc::size_t,
	pub mir_surface: libc::size_t,
}

#[repr(C)]
#[cfg(vk_khr_wayland_surface)]
pub struct WaylandSurfaceCreateInfoKHR {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: WaylandSurfaceCreateFlagsKHR,
	pub display: libc::size_t,
	pub surface: libc::size_t,
}

#[repr(C)]
#[cfg(vk_khr_win32_surface)]
pub struct Win32SurfaceCreateInfoKHR {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: Win32SurfaceCreateFlagsKHR,
	pub hinstance: libc::size_t,
	pub hwnd: libc::size_t,
}

#[repr(C)]
#[cfg(vk_khr_xlib_surface)]
pub struct XlibSurfaceCreateInfoKHR {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: XlibSurfaceCreateFlagsKHR,
	pub dpy: libc::size_t,
	pub window: libc::size_t,
}

#[repr(C)]
#[cfg(vk_khr_xcb_surface)]
pub struct XcbSurfaceCreateInfoKHR {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: XcbSurfaceCreateFlagsKHR,
	pub connection: libc::size_t,
	pub window: libc::size_t,
}

#[repr(C)]
pub struct SurfaceFormatKHR {
	pub format: Format,
	pub color_space: ColorSpaceKHR,
}

#[repr(C)]
pub struct SwapchainCreateInfoKHR {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: SwapchainCreateFlagsKHR,
	pub surface: u64,
	pub min_image_count: u32,
	pub image_format: Format,
	pub image_color_space: ColorSpaceKHR,
	pub image_extent: Extent2D,
	pub image_array_layers: u32,
	pub image_usage: ImageUsageFlags,
	pub image_sharing_mode: SharingMode,
	pub queue_family_index_count: u32,
	pub queue_family_indices: *const u32,
	pub pre_transform: SurfaceTransformFlagsKHR,
	pub composite_alpha: CompositeAlphaFlagsKHR,
	pub present_mode: PresentModeKHR,
	pub clipped: Bool32,
	pub old_swapchain: u64,
}

#[repr(C)]
pub struct PresentInfoKHR {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub wait_semaphore_count: u32,
	pub wait_semaphores: *const u64,
	pub swapchain_count: u32,
	pub swapchains: *const u64,
	pub image_indices: *const u32,
	pub results: *const Result,
}

#[repr(C)]
pub struct DebugReportCallbackCreateInfoEXT {
	pub structure_ty: StructureType,
	pub next: *const libc::c_void,
	pub flags: DebugReportFlagsEXT,
	pub callback: *const DebugReportCallbackEXTCallback,
	pub user_data: *const libc::c_void,
}


#[link(name = "vulkan-1")]
extern "C" {
	fn vkCreateInstance(create_info: *const InstanceCreateInfo, allocator: *const AllocationCallbacks, instance: *mut libc::size_t) -> Result;
	fn vkDestroyInstance(instance: libc::size_t, allocator: *const AllocationCallbacks);
	fn vkEnumeratePhysicalDevices(instance: libc::size_t, physical_device_count: *mut u32, physical_devices: *mut libc::size_t) -> Result;
	fn vkGetDeviceProcAddr(device: libc::size_t, name: *const libc::c_char) -> VoidFunctionCallback;
	fn vkGetInstanceProcAddr(instance: libc::size_t, name: *const libc::c_char) -> VoidFunctionCallback;
	fn vkGetPhysicalDeviceProperties(physical_device: libc::size_t, properties: *mut PhysicalDeviceProperties);
	fn vkGetPhysicalDeviceQueueFamilyProperties(physical_device: libc::size_t, queue_family_property_count: *mut u32, queue_family_properties: *mut QueueFamilyProperties);
	fn vkGetPhysicalDeviceMemoryProperties(physical_device: libc::size_t, memory_properties: *mut PhysicalDeviceMemoryProperties);
	fn vkGetPhysicalDeviceFeatures(physical_device: libc::size_t, features: *mut PhysicalDeviceFeatures);
	fn vkGetPhysicalDeviceFormatProperties(physical_device: libc::size_t, format: Format, format_properties: *mut FormatProperties);
	fn vkGetPhysicalDeviceImageFormatProperties(physical_device: libc::size_t, format: Format, ty: ImageType, tiling: ImageTiling, usage: ImageUsageFlags, flags: ImageCreateFlags, image_format_properties: *mut ImageFormatProperties) -> Result;
	fn vkCreateDevice(physical_device: libc::size_t, create_info: *const DeviceCreateInfo, allocator: *const AllocationCallbacks, device: *mut libc::size_t) -> Result;
	fn vkDestroyDevice(device: libc::size_t, allocator: *const AllocationCallbacks);
	fn vkEnumerateInstanceLayerProperties(property_count: *mut u32, properties: *mut LayerProperties) -> Result;
	fn vkEnumerateInstanceExtensionProperties(layer_name: *const libc::c_char, property_count: *mut u32, properties: *mut ExtensionProperties) -> Result;
	fn vkEnumerateDeviceLayerProperties(physical_device: libc::size_t, property_count: *mut u32, properties: *mut LayerProperties) -> Result;
	fn vkEnumerateDeviceExtensionProperties(physical_device: libc::size_t, layer_name: *const libc::c_char, property_count: *mut u32, properties: *mut ExtensionProperties) -> Result;
	fn vkGetDeviceQueue(device: libc::size_t, queue_family_index: u32, queue_index: u32, queue: *mut libc::size_t);
	fn vkQueueSubmit(queue: libc::size_t, submit_count: u32, submits: *const SubmitInfo, fence: u64) -> Result;
	fn vkQueueWaitIdle(queue: libc::size_t) -> Result;
	fn vkDeviceWaitIdle(device: libc::size_t) -> Result;
	fn vkAllocateMemory(device: libc::size_t, allocate_info: *const MemoryAllocateInfo, allocator: *const AllocationCallbacks, memory: *mut u64) -> Result;
	fn vkFreeMemory(device: libc::size_t, memory: u64, allocator: *const AllocationCallbacks);
	fn vkMapMemory(device: libc::size_t, memory: u64, offset: DeviceSize, size: DeviceSize, flags: MemoryMapFlags, data: *const *const libc::c_void) -> Result;
	fn vkUnmapMemory(device: libc::size_t, memory: u64);
	fn vkFlushMappedMemoryRanges(device: libc::size_t, memory_range_count: u32, memory_ranges: *const MappedMemoryRange) -> Result;
	fn vkInvalidateMappedMemoryRanges(device: libc::size_t, memory_range_count: u32, memory_ranges: *const MappedMemoryRange) -> Result;
	fn vkGetDeviceMemoryCommitment(device: libc::size_t, memory: u64, committed_memory_in_bytes: *mut DeviceSize);
	fn vkGetBufferMemoryRequirements(device: libc::size_t, buffer: u64, memory_requirements: *mut MemoryRequirements);
	fn vkBindBufferMemory(device: libc::size_t, buffer: u64, memory: u64, memory_offset: DeviceSize) -> Result;
	fn vkGetImageMemoryRequirements(device: libc::size_t, image: u64, memory_requirements: *mut MemoryRequirements);
	fn vkBindImageMemory(device: libc::size_t, image: u64, memory: u64, memory_offset: DeviceSize) -> Result;
	fn vkGetImageSparseMemoryRequirements(device: libc::size_t, image: u64, sparse_memory_requirement_count: *mut u32, sparse_memory_requirements: *mut SparseImageMemoryRequirements);
	fn vkGetPhysicalDeviceSparseImageFormatProperties(physical_device: libc::size_t, format: Format, ty: ImageType, samples: SampleCountFlags, usage: ImageUsageFlags, tiling: ImageTiling, property_count: *mut u32, properties: *mut SparseImageFormatProperties);
	fn vkQueueBindSparse(queue: libc::size_t, bind_info_count: u32, bind_info: *const BindSparseInfo, fence: u64) -> Result;
	fn vkCreateFence(device: libc::size_t, create_info: *const FenceCreateInfo, allocator: *const AllocationCallbacks, fence: *mut u64) -> Result;
	fn vkDestroyFence(device: libc::size_t, fence: u64, allocator: *const AllocationCallbacks);
	fn vkResetFences(device: libc::size_t, fence_count: u32, fences: *const u64) -> Result;
	fn vkGetFenceStatus(device: libc::size_t, fence: u64) -> Result;
	fn vkWaitForFences(device: libc::size_t, fence_count: u32, fences: *const u64, wait_all: Bool32, timeout: u64) -> Result;
	fn vkCreateSemaphore(device: libc::size_t, create_info: *const SemaphoreCreateInfo, allocator: *const AllocationCallbacks, semaphore: *mut u64) -> Result;
	fn vkDestroySemaphore(device: libc::size_t, semaphore: u64, allocator: *const AllocationCallbacks);
	fn vkCreateEvent(device: libc::size_t, create_info: *const EventCreateInfo, allocator: *const AllocationCallbacks, event: *mut u64) -> Result;
	fn vkDestroyEvent(device: libc::size_t, event: u64, allocator: *const AllocationCallbacks);
	fn vkGetEventStatus(device: libc::size_t, event: u64) -> Result;
	fn vkSetEvent(device: libc::size_t, event: u64) -> Result;
	fn vkResetEvent(device: libc::size_t, event: u64) -> Result;
	fn vkCreateQueryPool(device: libc::size_t, create_info: *const QueryPoolCreateInfo, allocator: *const AllocationCallbacks, query_pool: *mut u64) -> Result;
	fn vkDestroyQueryPool(device: libc::size_t, query_pool: u64, allocator: *const AllocationCallbacks);
	fn vkGetQueryPoolResults(device: libc::size_t, query_pool: u64, first_query: u32, query_count: u32, data_size: libc::size_t, data: *mut libc::c_void, stride: DeviceSize, flags: QueryResultFlags) -> Result;
	fn vkCreateBuffer(device: libc::size_t, create_info: *const BufferCreateInfo, allocator: *const AllocationCallbacks, buffer: *mut u64) -> Result;
	fn vkDestroyBuffer(device: libc::size_t, buffer: u64, allocator: *const AllocationCallbacks);
	fn vkCreateBufferView(device: libc::size_t, create_info: *const BufferViewCreateInfo, allocator: *const AllocationCallbacks, view: *mut u64) -> Result;
	fn vkDestroyBufferView(device: libc::size_t, buffer_view: u64, allocator: *const AllocationCallbacks);
	fn vkCreateImage(device: libc::size_t, create_info: *const ImageCreateInfo, allocator: *const AllocationCallbacks, image: *mut u64) -> Result;
	fn vkDestroyImage(device: libc::size_t, image: u64, allocator: *const AllocationCallbacks);
	fn vkGetImageSubresourceLayout(device: libc::size_t, image: u64, subresource: *const ImageSubresource, layout: *mut SubresourceLayout);
	fn vkCreateImageView(device: libc::size_t, create_info: *const ImageViewCreateInfo, allocator: *const AllocationCallbacks, view: *mut u64) -> Result;
	fn vkDestroyImageView(device: libc::size_t, image_view: u64, allocator: *const AllocationCallbacks);
	fn vkCreateShaderModule(device: libc::size_t, create_info: *const ShaderModuleCreateInfo, allocator: *const AllocationCallbacks, shader_module: *mut u64) -> Result;
	fn vkDestroyShaderModule(device: libc::size_t, shader_module: u64, allocator: *const AllocationCallbacks);
	fn vkCreatePipelineCache(device: libc::size_t, create_info: *const PipelineCacheCreateInfo, allocator: *const AllocationCallbacks, pipeline_cache: *mut u64) -> Result;
	fn vkDestroyPipelineCache(device: libc::size_t, pipeline_cache: u64, allocator: *const AllocationCallbacks);
	fn vkGetPipelineCacheData(device: libc::size_t, pipeline_cache: u64, data_size: *mut libc::size_t, data: *mut libc::c_void) -> Result;
	fn vkMergePipelineCaches(device: libc::size_t, dst_cache: u64, src_cache_count: u32, src_caches: *const u64) -> Result;
	fn vkCreateGraphicsPipelines(device: libc::size_t, pipeline_cache: u64, create_info_count: u32, create_infos: *const GraphicsPipelineCreateInfo, allocator: *const AllocationCallbacks, pipelines: *mut u64) -> Result;
	fn vkCreateComputePipelines(device: libc::size_t, pipeline_cache: u64, create_info_count: u32, create_infos: *const ComputePipelineCreateInfo, allocator: *const AllocationCallbacks, pipelines: *mut u64) -> Result;
	fn vkDestroyPipeline(device: libc::size_t, pipeline: u64, allocator: *const AllocationCallbacks);
	fn vkCreatePipelineLayout(device: libc::size_t, create_info: *const PipelineLayoutCreateInfo, allocator: *const AllocationCallbacks, pipeline_layout: *mut u64) -> Result;
	fn vkDestroyPipelineLayout(device: libc::size_t, pipeline_layout: u64, allocator: *const AllocationCallbacks);
	fn vkCreateSampler(device: libc::size_t, create_info: *const SamplerCreateInfo, allocator: *const AllocationCallbacks, sampler: *mut u64) -> Result;
	fn vkDestroySampler(device: libc::size_t, sampler: u64, allocator: *const AllocationCallbacks);
	fn vkCreateDescriptorSetLayout(device: libc::size_t, create_info: *const DescriptorSetLayoutCreateInfo, allocator: *const AllocationCallbacks, set_layout: *mut u64) -> Result;
	fn vkDestroyDescriptorSetLayout(device: libc::size_t, descriptor_set_layout: u64, allocator: *const AllocationCallbacks);
	fn vkCreateDescriptorPool(device: libc::size_t, create_info: *const DescriptorPoolCreateInfo, allocator: *const AllocationCallbacks, descriptor_pool: *mut u64) -> Result;
	fn vkDestroyDescriptorPool(device: libc::size_t, descriptor_pool: u64, allocator: *const AllocationCallbacks);
	fn vkResetDescriptorPool(device: libc::size_t, descriptor_pool: u64, flags: DescriptorPoolResetFlags) -> Result;
	fn vkAllocateDescriptorSets(device: libc::size_t, allocate_info: *const DescriptorSetAllocateInfo, descriptor_sets: *mut u64) -> Result;
	fn vkFreeDescriptorSets(device: libc::size_t, descriptor_pool: u64, descriptor_set_count: u32, descriptor_sets: *const u64) -> Result;
	fn vkUpdateDescriptorSets(device: libc::size_t, descriptor_write_count: u32, descriptor_writes: *const WriteDescriptorSet, descriptor_copy_count: u32, descriptor_copies: *const CopyDescriptorSet);
	fn vkCreateFramebuffer(device: libc::size_t, create_info: *const FramebufferCreateInfo, allocator: *const AllocationCallbacks, framebuffer: *mut u64) -> Result;
	fn vkDestroyFramebuffer(device: libc::size_t, framebuffer: u64, allocator: *const AllocationCallbacks);
	fn vkCreateRenderPass(device: libc::size_t, create_info: *const RenderPassCreateInfo, allocator: *const AllocationCallbacks, render_pass: *mut u64) -> Result;
	fn vkDestroyRenderPass(device: libc::size_t, render_pass: u64, allocator: *const AllocationCallbacks);
	fn vkGetRenderAreaGranularity(device: libc::size_t, render_pass: u64, granularity: *mut Extent2D);
	fn vkCreateCommandPool(device: libc::size_t, create_info: *const CommandPoolCreateInfo, allocator: *const AllocationCallbacks, command_pool: *mut u64) -> Result;
	fn vkDestroyCommandPool(device: libc::size_t, command_pool: u64, allocator: *const AllocationCallbacks);
	fn vkResetCommandPool(device: libc::size_t, command_pool: u64, flags: CommandPoolResetFlags) -> Result;
	fn vkAllocateCommandBuffers(device: libc::size_t, allocate_info: *const CommandBufferAllocateInfo, command_buffers: *mut libc::size_t) -> Result;
	fn vkFreeCommandBuffers(device: libc::size_t, command_pool: u64, command_buffer_count: u32, command_buffers: *const libc::size_t);
	fn vkBeginCommandBuffer(command_buffer: libc::size_t, begin_info: *const CommandBufferBeginInfo) -> Result;
	fn vkEndCommandBuffer(command_buffer: libc::size_t) -> Result;
	fn vkResetCommandBuffer(command_buffer: libc::size_t, flags: CommandBufferResetFlags) -> Result;
	fn vkCmdBindPipeline(command_buffer: libc::size_t, pipeline_bind_point: PipelineBindPoint, pipeline: u64);
	fn vkCmdSetViewport(command_buffer: libc::size_t, first_viewport: u32, viewport_count: u32, viewports: *const Viewport);
	fn vkCmdSetScissor(command_buffer: libc::size_t, first_scissor: u32, scissor_count: u32, scissors: *const Rect2D);
	fn vkCmdSetLineWidth(command_buffer: libc::size_t, line_width: f32);
	fn vkCmdSetDepthBias(command_buffer: libc::size_t, depth_bias_constant_factor: f32, depth_bias_clamp: f32, depth_bias_slope_factor: f32);
	fn vkCmdSetBlendConstants(command_buffer: libc::size_t, blend_constants: f32);
	fn vkCmdSetDepthBounds(command_buffer: libc::size_t, min_depth_bounds: f32, max_depth_bounds: f32);
	fn vkCmdSetStencilCompareMask(command_buffer: libc::size_t, face_mask: StencilFaceFlags, compare_mask: u32);
	fn vkCmdSetStencilWriteMask(command_buffer: libc::size_t, face_mask: StencilFaceFlags, write_mask: u32);
	fn vkCmdSetStencilReference(command_buffer: libc::size_t, face_mask: StencilFaceFlags, reference: u32);
	fn vkCmdBindDescriptorSets(command_buffer: libc::size_t, pipeline_bind_point: PipelineBindPoint, layout: u64, first_set: u32, descriptor_set_count: u32, descriptor_sets: *const u64, dynamic_offset_count: u32, dynamic_offsets: *const u32);
	fn vkCmdBindIndexBuffer(command_buffer: libc::size_t, buffer: u64, offset: DeviceSize, index_type: IndexType);
	fn vkCmdBindVertexBuffers(command_buffer: libc::size_t, first_binding: u32, binding_count: u32, buffers: *const u64, offsets: *const DeviceSize);
	fn vkCmdDraw(command_buffer: libc::size_t, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32);
	fn vkCmdDrawIndexed(command_buffer: libc::size_t, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32);
	fn vkCmdDrawIndirect(command_buffer: libc::size_t, buffer: u64, offset: DeviceSize, draw_count: u32, stride: u32);
	fn vkCmdDrawIndexedIndirect(command_buffer: libc::size_t, buffer: u64, offset: DeviceSize, draw_count: u32, stride: u32);
	fn vkCmdDispatch(command_buffer: libc::size_t, x: u32, y: u32, z: u32);
	fn vkCmdDispatchIndirect(command_buffer: libc::size_t, buffer: u64, offset: DeviceSize);
	fn vkCmdCopyBuffer(command_buffer: libc::size_t, src_buffer: u64, dst_buffer: u64, region_count: u32, regions: *const BufferCopy);
	fn vkCmdCopyImage(command_buffer: libc::size_t, src_image: u64, src_image_layout: ImageLayout, dst_image: u64, dst_image_layout: ImageLayout, region_count: u32, regions: *const ImageCopy);
	fn vkCmdBlitImage(command_buffer: libc::size_t, src_image: u64, src_image_layout: ImageLayout, dst_image: u64, dst_image_layout: ImageLayout, region_count: u32, regions: *const ImageBlit, filter: Filter);
	fn vkCmdCopyBufferToImage(command_buffer: libc::size_t, src_buffer: u64, dst_image: u64, dst_image_layout: ImageLayout, region_count: u32, regions: *const BufferImageCopy);
	fn vkCmdCopyImageToBuffer(command_buffer: libc::size_t, src_image: u64, src_image_layout: ImageLayout, dst_buffer: u64, region_count: u32, regions: *const BufferImageCopy);
	fn vkCmdUpdateBuffer(command_buffer: libc::size_t, dst_buffer: u64, dst_offset: DeviceSize, data_size: DeviceSize, data: *const u32);
	fn vkCmdFillBuffer(command_buffer: libc::size_t, dst_buffer: u64, dst_offset: DeviceSize, size: DeviceSize, data: u32);
	fn vkCmdClearColorImage(command_buffer: libc::size_t, image: u64, image_layout: ImageLayout, color: *const libc::size_t, range_count: u32, ranges: *const ImageSubresourceRange);
	fn vkCmdClearDepthStencilImage(command_buffer: libc::size_t, image: u64, image_layout: ImageLayout, depth_stencil: *const ClearDepthStencilValue, range_count: u32, ranges: *const ImageSubresourceRange);
	fn vkCmdClearAttachments(command_buffer: libc::size_t, attachment_count: u32, attachments: *const ClearAttachment, rect_count: u32, rects: *const ClearRect);
	fn vkCmdResolveImage(command_buffer: libc::size_t, src_image: u64, src_image_layout: ImageLayout, dst_image: u64, dst_image_layout: ImageLayout, region_count: u32, regions: *const ImageResolve);
	fn vkCmdSetEvent(command_buffer: libc::size_t, event: u64, stage_mask: PipelineStageFlags);
	fn vkCmdResetEvent(command_buffer: libc::size_t, event: u64, stage_mask: PipelineStageFlags);
	fn vkCmdWaitEvents(command_buffer: libc::size_t, event_count: u32, events: *const u64, src_stage_mask: PipelineStageFlags, dst_stage_mask: PipelineStageFlags, memory_barrier_count: u32, memory_barriers: *const MemoryBarrier, buffer_memory_barrier_count: u32, buffer_memory_barriers: *const BufferMemoryBarrier, image_memory_barrier_count: u32, image_memory_barriers: *const ImageMemoryBarrier);
	fn vkCmdPipelineBarrier(command_buffer: libc::size_t, src_stage_mask: PipelineStageFlags, dst_stage_mask: PipelineStageFlags, dependency_flags: DependencyFlags, memory_barrier_count: u32, memory_barriers: *const MemoryBarrier, buffer_memory_barrier_count: u32, buffer_memory_barriers: *const BufferMemoryBarrier, image_memory_barrier_count: u32, image_memory_barriers: *const ImageMemoryBarrier);
	fn vkCmdBeginQuery(command_buffer: libc::size_t, query_pool: u64, query: u32, flags: QueryControlFlags);
	fn vkCmdEndQuery(command_buffer: libc::size_t, query_pool: u64, query: u32);
	fn vkCmdResetQueryPool(command_buffer: libc::size_t, query_pool: u64, first_query: u32, query_count: u32);
	fn vkCmdWriteTimestamp(command_buffer: libc::size_t, pipeline_stage: PipelineStageFlags, query_pool: u64, query: u32);
	fn vkCmdCopyQueryPoolResults(command_buffer: libc::size_t, query_pool: u64, first_query: u32, query_count: u32, dst_buffer: u64, dst_offset: DeviceSize, stride: DeviceSize, flags: QueryResultFlags);
	fn vkCmdPushConstants(command_buffer: libc::size_t, layout: u64, stage_flags: ShaderStageFlags, offset: u32, size: u32, values: *const libc::c_void);
	fn vkCmdBeginRenderPass(command_buffer: libc::size_t, render_pass_begin: *const RenderPassBeginInfo, contents: SubpassContents);
	fn vkCmdNextSubpass(command_buffer: libc::size_t, contents: SubpassContents);
	fn vkCmdEndRenderPass(command_buffer: libc::size_t);
	fn vkCmdExecuteCommands(command_buffer: libc::size_t, command_buffer_count: u32, command_buffers: *const libc::size_t);
	#[cfg(vk_khr_android_surface)]
	fn vkCreateAndroidSurfaceKHR(instance: libc::size_t, create_info: *const AndroidSurfaceCreateInfoKHR, allocator: *const AllocationCallbacks, surface: *mut u64) -> Result;
	#[cfg(vk_khr_display)]
	fn vkGetPhysicalDeviceDisplayPropertiesKHR(physical_device: libc::size_t, property_count: *mut u32, properties: *mut DisplayPropertiesKHR) -> Result;
	#[cfg(vk_khr_display)]
	fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physical_device: libc::size_t, property_count: *mut u32, properties: *mut DisplayPlanePropertiesKHR) -> Result;
	#[cfg(vk_khr_display)]
	fn vkGetDisplayPlaneSupportedDisplaysKHR(physical_device: libc::size_t, plane_index: u32, display_count: *mut u32, displays: *mut u64) -> Result;
	#[cfg(vk_khr_display)]
	fn vkGetDisplayModePropertiesKHR(physical_device: libc::size_t, display: u64, property_count: *mut u32, properties: *mut DisplayModePropertiesKHR) -> Result;
	#[cfg(vk_khr_display)]
	fn vkCreateDisplayModeKHR(physical_device: libc::size_t, display: u64, create_info: *const DisplayModeCreateInfoKHR, allocator: *const AllocationCallbacks, mode: *mut u64) -> Result;
	#[cfg(vk_khr_display)]
	fn vkGetDisplayPlaneCapabilitiesKHR(physical_device: libc::size_t, mode: u64, plane_index: u32, capabilities: *mut DisplayPlaneCapabilitiesKHR) -> Result;
	#[cfg(vk_khr_display)]
	fn vkCreateDisplayPlaneSurfaceKHR(instance: libc::size_t, create_info: *const DisplaySurfaceCreateInfoKHR, allocator: *const AllocationCallbacks, surface: *mut u64) -> Result;
	#[cfg(vk_khr_display_swapchain)]
	fn vkCreateSharedSwapchainsKHR(device: libc::size_t, swapchain_count: u32, create_infos: *const SwapchainCreateInfoKHR, allocator: *const AllocationCallbacks, swapchains: *mut u64) -> Result;
	#[cfg(vk_khr_mir_surface)]
	fn vkCreateMirSurfaceKHR(instance: libc::size_t, create_info: *const MirSurfaceCreateInfoKHR, allocator: *const AllocationCallbacks, surface: *mut u64) -> Result;
	#[cfg(vk_khr_mir_surface)]
	fn vkGetPhysicalDeviceMirPresentationSupportKHR(physical_device: libc::size_t, queue_family_index: u32, connection: libc::size_t) -> Bool32;
	#[cfg(vk_khr_surface)]
	fn vkDestroySurfaceKHR(instance: libc::size_t, surface: u64, allocator: *const AllocationCallbacks);
	#[cfg(vk_khr_surface)]
	fn vkGetPhysicalDeviceSurfaceSupportKHR(physical_device: libc::size_t, queue_family_index: u32, surface: u64, supported: *mut Bool32) -> Result;
	#[cfg(vk_khr_surface)]
	fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device: libc::size_t, surface: u64, surface_capabilities: *mut SurfaceCapabilitiesKHR) -> Result;
	#[cfg(vk_khr_surface)]
	fn vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device: libc::size_t, surface: u64, surface_format_count: *mut u32, surface_formats: *mut SurfaceFormatKHR) -> Result;
	#[cfg(vk_khr_surface)]
	fn vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device: libc::size_t, surface: u64, present_mode_count: *mut u32, present_modes: *mut PresentModeKHR) -> Result;
	#[cfg(vk_khr_swapchain)]
	fn vkCreateSwapchainKHR(device: libc::size_t, create_info: *const SwapchainCreateInfoKHR, allocator: *const AllocationCallbacks, swapchain: *mut u64) -> Result;
	#[cfg(vk_khr_swapchain)]
	fn vkDestroySwapchainKHR(device: libc::size_t, swapchain: u64, allocator: *const AllocationCallbacks);
	#[cfg(vk_khr_swapchain)]
	fn vkGetSwapchainImagesKHR(device: libc::size_t, swapchain: u64, swapchain_image_count: *mut u32, swapchain_images: *mut u64) -> Result;
	#[cfg(vk_khr_swapchain)]
	fn vkAcquireNextImageKHR(device: libc::size_t, swapchain: u64, timeout: u64, semaphore: u64, fence: u64, image_index: *mut u32) -> Result;
	#[cfg(vk_khr_swapchain)]
	fn vkQueuePresentKHR(queue: libc::size_t, present_info: *const PresentInfoKHR) -> Result;
	#[cfg(vk_khr_wayland_surface)]
	fn vkCreateWaylandSurfaceKHR(instance: libc::size_t, create_info: *const WaylandSurfaceCreateInfoKHR, allocator: *const AllocationCallbacks, surface: *mut u64) -> Result;
	#[cfg(vk_khr_wayland_surface)]
	fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(physical_device: libc::size_t, queue_family_index: u32, display: libc::size_t) -> Bool32;
	#[cfg(vk_khr_win32_surface)]
	fn vkCreateWin32SurfaceKHR(instance: libc::size_t, create_info: *const Win32SurfaceCreateInfoKHR, allocator: *const AllocationCallbacks, surface: *mut u64) -> Result;
	#[cfg(vk_khr_win32_surface)]
	fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physical_device: libc::size_t, queue_family_index: u32) -> Bool32;
	#[cfg(vk_khr_xlib_surface)]
	fn vkCreateXlibSurfaceKHR(instance: libc::size_t, create_info: *const XlibSurfaceCreateInfoKHR, allocator: *const AllocationCallbacks, surface: *mut u64) -> Result;
	#[cfg(vk_khr_xlib_surface)]
	fn vkGetPhysicalDeviceXlibPresentationSupportKHR(physical_device: libc::size_t, queue_family_index: u32, dpy: libc::size_t, visual_id: libc::size_t) -> Bool32;
	#[cfg(vk_khr_xcb_surface)]
	fn vkCreateXcbSurfaceKHR(instance: libc::size_t, create_info: *const XcbSurfaceCreateInfoKHR, allocator: *const AllocationCallbacks, surface: *mut u64) -> Result;
	#[cfg(vk_khr_xcb_surface)]
	fn vkGetPhysicalDeviceXcbPresentationSupportKHR(physical_device: libc::size_t, queue_family_index: u32, connection: libc::size_t, visual_id: libc::size_t) -> Bool32;
	#[cfg(vk_ext_debug_report)]
	fn vkCreateDebugReportCallbackEXT(instance: libc::size_t, create_info: *const DebugReportCallbackCreateInfoEXT, allocator: *const AllocationCallbacks, callback: *mut u64) -> Result;
	#[cfg(vk_ext_debug_report)]
	fn vkDestroyDebugReportCallbackEXT(instance: libc::size_t, callback: u64, allocator: *const AllocationCallbacks);
	#[cfg(vk_ext_debug_report)]
	fn vkDebugReportMessageEXT(instance: libc::size_t, flags: DebugReportFlagsEXT, object_type: DebugReportObjectTypeEXT, object: u64, location: libc::size_t, message_code: i32, layer_prefix: *const libc::c_char, message: *const libc::c_char);
}

pub unsafe fn create_instance(create_info: *const InstanceCreateInfo, allocator: *const AllocationCallbacks, instance: *mut libc::size_t) -> Result {
	return vkCreateInstance(create_info, allocator, instance);
}

pub unsafe fn destroy_instance(instance: libc::size_t, allocator: *const AllocationCallbacks) {
	return vkDestroyInstance(instance, allocator);
}

pub unsafe fn enumerate_physical_devices(instance: libc::size_t, physical_device_count: *mut u32, physical_devices: *mut libc::size_t) -> Result {
	return vkEnumeratePhysicalDevices(instance, physical_device_count, physical_devices);
}

pub unsafe fn get_device_proc_addr(device: libc::size_t, name: *const libc::c_char) -> VoidFunctionCallback {
	return vkGetDeviceProcAddr(device, name);
}

pub unsafe fn get_instance_proc_addr(instance: libc::size_t, name: *const libc::c_char) -> VoidFunctionCallback {
	return vkGetInstanceProcAddr(instance, name);
}

pub unsafe fn get_physical_device_properties(physical_device: libc::size_t, properties: *mut PhysicalDeviceProperties) {
	return vkGetPhysicalDeviceProperties(physical_device, properties);
}

pub unsafe fn get_physical_device_queue_family_properties(physical_device: libc::size_t, queue_family_property_count: *mut u32, queue_family_properties: *mut QueueFamilyProperties) {
	return vkGetPhysicalDeviceQueueFamilyProperties(physical_device, queue_family_property_count, queue_family_properties);
}

pub unsafe fn get_physical_device_memory_properties(physical_device: libc::size_t, memory_properties: *mut PhysicalDeviceMemoryProperties) {
	return vkGetPhysicalDeviceMemoryProperties(physical_device, memory_properties);
}

pub unsafe fn get_physical_device_features(physical_device: libc::size_t, features: *mut PhysicalDeviceFeatures) {
	return vkGetPhysicalDeviceFeatures(physical_device, features);
}

pub unsafe fn get_physical_device_format_properties(physical_device: libc::size_t, format: Format, format_properties: *mut FormatProperties) {
	return vkGetPhysicalDeviceFormatProperties(physical_device, format, format_properties);
}

pub unsafe fn get_physical_device_image_format_properties(physical_device: libc::size_t, format: Format, ty: ImageType, tiling: ImageTiling, usage: ImageUsageFlags, flags: ImageCreateFlags, image_format_properties: *mut ImageFormatProperties) -> Result {
	return vkGetPhysicalDeviceImageFormatProperties(physical_device, format, ty, tiling, usage, flags, image_format_properties);
}

pub unsafe fn create_device(physical_device: libc::size_t, create_info: *const DeviceCreateInfo, allocator: *const AllocationCallbacks, device: *mut libc::size_t) -> Result {
	return vkCreateDevice(physical_device, create_info, allocator, device);
}

pub unsafe fn destroy_device(device: libc::size_t, allocator: *const AllocationCallbacks) {
	return vkDestroyDevice(device, allocator);
}

pub unsafe fn enumerate_instance_layer_properties(property_count: *mut u32, properties: *mut LayerProperties) -> Result {
	return vkEnumerateInstanceLayerProperties(property_count, properties);
}

pub unsafe fn enumerate_instance_extension_properties(layer_name: *const libc::c_char, property_count: *mut u32, properties: *mut ExtensionProperties) -> Result {
	return vkEnumerateInstanceExtensionProperties(layer_name, property_count, properties);
}

pub unsafe fn enumerate_device_layer_properties(physical_device: libc::size_t, property_count: *mut u32, properties: *mut LayerProperties) -> Result {
	return vkEnumerateDeviceLayerProperties(physical_device, property_count, properties);
}

pub unsafe fn enumerate_device_extension_properties(physical_device: libc::size_t, layer_name: *const libc::c_char, property_count: *mut u32, properties: *mut ExtensionProperties) -> Result {
	return vkEnumerateDeviceExtensionProperties(physical_device, layer_name, property_count, properties);
}

pub unsafe fn get_device_queue(device: libc::size_t, queue_family_index: u32, queue_index: u32, queue: *mut libc::size_t) {
	return vkGetDeviceQueue(device, queue_family_index, queue_index, queue);
}

pub unsafe fn queue_submit(queue: libc::size_t, submit_count: u32, submits: *const SubmitInfo, fence: u64) -> Result {
	return vkQueueSubmit(queue, submit_count, submits, fence);
}

pub unsafe fn queue_wait_idle(queue: libc::size_t) -> Result {
	return vkQueueWaitIdle(queue);
}

pub unsafe fn device_wait_idle(device: libc::size_t) -> Result {
	return vkDeviceWaitIdle(device);
}

pub unsafe fn allocate_memory(device: libc::size_t, allocate_info: *const MemoryAllocateInfo, allocator: *const AllocationCallbacks, memory: *mut u64) -> Result {
	return vkAllocateMemory(device, allocate_info, allocator, memory);
}

pub unsafe fn free_memory(device: libc::size_t, memory: u64, allocator: *const AllocationCallbacks) {
	return vkFreeMemory(device, memory, allocator);
}

pub unsafe fn map_memory(device: libc::size_t, memory: u64, offset: DeviceSize, size: DeviceSize, flags: MemoryMapFlags, data: *const *const libc::c_void) -> Result {
	return vkMapMemory(device, memory, offset, size, flags, data);
}

pub unsafe fn unmap_memory(device: libc::size_t, memory: u64) {
	return vkUnmapMemory(device, memory);
}

pub unsafe fn flush_mapped_memory_ranges(device: libc::size_t, memory_range_count: u32, memory_ranges: *const MappedMemoryRange) -> Result {
	return vkFlushMappedMemoryRanges(device, memory_range_count, memory_ranges);
}

pub unsafe fn invalidate_mapped_memory_ranges(device: libc::size_t, memory_range_count: u32, memory_ranges: *const MappedMemoryRange) -> Result {
	return vkInvalidateMappedMemoryRanges(device, memory_range_count, memory_ranges);
}

pub unsafe fn get_device_memory_commitment(device: libc::size_t, memory: u64, committed_memory_in_bytes: *mut DeviceSize) {
	return vkGetDeviceMemoryCommitment(device, memory, committed_memory_in_bytes);
}

pub unsafe fn get_buffer_memory_requirements(device: libc::size_t, buffer: u64, memory_requirements: *mut MemoryRequirements) {
	return vkGetBufferMemoryRequirements(device, buffer, memory_requirements);
}

pub unsafe fn bind_buffer_memory(device: libc::size_t, buffer: u64, memory: u64, memory_offset: DeviceSize) -> Result {
	return vkBindBufferMemory(device, buffer, memory, memory_offset);
}

pub unsafe fn get_image_memory_requirements(device: libc::size_t, image: u64, memory_requirements: *mut MemoryRequirements) {
	return vkGetImageMemoryRequirements(device, image, memory_requirements);
}

pub unsafe fn bind_image_memory(device: libc::size_t, image: u64, memory: u64, memory_offset: DeviceSize) -> Result {
	return vkBindImageMemory(device, image, memory, memory_offset);
}

pub unsafe fn get_image_sparse_memory_requirements(device: libc::size_t, image: u64, sparse_memory_requirement_count: *mut u32, sparse_memory_requirements: *mut SparseImageMemoryRequirements) {
	return vkGetImageSparseMemoryRequirements(device, image, sparse_memory_requirement_count, sparse_memory_requirements);
}

pub unsafe fn get_physical_device_sparse_image_format_properties(physical_device: libc::size_t, format: Format, ty: ImageType, samples: SampleCountFlags, usage: ImageUsageFlags, tiling: ImageTiling, property_count: *mut u32, properties: *mut SparseImageFormatProperties) {
	return vkGetPhysicalDeviceSparseImageFormatProperties(physical_device, format, ty, samples, usage, tiling, property_count, properties);
}

pub unsafe fn queue_bind_sparse(queue: libc::size_t, bind_info_count: u32, bind_info: *const BindSparseInfo, fence: u64) -> Result {
	return vkQueueBindSparse(queue, bind_info_count, bind_info, fence);
}

pub unsafe fn create_fence(device: libc::size_t, create_info: *const FenceCreateInfo, allocator: *const AllocationCallbacks, fence: *mut u64) -> Result {
	return vkCreateFence(device, create_info, allocator, fence);
}

pub unsafe fn destroy_fence(device: libc::size_t, fence: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyFence(device, fence, allocator);
}

pub unsafe fn reset_fences(device: libc::size_t, fence_count: u32, fences: *const u64) -> Result {
	return vkResetFences(device, fence_count, fences);
}

pub unsafe fn get_fence_status(device: libc::size_t, fence: u64) -> Result {
	return vkGetFenceStatus(device, fence);
}

pub unsafe fn wait_for_fences(device: libc::size_t, fence_count: u32, fences: *const u64, wait_all: Bool32, timeout: u64) -> Result {
	return vkWaitForFences(device, fence_count, fences, wait_all, timeout);
}

pub unsafe fn create_semaphore(device: libc::size_t, create_info: *const SemaphoreCreateInfo, allocator: *const AllocationCallbacks, semaphore: *mut u64) -> Result {
	return vkCreateSemaphore(device, create_info, allocator, semaphore);
}

pub unsafe fn destroy_semaphore(device: libc::size_t, semaphore: u64, allocator: *const AllocationCallbacks) {
	return vkDestroySemaphore(device, semaphore, allocator);
}

pub unsafe fn create_event(device: libc::size_t, create_info: *const EventCreateInfo, allocator: *const AllocationCallbacks, event: *mut u64) -> Result {
	return vkCreateEvent(device, create_info, allocator, event);
}

pub unsafe fn destroy_event(device: libc::size_t, event: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyEvent(device, event, allocator);
}

pub unsafe fn get_event_status(device: libc::size_t, event: u64) -> Result {
	return vkGetEventStatus(device, event);
}

pub unsafe fn set_event(device: libc::size_t, event: u64) -> Result {
	return vkSetEvent(device, event);
}

pub unsafe fn reset_event(device: libc::size_t, event: u64) -> Result {
	return vkResetEvent(device, event);
}

pub unsafe fn create_query_pool(device: libc::size_t, create_info: *const QueryPoolCreateInfo, allocator: *const AllocationCallbacks, query_pool: *mut u64) -> Result {
	return vkCreateQueryPool(device, create_info, allocator, query_pool);
}

pub unsafe fn destroy_query_pool(device: libc::size_t, query_pool: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyQueryPool(device, query_pool, allocator);
}

pub unsafe fn get_query_pool_results(device: libc::size_t, query_pool: u64, first_query: u32, query_count: u32, data_size: libc::size_t, data: *mut libc::c_void, stride: DeviceSize, flags: QueryResultFlags) -> Result {
	return vkGetQueryPoolResults(device, query_pool, first_query, query_count, data_size, data, stride, flags);
}

pub unsafe fn create_buffer(device: libc::size_t, create_info: *const BufferCreateInfo, allocator: *const AllocationCallbacks, buffer: *mut u64) -> Result {
	return vkCreateBuffer(device, create_info, allocator, buffer);
}

pub unsafe fn destroy_buffer(device: libc::size_t, buffer: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyBuffer(device, buffer, allocator);
}

pub unsafe fn create_buffer_view(device: libc::size_t, create_info: *const BufferViewCreateInfo, allocator: *const AllocationCallbacks, view: *mut u64) -> Result {
	return vkCreateBufferView(device, create_info, allocator, view);
}

pub unsafe fn destroy_buffer_view(device: libc::size_t, buffer_view: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyBufferView(device, buffer_view, allocator);
}

pub unsafe fn create_image(device: libc::size_t, create_info: *const ImageCreateInfo, allocator: *const AllocationCallbacks, image: *mut u64) -> Result {
	return vkCreateImage(device, create_info, allocator, image);
}

pub unsafe fn destroy_image(device: libc::size_t, image: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyImage(device, image, allocator);
}

pub unsafe fn get_image_subresource_layout(device: libc::size_t, image: u64, subresource: *const ImageSubresource, layout: *mut SubresourceLayout) {
	return vkGetImageSubresourceLayout(device, image, subresource, layout);
}

pub unsafe fn create_image_view(device: libc::size_t, create_info: *const ImageViewCreateInfo, allocator: *const AllocationCallbacks, view: *mut u64) -> Result {
	return vkCreateImageView(device, create_info, allocator, view);
}

pub unsafe fn destroy_image_view(device: libc::size_t, image_view: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyImageView(device, image_view, allocator);
}

pub unsafe fn create_shader_module(device: libc::size_t, create_info: *const ShaderModuleCreateInfo, allocator: *const AllocationCallbacks, shader_module: *mut u64) -> Result {
	return vkCreateShaderModule(device, create_info, allocator, shader_module);
}

pub unsafe fn destroy_shader_module(device: libc::size_t, shader_module: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyShaderModule(device, shader_module, allocator);
}

pub unsafe fn create_pipeline_cache(device: libc::size_t, create_info: *const PipelineCacheCreateInfo, allocator: *const AllocationCallbacks, pipeline_cache: *mut u64) -> Result {
	return vkCreatePipelineCache(device, create_info, allocator, pipeline_cache);
}

pub unsafe fn destroy_pipeline_cache(device: libc::size_t, pipeline_cache: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyPipelineCache(device, pipeline_cache, allocator);
}

pub unsafe fn get_pipeline_cache_data(device: libc::size_t, pipeline_cache: u64, data_size: *mut libc::size_t, data: *mut libc::c_void) -> Result {
	return vkGetPipelineCacheData(device, pipeline_cache, data_size, data);
}

pub unsafe fn merge_pipeline_caches(device: libc::size_t, dst_cache: u64, src_cache_count: u32, src_caches: *const u64) -> Result {
	return vkMergePipelineCaches(device, dst_cache, src_cache_count, src_caches);
}

pub unsafe fn create_graphics_pipelines(device: libc::size_t, pipeline_cache: u64, create_info_count: u32, create_infos: *const GraphicsPipelineCreateInfo, allocator: *const AllocationCallbacks, pipelines: *mut u64) -> Result {
	return vkCreateGraphicsPipelines(device, pipeline_cache, create_info_count, create_infos, allocator, pipelines);
}

pub unsafe fn create_compute_pipelines(device: libc::size_t, pipeline_cache: u64, create_info_count: u32, create_infos: *const ComputePipelineCreateInfo, allocator: *const AllocationCallbacks, pipelines: *mut u64) -> Result {
	return vkCreateComputePipelines(device, pipeline_cache, create_info_count, create_infos, allocator, pipelines);
}

pub unsafe fn destroy_pipeline(device: libc::size_t, pipeline: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyPipeline(device, pipeline, allocator);
}

pub unsafe fn create_pipeline_layout(device: libc::size_t, create_info: *const PipelineLayoutCreateInfo, allocator: *const AllocationCallbacks, pipeline_layout: *mut u64) -> Result {
	return vkCreatePipelineLayout(device, create_info, allocator, pipeline_layout);
}

pub unsafe fn destroy_pipeline_layout(device: libc::size_t, pipeline_layout: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyPipelineLayout(device, pipeline_layout, allocator);
}

pub unsafe fn create_sampler(device: libc::size_t, create_info: *const SamplerCreateInfo, allocator: *const AllocationCallbacks, sampler: *mut u64) -> Result {
	return vkCreateSampler(device, create_info, allocator, sampler);
}

pub unsafe fn destroy_sampler(device: libc::size_t, sampler: u64, allocator: *const AllocationCallbacks) {
	return vkDestroySampler(device, sampler, allocator);
}

pub unsafe fn create_descriptor_set_layout(device: libc::size_t, create_info: *const DescriptorSetLayoutCreateInfo, allocator: *const AllocationCallbacks, set_layout: *mut u64) -> Result {
	return vkCreateDescriptorSetLayout(device, create_info, allocator, set_layout);
}

pub unsafe fn destroy_descriptor_set_layout(device: libc::size_t, descriptor_set_layout: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyDescriptorSetLayout(device, descriptor_set_layout, allocator);
}

pub unsafe fn create_descriptor_pool(device: libc::size_t, create_info: *const DescriptorPoolCreateInfo, allocator: *const AllocationCallbacks, descriptor_pool: *mut u64) -> Result {
	return vkCreateDescriptorPool(device, create_info, allocator, descriptor_pool);
}

pub unsafe fn destroy_descriptor_pool(device: libc::size_t, descriptor_pool: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyDescriptorPool(device, descriptor_pool, allocator);
}

pub unsafe fn reset_descriptor_pool(device: libc::size_t, descriptor_pool: u64, flags: DescriptorPoolResetFlags) -> Result {
	return vkResetDescriptorPool(device, descriptor_pool, flags);
}

pub unsafe fn allocate_descriptor_sets(device: libc::size_t, allocate_info: *const DescriptorSetAllocateInfo, descriptor_sets: *mut u64) -> Result {
	return vkAllocateDescriptorSets(device, allocate_info, descriptor_sets);
}

pub unsafe fn free_descriptor_sets(device: libc::size_t, descriptor_pool: u64, descriptor_set_count: u32, descriptor_sets: *const u64) -> Result {
	return vkFreeDescriptorSets(device, descriptor_pool, descriptor_set_count, descriptor_sets);
}

pub unsafe fn update_descriptor_sets(device: libc::size_t, descriptor_write_count: u32, descriptor_writes: *const WriteDescriptorSet, descriptor_copy_count: u32, descriptor_copies: *const CopyDescriptorSet) {
	return vkUpdateDescriptorSets(device, descriptor_write_count, descriptor_writes, descriptor_copy_count, descriptor_copies);
}

pub unsafe fn create_framebuffer(device: libc::size_t, create_info: *const FramebufferCreateInfo, allocator: *const AllocationCallbacks, framebuffer: *mut u64) -> Result {
	return vkCreateFramebuffer(device, create_info, allocator, framebuffer);
}

pub unsafe fn destroy_framebuffer(device: libc::size_t, framebuffer: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyFramebuffer(device, framebuffer, allocator);
}

pub unsafe fn create_render_pass(device: libc::size_t, create_info: *const RenderPassCreateInfo, allocator: *const AllocationCallbacks, render_pass: *mut u64) -> Result {
	return vkCreateRenderPass(device, create_info, allocator, render_pass);
}

pub unsafe fn destroy_render_pass(device: libc::size_t, render_pass: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyRenderPass(device, render_pass, allocator);
}

pub unsafe fn get_render_area_granularity(device: libc::size_t, render_pass: u64, granularity: *mut Extent2D) {
	return vkGetRenderAreaGranularity(device, render_pass, granularity);
}

pub unsafe fn create_command_pool(device: libc::size_t, create_info: *const CommandPoolCreateInfo, allocator: *const AllocationCallbacks, command_pool: *mut u64) -> Result {
	return vkCreateCommandPool(device, create_info, allocator, command_pool);
}

pub unsafe fn destroy_command_pool(device: libc::size_t, command_pool: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyCommandPool(device, command_pool, allocator);
}

pub unsafe fn reset_command_pool(device: libc::size_t, command_pool: u64, flags: CommandPoolResetFlags) -> Result {
	return vkResetCommandPool(device, command_pool, flags);
}

pub unsafe fn allocate_command_buffers(device: libc::size_t, allocate_info: *const CommandBufferAllocateInfo, command_buffers: *mut libc::size_t) -> Result {
	return vkAllocateCommandBuffers(device, allocate_info, command_buffers);
}

pub unsafe fn free_command_buffers(device: libc::size_t, command_pool: u64, command_buffer_count: u32, command_buffers: *const libc::size_t) {
	return vkFreeCommandBuffers(device, command_pool, command_buffer_count, command_buffers);
}

pub unsafe fn begin_command_buffer(command_buffer: libc::size_t, begin_info: *const CommandBufferBeginInfo) -> Result {
	return vkBeginCommandBuffer(command_buffer, begin_info);
}

pub unsafe fn end_command_buffer(command_buffer: libc::size_t) -> Result {
	return vkEndCommandBuffer(command_buffer);
}

pub unsafe fn reset_command_buffer(command_buffer: libc::size_t, flags: CommandBufferResetFlags) -> Result {
	return vkResetCommandBuffer(command_buffer, flags);
}

pub unsafe fn cmd_bind_pipeline(command_buffer: libc::size_t, pipeline_bind_point: PipelineBindPoint, pipeline: u64) {
	return vkCmdBindPipeline(command_buffer, pipeline_bind_point, pipeline);
}

pub unsafe fn cmd_set_viewport(command_buffer: libc::size_t, first_viewport: u32, viewport_count: u32, viewports: *const Viewport) {
	return vkCmdSetViewport(command_buffer, first_viewport, viewport_count, viewports);
}

pub unsafe fn cmd_set_scissor(command_buffer: libc::size_t, first_scissor: u32, scissor_count: u32, scissors: *const Rect2D) {
	return vkCmdSetScissor(command_buffer, first_scissor, scissor_count, scissors);
}

pub unsafe fn cmd_set_line_width(command_buffer: libc::size_t, line_width: f32) {
	return vkCmdSetLineWidth(command_buffer, line_width);
}

pub unsafe fn cmd_set_depth_bias(command_buffer: libc::size_t, depth_bias_constant_factor: f32, depth_bias_clamp: f32, depth_bias_slope_factor: f32) {
	return vkCmdSetDepthBias(command_buffer, depth_bias_constant_factor, depth_bias_clamp, depth_bias_slope_factor);
}

pub unsafe fn cmd_set_blend_constants(command_buffer: libc::size_t, blend_constants: f32) {
	return vkCmdSetBlendConstants(command_buffer, blend_constants);
}

pub unsafe fn cmd_set_depth_bounds(command_buffer: libc::size_t, min_depth_bounds: f32, max_depth_bounds: f32) {
	return vkCmdSetDepthBounds(command_buffer, min_depth_bounds, max_depth_bounds);
}

pub unsafe fn cmd_set_stencil_compare_mask(command_buffer: libc::size_t, face_mask: StencilFaceFlags, compare_mask: u32) {
	return vkCmdSetStencilCompareMask(command_buffer, face_mask, compare_mask);
}

pub unsafe fn cmd_set_stencil_write_mask(command_buffer: libc::size_t, face_mask: StencilFaceFlags, write_mask: u32) {
	return vkCmdSetStencilWriteMask(command_buffer, face_mask, write_mask);
}

pub unsafe fn cmd_set_stencil_reference(command_buffer: libc::size_t, face_mask: StencilFaceFlags, reference: u32) {
	return vkCmdSetStencilReference(command_buffer, face_mask, reference);
}

pub unsafe fn cmd_bind_descriptor_sets(command_buffer: libc::size_t, pipeline_bind_point: PipelineBindPoint, layout: u64, first_set: u32, descriptor_set_count: u32, descriptor_sets: *const u64, dynamic_offset_count: u32, dynamic_offsets: *const u32) {
	return vkCmdBindDescriptorSets(command_buffer, pipeline_bind_point, layout, first_set, descriptor_set_count, descriptor_sets, dynamic_offset_count, dynamic_offsets);
}

pub unsafe fn cmd_bind_index_buffer(command_buffer: libc::size_t, buffer: u64, offset: DeviceSize, index_type: IndexType) {
	return vkCmdBindIndexBuffer(command_buffer, buffer, offset, index_type);
}

pub unsafe fn cmd_bind_vertex_buffers(command_buffer: libc::size_t, first_binding: u32, binding_count: u32, buffers: *const u64, offsets: *const DeviceSize) {
	return vkCmdBindVertexBuffers(command_buffer, first_binding, binding_count, buffers, offsets);
}

pub unsafe fn cmd_draw(command_buffer: libc::size_t, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) {
	return vkCmdDraw(command_buffer, vertex_count, instance_count, first_vertex, first_instance);
}

pub unsafe fn cmd_draw_indexed(command_buffer: libc::size_t, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) {
	return vkCmdDrawIndexed(command_buffer, index_count, instance_count, first_index, vertex_offset, first_instance);
}

pub unsafe fn cmd_draw_indirect(command_buffer: libc::size_t, buffer: u64, offset: DeviceSize, draw_count: u32, stride: u32) {
	return vkCmdDrawIndirect(command_buffer, buffer, offset, draw_count, stride);
}

pub unsafe fn cmd_draw_indexed_indirect(command_buffer: libc::size_t, buffer: u64, offset: DeviceSize, draw_count: u32, stride: u32) {
	return vkCmdDrawIndexedIndirect(command_buffer, buffer, offset, draw_count, stride);
}

pub unsafe fn cmd_dispatch(command_buffer: libc::size_t, x: u32, y: u32, z: u32) {
	return vkCmdDispatch(command_buffer, x, y, z);
}

pub unsafe fn cmd_dispatch_indirect(command_buffer: libc::size_t, buffer: u64, offset: DeviceSize) {
	return vkCmdDispatchIndirect(command_buffer, buffer, offset);
}

pub unsafe fn cmd_copy_buffer(command_buffer: libc::size_t, src_buffer: u64, dst_buffer: u64, region_count: u32, regions: *const BufferCopy) {
	return vkCmdCopyBuffer(command_buffer, src_buffer, dst_buffer, region_count, regions);
}

pub unsafe fn cmd_copy_image(command_buffer: libc::size_t, src_image: u64, src_image_layout: ImageLayout, dst_image: u64, dst_image_layout: ImageLayout, region_count: u32, regions: *const ImageCopy) {
	return vkCmdCopyImage(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions);
}

pub unsafe fn cmd_blit_image(command_buffer: libc::size_t, src_image: u64, src_image_layout: ImageLayout, dst_image: u64, dst_image_layout: ImageLayout, region_count: u32, regions: *const ImageBlit, filter: Filter) {
	return vkCmdBlitImage(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions, filter);
}

pub unsafe fn cmd_copy_buffer_to_image(command_buffer: libc::size_t, src_buffer: u64, dst_image: u64, dst_image_layout: ImageLayout, region_count: u32, regions: *const BufferImageCopy) {
	return vkCmdCopyBufferToImage(command_buffer, src_buffer, dst_image, dst_image_layout, region_count, regions);
}

pub unsafe fn cmd_copy_image_to_buffer(command_buffer: libc::size_t, src_image: u64, src_image_layout: ImageLayout, dst_buffer: u64, region_count: u32, regions: *const BufferImageCopy) {
	return vkCmdCopyImageToBuffer(command_buffer, src_image, src_image_layout, dst_buffer, region_count, regions);
}

pub unsafe fn cmd_update_buffer(command_buffer: libc::size_t, dst_buffer: u64, dst_offset: DeviceSize, data_size: DeviceSize, data: *const u32) {
	return vkCmdUpdateBuffer(command_buffer, dst_buffer, dst_offset, data_size, data);
}

pub unsafe fn cmd_fill_buffer(command_buffer: libc::size_t, dst_buffer: u64, dst_offset: DeviceSize, size: DeviceSize, data: u32) {
	return vkCmdFillBuffer(command_buffer, dst_buffer, dst_offset, size, data);
}

pub unsafe fn cmd_clear_color_image(command_buffer: libc::size_t, image: u64, image_layout: ImageLayout, color: *const libc::size_t, range_count: u32, ranges: *const ImageSubresourceRange) {
	return vkCmdClearColorImage(command_buffer, image, image_layout, color, range_count, ranges);
}

pub unsafe fn cmd_clear_depth_stencil_image(command_buffer: libc::size_t, image: u64, image_layout: ImageLayout, depth_stencil: *const ClearDepthStencilValue, range_count: u32, ranges: *const ImageSubresourceRange) {
	return vkCmdClearDepthStencilImage(command_buffer, image, image_layout, depth_stencil, range_count, ranges);
}

pub unsafe fn cmd_clear_attachments(command_buffer: libc::size_t, attachment_count: u32, attachments: *const ClearAttachment, rect_count: u32, rects: *const ClearRect) {
	return vkCmdClearAttachments(command_buffer, attachment_count, attachments, rect_count, rects);
}

pub unsafe fn cmd_resolve_image(command_buffer: libc::size_t, src_image: u64, src_image_layout: ImageLayout, dst_image: u64, dst_image_layout: ImageLayout, region_count: u32, regions: *const ImageResolve) {
	return vkCmdResolveImage(command_buffer, src_image, src_image_layout, dst_image, dst_image_layout, region_count, regions);
}

pub unsafe fn cmd_set_event(command_buffer: libc::size_t, event: u64, stage_mask: PipelineStageFlags) {
	return vkCmdSetEvent(command_buffer, event, stage_mask);
}

pub unsafe fn cmd_reset_event(command_buffer: libc::size_t, event: u64, stage_mask: PipelineStageFlags) {
	return vkCmdResetEvent(command_buffer, event, stage_mask);
}

pub unsafe fn cmd_wait_events(command_buffer: libc::size_t, event_count: u32, events: *const u64, src_stage_mask: PipelineStageFlags, dst_stage_mask: PipelineStageFlags, memory_barrier_count: u32, memory_barriers: *const MemoryBarrier, buffer_memory_barrier_count: u32, buffer_memory_barriers: *const BufferMemoryBarrier, image_memory_barrier_count: u32, image_memory_barriers: *const ImageMemoryBarrier) {
	return vkCmdWaitEvents(command_buffer, event_count, events, src_stage_mask, dst_stage_mask, memory_barrier_count, memory_barriers, buffer_memory_barrier_count, buffer_memory_barriers, image_memory_barrier_count, image_memory_barriers);
}

pub unsafe fn cmd_pipeline_barrier(command_buffer: libc::size_t, src_stage_mask: PipelineStageFlags, dst_stage_mask: PipelineStageFlags, dependency_flags: DependencyFlags, memory_barrier_count: u32, memory_barriers: *const MemoryBarrier, buffer_memory_barrier_count: u32, buffer_memory_barriers: *const BufferMemoryBarrier, image_memory_barrier_count: u32, image_memory_barriers: *const ImageMemoryBarrier) {
	return vkCmdPipelineBarrier(command_buffer, src_stage_mask, dst_stage_mask, dependency_flags, memory_barrier_count, memory_barriers, buffer_memory_barrier_count, buffer_memory_barriers, image_memory_barrier_count, image_memory_barriers);
}

pub unsafe fn cmd_begin_query(command_buffer: libc::size_t, query_pool: u64, query: u32, flags: QueryControlFlags) {
	return vkCmdBeginQuery(command_buffer, query_pool, query, flags);
}

pub unsafe fn cmd_end_query(command_buffer: libc::size_t, query_pool: u64, query: u32) {
	return vkCmdEndQuery(command_buffer, query_pool, query);
}

pub unsafe fn cmd_reset_query_pool(command_buffer: libc::size_t, query_pool: u64, first_query: u32, query_count: u32) {
	return vkCmdResetQueryPool(command_buffer, query_pool, first_query, query_count);
}

pub unsafe fn cmd_write_timestamp(command_buffer: libc::size_t, pipeline_stage: PipelineStageFlags, query_pool: u64, query: u32) {
	return vkCmdWriteTimestamp(command_buffer, pipeline_stage, query_pool, query);
}

pub unsafe fn cmd_copy_query_pool_results(command_buffer: libc::size_t, query_pool: u64, first_query: u32, query_count: u32, dst_buffer: u64, dst_offset: DeviceSize, stride: DeviceSize, flags: QueryResultFlags) {
	return vkCmdCopyQueryPoolResults(command_buffer, query_pool, first_query, query_count, dst_buffer, dst_offset, stride, flags);
}

pub unsafe fn cmd_push_constants(command_buffer: libc::size_t, layout: u64, stage_flags: ShaderStageFlags, offset: u32, size: u32, values: *const libc::c_void) {
	return vkCmdPushConstants(command_buffer, layout, stage_flags, offset, size, values);
}

pub unsafe fn cmd_begin_render_pass(command_buffer: libc::size_t, render_pass_begin: *const RenderPassBeginInfo, contents: SubpassContents) {
	return vkCmdBeginRenderPass(command_buffer, render_pass_begin, contents);
}

pub unsafe fn cmd_next_subpass(command_buffer: libc::size_t, contents: SubpassContents) {
	return vkCmdNextSubpass(command_buffer, contents);
}

pub unsafe fn cmd_end_render_pass(command_buffer: libc::size_t) {
	return vkCmdEndRenderPass(command_buffer);
}

pub unsafe fn cmd_execute_commands(command_buffer: libc::size_t, command_buffer_count: u32, command_buffers: *const libc::size_t) {
	return vkCmdExecuteCommands(command_buffer, command_buffer_count, command_buffers);
}

#[cfg(vk_khr_android_surface)]
pub unsafe fn create_android_surface_khr(instance: libc::size_t, create_info: *const AndroidSurfaceCreateInfoKHR, allocator: *const AllocationCallbacks, surface: *mut u64) -> Result {
	return vkCreateAndroidSurfaceKHR(instance, create_info, allocator, surface);
}

#[cfg(vk_khr_display)]
pub unsafe fn get_physical_device_display_properties_khr(physical_device: libc::size_t, property_count: *mut u32, properties: *mut DisplayPropertiesKHR) -> Result {
	return vkGetPhysicalDeviceDisplayPropertiesKHR(physical_device, property_count, properties);
}

#[cfg(vk_khr_display)]
pub unsafe fn get_physical_device_display_plane_properties_khr(physical_device: libc::size_t, property_count: *mut u32, properties: *mut DisplayPlanePropertiesKHR) -> Result {
	return vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physical_device, property_count, properties);
}

#[cfg(vk_khr_display)]
pub unsafe fn get_display_plane_supported_displays_khr(physical_device: libc::size_t, plane_index: u32, display_count: *mut u32, displays: *mut u64) -> Result {
	return vkGetDisplayPlaneSupportedDisplaysKHR(physical_device, plane_index, display_count, displays);
}

#[cfg(vk_khr_display)]
pub unsafe fn get_display_mode_properties_khr(physical_device: libc::size_t, display: u64, property_count: *mut u32, properties: *mut DisplayModePropertiesKHR) -> Result {
	return vkGetDisplayModePropertiesKHR(physical_device, display, property_count, properties);
}

#[cfg(vk_khr_display)]
pub unsafe fn create_display_mode_khr(physical_device: libc::size_t, display: u64, create_info: *const DisplayModeCreateInfoKHR, allocator: *const AllocationCallbacks, mode: *mut u64) -> Result {
	return vkCreateDisplayModeKHR(physical_device, display, create_info, allocator, mode);
}

#[cfg(vk_khr_display)]
pub unsafe fn get_display_plane_capabilities_khr(physical_device: libc::size_t, mode: u64, plane_index: u32, capabilities: *mut DisplayPlaneCapabilitiesKHR) -> Result {
	return vkGetDisplayPlaneCapabilitiesKHR(physical_device, mode, plane_index, capabilities);
}

#[cfg(vk_khr_display)]
pub unsafe fn create_display_plane_surface_khr(instance: libc::size_t, create_info: *const DisplaySurfaceCreateInfoKHR, allocator: *const AllocationCallbacks, surface: *mut u64) -> Result {
	return vkCreateDisplayPlaneSurfaceKHR(instance, create_info, allocator, surface);
}

#[cfg(vk_khr_display_swapchain)]
pub unsafe fn create_shared_swapchains_khr(device: libc::size_t, swapchain_count: u32, create_infos: *const SwapchainCreateInfoKHR, allocator: *const AllocationCallbacks, swapchains: *mut u64) -> Result {
	return vkCreateSharedSwapchainsKHR(device, swapchain_count, create_infos, allocator, swapchains);
}

#[cfg(vk_khr_mir_surface)]
pub unsafe fn create_mir_surface_khr(instance: libc::size_t, create_info: *const MirSurfaceCreateInfoKHR, allocator: *const AllocationCallbacks, surface: *mut u64) -> Result {
	return vkCreateMirSurfaceKHR(instance, create_info, allocator, surface);
}

#[cfg(vk_khr_mir_surface)]
pub unsafe fn get_physical_device_mir_presentation_support_khr(physical_device: libc::size_t, queue_family_index: u32, connection: libc::size_t) -> Bool32 {
	return vkGetPhysicalDeviceMirPresentationSupportKHR(physical_device, queue_family_index, connection);
}

#[cfg(vk_khr_surface)]
pub unsafe fn destroy_surface_khr(instance: libc::size_t, surface: u64, allocator: *const AllocationCallbacks) {
	return vkDestroySurfaceKHR(instance, surface, allocator);
}

#[cfg(vk_khr_surface)]
pub unsafe fn get_physical_device_surface_support_khr(physical_device: libc::size_t, queue_family_index: u32, surface: u64, supported: *mut Bool32) -> Result {
	return vkGetPhysicalDeviceSurfaceSupportKHR(physical_device, queue_family_index, surface, supported);
}

#[cfg(vk_khr_surface)]
pub unsafe fn get_physical_device_surface_capabilities_khr(physical_device: libc::size_t, surface: u64, surface_capabilities: *mut SurfaceCapabilitiesKHR) -> Result {
	return vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device, surface, surface_capabilities);
}

#[cfg(vk_khr_surface)]
pub unsafe fn get_physical_device_surface_formats_khr(physical_device: libc::size_t, surface: u64, surface_format_count: *mut u32, surface_formats: *mut SurfaceFormatKHR) -> Result {
	return vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device, surface, surface_format_count, surface_formats);
}

#[cfg(vk_khr_surface)]
pub unsafe fn get_physical_device_surface_present_modes_khr(physical_device: libc::size_t, surface: u64, present_mode_count: *mut u32, present_modes: *mut PresentModeKHR) -> Result {
	return vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device, surface, present_mode_count, present_modes);
}

#[cfg(vk_khr_swapchain)]
pub unsafe fn create_swapchain_khr(device: libc::size_t, create_info: *const SwapchainCreateInfoKHR, allocator: *const AllocationCallbacks, swapchain: *mut u64) -> Result {
	return vkCreateSwapchainKHR(device, create_info, allocator, swapchain);
}

#[cfg(vk_khr_swapchain)]
pub unsafe fn destroy_swapchain_khr(device: libc::size_t, swapchain: u64, allocator: *const AllocationCallbacks) {
	return vkDestroySwapchainKHR(device, swapchain, allocator);
}

#[cfg(vk_khr_swapchain)]
pub unsafe fn get_swapchain_images_khr(device: libc::size_t, swapchain: u64, swapchain_image_count: *mut u32, swapchain_images: *mut u64) -> Result {
	return vkGetSwapchainImagesKHR(device, swapchain, swapchain_image_count, swapchain_images);
}

#[cfg(vk_khr_swapchain)]
pub unsafe fn acquire_next_image_khr(device: libc::size_t, swapchain: u64, timeout: u64, semaphore: u64, fence: u64, image_index: *mut u32) -> Result {
	return vkAcquireNextImageKHR(device, swapchain, timeout, semaphore, fence, image_index);
}

#[cfg(vk_khr_swapchain)]
pub unsafe fn queue_present_khr(queue: libc::size_t, present_info: *const PresentInfoKHR) -> Result {
	return vkQueuePresentKHR(queue, present_info);
}

#[cfg(vk_khr_wayland_surface)]
pub unsafe fn create_wayland_surface_khr(instance: libc::size_t, create_info: *const WaylandSurfaceCreateInfoKHR, allocator: *const AllocationCallbacks, surface: *mut u64) -> Result {
	return vkCreateWaylandSurfaceKHR(instance, create_info, allocator, surface);
}

#[cfg(vk_khr_wayland_surface)]
pub unsafe fn get_physical_device_wayland_presentation_support_khr(physical_device: libc::size_t, queue_family_index: u32, display: libc::size_t) -> Bool32 {
	return vkGetPhysicalDeviceWaylandPresentationSupportKHR(physical_device, queue_family_index, display);
}

#[cfg(vk_khr_win32_surface)]
pub unsafe fn create_win32_surface_khr(instance: libc::size_t, create_info: *const Win32SurfaceCreateInfoKHR, allocator: *const AllocationCallbacks, surface: *mut u64) -> Result {
	return vkCreateWin32SurfaceKHR(instance, create_info, allocator, surface);
}

#[cfg(vk_khr_win32_surface)]
pub unsafe fn get_physical_device_win32_presentation_support_khr(physical_device: libc::size_t, queue_family_index: u32) -> Bool32 {
	return vkGetPhysicalDeviceWin32PresentationSupportKHR(physical_device, queue_family_index);
}

#[cfg(vk_khr_xlib_surface)]
pub unsafe fn create_xlib_surface_khr(instance: libc::size_t, create_info: *const XlibSurfaceCreateInfoKHR, allocator: *const AllocationCallbacks, surface: *mut u64) -> Result {
	return vkCreateXlibSurfaceKHR(instance, create_info, allocator, surface);
}

#[cfg(vk_khr_xlib_surface)]
pub unsafe fn get_physical_device_xlib_presentation_support_khr(physical_device: libc::size_t, queue_family_index: u32, dpy: libc::size_t, visual_id: libc::size_t) -> Bool32 {
	return vkGetPhysicalDeviceXlibPresentationSupportKHR(physical_device, queue_family_index, dpy, visual_id);
}

#[cfg(vk_khr_xcb_surface)]
pub unsafe fn create_xcb_surface_khr(instance: libc::size_t, create_info: *const XcbSurfaceCreateInfoKHR, allocator: *const AllocationCallbacks, surface: *mut u64) -> Result {
	return vkCreateXcbSurfaceKHR(instance, create_info, allocator, surface);
}

#[cfg(vk_khr_xcb_surface)]
pub unsafe fn get_physical_device_xcb_presentation_support_khr(physical_device: libc::size_t, queue_family_index: u32, connection: libc::size_t, visual_id: libc::size_t) -> Bool32 {
	return vkGetPhysicalDeviceXcbPresentationSupportKHR(physical_device, queue_family_index, connection, visual_id);
}

#[cfg(vk_ext_debug_report)]
pub unsafe fn create_debug_report_callback_ext(instance: libc::size_t, create_info: *const DebugReportCallbackCreateInfoEXT, allocator: *const AllocationCallbacks, callback: *mut u64) -> Result {
	return vkCreateDebugReportCallbackEXT(instance, create_info, allocator, callback);
}

#[cfg(vk_ext_debug_report)]
pub unsafe fn destroy_debug_report_callback_ext(instance: libc::size_t, callback: u64, allocator: *const AllocationCallbacks) {
	return vkDestroyDebugReportCallbackEXT(instance, callback, allocator);
}

#[cfg(vk_ext_debug_report)]
pub unsafe fn debug_report_message_ext(instance: libc::size_t, flags: DebugReportFlagsEXT, object_type: DebugReportObjectTypeEXT, object: u64, location: libc::size_t, message_code: i32, layer_prefix: *const libc::c_char, message: *const libc::c_char) {
	return vkDebugReportMessageEXT(instance, flags, object_type, object, location, message_code, layer_prefix, message);
}

