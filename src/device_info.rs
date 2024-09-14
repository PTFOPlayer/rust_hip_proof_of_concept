use libloading::Symbol;

extern "C" {
    pub fn wrapped_hipGetDeviceProperties(id: i32) -> hipDeviceProp_t;
}

#[derive(Default)]
pub struct hipDeviceArch_t {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: [u8; 3usize],
    pub __bindgen_padding_0: u8,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct hipUUID_t {
    pub bytes: [::libc::c_char; 16usize],
}

pub struct hipDeviceProp_t {
    #[doc = "< Device name."]
    pub name: [u8; 256usize],
    #[doc = "< Size of global memory region (in bytes)."]
    pub totalGlobalMem: usize,
    #[doc = "< Size of shared memory region (in bytes)."]
    pub sharedMemPerBlock: usize,
    #[doc = "< Registers per block."]
    pub regsPerBlock: ::libc::c_int,
    #[doc = "< Warp size."]
    pub warpSize: ::libc::c_int,
    #[doc = "< Max work items per work group or workgroup max size."]
    pub maxThreadsPerBlock: ::libc::c_int,
    #[doc = "< Max number of threads in each dimension (XYZ) of a block."]
    pub maxThreadsDim: [::libc::c_int; 3usize],
    #[doc = "< Max grid dimensions (XYZ)."]
    pub maxGridSize: [::libc::c_int; 3usize],
    #[doc = "< Max clock frequency of the multiProcessors in khz."]
    pub clockRate: ::libc::c_int,
    #[doc = "< Max global memory clock frequency in khz."]
    pub memoryClockRate: ::libc::c_int,
    #[doc = "< Global memory bus width in bits."]
    pub memoryBusWidth: ::libc::c_int,
    #[doc = "< Size of shared memory region (in bytes)."]
    pub totalConstMem: usize,
    #[doc = "< Major compute capability.  On HCC, this is an approximation and features may\n< differ from CUDA CC.  See the arch feature flags for portable ways to query\n< feature caps."]
    pub major: ::libc::c_int,
    #[doc = "< Minor compute capability.  On HCC, this is an approximation and features may\n< differ from CUDA CC.  See the arch feature flags for portable ways to query\n< feature caps."]
    pub minor: ::libc::c_int,
    #[doc = "< Number of multi-processors (compute units)."]
    pub multiProcessorCount: ::libc::c_int,
    #[doc = "< L2 cache size."]
    pub l2CacheSize: ::libc::c_int,
    #[doc = "< Maximum resident threads per multi-processor."]
    pub maxThreadsPerMultiProcessor: ::libc::c_int,
    #[doc = "< Compute mode."]
    pub computeMode: ::libc::c_int,
    #[doc = "< Frequency in khz of the timer used by the device-side \"clock*\"\n< instructions.  New for HIP."]
    pub clockInstructionRate: ::libc::c_int,
    #[doc = "< Architectural feature flags.  New for HIP."]
    pub arch: hipDeviceArch_t,
    #[doc = "< Device can possibly execute multiple kernels concurrently."]
    pub concurrentKernels: ::libc::c_int,
    #[doc = "< PCI Domain ID"]
    pub pciDomainID: ::libc::c_int,
    #[doc = "< PCI Bus ID."]
    pub pciBusID: ::libc::c_int,
    #[doc = "< PCI Device ID."]
    pub pciDeviceID: ::libc::c_int,
    #[doc = "< Maximum Shared Memory Per Multiprocessor."]
    pub maxSharedMemoryPerMultiProcessor: usize,
    #[doc = "< 1 if device is on a multi-GPU board, 0 if not."]
    pub isMultiGpuBoard: ::libc::c_int,
    #[doc = "< Check whether HIP can map host memory"]
    pub canMapHostMemory: ::libc::c_int,
    #[doc = "< DEPRECATED: use gcnArchName instead"]
    pub gcnArch: ::libc::c_int,
    #[doc = "< AMD GCN Arch Name."]
    pub gcnArchName: [::libc::c_char; 256usize],
    #[doc = "< APU vs dGPU"]
    pub integrated: ::libc::c_int,
    #[doc = "< HIP device supports cooperative launch"]
    pub cooperativeLaunch: ::libc::c_int,
    #[doc = "< HIP device supports cooperative launch on multiple devices"]
    pub cooperativeMultiDeviceLaunch: ::libc::c_int,
    #[doc = "< Maximum size for 1D textures bound to linear memory"]
    pub maxTexture1DLinear: ::libc::c_int,
    #[doc = "< Maximum number of elements in 1D images"]
    pub maxTexture1D: ::libc::c_int,
    #[doc = "< Maximum dimensions (width, height) of 2D images, in image elements"]
    pub maxTexture2D: [::libc::c_int; 2usize],
    #[doc = "< Maximum dimensions (width, height, depth) of 3D images, in image elements"]
    pub maxTexture3D: [::libc::c_int; 3usize],
    #[doc = "< Addres of HDP_MEM_COHERENCY_FLUSH_CNTL register"]
    pub hdpMemFlushCntl: *mut ::libc::c_uint,
    #[doc = "< Addres of HDP_REG_COHERENCY_FLUSH_CNTL register"]
    pub hdpRegFlushCntl: *mut ::libc::c_uint,
    #[doc = "<Maximum pitch in bytes allowed by memory copies"]
    pub memPitch: usize,
    #[doc = "<Alignment requirement for textures"]
    pub textureAlignment: usize,
    #[doc = "<Pitch alignment requirement for texture references bound to pitched memory"]
    pub texturePitchAlignment: usize,
    #[doc = "<Run time limit for kernels executed on the device"]
    pub kernelExecTimeoutEnabled: ::libc::c_int,
    #[doc = "<Device has ECC support enabled"]
    pub ECCEnabled: ::libc::c_int,
    #[doc = "< 1:If device is Tesla device using TCC driver, else 0"]
    pub tccDriver: ::libc::c_int,
    #[doc = "< HIP device supports cooperative launch on multiple"]
    pub cooperativeMultiDeviceUnmatchedFunc: ::libc::c_int,
    #[doc = "< HIP device supports cooperative launch on multiple"]
    pub cooperativeMultiDeviceUnmatchedGridDim: ::libc::c_int,
    #[doc = "< HIP device supports cooperative launch on multiple"]
    pub cooperativeMultiDeviceUnmatchedBlockDim: ::libc::c_int,
    #[doc = "< HIP device supports cooperative launch on multiple"]
    pub cooperativeMultiDeviceUnmatchedSharedMem: ::libc::c_int,
    #[doc = "< 1: if it is a large PCI bar device, else 0"]
    pub isLargeBar: ::libc::c_int,
    #[doc = "< Revision of the GPU in this device"]
    pub asicRevision: ::libc::c_int,
    #[doc = "< Device supports allocating managed memory on this system"]
    pub managedMemory: ::libc::c_int,
    #[doc = "< Host can directly access managed memory on the device without migration"]
    pub directManagedMemAccessFromHost: ::libc::c_int,
    #[doc = "< Device can coherently access managed memory concurrently with the CPU"]
    pub concurrentManagedAccess: ::libc::c_int,
    #[doc = "< Device supports coherently accessing pageable memory\n< without calling hipHostRegister on it"]
    pub pageableMemoryAccess: ::libc::c_int,
    #[doc = "< Device accesses pageable memory via the host's page tables"]
    pub pageableMemoryAccessUsesHostPageTables: ::libc::c_int,
}

impl Default for hipDeviceProp_t {
    fn default() -> Self {
        Self {
            name: [0; 256],
            totalGlobalMem: Default::default(),
            sharedMemPerBlock: Default::default(),
            regsPerBlock: Default::default(),
            warpSize: Default::default(),
            maxThreadsPerBlock: Default::default(),
            maxThreadsDim: Default::default(),
            maxGridSize: Default::default(),
            clockRate: Default::default(),
            memoryClockRate: Default::default(),
            memoryBusWidth: Default::default(),
            totalConstMem: Default::default(),
            major: Default::default(),
            minor: Default::default(),
            multiProcessorCount: Default::default(),
            l2CacheSize: Default::default(),
            maxThreadsPerMultiProcessor: Default::default(),
            computeMode: Default::default(),
            clockInstructionRate: Default::default(),
            arch: Default::default(),
            concurrentKernels: Default::default(),
            pciDomainID: Default::default(),
            pciBusID: Default::default(),
            pciDeviceID: Default::default(),
            maxSharedMemoryPerMultiProcessor: Default::default(),
            isMultiGpuBoard: Default::default(),
            canMapHostMemory: Default::default(),
            gcnArch: Default::default(),
            gcnArchName: [0; 256],
            integrated: Default::default(),
            cooperativeLaunch: Default::default(),
            cooperativeMultiDeviceLaunch: Default::default(),
            maxTexture1DLinear: Default::default(),
            maxTexture1D: Default::default(),
            maxTexture2D: Default::default(),
            maxTexture3D: Default::default(),
            hdpMemFlushCntl: std::ptr::null_mut(),
            hdpRegFlushCntl: std::ptr::null_mut(),
            memPitch: Default::default(),
            textureAlignment: Default::default(),
            texturePitchAlignment: Default::default(),
            kernelExecTimeoutEnabled: Default::default(),
            ECCEnabled: Default::default(),
            tccDriver: Default::default(),
            cooperativeMultiDeviceUnmatchedFunc: Default::default(),
            cooperativeMultiDeviceUnmatchedGridDim: Default::default(),
            cooperativeMultiDeviceUnmatchedBlockDim: Default::default(),
            cooperativeMultiDeviceUnmatchedSharedMem: Default::default(),
            isLargeBar: Default::default(),
            asicRevision: Default::default(),
            managedMemory: Default::default(),
            directManagedMemAccessFromHost: Default::default(),
            concurrentManagedAccess: Default::default(),
            pageableMemoryAccess: Default::default(),
            pageableMemoryAccessUsesHostPageTables: Default::default(),
        }
    }
}
