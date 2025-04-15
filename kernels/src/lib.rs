#![feature(abi_gpu_kernel)]
#![feature(core_intrinsics, link_llvm_intrinsics)]
#![no_std]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

unsafe extern "C" {
    #[link_name = "llvm.amdgcn.workitem.id.x"]
    pub safe fn workitem_id_x() -> u32;
    #[link_name = "llvm.amdgcn.workitem.id.y"]
    pub safe fn workitem_id_y() -> u32;
    #[link_name = "llvm.amdgcn.workitem.id.z"]
    pub safe fn workitem_id_z() -> u32;

    #[link_name = "llvm.amdgcn.workgroup.id.x"]
    pub safe fn workgroup_id_x() -> u32;
    #[link_name = "llvm.amdgcn.workgroup.id.y"]
    pub safe fn workgroup_id_y() -> u32;
    #[link_name = "llvm.amdgcn.workgroup.id.z"]
    pub safe fn workgroup_id_z() -> u32;
}

#[unsafe(no_mangle)]
pub extern "gpu-kernel" fn kernel(input: *mut u8, output: *mut u8) {
    let id = workitem_id_x() as usize;

    unsafe {
        *output.add(id) = *input.add(id)* 2;
    }
}