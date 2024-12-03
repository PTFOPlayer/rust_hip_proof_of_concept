mod wrapper;
use itertools::izip;
use wrapper::{Dim3, KernelSettings, Wrapper};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const NUM: usize = WIDTH * HEIGHT;
const THREADS_X: usize = 16;
const THREADS_Y: usize = 16;
const SIZE: usize = NUM * size_of::<f32>();

#[repr(C)]
struct Data {
    a: *mut f32,
    b: *mut f32,
    c: *mut f32,
    width: i32,
    height: i32,
}

fn main() {
    let mut wrapper = Wrapper::new().unwrap();

    let mut host_a = vec![0f32; SIZE];
    let mut host_b = vec![0f32; SIZE];
    let mut host_c = vec![0f32; SIZE];
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

    let data = Data {
        a: device_a.ptr,
        b: device_b.ptr,
        c: device_c.ptr,
        width: WIDTH as i32,
        height: HEIGHT as i32,
    };

    let kernel = wrapper.read_kernel::<Data>("./kernels/libfile.so", "vectoradd_float");
    let settings = KernelSettings { d1, d2 };

    wrapper.launch_kernel(kernel, settings, data);

    wrapper
        .copy_from_device(&device_a, &mut host_a, SIZE)
        .unwrap();

    let err = izip!(host_a, host_b, host_c).fold(0, |org, (a, b, c)| org + (a != b + c) as u32);
    println!("errors: {}", err);
}
