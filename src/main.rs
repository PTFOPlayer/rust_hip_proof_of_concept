use std::ffi::CString;

use hip_runtime_sys::*;
const LEN: usize = 1024;
fn main() {
    unsafe {
        let result = hipSetDevice(0);
        assert_eq!(result, hipError_t::hipSuccess);

        let mut input: hipDeviceptr_t = std::ptr::null_mut();
        let mut output: hipDeviceptr_t = std::ptr::null_mut();
        let result = hipMalloc(&mut input, LEN);
        assert_eq!(result, hipError_t::hipSuccess);
        let result = hipMalloc(&mut output, LEN);
        assert_eq!(result, hipError_t::hipSuccess);

        const LEN: usize = 1024;
        let mut in_host: Vec<u8> = vec![0; LEN];
        let mut out_host: Vec<u8> = vec![0; LEN];

        for i in 0..LEN {
            in_host[i] = (i % 10) as u8;
        }

        let result = hipMemcpyHtoD(input, in_host.as_mut_ptr() as *mut std::ffi::c_void, LEN);
        assert_eq!(result, hipError_t::hipSuccess);
        let module_data =
            std::fs::read("kernels/target/amdgcn-amd-amdhsa/release/kernels.elf").unwrap();
        let mut module: hipModule_t = std::ptr::null_mut();
        let result =
            hipModuleLoadData(&mut module, module_data.as_ptr() as *const std::ffi::c_void);
        assert_eq!(result, hipError_t::hipSuccess);

        let mut function: hipFunction_t = std::ptr::null_mut();
        let kernel_name = CString::new("kernel").expect("Invalid kernel name");
        let result = hipModuleGetFunction(&mut function, module, kernel_name.as_ptr());
        assert_eq!(result, hipError_t::hipSuccess);

        // Assemble arguments for the kernel.
        // Pass two pointers,
        let kernel_args: &mut [*mut std::ffi::c_void] = &mut [
            input as *mut std::ffi::c_void,
            output as *mut std::ffi::c_void,
        ];
        let mut size = std::mem::size_of_val(kernel_args);

        let mut config = [
            0x1 as *mut std::ffi::c_void,                   // Next come arguments
            kernel_args as *mut _ as *mut std::ffi::c_void, // Pointer to arguments
            0x2 as *mut std::ffi::c_void,                   // Next comes size
            std::ptr::addr_of_mut!(size) as *mut std::ffi::c_void, // Pointer to size of arguments
            0x3 as *mut std::ffi::c_void,                   // End
        ];
        let result = hipModuleLaunchKernel(
            function,
            2,                    // Workgroup count x
            1,                    // Workgroup count y
            1,                    // Workgroup count z
            LEN as u32 / 2,       // Workgroup dim x
            1,                    // Workgroup dim y
            1,                    // Workgroup dim z
            LEN as u32 / 2,       // sharedMemBytes for extern shared variables
            std::ptr::null_mut(), // stream
            std::ptr::null_mut(), // params (unimplemented in hip)
            config.as_mut_ptr(),  // arguments
        );
        assert_eq!(result, hipError_t::hipSuccess);

        let result = hipDeviceSynchronize();
        assert_eq!(result, hipError_t::hipSuccess);

        let result = hipMemcpyDtoH(out_host.as_mut_ptr() as *mut std::ffi::c_void, output, LEN);
        assert_eq!(result, hipError_t::hipSuccess);

        println!("Output: {:?}", &out_host[..32]);
    }
}
