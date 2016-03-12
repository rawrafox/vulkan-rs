#[macro_use]

use libc;
pub type SampleMask = u32;
pub type Bool32 = u32;
pub type Flags = u32;
pub type DeviceSize = u64;

pub static MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub static UUID_SIZE: usize = 16;
pub static MAX_EXTENSION_NAME_SIZE: usize = 256;
pub static MAX_DESCRIPTION_SIZE: usize = 256;
pub static MAX_MEMORY_TYPES: usize = 32;
pub static MAX_MEMORY_HEAPS: usize = 16;
pub static LOD_CLAMP_NONE: f32 = 1000.0f32;
pub static REMAINING_MIP_LEVELS: usize = usize::max_value();
pub static REMAINING_ARRAY_LAYERS: usize = usize::max_value();
pub static WHOLE_SIZE: usize = usize::max_value();
pub static ATTACHMENT_UNUSED: usize = usize::max_value();
pub static TRUE: usize = 1;
pub static FALSE: usize = 0;
pub static QUEUE_FAMILY_IGNORED: usize = usize::max_value();
pub static SUBPASS_EXTERNAL: usize = usize::max_value();

#[repr(C)] #[derive(PartialEq)] pub enum ImageLayout {
	Undefined = 0,
	General = 1,
	ColorAttachmentOptimal = 2,
	DepthStencilAttachmentOptimal = 3,
	DepthStencilReadOnlyOptimal = 4,
	ShaderReadOnlyOptimal = 5,
	TransferSrcOptimal = 6,
	TransferDstOptimal = 7,
	Preinitialized = 8,
}

#[repr(C)] #[derive(PartialEq)] pub enum AttachmentLoadOp {
	Load = 0,
	Clear = 1,
	DontCare = 2,
}

#[repr(C)] #[derive(PartialEq)] pub enum AttachmentStoreOp {
	Store = 0,
	DontCare = 1,
}

#[repr(C)] #[derive(PartialEq)] pub enum ImageType {
	OneD = 0,
	TwoD = 1,
	ThreeD = 2,
}

#[repr(C)] #[derive(PartialEq)] pub enum ImageTiling {
	Optimal = 0,
	Linear = 1,
}

#[repr(C)] #[derive(PartialEq)] pub enum ImageViewType {
	OneD = 0,
	TwoD = 1,
	ThreeD = 2,
	Cube = 3,
	OneDArray = 4,
	TwoDArray = 5,
	CubeArray = 6,
}

#[repr(C)] #[derive(PartialEq)] pub enum CommandBufferLevel {
	Primary = 0,
	Secondary = 1,
}

#[repr(C)] #[derive(PartialEq)] pub enum ComponentSwizzle {
	Identity = 0,
	Zero = 1,
	One = 2,
	R = 3,
	G = 4,
	B = 5,
	A = 6,
}

#[repr(C)] #[derive(PartialEq)] pub enum DescriptorType {
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

#[repr(C)] #[derive(PartialEq)] pub enum QueryType {
	Occlusion = 0,
	PipelineStatistic = 1,
	Timestamp = 2,
}

#[repr(C)] #[derive(PartialEq)] pub enum BorderColor {
	FloatTransparentBlack = 0,
	IntTransparentBlack = 1,
	FloatOpaqueBlack = 2,
	IntOpaqueBlack = 3,
	FloatOpaqueWhite = 4,
	IntOpaqueWhite = 5,
}

#[repr(C)] #[derive(PartialEq)] pub enum PipelineBindPoint {
	Graphic = 0,
	Compute = 1,
}

#[repr(C)] #[derive(PartialEq)] pub enum PipelineCacheHeaderVersion {
	One = 1,
}

#[repr(C)] #[derive(PartialEq)] pub enum PrimitiveTopology {
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

#[repr(C)] #[derive(PartialEq)] pub enum SharingMode {
	Exclusive = 0,
	Concurrent = 1,
}

#[repr(C)] #[derive(PartialEq)] pub enum IndexType {
	Uint16 = 0,
	Uint32 = 1,
}

#[repr(C)] #[derive(PartialEq)] pub enum Filter {
	Nearest = 0,
	Linear = 1,
}

#[repr(C)] #[derive(PartialEq)] pub enum SamplerMipmapMode {
	Nearest = 0,
	Linear = 1,
}

#[repr(C)] #[derive(PartialEq)] pub enum SamplerAddressMode {
	Repeat = 0,
	MirroredRepeat = 1,
	ClampToEdge = 2,
	ClampToBorder = 3,
}

#[repr(C)] #[derive(PartialEq)] pub enum CompareOp {
	Never = 0,
	Less = 1,
	Equal = 2,
	LessOrEqual = 3,
	Greater = 4,
	NotEqual = 5,
	GreaterOrEqual = 6,
	Alway = 7,
}

#[repr(C)] #[derive(PartialEq)] pub enum PolygonMode {
	Fill = 0,
	Line = 1,
	Point = 2,
}

bitflags! {
	#[repr(C)] flags CullModeFlagBits: Flags {
		const CULL_MODE_NONE = 1,
		const CULL_MODE_FRONT_BIT = 1,
		const CULL_MODE_BACK_BIT = 2,
		const CULL_MODE_FRONT_AND_BACK = 1,
	}

}

#[repr(C)] #[derive(PartialEq)] pub enum FrontFace {
	CounterClockwise = 0,
	Clockwise = 1,
}

#[repr(C)] #[derive(PartialEq)] pub enum BlendFactor {
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

#[repr(C)] #[derive(PartialEq)] pub enum BlendOp {
	Add = 0,
	Subtract = 1,
	ReverseSubtract = 2,
	Min = 3,
	Max = 4,
}

#[repr(C)] #[derive(PartialEq)] pub enum StencilOp {
	Keep = 0,
	Zero = 1,
	Replace = 2,
	IncrementAndClamp = 3,
	DecrementAndClamp = 4,
	Invert = 5,
	IncrementAndWrap = 6,
	DecrementAndWrap = 7,
}

#[repr(C)] #[derive(PartialEq)] pub enum LogicOp {
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

#[repr(C)] #[derive(PartialEq)] pub enum InternalAllocationType {
	Executable = 0,
}

#[repr(C)] #[derive(PartialEq)] pub enum SystemAllocationScope {
	Command = 0,
	Object = 1,
	Cache = 2,
	Device = 3,
	Instance = 4,
}

#[repr(C)] #[derive(PartialEq)] pub enum PhysicalDeviceType {
	Other = 0,
	IntegratedGpu = 1,
	DiscreteGpu = 2,
	VirtualGpu = 3,
	Cpu = 4,
}

#[repr(C)] #[derive(PartialEq)] pub enum VertexInputRate {
	Vertex = 0,
	Instance = 1,
}

#[repr(C)] #[derive(PartialEq)] pub enum Format {
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

#[repr(C)] #[derive(PartialEq)] pub enum StructureType {
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

#[repr(C)] #[derive(PartialEq)] pub enum SubpassContents {
	Inline = 0,
	SecondaryCommandBuffer = 1,
}

#[repr(C)] #[derive(PartialEq)] pub enum Result {
	Success = 0,
	NotReady = 1,
	Timeout = 2,
	EventSet = 3,
	EventReset = 4,
	Incomplete = 5,
	ErrorOutOfHostMemory = -1,
	ErrorOutOfDeviceMemory = -2,
	ErrorInitializationFailed = -3,
	ErrorDeviceLost = -4,
	ErrorMemoryMapFailed = -5,
	ErrorLayerNotPresent = -6,
	ErrorExtensionNotPresent = -7,
	ErrorFeatureNotPresent = -8,
	ErrorIncompatibleDriver = -9,
	ErrorTooManyObject = -10,
	ErrorFormatNotSupported = -11,
}

#[repr(C)] #[derive(PartialEq)] pub enum DynamicState {
	Viewport = 0,
	Scissor = 1,
	LineWidth = 2,
	DepthBia = 3,
	BlendConstant = 4,
	DepthBound = 5,
	StencilCompareMask = 6,
	StencilWriteMask = 7,
	StencilReference = 8,
}

bitflags! {
	#[repr(C)] flags QueueFlagBits: Flags {
		const QUEUE_GRAPHICS_BIT = 1,
		const QUEUE_COMPUTE_BIT = 2,
		const QUEUE_TRANSFER_BIT = 4,
		const QUEUE_SPARSE_BINDING_BIT = 8,
	}

}

bitflags! {
	#[repr(C)] flags MemoryPropertyFlagBits: Flags {
		const MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 1,
		const MEMORY_PROPERTY_HOST_VISIBLE_BIT = 2,
		const MEMORY_PROPERTY_HOST_COHERENT_BIT = 4,
		const MEMORY_PROPERTY_HOST_CACHED_BIT = 8,
		const MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 16,
	}

}

bitflags! {
	#[repr(C)] flags MemoryHeapFlagBits: Flags {
		const MEMORY_HEAP_DEVICE_LOCAL_BIT = 1,
	}

}

bitflags! {
	#[repr(C)] flags AccessFlagBits: Flags {
		const ACCESS_INDIRECT_COMMAND_READ_BIT = 1,
		const ACCESS_INDEX_READ_BIT = 2,
		const ACCESS_VERTEX_ATTRIBUTE_READ_BIT = 4,
		const ACCESS_UNIFORM_READ_BIT = 8,
		const ACCESS_INPUT_ATTACHMENT_READ_BIT = 16,
		const ACCESS_SHADER_READ_BIT = 32,
		const ACCESS_SHADER_WRITE_BIT = 64,
		const ACCESS_COLOR_ATTACHMENT_READ_BIT = 128,
		const ACCESS_COLOR_ATTACHMENT_WRITE_BIT = 256,
		const ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT = 512,
		const ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 1024,
		const ACCESS_TRANSFER_READ_BIT = 2048,
		const ACCESS_TRANSFER_WRITE_BIT = 4096,
		const ACCESS_HOST_READ_BIT = 8192,
		const ACCESS_HOST_WRITE_BIT = 16384,
		const ACCESS_MEMORY_READ_BIT = 32768,
		const ACCESS_MEMORY_WRITE_BIT = 65536,
	}

}

bitflags! {
	#[repr(C)] flags BufferUsageFlagBits: Flags {
		const BUFFER_USAGE_TRANSFER_SRC_BIT = 1,
		const BUFFER_USAGE_TRANSFER_DST_BIT = 2,
		const BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = 4,
		const BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = 8,
		const BUFFER_USAGE_UNIFORM_BUFFER_BIT = 16,
		const BUFFER_USAGE_STORAGE_BUFFER_BIT = 32,
		const BUFFER_USAGE_INDEX_BUFFER_BIT = 64,
		const BUFFER_USAGE_VERTEX_BUFFER_BIT = 128,
		const BUFFER_USAGE_INDIRECT_BUFFER_BIT = 256,
	}

}

bitflags! {
	#[repr(C)] flags BufferCreateFlagBits: Flags {
		const BUFFER_CREATE_SPARSE_BINDING_BIT = 1,
		const BUFFER_CREATE_SPARSE_RESIDENCY_BIT = 2,
		const BUFFER_CREATE_SPARSE_ALIASED_BIT = 4,
	}

}

