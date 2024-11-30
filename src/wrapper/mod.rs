use std::{os::raw::c_void, ptr::null_mut};

use libloading::{Library, Symbol};
mod error;
pub struct Wrapper {
    lib: Library,
}

impl Wrapper {
    pub fn new() -> Result<Self, libloading::Error> {
        Ok(Self {
            lib: unsafe { libloading::Library::new("libamdhip64.so")? },
        })
    }

    pub fn create_device_memory<T>(
        &mut self,
        size: usize,
    ) -> Result<DeviceMemory<T>, libloading::Error> {
        unsafe {
            let dev_mem = DeviceMemory::<T> { ptr: null_mut() };
            let symbol = self.lib.get::<Symbol<
                extern "C" fn(*mut libc::c_void, usize) -> error::hipError_t,
            >>(b"hipMalloc")?;
            symbol(dev_mem.ptr.cast(), size);
            Ok(dev_mem)
        }
    }

    pub fn copy_to_device<T>(
        &mut self,
        dev_mem: &DeviceMemory<T>,
        src: *mut T,
        size: usize,
    ) -> Result<(), libloading::Error> {
        unsafe {
            let symbol = self.lib.get::<Symbol<
                extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    usize,
                    usize,
                ) -> error::hipError_t,
            >>(b"hipMemcpy")?;
            symbol(dev_mem.ptr.cast(), src.cast(), size, 1);

            Ok(())
        }
    }

    pub fn copy_from_device<T>(
        &mut self,
        dev_mem: &DeviceMemory<T>,
        dst: *mut T,
        size: usize,
    ) -> Result<(), libloading::Error> {
        unsafe {
            let symbol = self.lib.get::<Symbol<
                extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    usize,
                    usize,
                ) -> error::hipError_t,
            >>(b"hipMemcpy")?;
            symbol(dev_mem.ptr.cast(), dst.cast(), size, 2);

            Ok(())
        }
    }
}

pub struct DeviceMemory<T> {
    ptr: *mut T,
}
