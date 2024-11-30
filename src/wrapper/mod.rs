use std::ptr::null_mut;

use error::HipErrorT;
use libc::size_t;
use libloading::{os::unix::Symbol, Library};
mod error;

#[repr(C)]
pub struct Dim3 (pub u32, pub u32, pub u32);

#[derive(Debug)]
pub struct Wrapper {
    _lib: Library,
    hip_malloc: Symbol<extern "C" fn(*mut *mut libc::c_void, usize) -> error::HipErrorT>,
    hip_memcpy: Symbol<
        extern "C" fn(*mut libc::c_void, *mut libc::c_void, usize, usize) -> error::HipErrorT,
    >,
    // pub launch_kernel: Symbol<extern "C" fn(*mut libc::c_void, Dim3, Dim3, size_t, size_t, ...) -> error::HipErrorT>,
}

impl Wrapper {
    pub fn new() -> Result<Self, libloading::Error> {
        unsafe {
            let lib = libloading::Library::new("libamdhip64.so")?;
            let hip_malloc = lib
                .get::<extern "C" fn(*mut *mut libc::c_void, usize) -> error::HipErrorT>(
                    b"hipMalloc",
                )?
                .into_raw();
            let hip_memcpy = lib
                .get::<extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    usize,
                    usize,
                ) -> error::HipErrorT>(b"hipMemcpy")?
                .into_raw();

                // let launch_kernel = lib
                // .get::<extern "C" fn(*mut libc::c_void, Dim3, Dim3, size_t, size_t, ...) -> error::HipErrorT>(b"hipLaunchKernelGGL")?
                // .into_raw();
            Ok(Self {
                _lib: lib,
                hip_malloc,
                hip_memcpy,
                // launch_kernel
            })
        }
    }

    pub fn create_device_memory<T>(&mut self, size: usize) -> Result<DeviceMemory<T>, HipErrorT> {
        let mut dev_mem = DeviceMemory::<T> { ptr: null_mut() };
        let ptr = &mut dev_mem.ptr as *mut *mut T;
        self.hip_malloc.clone()(ptr.cast(), size).guard()?;
        Ok(dev_mem)
    }

    pub fn copy_to_device<T>(
        &mut self,
        dev_mem: &DeviceMemory<T>,
        src: *mut T,
        size: usize,
    ) -> Result<(), HipErrorT> {
        self.hip_memcpy.clone()(dev_mem.ptr.cast(), src.cast(), size, 1).guard()
    }

    pub fn copy_from_device<T>(
        &mut self,
        dev_mem: &DeviceMemory<T>,
        dst: *mut T,
        size: usize,
    ) -> Result<(), HipErrorT> {
        self.hip_memcpy.clone()(dst.cast(), dev_mem.ptr.cast(), size, 2).guard()
    }

}

pub struct DeviceMemory<T> {
    pub ptr: *mut T,
}