bitflags! {
	#[repr(C)] flags ShaderStageFlagBits: Flags {
		const SHADER_STAGE_VERTEX_BIT = 1,
		const SHADER_STAGE_TESSELLATION_CONTROL_BIT = 2,
		const SHADER_STAGE_TESSELLATION_EVALUATION_BIT = 4,
		const SHADER_STAGE_GEOMETRY_BIT = 8,
		const SHADER_STAGE_FRAGMENT_BIT = 16,
		const SHADER_STAGE_COMPUTE_BIT = 32,
		const SHADER_STAGE_ALL_GRAPHICS = 1,
		const SHADER_STAGE_ALL = 1,
	}

}

bitflags! {
	#[repr(C)] flags ImageUsageFlagBits: Flags {
		const IMAGE_USAGE_TRANSFER_SRC_BIT = 1,
		const IMAGE_USAGE_TRANSFER_DST_BIT = 2,
		const IMAGE_USAGE_SAMPLED_BIT = 4,
		const IMAGE_USAGE_STORAGE_BIT = 8,
		const IMAGE_USAGE_COLOR_ATTACHMENT_BIT = 16,
		const IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = 32,
		const IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = 64,
		const IMAGE_USAGE_INPUT_ATTACHMENT_BIT = 128,
	}

}

bitflags! {
	#[repr(C)] flags ImageCreateFlagBits: Flags {
		const IMAGE_CREATE_SPARSE_BINDING_BIT = 1,
		const IMAGE_CREATE_SPARSE_RESIDENCY_BIT = 2,
		const IMAGE_CREATE_SPARSE_ALIASED_BIT = 4,
		const IMAGE_CREATE_MUTABLE_FORMAT_BIT = 8,
		const IMAGE_CREATE_CUBE_COMPATIBLE_BIT = 16,
	}

}

bitflags! {
	#[repr(C)] flags PipelineCreateFlagBits: Flags {
		const PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = 1,
		const PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = 2,
		const PIPELINE_CREATE_DERIVATIVE_BIT = 4,
	}

}

bitflags! {
	#[repr(C)] flags ColorComponentFlagBits: Flags {
		const COLOR_COMPONENT_R_BIT = 1,
		const COLOR_COMPONENT_G_BIT = 2,
		const COLOR_COMPONENT_B_BIT = 4,
		const COLOR_COMPONENT_A_BIT = 8,
	}

}

bitflags! {
	#[repr(C)] flags FenceCreateFlagBits: Flags {
		const FENCE_CREATE_SIGNALED_BIT = 1,
	}

}

bitflags! {
	#[repr(C)] flags FormatFeatureFlagBits: Flags {
		const FORMAT_FEATURE_SAMPLED_IMAGE_BIT = 1,
		const FORMAT_FEATURE_STORAGE_IMAGE_BIT = 2,
		const FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT = 4,
		const FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT = 8,
		const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT = 16,
		const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 32,
		const FORMAT_FEATURE_VERTEX_BUFFER_BIT = 64,
		const FORMAT_FEATURE_COLOR_ATTACHMENT_BIT = 128,
		const FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT = 256,
		const FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT = 512,
		const FORMAT_FEATURE_BLIT_SRC_BIT = 1024,
		const FORMAT_FEATURE_BLIT_DST_BIT = 2048,
		const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT = 4096,
	}

}

bitflags! {
	#[repr(C)] flags QueryControlFlagBits: Flags {
		const QUERY_CONTROL_PRECISE_BIT = 1,
	}

}

bitflags! {
	#[repr(C)] flags QueryResultFlagBits: Flags {
		const QUERY_RESULT_64_BIT = 1,
		const QUERY_RESULT_WAIT_BIT = 2,
		const QUERY_RESULT_WITH_AVAILABILITY_BIT = 4,
		const QUERY_RESULT_PARTIAL_BIT = 8,
	}

}

bitflags! {
	#[repr(C)] flags CommandBufferUsageFlagBits: Flags {
		const COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = 1,
		const COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = 2,
		const COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = 4,
	}

}

bitflags! {
	#[repr(C)] flags QueryPipelineStatisticFlagBits: Flags {
		const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT = 1,
		const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT = 2,
		const QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT = 4,
		const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT = 8,
		const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT = 16,
		const QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT = 32,
		const QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT = 64,
		const QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT = 128,
		const QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 256,
		const QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 512,
		const QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT = 1024,
	}

}

bitflags! {
	#[repr(C)] flags ImageAspectFlagBits: Flags {
		const IMAGE_ASPECT_COLOR_BIT = 1,
		const IMAGE_ASPECT_DEPTH_BIT = 2,
		const IMAGE_ASPECT_STENCIL_BIT = 4,
		const IMAGE_ASPECT_METADATA_BIT = 8,
	}

}

bitflags! {
	#[repr(C)] flags SparseImageFormatFlagBits: Flags {
		const SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT = 1,
		const SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT = 2,
		const SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT = 4,
	}

}

bitflags! {
	#[repr(C)] flags SparseMemoryBindFlagBits: Flags {
		const SPARSE_MEMORY_BIND_METADATA_BIT = 1,
	}

}

bitflags! {
	#[repr(C)] flags PipelineStageFlagBits: Flags {
		const PIPELINE_STAGE_TOP_OF_PIPE_BIT = 1,
		const PIPELINE_STAGE_DRAW_INDIRECT_BIT = 2,
		const PIPELINE_STAGE_VERTEX_INPUT_BIT = 4,
		const PIPELINE_STAGE_VERTEX_SHADER_BIT = 8,
		const PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT = 16,
		const PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT = 32,
		const PIPELINE_STAGE_GEOMETRY_SHADER_BIT = 64,
		const PIPELINE_STAGE_FRAGMENT_SHADER_BIT = 128,
		const PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT = 256,
		const PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT = 512,
		const PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT = 1024,
		const PIPELINE_STAGE_COMPUTE_SHADER_BIT = 2048,
		const PIPELINE_STAGE_TRANSFER_BIT = 4096,
		const PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT = 8192,
		const PIPELINE_STAGE_HOST_BIT = 16384,
		const PIPELINE_STAGE_ALL_GRAPHICS_BIT = 32768,
		const PIPELINE_STAGE_ALL_COMMANDS_BIT = 65536,
	}

}

bitflags! {
	#[repr(C)] flags CommandPoolCreateFlagBits: Flags {
		const COMMAND_POOL_CREATE_TRANSIENT_BIT = 1,
		const COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = 2,
	}

}

bitflags! {
	#[repr(C)] flags CommandPoolResetFlagBits: Flags {
		const COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = 1,
	}

}

bitflags! {
	#[repr(C)] flags CommandBufferResetFlagBits: Flags {
		const COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = 1,
	}

}

bitflags! {
	#[repr(C)] flags SampleCountFlagBits: Flags {
		const SAMPLE_COUNT_1_BIT = 1,
		const SAMPLE_COUNT_2_BIT = 2,
		const SAMPLE_COUNT_4_BIT = 4,
		const SAMPLE_COUNT_8_BIT = 8,
		const SAMPLE_COUNT_16_BIT = 16,
		const SAMPLE_COUNT_32_BIT = 32,
		const SAMPLE_COUNT_64_BIT = 64,
	}

}

bitflags! {
	#[repr(C)] flags AttachmentDescriptionFlagBits: Flags {
		const ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = 1,
	}

}

bitflags! {
	#[repr(C)] flags StencilFaceFlagBits: Flags {
		const STENCIL_FACE_FRONT_BIT = 1,
		const STENCIL_FACE_BACK_BIT = 2,
		const STENCIL_FRONT_AND_BACK = 1,
	}

}

bitflags! {
	#[repr(C)] flags DescriptorPoolCreateFlagBits: Flags {
		const DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = 1,
	}

}

bitflags! {
	#[repr(C)] flags DependencyFlagBits: Flags {
		const DEPENDENCY_BY_REGION_BIT = 1,
	}

}

#[repr(C)] #[derive(PartialEq)] pub enum PresentModeKHR {
	ImmediateKhr = 0,
	MailboxKhr = 1,
	FifoKhr = 2,
	FifoRelaxedKhr = 3,
}

#[repr(C)] #[derive(PartialEq)] pub enum ColorSpaceKHR {
	ColorspaceSrgbNonlinearKhr = 0,
}

bitflags! {
	#[repr(C)] flags DisplayPlaneAlphaFlagBitsKHR: Flags {
		const DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = 1,
		const DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = 2,
		const DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = 4,
		const DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = 8,
	}

}

bitflags! {
	#[repr(C)] flags CompositeAlphaFlagBitsKHR: Flags {
		const COMPOSITE_ALPHA_OPAQUE_BIT_KHR = 1,
		const COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = 2,
		const COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = 4,
		const COMPOSITE_ALPHA_INHERIT_BIT_KHR = 8,
	}

}

bitflags! {
	#[repr(C)] flags SurfaceTransformFlagBitsKHR: Flags {
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
	#[repr(C)] flags DebugReportFlagBitsEXT: Flags {
		const DEBUG_REPORT_INFORMATION_BIT_EXT = 1,
		const DEBUG_REPORT_WARNING_BIT_EXT = 2,
		const DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT = 4,
		const DEBUG_REPORT_ERROR_BIT_EXT = 8,
		const DEBUG_REPORT_DEBUG_BIT_EXT = 16,
	}

}

#[repr(C)] #[derive(PartialEq)] pub enum DebugReportObjectTypeEXT {
	UnknownExt = 0,
	InstanceExt = 1,
	PhysicalDeviceExt = 2,
	DeviceExt = 3,
	QueueExt = 4,
	SemaphoreExt = 5,
	CommandBufferExt = 6,
	FenceExt = 7,
	DeviceMemoryExt = 8,
	BufferExt = 9,
	ImageExt = 10,
	EventExt = 11,
	QueryPoolExt = 12,
	BufferViewExt = 13,
	ImageViewExt = 14,
	ShaderModuleExt = 15,
	PipelineCacheExt = 16,
	PipelineLayoutExt = 17,
	RenderPassExt = 18,
	PipelineExt = 19,
	DescriptorSetLayoutExt = 20,
	SamplerExt = 21,
	DescriptorPoolExt = 22,
	DescriptorSetExt = 23,
	FramebufferExt = 24,
	CommandPoolExt = 25,
	SurfaceKhrExt = 26,
	SwapchainKhrExt = 27,
	DebugReportExt = 28,
}

#[repr(C)] #[derive(PartialEq)] pub enum DebugReportErrorEXT {
	NoneExt = 0,
	CallbackRefExt = 1,
}

