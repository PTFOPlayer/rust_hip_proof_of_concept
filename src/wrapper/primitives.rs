use libc::c_void;

pub type VoidPtr = *mut c_void;

pub type GPUMemPtr<T> = *mut T;

#[derive(Clone, Copy)]
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