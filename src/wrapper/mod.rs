use std::{marker::PhantomData, ptr::null_mut};

use error::HipErrorT;
use libloading::{os::unix::Symbol, Library};
mod error;

#[repr(C)]
pub struct Dim3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Dim3 {
    pub fn from_x(x: u32) -> Self {
        Dim3 { x, y: 1, z: 1 }
    }

    pub fn from_y(y: u32) -> Self {
        Dim3 { x: 1, y, z: 1 }
    }

    pub fn from_z(z: u32) -> Self {
        Dim3 { x: 1, y: 1, z }
    }

    pub fn from_xy(x: u32, y: u32) -> Self {
        Dim3 { x, y, z: 1 }
    }

    pub fn from_xz(x: u32, z: u32) -> Self {
        Dim3 { x, y: 1, z }
    }

    pub fn from_yz(y: u32, z: u32) -> Self {
        Dim3 { x: 1, y, z }
    }
}

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

    pub fn create_device_memory_from_host<T>(&mut self, src: &mut [T], size: usize) -> Result<DeviceMemory<T>, HipErrorT> {
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
        self.hip_memcpy.clone()(dev_mem.ptr.cast(), src.as_mut_ptr().cast(), size, 1).guard()
    }

    pub fn copy_from_device<T>(
        &mut self,
        dev_mem: &DeviceMemory<T>,
        dst: &mut [T],
        size: usize,
    ) -> Result<(), HipErrorT> {
        self.hip_memcpy.clone()(dst.as_mut_ptr().cast(), dev_mem.ptr.cast(), size, 2).guard()
    }

    pub fn read_kernel<T>(&self, path: &str, entry: &'static str) -> Kernel<T> {
        Kernel {
            lib: unsafe { libloading::Library::new(&path).unwrap() },
            entry: &entry,
            _phantom: PhantomData,
        }
    }

    pub fn launch_kernel<T>(&self, kernel: Kernel<T>, settings: KernelSettings, data: T) {
        unsafe {
            let gpu_kernel: libloading::Symbol<unsafe extern "C" fn(T)> =
                kernel.lib.get(kernel.entry.as_bytes()).unwrap();

            let launcher: libloading::Symbol<
                unsafe extern "C" fn(*mut libc::c_void, Dim3, Dim3, ...),
            > = kernel.lib.get(b"launcher").unwrap();

            launcher(
                gpu_kernel.try_as_raw_ptr().unwrap(),
                settings.d1,
                settings.d2,
                data,
            );
        }
    }
}

pub struct DeviceMemory<T> {
    pub ptr: *mut T,
}

pub struct Kernel<T> {
    lib: Library,
    entry: &'static str,
    _phantom: PhantomData<T>,
}

pub struct KernelSettings {
    pub d1: Dim3,
    pub d2: Dim3,
}