pub type FramebufferCreateFlags = Flags; // Reserved
pub type QueryPoolCreateFlags = Flags; // Reserved
pub type RenderPassCreateFlags = Flags; // Reserved
pub type SamplerCreateFlags = Flags; // Reserved
pub type PipelineLayoutCreateFlags = Flags; // Reserved
pub type PipelineCacheCreateFlags = Flags; // Reserved
pub type PipelineDepthStencilStateCreateFlags = Flags; // Reserved
pub type PipelineDynamicStateCreateFlags = Flags; // Reserved
pub type PipelineColorBlendStateCreateFlags = Flags; // Reserved
pub type PipelineMultisampleStateCreateFlags = Flags; // Reserved
pub type PipelineRasterizationStateCreateFlags = Flags; // Reserved
pub type PipelineViewportStateCreateFlags = Flags; // Reserved
pub type PipelineTessellationStateCreateFlags = Flags; // Reserved
pub type PipelineInputAssemblyStateCreateFlags = Flags; // Reserved
pub type PipelineVertexInputStateCreateFlags = Flags; // Reserved
pub type PipelineShaderStageCreateFlags = Flags; // Reserved
pub type DescriptorSetLayoutCreateFlags = Flags; // Reserved
pub type BufferViewCreateFlags = Flags; // Reserved
pub type InstanceCreateFlags = Flags; // Reserved
pub type DeviceCreateFlags = Flags; // Reserved
pub type DeviceQueueCreateFlags = Flags; // Reserved
pub type QueueFlags = Flags; // Reserved
pub type MemoryPropertyFlags = Flags; // Reserved
pub type MemoryHeapFlags = Flags; // Reserved
pub type AccessFlags = Flags; // Reserved
pub type BufferUsageFlags = Flags; // Reserved
pub type BufferCreateFlags = Flags; // Reserved
pub type ShaderStageFlags = Flags; // Reserved
pub type ImageUsageFlags = Flags; // Reserved
pub type ImageCreateFlags = Flags; // Reserved
pub type ImageViewCreateFlags = Flags; // Reserved
pub type PipelineCreateFlags = Flags; // Reserved
pub type ColorComponentFlags = Flags; // Reserved
pub type FenceCreateFlags = Flags; // Reserved
pub type SemaphoreCreateFlags = Flags; // Reserved
pub type FormatFeatureFlags = Flags; // Reserved
pub type QueryControlFlags = Flags; // Reserved
pub type QueryResultFlags = Flags; // Reserved
pub type ShaderModuleCreateFlags = Flags; // Reserved
pub type EventCreateFlags = Flags; // Reserved
pub type CommandPoolCreateFlags = Flags; // Reserved
pub type CommandPoolResetFlags = Flags; // Reserved
pub type CommandBufferResetFlags = Flags; // Reserved
pub type CommandBufferUsageFlags = Flags; // Reserved
pub type QueryPipelineStatisticFlags = Flags; // Reserved
pub type MemoryMapFlags = Flags; // Reserved
pub type ImageAspectFlags = Flags; // Reserved
pub type SparseMemoryBindFlags = Flags; // Reserved
pub type SparseImageFormatFlags = Flags; // Reserved
pub type SubpassDescriptionFlags = Flags; // Reserved
pub type PipelineStageFlags = Flags; // Reserved
pub type SampleCountFlags = Flags; // Reserved
pub type AttachmentDescriptionFlags = Flags; // Reserved
pub type StencilFaceFlags = Flags; // Reserved
pub type CullModeFlags = Flags; // Reserved
pub type DescriptorPoolCreateFlags = Flags; // Reserved
pub type DescriptorPoolResetFlags = Flags; // Reserved
pub type DependencyFlags = Flags; // Reserved
pub type CompositeAlphaFlagsKHR = Flags; // Reserved
pub type DisplayPlaneAlphaFlagsKHR = Flags; // Reserved
pub type SurfaceTransformFlagsKHR = Flags; // Reserved
pub type SwapchainCreateFlagsKHR = Flags; // Reserved
pub type DisplayModeCreateFlagsKHR = Flags; // Reserved
pub type DisplaySurfaceCreateFlagsKHR = Flags; // Reserved
pub type AndroidSurfaceCreateFlagsKHR = Flags; // Reserved
pub type MirSurfaceCreateFlagsKHR = Flags; // Reserved
pub type WaylandSurfaceCreateFlagsKHR = Flags; // Reserved
pub type Win32SurfaceCreateFlagsKHR = Flags; // Reserved
pub type XlibSurfaceCreateFlagsKHR = Flags; // Reserved
pub type XcbSurfaceCreateFlagsKHR = Flags; // Reserved
pub type DebugReportFlagsEXT = Flags; // Reserved
#[repr(C)] pub struct Instance(pub *mut libc::c_void);
#[repr(C)] pub struct PhysicalDevice(pub *mut libc::c_void);
#[repr(C)] pub struct Device(pub *mut libc::c_void);
#[repr(C)] pub struct Queue(pub *mut libc::c_void);
#[repr(C)] pub struct CommandBuffer(pub *mut libc::c_void);
#[repr(C)] pub struct DeviceMemory(pub u64);
#[repr(C)] pub struct CommandPool(pub u64);
#[repr(C)] pub struct Buffer(pub u64);
#[repr(C)] pub struct BufferView(pub u64);
#[repr(C)] pub struct Image(pub u64);
#[repr(C)] pub struct ImageView(pub u64);
#[repr(C)] pub struct ShaderModule(pub u64);
#[repr(C)] pub struct Pipeline(pub u64);
#[repr(C)] pub struct PipelineLayout(pub u64);
#[repr(C)] pub struct Sampler(pub u64);
#[repr(C)] pub struct DescriptorSet(pub u64);
#[repr(C)] pub struct DescriptorSetLayout(pub u64);
#[repr(C)] pub struct DescriptorPool(pub u64);
#[repr(C)] pub struct Fence(pub u64);
#[repr(C)] pub struct Semaphore(pub u64);
#[repr(C)] pub struct Event(pub u64);
#[repr(C)] pub struct QueryPool(pub u64);
#[repr(C)] pub struct Framebuffer(pub u64);
#[repr(C)] pub struct RenderPass(pub u64);
#[repr(C)] pub struct PipelineCache(pub u64);
#[repr(C)] pub struct DisplayKHR(pub u64);
#[repr(C)] pub struct DisplayModeKHR(pub u64);
#[repr(C)] pub struct SurfaceKHR(pub u64);
#[repr(C)] pub struct SwapchainKHR(pub u64);
#[repr(C)] pub struct DebugReportCallbackEXT(pub u64);
pub type PFN_vkInternalAllocationNotification = *mut libc::c_void; // FOOTLONG: Mildly wrong
pub type PFN_vkInternalFreeNotification = *mut libc::c_void; // FOOTLONG: Mildly wrong
pub type PFN_vkReallocationFunction = *mut libc::c_void; // FOOTLONG: Mildly wrong
pub type PFN_vkAllocationFunction = *mut libc::c_void; // FOOTLONG: Mildly wrong
pub type PFN_vkFreeFunction = *mut libc::c_void; // FOOTLONG: Mildly wrong
pub type PFN_vkVoidFunction = *mut libc::c_void; // FOOTLONG: Mildly wrong
pub type PFN_vkDebugReportCallbackEXT = *mut libc::c_void; // FOOTLONG: Mildly wrong
#[repr(C)] pub struct Offset2D {
	pub x: i32,
	pub y: i32,
}

#[repr(C)] pub struct Offset3D {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}

#[repr(C)] pub struct Extent2D {
	pub width: u32,
	pub height: u32,
}

#[repr(C)] pub struct Extent3D {
	pub width: u32,
	pub height: u32,
	pub depth: u32,
}

#[repr(C)] pub struct Viewport {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
	pub min_depth: f32,
	pub max_depth: f32,
}

#[repr(C)] pub struct Rect2D {
	pub offset: Offset2D,
	pub extent: Extent2D,
}

#[repr(C)] pub struct Rect3D {
	pub offset: Offset3D,
	pub extent: Extent3D,
}

#[repr(C)] pub struct ClearRect {
	pub rect: Rect2D,
	pub base_array_layer: u32,
	pub layer_count: u32,
}

#[repr(C)] pub struct ComponentMapping {
	pub r: ComponentSwizzle,
	pub g: ComponentSwizzle,
	pub b: ComponentSwizzle,
	pub a: ComponentSwizzle,
}

#[repr(C)] pub struct PhysicalDeviceProperties {
	pub api_version: u32,
	pub driver_version: u32,
	pub vendor_id: u32,
	pub device_id: u32,
	pub device_type: PhysicalDeviceType,
	pub device_name: u8,
	pub pipeline_cache_uuid: u8,
	pub limits: PhysicalDeviceLimits,
	pub sparse_properties: PhysicalDeviceSparseProperties,
}

#[repr(C)] pub struct ExtensionProperties {
	pub extension_name: u8,
	pub spec_version: u32,
}

#[repr(C)] pub struct LayerProperties {
	pub layer_name: u8,
	pub spec_version: u32,
	pub implementation_version: u32,
	pub description: u8,
}

#[repr(C)] pub struct ApplicationInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub p_application_name: *const u8,
	pub application_version: u32,
	pub p_engine_name: *const u8,
	pub engine_version: u32,
	pub api_version: u32,
}

#[repr(C)] pub struct AllocationCallbacks {
	pub p_user_data: *mut libc::c_void,
	pub pfn_allocation: PFN_vkAllocationFunction,
	pub pfn_reallocation: PFN_vkReallocationFunction,
	pub pfn_free: PFN_vkFreeFunction,
	pub pfn_internal_allocation: PFN_vkInternalAllocationNotification,
	pub pfn_internal_free: PFN_vkInternalFreeNotification,
}

#[repr(C)] pub struct DeviceQueueCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: DeviceQueueCreateFlags,
	pub queue_family_index: u32,
	pub queue_count: u32,
	pub p_queue_priorities: *const f32,
}

#[repr(C)] pub struct DeviceCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: DeviceCreateFlags,
	pub queue_create_info_count: u32,
	pub p_queue_create_infos: *const DeviceQueueCreateInfo,
	pub enabled_layer_count: u32,
	pub pp_enabled_layer_names: *const u8,
	pub enabled_extension_count: u32,
	pub pp_enabled_extension_names: *const u8,
	pub p_enabled_features: *const PhysicalDeviceFeatures,
}

#[repr(C)] pub struct InstanceCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: InstanceCreateFlags,
	pub p_application_info: *const ApplicationInfo,
	pub enabled_layer_count: u32,
	pub pp_enabled_layer_names: *const u8,
	pub enabled_extension_count: u32,
	pub pp_enabled_extension_names: *const u8,
}

#[repr(C)] pub struct QueueFamilyProperties {
	pub queue_flags: QueueFlags,
	pub queue_count: u32,
	pub timestamp_valid_bits: u32,
	pub min_image_transfer_granularity: Extent3D,
}

#[repr(C)] pub struct PhysicalDeviceMemoryProperties {
	pub memory_type_count: u32,
	pub memory_types: MemoryType,
	pub memory_heap_count: u32,
	pub memory_heaps: MemoryHeap,
}

#[repr(C)] pub struct MemoryAllocateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub allocation_size: DeviceSize,
	pub memory_type_index: u32,
}

#[repr(C)] pub struct MemoryRequirements {
	pub size: DeviceSize,
	pub alignment: DeviceSize,
	pub memory_type_bits: u32,
}

