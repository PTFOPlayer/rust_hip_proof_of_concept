mod wrapper;
use itertools::izip;
use wrapper::{GPUMemPtr, Dim3, KernelSettings, Wrapper};

const WIDTH: usize = 10240;
const HEIGHT: usize = 10240;
const NUM: usize = WIDTH * HEIGHT;
const THREADS_X: usize = 16;
const THREADS_Y: usize = 16;
const SIZE: usize = NUM * size_of::<f32>();

#[repr(C)]
struct SaxpyData {
    a: GPUMemPtr<f32>,
    b: GPUMemPtr<f32>,
    c: GPUMemPtr<f32>,
    width: i32,
    height: i32,
}

#[repr(C)]
struct X2Data {
    data: GPUMemPtr<f32>,
    width: i32,
    height: i32,
}

fn main() {
    let mut wrapper = Wrapper::new().unwrap();

    let mut host_a = vec![0f32; NUM];
    let mut host_b = vec![0f32; NUM];
    let mut host_c = vec![0f32; NUM];
    for i in 0..NUM {
        host_b[i] = i as f32;
        host_c[i] = i as f32 * 100.0f32;
    }

    let device_a = wrapper.create_device_memory::<f32>(SIZE).unwrap();
    let device_b = wrapper
        .create_device_memory_from_host::<f32>(&mut host_b, SIZE)
        .unwrap();
    let device_c = wrapper
        .create_device_memory_from_host::<f32>(&mut host_c, SIZE)
        .unwrap();

    let d1 = Dim3::from_xy((WIDTH / THREADS_X) as u32, (HEIGHT / THREADS_Y) as u32);
    let d2 = Dim3::from_xy(THREADS_X as u32, THREADS_Y as u32);

    let settings = KernelSettings { d1, d2 };
    let saxpy = wrapper.read_kernel::<SaxpyData>("./kernels/libfile.so", "saxpy");
    let x2 = wrapper.read_kernel::<X2Data>("./kernels/libfile.so", "x2");

    let saxpy_data = SaxpyData {
        a: device_a.ptr,
        b: device_b.ptr,
        c: device_c.ptr,
        width: WIDTH as i32,
        height: HEIGHT as i32,
    };

    wrapper.launch_kernel(saxpy, settings, saxpy_data);

    let x2_data = X2Data {
        data: device_a.ptr,
        width: WIDTH as i32,
        height: HEIGHT as i32,
    };

    wrapper.launch_kernel(x2, settings, x2_data);

    wrapper
        .copy_from_device(&device_a, &mut host_a, SIZE)
        .unwrap();

    let err =
        izip!(host_a, host_b, host_c).fold(0, |org, (a, b, c)| org + (a != (b + c) * 2.0) as u32);
    println!("errors: {}", err);
}
