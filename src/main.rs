use hip_macros::*;
mod wrapper;
use itertools::izip;
use libloading::{self, Symbol};
use wrapper::{Dim3, Wrapper};

fn main() {
    let a = my_macro!(
        r#"
#define __HIP_PLATFORM_AMD__

#include <algorithm>
#include <stdlib.h>
#include <iostream>
#include <assert.h>
#include <hip/hip_runtime.h>
#include <hip/amd_detail/amd_hip_runtime.h>

#ifdef NDEBUG
#define HIP_ASSERT(x) x
#else
#define HIP_ASSERT(x) (assert((x) == hipSuccess))
#endif

#define WIDTH 1024
#define HEIGHT 1024

#define NUM (WIDTH * HEIGHT)

#define THREADS_PER_BLOCK_X 16
#define THREADS_PER_BLOCK_Y 16
#define THREADS_PER_BLOCK_Z 1

extern "C" __global__ void vectoradd_float(float *__restrict__ a, const float *__restrict__ b, const float *__restrict__ c, int width, int height)
{

  int x = hipBlockDim_x * hipBlockIdx_x + hipThreadIdx_x;
  int y = hipBlockDim_y * hipBlockIdx_y + hipThreadIdx_y;

  int i = y * width + x;
  if (i < (width * height))
  {
    a[i] = b[i] + c[i];
  }
}

using namespace std;

extern "C" void launcher(float *__restrict__ a, const float *__restrict__ b, const float *__restrict__ c, int width, int height) {
  vectoradd_float<<<dim3(WIDTH / THREADS_PER_BLOCK_X, HEIGHT / THREADS_PER_BLOCK_Y),
                    dim3(THREADS_PER_BLOCK_X, THREADS_PER_BLOCK_Y), 0, 0>>>(a, b, c, WIDTH, HEIGHT);

}

"#
    );
    let a = a.unwrap();

    unsafe {
        let func: Result<Symbol<unsafe extern "C" fn()>, libloading::Error> =
            a.get("hello_from_macro".as_bytes());
        println!("{:?}", func);
        if let Ok(f) = func {
            f();
        }
    }

    const WIDTH: usize = 1024;
    const HEIGHT: usize = 1024;
    const NUM: usize = (WIDTH * HEIGHT);
    const THREADS_PER_BLOCK_X: usize = 16;
    const THREADS_PER_BLOCK_Y: usize = 16;
    const THREADS_PER_BLOCK_Z: usize = 1;

    const DATA_SIZE: usize = NUM * size_of::<f32>();

    let mut wrapper = Wrapper::new().unwrap();

    let mut host_a = vec![0f32; DATA_SIZE];
    let mut host_b = vec![0f32; DATA_SIZE];
    let mut host_c = vec![0f32; DATA_SIZE];
    for i in 0..NUM {
        host_b[i] = i as f32;
        host_c[i] = i as f32 * 100.0f32;
    }

    let device_a = wrapper.create_device_memory::<f32>(DATA_SIZE).unwrap();
    let device_b = wrapper.create_device_memory::<f32>(DATA_SIZE).unwrap();
    let device_c = wrapper.create_device_memory::<f32>(DATA_SIZE).unwrap();

    wrapper
        .copy_to_device(&device_b, host_b.as_mut_ptr(), DATA_SIZE)
        .unwrap();
    wrapper
        .copy_to_device(&device_c, host_c.as_mut_ptr(), DATA_SIZE)
        .unwrap();

    unsafe {
        let gpu_kernel: Result<
            Symbol<unsafe extern "C" fn(*mut f32, *mut f32, *mut f32, i32, i32)>,
            libloading::Error,
        > = a.get(b"vectoradd_float");
        println!("{:?}", gpu_kernel);

        let launcher: Result<
            Symbol<unsafe extern "C" fn(*mut f32, *mut f32, *mut f32, i32, i32)>,
            libloading::Error,
        > = a.get(b"launcher");
        println!("{:?}", launcher);

        launcher.unwrap()(
            device_a.ptr,
            device_b.ptr,
            device_c.ptr,
            WIDTH as i32,
            HEIGHT as i32,
        );
    }

    wrapper
        .copy_from_device(&device_a, host_a.as_mut_ptr(), DATA_SIZE)
        .unwrap();

    let err = izip!(host_a, host_b, host_c).fold(
        0,
        |org, (a, b, c)| {
            if a != b + c {
                org + 1
            } else {
                org
            }
        },
    );
    println!("errors: {}", err);
}