#[repr(C)] pub struct SparseImageFormatProperties {
	pub aspect_mask: ImageAspectFlags,
	pub image_granularity: Extent3D,
	pub flags: SparseImageFormatFlags,
}

#[repr(C)] pub struct SparseImageMemoryRequirements {
	pub format_properties: SparseImageFormatProperties,
	pub image_mip_tail_first_lod: u32,
	pub image_mip_tail_size: DeviceSize,
	pub image_mip_tail_offset: DeviceSize,
	pub image_mip_tail_stride: DeviceSize,
}

#[repr(C)] pub struct MemoryType {
	pub property_flags: MemoryPropertyFlags,
	pub heap_index: u32,
}

#[repr(C)] pub struct MemoryHeap {
	pub size: DeviceSize,
	pub flags: MemoryHeapFlags,
}

#[repr(C)] pub struct MappedMemoryRange {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub memory: DeviceMemory,
	pub offset: DeviceSize,
	pub size: DeviceSize,
}

#[repr(C)] pub struct FormatProperties {
	pub linear_tiling_features: FormatFeatureFlags,
	pub optimal_tiling_features: FormatFeatureFlags,
	pub buffer_features: FormatFeatureFlags,
}

#[repr(C)] pub struct ImageFormatProperties {
	pub max_extent: Extent3D,
	pub max_mip_levels: u32,
	pub max_array_layers: u32,
	pub sample_counts: SampleCountFlags,
	pub max_resource_size: DeviceSize,
}

#[repr(C)] pub struct DescriptorBufferInfo {
	pub buffer: Buffer,
	pub offset: DeviceSize,
	pub range: DeviceSize,
}

#[repr(C)] pub struct DescriptorImageInfo {
	pub sampler: Sampler,
	pub image_view: ImageView,
	pub image_layout: ImageLayout,
}

#[repr(C)] pub struct WriteDescriptorSet {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub dst_set: DescriptorSet,
	pub dst_binding: u32,
	pub dst_array_element: u32,
	pub descriptor_count: u32,
	pub descriptor_type: DescriptorType,
	pub p_image_info: *const DescriptorImageInfo,
	pub p_buffer_info: *const DescriptorBufferInfo,
	pub p_texel_buffer_view: *const BufferView,
}

#[repr(C)] pub struct CopyDescriptorSet {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub src_set: DescriptorSet,
	pub src_binding: u32,
	pub src_array_element: u32,
	pub dst_set: DescriptorSet,
	pub dst_binding: u32,
	pub dst_array_element: u32,
	pub descriptor_count: u32,
}

#[repr(C)] pub struct BufferCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: BufferCreateFlags,
	pub size: DeviceSize,
	pub usage: BufferUsageFlags,
	pub sharing_mode: SharingMode,
	pub queue_family_index_count: u32,
	pub p_queue_family_indices: *const u32,
}

#[repr(C)] pub struct BufferViewCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: BufferViewCreateFlags,
	pub buffer: Buffer,
	pub format: Format,
	pub offset: DeviceSize,
	pub range: DeviceSize,
}

#[repr(C)] pub struct ImageSubresource {
	pub aspect_mask: ImageAspectFlags,
	pub mip_level: u32,
	pub array_layer: u32,
}

#[repr(C)] pub struct ImageSubresourceLayers {
	pub aspect_mask: ImageAspectFlags,
	pub mip_level: u32,
	pub base_array_layer: u32,
	pub layer_count: u32,
}

#[repr(C)] pub struct ImageSubresourceRange {
	pub aspect_mask: ImageAspectFlags,
	pub base_mip_level: u32,
	pub level_count: u32,
	pub base_array_layer: u32,
	pub layer_count: u32,
}

#[repr(C)] pub struct MemoryBarrier {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub src_access_mask: AccessFlags,
	pub dst_access_mask: AccessFlags,
}

#[repr(C)] pub struct BufferMemoryBarrier {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub src_access_mask: AccessFlags,
	pub dst_access_mask: AccessFlags,
	pub src_queue_family_index: u32,
	pub dst_queue_family_index: u32,
	pub buffer: Buffer,
	pub offset: DeviceSize,
	pub size: DeviceSize,
}

#[repr(C)] pub struct ImageMemoryBarrier {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub src_access_mask: AccessFlags,
	pub dst_access_mask: AccessFlags,
	pub old_layout: ImageLayout,
	pub new_layout: ImageLayout,
	pub src_queue_family_index: u32,
	pub dst_queue_family_index: u32,
	pub image: Image,
	pub subresource_range: ImageSubresourceRange,
}

#[repr(C)] pub struct ImageCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: ImageCreateFlags,
	pub image_type: ImageType,
	pub format: Format,
	pub extent: Extent3D,
	pub mip_levels: u32,
	pub array_layers: u32,
	pub samples: SampleCountFlagBits,
	pub tiling: ImageTiling,
	pub usage: ImageUsageFlags,
	pub sharing_mode: SharingMode,
	pub queue_family_index_count: u32,
	pub p_queue_family_indices: *const u32,
	pub initial_layout: ImageLayout,
}

#[repr(C)] pub struct SubresourceLayout {
	pub offset: DeviceSize,
	pub size: DeviceSize,
	pub row_pitch: DeviceSize,
	pub array_pitch: DeviceSize,
	pub depth_pitch: DeviceSize,
}

#[repr(C)] pub struct ImageViewCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: ImageViewCreateFlags,
	pub image: Image,
	pub view_type: ImageViewType,
	pub format: Format,
	pub components: ComponentMapping,
	pub subresource_range: ImageSubresourceRange,
}

#[repr(C)] pub struct BufferCopy {
	pub src_offset: DeviceSize,
	pub dst_offset: DeviceSize,
	pub size: DeviceSize,
}

#[repr(C)] pub struct SparseMemoryBind {
	pub resource_offset: DeviceSize,
	pub size: DeviceSize,
	pub memory: DeviceMemory,
	pub memory_offset: DeviceSize,
	pub flags: SparseMemoryBindFlags,
}

#[repr(C)] pub struct SparseImageMemoryBind {
	pub subresource: ImageSubresource,
	pub offset: Offset3D,
	pub extent: Extent3D,
	pub memory: DeviceMemory,
	pub memory_offset: DeviceSize,
	pub flags: SparseMemoryBindFlags,
}

#[repr(C)] pub struct SparseBufferMemoryBindInfo {
	pub buffer: Buffer,
	pub bind_count: u32,
	pub p_binds: *const SparseMemoryBind,
}

#[repr(C)] pub struct SparseImageOpaqueMemoryBindInfo {
	pub image: Image,
	pub bind_count: u32,
	pub p_binds: *const SparseMemoryBind,
}

#[repr(C)] pub struct SparseImageMemoryBindInfo {
	pub image: Image,
	pub bind_count: u32,
	pub p_binds: *const SparseImageMemoryBind,
}

#[repr(C)] pub struct BindSparseInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub wait_semaphore_count: u32,
	pub p_wait_semaphores: *const Semaphore,
	pub buffer_bind_count: u32,
	pub p_buffer_binds: *const SparseBufferMemoryBindInfo,
	pub image_opaque_bind_count: u32,
	pub p_image_opaque_binds: *const SparseImageOpaqueMemoryBindInfo,
	pub image_bind_count: u32,
	pub p_image_binds: *const SparseImageMemoryBindInfo,
	pub signal_semaphore_count: u32,
	pub p_signal_semaphores: *const Semaphore,
}

#[repr(C)] pub struct ImageCopy {
	pub src_subresource: ImageSubresourceLayers,
	pub src_offset: Offset3D,
	pub dst_subresource: ImageSubresourceLayers,
	pub dst_offset: Offset3D,
	pub extent: Extent3D,
}

#[repr(C)] pub struct ImageBlit {
	pub src_subresource: ImageSubresourceLayers,
	pub src_offsets: Offset3D,
	pub dst_subresource: ImageSubresourceLayers,
	pub dst_offsets: Offset3D,
}

#[repr(C)] pub struct BufferImageCopy {
	pub buffer_offset: DeviceSize,
	pub buffer_row_length: u32,
	pub buffer_image_height: u32,
	pub image_subresource: ImageSubresourceLayers,
	pub image_offset: Offset3D,
	pub image_extent: Extent3D,
}

#[repr(C)] pub struct ImageResolve {
	pub src_subresource: ImageSubresourceLayers,
	pub src_offset: Offset3D,
	pub dst_subresource: ImageSubresourceLayers,
	pub dst_offset: Offset3D,
	pub extent: Extent3D,
}

#[repr(C)] pub struct ShaderModuleCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: ShaderModuleCreateFlags,
	pub code_size: usize,
	pub p_code: *const u32,
}

#[repr(C)] pub struct DescriptorSetLayoutBinding {
	pub binding: u32,
	pub descriptor_type: DescriptorType,
	pub descriptor_count: u32,
	pub stage_flags: ShaderStageFlags,
	pub p_immutable_samplers: *const Sampler,
}

#[repr(C)] pub struct DescriptorSetLayoutCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: DescriptorSetLayoutCreateFlags,
	pub binding_count: u32,
	pub p_bindings: *const DescriptorSetLayoutBinding,
}

#[repr(C)] pub struct DescriptorPoolSize {
	pub ty: DescriptorType,
	pub descriptor_count: u32,
}

#[repr(C)] pub struct DescriptorPoolCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: DescriptorPoolCreateFlags,
	pub max_sets: u32,
	pub pool_size_count: u32,
	pub p_pool_sizes: *const DescriptorPoolSize,
}

#[repr(C)] pub struct DescriptorSetAllocateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub descriptor_pool: DescriptorPool,
	pub descriptor_set_count: u32,
	pub p_set_layouts: *const DescriptorSetLayout,
}

#[repr(C)] pub struct SpecializationMapEntry {
	pub constant_id: u32,
	pub offset: u32,
	pub size: usize,
}

#[repr(C)] pub struct SpecializationInfo {
	pub map_entry_count: u32,
	pub p_map_entries: *const SpecializationMapEntry,
	pub data_size: usize,
	pub p_data: *const libc::c_void,
}

#[repr(C)] pub struct PipelineShaderStageCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: PipelineShaderStageCreateFlags,
	pub stage: ShaderStageFlagBits,
	pub module: ShaderModule,
	pub p_name: *const u8,
	pub p_specialization_info: *const SpecializationInfo,
}

#[repr(C)] pub struct ComputePipelineCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: PipelineCreateFlags,
	pub stage: PipelineShaderStageCreateInfo,
	pub layout: PipelineLayout,
	pub base_pipeline_handle: Pipeline,
	pub base_pipeline_index: i32,
}

#[repr(C)] pub struct VertexInputBindingDescription {
	pub binding: u32,
	pub stride: u32,
	pub input_rate: VertexInputRate,
}

#[repr(C)] pub struct VertexInputAttributeDescription {
	pub location: u32,
	pub binding: u32,
	pub format: Format,
	pub offset: u32,
}

