use std::path::PathBuf;


use hip_poc_macros::amdgpu_kernel;
use rocm_rs::hip::*;

const LEN: usize = 1024;

amdgpu_kernel! {
    #[unsafe(no_mangle)]
    pub extern "gpu-kernel" fn kernel(input: *mut u8, output: *mut u8) {
        let id = unsafe { workitem_id_x() as usize };
        unsafe {
            *output.add(id) = *input.add(id) * 2;
        }
    }
}

fn main() -> Result<()> {
    let device = Device::new(0)?;
    device.set_current()?;

    let kernel_path = PathBuf::from("./kernel_sources/kernel/target/amdgcn-amd-amdhsa/release/kernels.elf");
    assert!(kernel_path.exists());

    let module = Module::load(kernel_path)?;

    let function = unsafe { module.get_function("kernel")? };

    let mut in_host: Vec<u8> = vec![0; LEN];
    let mut out_host: Vec<u8> = vec![0; LEN];

    for i in 0..LEN {
        in_host[i] = (i % 10) as u8;
    }


    let mut input = DeviceMemory::<u8>::new(LEN)?;
    let output = DeviceMemory::<u8>::new(LEN)?;

    input.copy_from_host(&in_host)?;

    let kernel_args = [
        input.as_kernel_arg(),
        output.as_kernel_arg(),
    ];

    let grid_dim = Dim3 { x: 2, y: 1, z: 1 };
    let block_dim = Dim3 { x: (LEN/ 2) as u32, y: 1, z: 1 };

    function.launch(
        grid_dim,
        block_dim,
        0, 
        None,
        &mut kernel_args.clone(),
    )?;

    output.copy_to_host(&mut out_host)?;

    println!("Output: {:?}", &out_host[..32]);

    Ok(())
}
