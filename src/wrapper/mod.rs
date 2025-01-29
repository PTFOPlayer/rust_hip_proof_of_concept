use std::{marker::PhantomData, ptr::null_mut};

use error::HipErrorT;
use libloading::{os::unix::Symbol, Library};
mod error;
pub mod primitives;
pub use primitives::*;

type HipMallocFn = unsafe extern "C" fn(*mut VoidPtr, usize) -> HipErrorT;
type HipMemcpyFn = unsafe extern "C" fn(VoidPtr, VoidPtr, usize, usize) -> HipErrorT;
#[derive(Debug)]
pub struct Wrapper {
    _lib: Library,
    hip_malloc: Symbol<HipMallocFn>,
    hip_memcpy: Symbol<HipMemcpyFn>,
}

impl Wrapper {
    pub fn new() -> Result<Self, libloading::Error> {
        unsafe {
            let lib = Library::new("libamdhip64.so")?;
            let hip_malloc = lib.get::<HipMallocFn>(b"hipMalloc")?.into_raw();
            let hip_memcpy = lib.get::<HipMemcpyFn>(b"hipMemcpy")?.into_raw();

            Ok(Self {
                _lib: lib,
                hip_malloc,
                hip_memcpy,
            })
        }
    }

    pub fn create_device_memory<T>(&mut self, size: usize) -> Result<DeviceMemory<T>, HipErrorT> {
        let mut dev_mem = DeviceMemory::<T> { ptr: null_mut() };
        let ptr = &mut dev_mem.ptr as *mut *mut T;
        unsafe { (self.hip_malloc)(ptr.cast(), size).guard()? }
        Ok(dev_mem)
    }

    pub fn create_device_memory_from_host<T>(
        &mut self,
        src: &mut [T],
        size: usize,
    ) -> Result<DeviceMemory<T>, HipErrorT> {
        let dev_mem = self.create_device_memory::<T>(size)?;
        self.copy_to_device(&dev_mem, src, size)?;
        Ok(dev_mem)
    }

    pub fn copy_to_device<T>(
        &mut self,
        dev_mem: &DeviceMemory<T>,
        src: &mut [T],
        size: usize,
    ) -> Result<(), HipErrorT> {
        unsafe { (self.hip_memcpy)(dev_mem.ptr.cast(), src.as_mut_ptr().cast(), size, 1).guard() }
    }

    pub fn copy_from_device<T>(
        &mut self,
        dev_mem: &DeviceMemory<T>,
        dst: &mut [T],
        size: usize,
    ) -> Result<(), HipErrorT> {
        unsafe { (self.hip_memcpy)(dst.as_mut_ptr().cast(), dev_mem.ptr.cast(), size, 2).guard() }
    }

    pub fn read_kernel<T>(&self, path: &str, entry: &str) -> Kernel<T> {
        Kernel {
            lib: unsafe { Library::new(&path).unwrap() },
            entry: entry.to_owned() + "_launcher",
            _phantom: PhantomData,
        }
    }

    pub fn launch_kernel<T>(&self, kernel: Kernel<T>, settings: KernelSettings, data: T) {
        unsafe {
            kernel
                .lib
                .get::<unsafe extern "C" fn(Dim3, Dim3, T)>(kernel.entry.as_bytes())
                .unwrap()(settings.d1, settings.d2, data);
        }
    }
}

pub struct DeviceMemory<T> {
    pub ptr: GPUMemPtr<T>,
}

pub struct Kernel<T> {
    lib: Library,
    entry: String,
    _phantom: PhantomData<T>,
}
#[derive(Clone, Copy)]
pub struct KernelSettings {
    pub d1: Dim3,
    pub d2: Dim3,
}