#[repr(C)] pub struct PipelineVertexInputStateCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: PipelineVertexInputStateCreateFlags,
	pub vertex_binding_description_count: u32,
	pub p_vertex_binding_descriptions: *const VertexInputBindingDescription,
	pub vertex_attribute_description_count: u32,
	pub p_vertex_attribute_descriptions: *const VertexInputAttributeDescription,
}

#[repr(C)] pub struct PipelineInputAssemblyStateCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: PipelineInputAssemblyStateCreateFlags,
	pub topology: PrimitiveTopology,
	pub primitive_restart_enable: Bool32,
}

#[repr(C)] pub struct PipelineTessellationStateCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: PipelineTessellationStateCreateFlags,
	pub patch_control_points: u32,
}

#[repr(C)] pub struct PipelineViewportStateCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: PipelineViewportStateCreateFlags,
	pub viewport_count: u32,
	pub p_viewports: *const Viewport,
	pub scissor_count: u32,
	pub p_scissors: *const Rect2D,
}

#[repr(C)] pub struct PipelineRasterizationStateCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
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

#[repr(C)] pub struct PipelineMultisampleStateCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: PipelineMultisampleStateCreateFlags,
	pub rasterization_samples: SampleCountFlagBits,
	pub sample_shading_enable: Bool32,
	pub min_sample_shading: f32,
	pub p_sample_mask: *const SampleMask,
	pub alpha_to_coverage_enable: Bool32,
	pub alpha_to_one_enable: Bool32,
}

#[repr(C)] pub struct PipelineColorBlendAttachmentState {
	pub blend_enable: Bool32,
	pub src_color_blend_factor: BlendFactor,
	pub dst_color_blend_factor: BlendFactor,
	pub color_blend_op: BlendOp,
	pub src_alpha_blend_factor: BlendFactor,
	pub dst_alpha_blend_factor: BlendFactor,
	pub alpha_blend_op: BlendOp,
	pub color_write_mask: ColorComponentFlags,
}

#[repr(C)] pub struct PipelineColorBlendStateCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: PipelineColorBlendStateCreateFlags,
	pub logic_op_enable: Bool32,
	pub logic_op: LogicOp,
	pub attachment_count: u32,
	pub p_attachments: *const PipelineColorBlendAttachmentState,
	pub blend_constants: f32,
}

#[repr(C)] pub struct PipelineDynamicStateCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: PipelineDynamicStateCreateFlags,
	pub dynamic_state_count: u32,
	pub p_dynamic_states: *const DynamicState,
}

#[repr(C)] pub struct StencilOpState {
	pub fail_op: StencilOp,
	pub pass_op: StencilOp,
	pub depth_fail_op: StencilOp,
	pub compare_op: CompareOp,
	pub compare_mask: u32,
	pub write_mask: u32,
	pub reference: u32,
}

#[repr(C)] pub struct PipelineDepthStencilStateCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
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

#[repr(C)] pub struct GraphicsPipelineCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: PipelineCreateFlags,
	pub stage_count: u32,
	pub p_stages: *const PipelineShaderStageCreateInfo,
	pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo,
	pub p_input_assembly_state: *const PipelineInputAssemblyStateCreateInfo,
	pub p_tessellation_state: *const PipelineTessellationStateCreateInfo,
	pub p_viewport_state: *const PipelineViewportStateCreateInfo,
	pub p_rasterization_state: *const PipelineRasterizationStateCreateInfo,
	pub p_multisample_state: *const PipelineMultisampleStateCreateInfo,
	pub p_depth_stencil_state: *const PipelineDepthStencilStateCreateInfo,
	pub p_color_blend_state: *const PipelineColorBlendStateCreateInfo,
	pub p_dynamic_state: *const PipelineDynamicStateCreateInfo,
	pub layout: PipelineLayout,
	pub render_pass: RenderPass,
	pub subpass: u32,
	pub base_pipeline_handle: Pipeline,
	pub base_pipeline_index: i32,
}

#[repr(C)] pub struct PipelineCacheCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: PipelineCacheCreateFlags,
	pub initial_data_size: usize,
	pub p_initial_data: *const libc::c_void,
}

#[repr(C)] pub struct PushConstantRange {
	pub stage_flags: ShaderStageFlags,
	pub offset: u32,
	pub size: u32,
}

#[repr(C)] pub struct PipelineLayoutCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: PipelineLayoutCreateFlags,
	pub set_layout_count: u32,
	pub p_set_layouts: *const DescriptorSetLayout,
	pub push_constant_range_count: u32,
	pub p_push_constant_ranges: *const PushConstantRange,
}

#[repr(C)] pub struct SamplerCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
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

#[repr(C)] pub struct CommandPoolCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: CommandPoolCreateFlags,
	pub queue_family_index: u32,
}

#[repr(C)] pub struct CommandBufferAllocateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub command_pool: CommandPool,
	pub level: CommandBufferLevel,
	pub command_buffer_count: u32,
}

#[repr(C)] pub struct CommandBufferInheritanceInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub render_pass: RenderPass,
	pub subpass: u32,
	pub framebuffer: Framebuffer,
	pub occlusion_query_enable: Bool32,
	pub query_flags: QueryControlFlags,
	pub pipeline_statistics: QueryPipelineStatisticFlags,
}

#[repr(C)] pub struct CommandBufferBeginInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: CommandBufferUsageFlags,
	pub p_inheritance_info: *const CommandBufferInheritanceInfo,
}

#[repr(C)] pub struct RenderPassBeginInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub render_pass: RenderPass,
	pub framebuffer: Framebuffer,
	pub render_area: Rect2D,
	pub clear_value_count: u32,
	pub p_clear_values: *const ClearValue,
}

pub type ClearColorValue = *mut libc::c_void; // FOOTLONG: Wildly wrong
#[repr(C)] pub struct ClearDepthStencilValue {
	pub depth: f32,
	pub stencil: u32,
}

pub type ClearValue = *mut libc::c_void; // FOOTLONG: Wildly wrong
#[repr(C)] pub struct ClearAttachment {
	pub aspect_mask: ImageAspectFlags,
	pub color_attachment: u32,
	pub clear_value: ClearValue,
}

#[repr(C)] pub struct AttachmentDescription {
	pub flags: AttachmentDescriptionFlags,
	pub format: Format,
	pub samples: SampleCountFlagBits,
	pub load_op: AttachmentLoadOp,
	pub store_op: AttachmentStoreOp,
	pub stencil_load_op: AttachmentLoadOp,
	pub stencil_store_op: AttachmentStoreOp,
	pub initial_layout: ImageLayout,
	pub final_layout: ImageLayout,
}

#[repr(C)] pub struct AttachmentReference {
	pub attachment: u32,
	pub layout: ImageLayout,
}

#[repr(C)] pub struct SubpassDescription {
	pub flags: SubpassDescriptionFlags,
	pub pipeline_bind_point: PipelineBindPoint,
	pub input_attachment_count: u32,
	pub p_input_attachments: *const AttachmentReference,
	pub color_attachment_count: u32,
	pub p_color_attachments: *const AttachmentReference,
	pub p_resolve_attachments: *const AttachmentReference,
	pub p_depth_stencil_attachment: *const AttachmentReference,
	pub preserve_attachment_count: u32,
	pub p_preserve_attachments: *const u32,
}

#[repr(C)] pub struct SubpassDependency {
	pub src_subpass: u32,
	pub dst_subpass: u32,
	pub src_stage_mask: PipelineStageFlags,
	pub dst_stage_mask: PipelineStageFlags,
	pub src_access_mask: AccessFlags,
	pub dst_access_mask: AccessFlags,
	pub dependency_flags: DependencyFlags,
}

#[repr(C)] pub struct RenderPassCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: RenderPassCreateFlags,
	pub attachment_count: u32,
	pub p_attachments: *const AttachmentDescription,
	pub subpass_count: u32,
	pub p_subpasses: *const SubpassDescription,
	pub dependency_count: u32,
	pub p_dependencies: *const SubpassDependency,
}

#[repr(C)] pub struct EventCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: EventCreateFlags,
}

#[repr(C)] pub struct FenceCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: FenceCreateFlags,
}

#[repr(C)] pub struct PhysicalDeviceFeatures {
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

#[repr(C)] pub struct PhysicalDeviceSparseProperties {
	pub residency_standard2_d_block_shape: Bool32,
	pub residency_standard2_d_multisample_block_shape: Bool32,
	pub residency_standard3_d_block_shape: Bool32,
	pub residency_aligned_mip_size: Bool32,
	pub residency_non_resident_strict: Bool32,
}

#[repr(C)] pub struct PhysicalDeviceLimits {
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
	pub min_memory_map_alignment: usize,
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

#[repr(C)] pub struct SemaphoreCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: SemaphoreCreateFlags,
}

#[repr(C)] pub struct QueryPoolCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: QueryPoolCreateFlags,
	pub query_type: QueryType,
	pub query_count: u32,
	pub pipeline_statistics: QueryPipelineStatisticFlags,
}

#[repr(C)] pub struct FramebufferCreateInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: FramebufferCreateFlags,
	pub render_pass: RenderPass,
	pub attachment_count: u32,
	pub p_attachments: *const ImageView,
	pub width: u32,
	pub height: u32,
	pub layers: u32,
}

#[repr(C)] pub struct DrawIndirectCommand {
	pub vertex_count: u32,
	pub instance_count: u32,
	pub first_vertex: u32,
	pub first_instance: u32,
}

#[repr(C)] pub struct DrawIndexedIndirectCommand {
	pub index_count: u32,
	pub instance_count: u32,
	pub first_index: u32,
	pub vertex_offset: i32,
	pub first_instance: u32,
}

#[repr(C)] pub struct DispatchIndirectCommand {
	pub x: u32,
	pub y: u32,
	pub z: u32,
}

#[repr(C)] pub struct SubmitInfo {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub wait_semaphore_count: u32,
	pub p_wait_semaphores: *const Semaphore,
	pub p_wait_dst_stage_mask: *const PipelineStageFlags,
	pub command_buffer_count: u32,
	pub p_command_buffers: *const CommandBuffer,
	pub signal_semaphore_count: u32,
	pub p_signal_semaphores: *const Semaphore,
}

#[repr(C)] pub struct DisplayPropertiesKHR {
	pub display: DisplayKHR,
	pub display_name: *const u8,
	pub physical_dimensions: Extent2D,
	pub physical_resolution: Extent2D,
	pub supported_transforms: SurfaceTransformFlagsKHR,
	pub plane_reorder_possible: Bool32,
	pub persistent_content: Bool32,
}

#[repr(C)] pub struct DisplayPlanePropertiesKHR {
	pub current_display: DisplayKHR,
	pub current_stack_index: u32,
}

#[repr(C)] pub struct DisplayModeParametersKHR {
	pub visible_region: Extent2D,
	pub refresh_rate: u32,
}

#[repr(C)] pub struct DisplayModePropertiesKHR {
	pub display_mode: DisplayModeKHR,
	pub parameters: DisplayModeParametersKHR,
}

