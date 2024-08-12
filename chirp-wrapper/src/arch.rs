use chirp_sys::taichi;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arch {
    X64,
    Arm64,
    JS,
    Cuda,
    Metal,
    OpenGL,
    Dx11,
    Dx12,
    OpenCL,
    AMDGPU,
    Vulkan,
    GLES,
}

impl Arch {
    pub(crate) fn to_sys(&self) -> taichi::Arch {
        match self {
            Arch::AMDGPU => taichi::Arch::amdgpu,
            Arch::JS => taichi::Arch::js,
            Arch::GLES => taichi::Arch::gles,
            Arch::Cuda => taichi::Arch::cuda,
            Arch::X64 => taichi::Arch::x64,
            Arch::Arm64 => taichi::Arch::arm64,
            Arch::Metal => taichi::Arch::metal,
            Arch::OpenGL => taichi::Arch::opengl,
            Arch::OpenCL => taichi::Arch::opencl,
            Arch::Dx11 => taichi::Arch::dx11,
            Arch::Dx12 => taichi::Arch::dx12,
            Arch::Vulkan => taichi::Arch::vulkan,
        }
    }
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arch::AMDGPU => write!(f, "AMDGPU"),
            Arch::JS => write!(f, "JS"),
            Arch::GLES => write!(f, "GLES"),
            Arch::Cuda => write!(f, "CUDA"),
            Arch::X64 => write!(f, "x64"),
            Arch::Arm64 => write!(f, "ARM64"),
            Arch::Metal => write!(f, "Metal"),
            Arch::OpenGL => write!(f, "OpenGL"),
            Arch::OpenCL => write!(f, "OpenCL"),
            Arch::Dx11 => write!(f, "DirectX 11"),
            Arch::Dx12 => write!(f, "DirectX 12"),
            Arch::Vulkan => write!(f, "Vulkan"),
        }
    }
}