#[repr(C)] pub struct DisplayModeCreateInfoKHR {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: DisplayModeCreateFlagsKHR,
	pub parameters: DisplayModeParametersKHR,
}

#[repr(C)] pub struct DisplayPlaneCapabilitiesKHR {
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

#[repr(C)] pub struct DisplaySurfaceCreateInfoKHR {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: DisplaySurfaceCreateFlagsKHR,
	pub display_mode: DisplayModeKHR,
	pub plane_index: u32,
	pub plane_stack_index: u32,
	pub transform: SurfaceTransformFlagBitsKHR,
	pub global_alpha: f32,
	pub alpha_mode: DisplayPlaneAlphaFlagBitsKHR,
	pub image_extent: Extent2D,
}

#[repr(C)] pub struct DisplayPresentInfoKHR {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub src_rect: Rect2D,
	pub dst_rect: Rect2D,
	pub persistent: Bool32,
}

#[repr(C)] pub struct SurfaceCapabilitiesKHR {
	pub min_image_count: u32,
	pub max_image_count: u32,
	pub current_extent: Extent2D,
	pub min_image_extent: Extent2D,
	pub max_image_extent: Extent2D,
	pub max_image_array_layers: u32,
	pub supported_transforms: SurfaceTransformFlagsKHR,
	pub current_transform: SurfaceTransformFlagBitsKHR,
	pub supported_composite_alpha: CompositeAlphaFlagsKHR,
	pub supported_usage_flags: ImageUsageFlags,
}

#[repr(C)] pub struct AndroidSurfaceCreateInfoKHR {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: AndroidSurfaceCreateFlagsKHR,
	pub window: *mut *mut libc::c_void,
}

#[repr(C)] pub struct MirSurfaceCreateInfoKHR {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: MirSurfaceCreateFlagsKHR,
	pub connection: *mut *mut libc::c_void,
	pub mir_surface: *mut *mut libc::c_void,
}

#[repr(C)] pub struct WaylandSurfaceCreateInfoKHR {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: WaylandSurfaceCreateFlagsKHR,
	pub display: *mut *mut libc::c_void,
	pub surface: *mut *mut libc::c_void,
}

#[repr(C)] pub struct Win32SurfaceCreateInfoKHR {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: Win32SurfaceCreateFlagsKHR,
	pub hinstance: *mut libc::c_void,
	pub hwnd: *mut libc::c_void,
}

#[repr(C)] pub struct XlibSurfaceCreateInfoKHR {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: XlibSurfaceCreateFlagsKHR,
	pub dpy: *mut *mut libc::c_void,
	pub window: *mut libc::c_void,
}

#[repr(C)] pub struct XcbSurfaceCreateInfoKHR {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: XcbSurfaceCreateFlagsKHR,
	pub connection: *mut *mut libc::c_void,
	pub window: *mut libc::c_void,
}

#[repr(C)] pub struct SurfaceFormatKHR {
	pub format: Format,
	pub color_space: ColorSpaceKHR,
}

#[repr(C)] pub struct SwapchainCreateInfoKHR {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: SwapchainCreateFlagsKHR,
	pub surface: SurfaceKHR,
	pub min_image_count: u32,
	pub image_format: Format,
	pub image_color_space: ColorSpaceKHR,
	pub image_extent: Extent2D,
	pub image_array_layers: u32,
	pub image_usage: ImageUsageFlags,
	pub image_sharing_mode: SharingMode,
	pub queue_family_index_count: u32,
	pub p_queue_family_indices: *const u32,
	pub pre_transform: SurfaceTransformFlagBitsKHR,
	pub composite_alpha: CompositeAlphaFlagBitsKHR,
	pub present_mode: PresentModeKHR,
	pub clipped: Bool32,
	pub old_swapchain: SwapchainKHR,
}

#[repr(C)] pub struct PresentInfoKHR {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub wait_semaphore_count: u32,
	pub p_wait_semaphores: *const Semaphore,
	pub swapchain_count: u32,
	pub p_swapchains: *const SwapchainKHR,
	pub p_image_indices: *const u32,
	pub p_results: *mut Result,
}

#[repr(C)] pub struct DebugReportCallbackCreateInfoEXT {
	pub s_type: StructureType,
	pub p_next: *const libc::c_void,
	pub flags: DebugReportFlagsEXT,
	pub pfn_callback: PFN_vkDebugReportCallbackEXT,
	pub p_user_data: *mut libc::c_void,
}

#[link(name = "vulkan-1")] extern "C" {
	pub fn vkCreateInstance(p_create_info: *const InstanceCreateInfo, p_allocator: *const AllocationCallbacks, p_instance: *mut Instance) -> Result;
	pub fn vkDestroyInstance(instance: Instance, p_allocator: *const AllocationCallbacks);
	pub fn vkEnumeratePhysicalDevices(instance: Instance, p_physical_device_count: *mut u32, p_physical_devices: *mut PhysicalDevice) -> Result;
	pub fn vkGetDeviceProcAddr(device: Device, p_name: *const u8) -> PFN_vkVoidFunction;
	pub fn vkGetInstanceProcAddr(instance: Instance, p_name: *const u8) -> PFN_vkVoidFunction;
	pub fn vkGetPhysicalDeviceProperties(physical_device: PhysicalDevice, p_properties: *mut PhysicalDeviceProperties);
	pub fn vkGetPhysicalDeviceQueueFamilyProperties(physical_device: PhysicalDevice, p_queue_family_property_count: *mut u32, p_queue_family_properties: *mut QueueFamilyProperties);
	pub fn vkGetPhysicalDeviceMemoryProperties(physical_device: PhysicalDevice, p_memory_properties: *mut PhysicalDeviceMemoryProperties);
	pub fn vkGetPhysicalDeviceFeatures(physical_device: PhysicalDevice, p_features: *mut PhysicalDeviceFeatures);
	pub fn vkGetPhysicalDeviceFormatProperties(physical_device: PhysicalDevice, format: Format, p_format_properties: *mut FormatProperties);
	pub fn vkGetPhysicalDeviceImageFormatProperties(physical_device: PhysicalDevice, format: Format, ty: ImageType, tiling: ImageTiling, usage: ImageUsageFlags, flags: ImageCreateFlags, p_image_format_properties: *mut ImageFormatProperties) -> Result;
	pub fn vkCreateDevice(physical_device: PhysicalDevice, p_create_info: *const DeviceCreateInfo, p_allocator: *const AllocationCallbacks, p_device: *mut Device) -> Result;
	pub fn vkDestroyDevice(device: Device, p_allocator: *const AllocationCallbacks);
	pub fn vkEnumerateInstanceLayerProperties(p_property_count: *mut u32, p_properties: *mut LayerProperties) -> Result;
	pub fn vkEnumerateInstanceExtensionProperties(p_layer_name: *const u8, p_property_count: *mut u32, p_properties: *mut ExtensionProperties) -> Result;
	pub fn vkEnumerateDeviceLayerProperties(physical_device: PhysicalDevice, p_property_count: *mut u32, p_properties: *mut LayerProperties) -> Result;
	pub fn vkEnumerateDeviceExtensionProperties(physical_device: PhysicalDevice, p_layer_name: *const u8, p_property_count: *mut u32, p_properties: *mut ExtensionProperties) -> Result;
	pub fn vkGetDeviceQueue(device: Device, queue_family_index: u32, queue_index: u32, p_queue: *mut Queue);
	pub fn vkQueueSubmit(queue: Queue, submit_count: u32, p_submits: *const SubmitInfo, fence: Fence) -> Result;
	pub fn vkQueueWaitIdle(queue: Queue) -> Result;
	pub fn vkDeviceWaitIdle(device: Device) -> Result;
	pub fn vkAllocateMemory(device: Device, p_allocate_info: *const MemoryAllocateInfo, p_allocator: *const AllocationCallbacks, p_memory: *mut DeviceMemory) -> Result;
	pub fn vkFreeMemory(device: Device, memory: DeviceMemory, p_allocator: *const AllocationCallbacks);
	pub fn vkMapMemory(device: Device, memory: DeviceMemory, offset: DeviceSize, size: DeviceSize, flags: MemoryMapFlags, pp_data: *mut libc::c_void) -> Result;
	pub fn vkUnmapMemory(device: Device, memory: DeviceMemory);
	pub fn vkFlushMappedMemoryRanges(device: Device, memory_range_count: u32, p_memory_ranges: *const MappedMemoryRange) -> Result;
	pub fn vkInvalidateMappedMemoryRanges(device: Device, memory_range_count: u32, p_memory_ranges: *const MappedMemoryRange) -> Result;
	pub fn vkGetDeviceMemoryCommitment(device: Device, memory: DeviceMemory, p_committed_memory_in_bytes: *mut DeviceSize);
	pub fn vkGetBufferMemoryRequirements(device: Device, buffer: Buffer, p_memory_requirements: *mut MemoryRequirements);
	pub fn vkBindBufferMemory(device: Device, buffer: Buffer, memory: DeviceMemory, memory_offset: DeviceSize) -> Result;
	pub fn vkGetImageMemoryRequirements(device: Device, image: Image, p_memory_requirements: *mut MemoryRequirements);
	pub fn vkBindImageMemory(device: Device, image: Image, memory: DeviceMemory, memory_offset: DeviceSize) -> Result;
	pub fn vkGetImageSparseMemoryRequirements(device: Device, image: Image, p_sparse_memory_requirement_count: *mut u32, p_sparse_memory_requirements: *mut SparseImageMemoryRequirements);
	pub fn vkGetPhysicalDeviceSparseImageFormatProperties(physical_device: PhysicalDevice, format: Format, ty: ImageType, samples: SampleCountFlagBits, usage: ImageUsageFlags, tiling: ImageTiling, p_property_count: *mut u32, p_properties: *mut SparseImageFormatProperties);
	pub fn vkQueueBindSparse(queue: Queue, bind_info_count: u32, p_bind_info: *const BindSparseInfo, fence: Fence) -> Result;
	pub fn vkCreateFence(device: Device, p_create_info: *const FenceCreateInfo, p_allocator: *const AllocationCallbacks, p_fence: *mut Fence) -> Result;
	pub fn vkDestroyFence(device: Device, fence: Fence, p_allocator: *const AllocationCallbacks);
	pub fn vkResetFences(device: Device, fence_count: u32, p_fences: *const Fence) -> Result;
	pub fn vkGetFenceStatus(device: Device, fence: Fence) -> Result;
	pub fn vkWaitForFences(device: Device, fence_count: u32, p_fences: *const Fence, wait_all: Bool32, timeout: u64) -> Result;
	pub fn vkCreateSemaphore(device: Device, p_create_info: *const SemaphoreCreateInfo, p_allocator: *const AllocationCallbacks, p_semaphore: *mut Semaphore) -> Result;
	pub fn vkDestroySemaphore(device: Device, semaphore: Semaphore, p_allocator: *const AllocationCallbacks);
	pub fn vkCreateEvent(device: Device, p_create_info: *const EventCreateInfo, p_allocator: *const AllocationCallbacks, p_event: *mut Event) -> Result;
	pub fn vkDestroyEvent(device: Device, event: Event, p_allocator: *const AllocationCallbacks);
	pub fn vkGetEventStatus(device: Device, event: Event) -> Result;
	pub fn vkSetEvent(device: Device, event: Event) -> Result;
	pub fn vkResetEvent(device: Device, event: Event) -> Result;
	pub fn vkCreateQueryPool(device: Device, p_create_info: *const QueryPoolCreateInfo, p_allocator: *const AllocationCallbacks, p_query_pool: *mut QueryPool) -> Result;
	pub fn vkDestroyQueryPool(device: Device, query_pool: QueryPool, p_allocator: *const AllocationCallbacks);
	pub fn vkGetQueryPoolResults(device: Device, query_pool: QueryPool, first_query: u32, query_count: u32, data_size: usize, p_data: *mut libc::c_void, stride: DeviceSize, flags: QueryResultFlags) -> Result;
	pub fn vkCreateBuffer(device: Device, p_create_info: *const BufferCreateInfo, p_allocator: *const AllocationCallbacks, p_buffer: *mut Buffer) -> Result;
	pub fn vkDestroyBuffer(device: Device, buffer: Buffer, p_allocator: *const AllocationCallbacks);
	pub fn vkCreateBufferView(device: Device, p_create_info: *const BufferViewCreateInfo, p_allocator: *const AllocationCallbacks, p_view: *mut BufferView) -> Result;
	pub fn vkDestroyBufferView(device: Device, buffer_view: BufferView, p_allocator: *const AllocationCallbacks);
	pub fn vkCreateImage(device: Device, p_create_info: *const ImageCreateInfo, p_allocator: *const AllocationCallbacks, p_image: *mut Image) -> Result;
	pub fn vkDestroyImage(device: Device, image: Image, p_allocator: *const AllocationCallbacks);
	pub fn vkGetImageSubresourceLayout(device: Device, image: Image, p_subresource: *const ImageSubresource, p_layout: *mut SubresourceLayout);
	pub fn vkCreateImageView(device: Device, p_create_info: *const ImageViewCreateInfo, p_allocator: *const AllocationCallbacks, p_view: *mut ImageView) -> Result;
	pub fn vkDestroyImageView(device: Device, image_view: ImageView, p_allocator: *const AllocationCallbacks);
	pub fn vkCreateShaderModule(device: Device, p_create_info: *const ShaderModuleCreateInfo, p_allocator: *const AllocationCallbacks, p_shader_module: *mut ShaderModule) -> Result;
	pub fn vkDestroyShaderModule(device: Device, shader_module: ShaderModule, p_allocator: *const AllocationCallbacks);
	pub fn vkCreatePipelineCache(device: Device, p_create_info: *const PipelineCacheCreateInfo, p_allocator: *const AllocationCallbacks, p_pipeline_cache: *mut PipelineCache) -> Result;
	pub fn vkDestroyPipelineCache(device: Device, pipeline_cache: PipelineCache, p_allocator: *const AllocationCallbacks);
	pub fn vkGetPipelineCacheData(device: Device, pipeline_cache: PipelineCache, p_data_size: *mut usize, p_data: *mut libc::c_void) -> Result;
	pub fn vkMergePipelineCaches(device: Device, dst_cache: PipelineCache, src_cache_count: u32, p_src_caches: *const PipelineCache) -> Result;
	pub fn vkCreateGraphicsPipelines(device: Device, pipeline_cache: PipelineCache, create_info_count: u32, p_create_infos: *const GraphicsPipelineCreateInfo, p_allocator: *const AllocationCallbacks, p_pipelines: *mut Pipeline) -> Result;
	pub fn vkCreateComputePipelines(device: Device, pipeline_cache: PipelineCache, create_info_count: u32, p_create_infos: *const ComputePipelineCreateInfo, p_allocator: *const AllocationCallbacks, p_pipelines: *mut Pipeline) -> Result;
	pub fn vkDestroyPipeline(device: Device, pipeline: Pipeline, p_allocator: *const AllocationCallbacks);
	pub fn vkCreatePipelineLayout(device: Device, p_create_info: *const PipelineLayoutCreateInfo, p_allocator: *const AllocationCallbacks, p_pipeline_layout: *mut PipelineLayout) -> Result;
	pub fn vkDestroyPipelineLayout(device: Device, pipeline_layout: PipelineLayout, p_allocator: *const AllocationCallbacks);
	pub fn vkCreateSampler(device: Device, p_create_info: *const SamplerCreateInfo, p_allocator: *const AllocationCallbacks, p_sampler: *mut Sampler) -> Result;
	pub fn vkDestroySampler(device: Device, sampler: Sampler, p_allocator: *const AllocationCallbacks);
	pub fn vkCreateDescriptorSetLayout(device: Device, p_create_info: *const DescriptorSetLayoutCreateInfo, p_allocator: *const AllocationCallbacks, p_set_layout: *mut DescriptorSetLayout) -> Result;
	pub fn vkDestroyDescriptorSetLayout(device: Device, descriptor_set_layout: DescriptorSetLayout, p_allocator: *const AllocationCallbacks);
	pub fn vkCreateDescriptorPool(device: Device, p_create_info: *const DescriptorPoolCreateInfo, p_allocator: *const AllocationCallbacks, p_descriptor_pool: *mut DescriptorPool) -> Result;
	pub fn vkDestroyDescriptorPool(device: Device, descriptor_pool: DescriptorPool, p_allocator: *const AllocationCallbacks);
	pub fn vkResetDescriptorPool(device: Device, descriptor_pool: DescriptorPool, flags: DescriptorPoolResetFlags) -> Result;
	pub fn vkAllocateDescriptorSets(device: Device, p_allocate_info: *const DescriptorSetAllocateInfo, p_descriptor_sets: *mut DescriptorSet) -> Result;
	pub fn vkFreeDescriptorSets(device: Device, descriptor_pool: DescriptorPool, descriptor_set_count: u32, p_descriptor_sets: *const DescriptorSet) -> Result;
	pub fn vkUpdateDescriptorSets(device: Device, descriptor_write_count: u32, p_descriptor_writes: *const WriteDescriptorSet, descriptor_copy_count: u32, p_descriptor_copies: *const CopyDescriptorSet);
	pub fn vkCreateFramebuffer(device: Device, p_create_info: *const FramebufferCreateInfo, p_allocator: *const AllocationCallbacks, p_framebuffer: *mut Framebuffer) -> Result;
	pub fn vkDestroyFramebuffer(device: Device, framebuffer: Framebuffer, p_allocator: *const AllocationCallbacks);
	pub fn vkCreateRenderPass(device: Device, p_create_info: *const RenderPassCreateInfo, p_allocator: *const AllocationCallbacks, p_render_pass: *mut RenderPass) -> Result;
	pub fn vkDestroyRenderPass(device: Device, render_pass: RenderPass, p_allocator: *const AllocationCallbacks);
	pub fn vkGetRenderAreaGranularity(device: Device, render_pass: RenderPass, p_granularity: *mut Extent2D);
	pub fn vkCreateCommandPool(device: Device, p_create_info: *const CommandPoolCreateInfo, p_allocator: *const AllocationCallbacks, p_command_pool: *mut CommandPool) -> Result;
	pub fn vkDestroyCommandPool(device: Device, command_pool: CommandPool, p_allocator: *const AllocationCallbacks);
	pub fn vkResetCommandPool(device: Device, command_pool: CommandPool, flags: CommandPoolResetFlags) -> Result;
	pub fn vkAllocateCommandBuffers(device: Device, p_allocate_info: *const CommandBufferAllocateInfo, p_command_buffers: *mut CommandBuffer) -> Result;
	pub fn vkFreeCommandBuffers(device: Device, command_pool: CommandPool, command_buffer_count: u32, p_command_buffers: *const CommandBuffer);
	pub fn vkBeginCommandBuffer(command_buffer: CommandBuffer, p_begin_info: *const CommandBufferBeginInfo) -> Result;
	pub fn vkEndCommandBuffer(command_buffer: CommandBuffer) -> Result;
	pub fn vkResetCommandBuffer(command_buffer: CommandBuffer, flags: CommandBufferResetFlags) -> Result;
	pub fn vkCmdBindPipeline(command_buffer: CommandBuffer, pipeline_bind_point: PipelineBindPoint, pipeline: Pipeline);
	pub fn vkCmdSetViewport(command_buffer: CommandBuffer, first_viewport: u32, viewport_count: u32, p_viewports: *const Viewport);
	pub fn vkCmdSetScissor(command_buffer: CommandBuffer, first_scissor: u32, scissor_count: u32, p_scissors: *const Rect2D);
	pub fn vkCmdSetLineWidth(command_buffer: CommandBuffer, line_width: f32);
	pub fn vkCmdSetDepthBias(command_buffer: CommandBuffer, depth_bias_constant_factor: f32, depth_bias_clamp: f32, depth_bias_slope_factor: f32);
	pub fn vkCmdSetBlendConstants(command_buffer: CommandBuffer, blend_constants: f32);
	pub fn vkCmdSetDepthBounds(command_buffer: CommandBuffer, min_depth_bounds: f32, max_depth_bounds: f32);
	pub fn vkCmdSetStencilCompareMask(command_buffer: CommandBuffer, face_mask: StencilFaceFlags, compare_mask: u32);
	pub fn vkCmdSetStencilWriteMask(command_buffer: CommandBuffer, face_mask: StencilFaceFlags, write_mask: u32);
	pub fn vkCmdSetStencilReference(command_buffer: CommandBuffer, face_mask: StencilFaceFlags, reference: u32);
	pub fn vkCmdBindDescriptorSets(command_buffer: CommandBuffer, pipeline_bind_point: PipelineBindPoint, layout: PipelineLayout, first_set: u32, descriptor_set_count: u32, p_descriptor_sets: *const DescriptorSet, dynamic_offset_count: u32, p_dynamic_offsets: *const u32);
	pub fn vkCmdBindIndexBuffer(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize, index_type: IndexType);
	pub fn vkCmdBindVertexBuffers(command_buffer: CommandBuffer, first_binding: u32, binding_count: u32, p_buffers: *const Buffer, p_offsets: *const DeviceSize);
	pub fn vkCmdDraw(command_buffer: CommandBuffer, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32);
	pub fn vkCmdDrawIndexed(command_buffer: CommandBuffer, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32);
	pub fn vkCmdDrawIndirect(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize, draw_count: u32, stride: u32);
	pub fn vkCmdDrawIndexedIndirect(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize, draw_count: u32, stride: u32);
	pub fn vkCmdDispatch(command_buffer: CommandBuffer, x: u32, y: u32, z: u32);
	pub fn vkCmdDispatchIndirect(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize);
	pub fn vkCmdCopyBuffer(command_buffer: CommandBuffer, src_buffer: Buffer, dst_buffer: Buffer, region_count: u32, p_regions: *const BufferCopy);
	pub fn vkCmdCopyImage(command_buffer: CommandBuffer, src_image: Image, src_image_layout: ImageLayout, dst_image: Image, dst_image_layout: ImageLayout, region_count: u32, p_regions: *const ImageCopy);
	pub fn vkCmdBlitImage(command_buffer: CommandBuffer, src_image: Image, src_image_layout: ImageLayout, dst_image: Image, dst_image_layout: ImageLayout, region_count: u32, p_regions: *const ImageBlit, filter: Filter);
	pub fn vkCmdCopyBufferToImage(command_buffer: CommandBuffer, src_buffer: Buffer, dst_image: Image, dst_image_layout: ImageLayout, region_count: u32, p_regions: *const BufferImageCopy);
	pub fn vkCmdCopyImageToBuffer(command_buffer: CommandBuffer, src_image: Image, src_image_layout: ImageLayout, dst_buffer: Buffer, region_count: u32, p_regions: *const BufferImageCopy);
	pub fn vkCmdUpdateBuffer(command_buffer: CommandBuffer, dst_buffer: Buffer, dst_offset: DeviceSize, data_size: DeviceSize, p_data: *const u32);
	pub fn vkCmdFillBuffer(command_buffer: CommandBuffer, dst_buffer: Buffer, dst_offset: DeviceSize, size: DeviceSize, data: u32);
	pub fn vkCmdClearColorImage(command_buffer: CommandBuffer, image: Image, image_layout: ImageLayout, p_color: *const ClearColorValue, range_count: u32, p_ranges: *const ImageSubresourceRange);
	pub fn vkCmdClearDepthStencilImage(command_buffer: CommandBuffer, image: Image, image_layout: ImageLayout, p_depth_stencil: *const ClearDepthStencilValue, range_count: u32, p_ranges: *const ImageSubresourceRange);
	pub fn vkCmdClearAttachments(command_buffer: CommandBuffer, attachment_count: u32, p_attachments: *const ClearAttachment, rect_count: u32, p_rects: *const ClearRect);
	pub fn vkCmdResolveImage(command_buffer: CommandBuffer, src_image: Image, src_image_layout: ImageLayout, dst_image: Image, dst_image_layout: ImageLayout, region_count: u32, p_regions: *const ImageResolve);
	pub fn vkCmdSetEvent(command_buffer: CommandBuffer, event: Event, stage_mask: PipelineStageFlags);
	pub fn vkCmdResetEvent(command_buffer: CommandBuffer, event: Event, stage_mask: PipelineStageFlags);
	pub fn vkCmdWaitEvents(command_buffer: CommandBuffer, event_count: u32, p_events: *const Event, src_stage_mask: PipelineStageFlags, dst_stage_mask: PipelineStageFlags, memory_barrier_count: u32, p_memory_barriers: *const MemoryBarrier, buffer_memory_barrier_count: u32, p_buffer_memory_barriers: *const BufferMemoryBarrier, image_memory_barrier_count: u32, p_image_memory_barriers: *const ImageMemoryBarrier);
	pub fn vkCmdPipelineBarrier(command_buffer: CommandBuffer, src_stage_mask: PipelineStageFlags, dst_stage_mask: PipelineStageFlags, dependency_flags: DependencyFlags, memory_barrier_count: u32, p_memory_barriers: *const MemoryBarrier, buffer_memory_barrier_count: u32, p_buffer_memory_barriers: *const BufferMemoryBarrier, image_memory_barrier_count: u32, p_image_memory_barriers: *const ImageMemoryBarrier);
	pub fn vkCmdBeginQuery(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32, flags: QueryControlFlags);
	pub fn vkCmdEndQuery(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32);
	pub fn vkCmdResetQueryPool(command_buffer: CommandBuffer, query_pool: QueryPool, first_query: u32, query_count: u32);
	pub fn vkCmdWriteTimestamp(command_buffer: CommandBuffer, pipeline_stage: PipelineStageFlagBits, query_pool: QueryPool, query: u32);
	pub fn vkCmdCopyQueryPoolResults(command_buffer: CommandBuffer, query_pool: QueryPool, first_query: u32, query_count: u32, dst_buffer: Buffer, dst_offset: DeviceSize, stride: DeviceSize, flags: QueryResultFlags);
	pub fn vkCmdPushConstants(command_buffer: CommandBuffer, layout: PipelineLayout, stage_flags: ShaderStageFlags, offset: u32, size: u32, p_values: *const libc::c_void);
	pub fn vkCmdBeginRenderPass(command_buffer: CommandBuffer, p_render_pass_begin: *const RenderPassBeginInfo, contents: SubpassContents);
	pub fn vkCmdNextSubpass(command_buffer: CommandBuffer, contents: SubpassContents);
	pub fn vkCmdEndRenderPass(command_buffer: CommandBuffer);
	pub fn vkCmdExecuteCommands(command_buffer: CommandBuffer, command_buffer_count: u32, p_command_buffers: *const CommandBuffer);
	pub fn vkCreateAndroidSurfaceKHR(instance: Instance, p_create_info: *const AndroidSurfaceCreateInfoKHR, p_allocator: *const AllocationCallbacks, p_surface: *mut SurfaceKHR) -> Result;
	pub fn vkGetPhysicalDeviceDisplayPropertiesKHR(physical_device: PhysicalDevice, p_property_count: *mut u32, p_properties: *mut DisplayPropertiesKHR) -> Result;
	pub fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physical_device: PhysicalDevice, p_property_count: *mut u32, p_properties: *mut DisplayPlanePropertiesKHR) -> Result;
	pub fn vkGetDisplayPlaneSupportedDisplaysKHR(physical_device: PhysicalDevice, plane_index: u32, p_display_count: *mut u32, p_displays: *mut DisplayKHR) -> Result;
	pub fn vkGetDisplayModePropertiesKHR(physical_device: PhysicalDevice, display: DisplayKHR, p_property_count: *mut u32, p_properties: *mut DisplayModePropertiesKHR) -> Result;
	pub fn vkCreateDisplayModeKHR(physical_device: PhysicalDevice, display: DisplayKHR, p_create_info: *const DisplayModeCreateInfoKHR, p_allocator: *const AllocationCallbacks, p_mode: *mut DisplayModeKHR) -> Result;
	pub fn vkGetDisplayPlaneCapabilitiesKHR(physical_device: PhysicalDevice, mode: DisplayModeKHR, plane_index: u32, p_capabilities: *mut DisplayPlaneCapabilitiesKHR) -> Result;
	pub fn vkCreateDisplayPlaneSurfaceKHR(instance: Instance, p_create_info: *const DisplaySurfaceCreateInfoKHR, p_allocator: *const AllocationCallbacks, p_surface: *mut SurfaceKHR) -> Result;
	pub fn vkCreateSharedSwapchainsKHR(device: Device, swapchain_count: u32, p_create_infos: *const SwapchainCreateInfoKHR, p_allocator: *const AllocationCallbacks, p_swapchains: *mut SwapchainKHR) -> Result;
	pub fn vkCreateMirSurfaceKHR(instance: Instance, p_create_info: *const MirSurfaceCreateInfoKHR, p_allocator: *const AllocationCallbacks, p_surface: *mut SurfaceKHR) -> Result;
	pub fn vkGetPhysicalDeviceMirPresentationSupportKHR(physical_device: PhysicalDevice, queue_family_index: u32, connection: *mut *mut libc::c_void) -> Bool32;
	pub fn vkDestroySurfaceKHR(instance: Instance, surface: SurfaceKHR, p_allocator: *const AllocationCallbacks);
	pub fn vkGetPhysicalDeviceSurfaceSupportKHR(physical_device: PhysicalDevice, queue_family_index: u32, surface: SurfaceKHR, p_supported: *mut Bool32) -> Result;
	pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device: PhysicalDevice, surface: SurfaceKHR, p_surface_capabilities: *mut SurfaceCapabilitiesKHR) -> Result;
	pub fn vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device: PhysicalDevice, surface: SurfaceKHR, p_surface_format_count: *mut u32, p_surface_formats: *mut SurfaceFormatKHR) -> Result;
	pub fn vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device: PhysicalDevice, surface: SurfaceKHR, p_present_mode_count: *mut u32, p_present_modes: *mut PresentModeKHR) -> Result;
	pub fn vkCreateSwapchainKHR(device: Device, p_create_info: *const SwapchainCreateInfoKHR, p_allocator: *const AllocationCallbacks, p_swapchain: *mut SwapchainKHR) -> Result;
	pub fn vkDestroySwapchainKHR(device: Device, swapchain: SwapchainKHR, p_allocator: *const AllocationCallbacks);
	pub fn vkGetSwapchainImagesKHR(device: Device, swapchain: SwapchainKHR, p_swapchain_image_count: *mut u32, p_swapchain_images: *mut Image) -> Result;
	pub fn vkAcquireNextImageKHR(device: Device, swapchain: SwapchainKHR, timeout: u64, semaphore: Semaphore, fence: Fence, p_image_index: *mut u32) -> Result;
	pub fn vkQueuePresentKHR(queue: Queue, p_present_info: *const PresentInfoKHR) -> Result;
	pub fn vkCreateWaylandSurfaceKHR(instance: Instance, p_create_info: *const WaylandSurfaceCreateInfoKHR, p_allocator: *const AllocationCallbacks, p_surface: *mut SurfaceKHR) -> Result;
	pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(physical_device: PhysicalDevice, queue_family_index: u32, display: *mut *mut libc::c_void) -> Bool32;
	pub fn vkCreateWin32SurfaceKHR(instance: Instance, p_create_info: *const Win32SurfaceCreateInfoKHR, p_allocator: *const AllocationCallbacks, p_surface: *mut SurfaceKHR) -> Result;
	pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR(physical_device: PhysicalDevice, queue_family_index: u32) -> Bool32;
	pub fn vkCreateXlibSurfaceKHR(instance: Instance, p_create_info: *const XlibSurfaceCreateInfoKHR, p_allocator: *const AllocationCallbacks, p_surface: *mut SurfaceKHR) -> Result;
	pub fn vkGetPhysicalDeviceXlibPresentationSupportKHR(physical_device: PhysicalDevice, queue_family_index: u32, dpy: *mut *mut libc::c_void, visual_id: *mut libc::c_void) -> Bool32;
	pub fn vkCreateXcbSurfaceKHR(instance: Instance, p_create_info: *const XcbSurfaceCreateInfoKHR, p_allocator: *const AllocationCallbacks, p_surface: *mut SurfaceKHR) -> Result;
	pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(physical_device: PhysicalDevice, queue_family_index: u32, connection: *mut *mut libc::c_void, visual_id: *mut libc::c_void) -> Bool32;
	pub fn vkCreateDebugReportCallbackEXT(instance: Instance, p_create_info: *const DebugReportCallbackCreateInfoEXT, p_allocator: *const AllocationCallbacks, p_callback: *mut DebugReportCallbackEXT) -> Result;
	pub fn vkDestroyDebugReportCallbackEXT(instance: Instance, callback: DebugReportCallbackEXT, p_allocator: *const AllocationCallbacks);
	pub fn vkDebugReportMessageEXT(instance: Instance, flags: DebugReportFlagsEXT, object_type: DebugReportObjectTypeEXT, object: u64, location: usize, message_code: i32, p_layer_prefix: *const u8, p_message: *const u8);
}

