use hip_macros::*;
mod wrapper;
use libloading::{self, Symbol};
use wrapper::Wrapper;

fn main() {

  let mut wrapper = Wrapper::new().unwrap();
  let mut host_data = [1f32, 2f32, 3f32, 4f32, 5f32, 6f32, 7f32, 8f32, 9f32, 10f32];
  let size = 10 * size_of::<f32>();
  let device_data = wrapper.create_device_memory(size).unwrap();
  wrapper
      .copy_to_device(&device_data, host_data.as_mut_ptr(), size)
      .unwrap();

  let mut new_data = [0f32; 10];
  wrapper
      .copy_from_device(&device_data, new_data.as_mut_ptr(), size)
      .unwrap();
  println!("{:?}", new_data);


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

extern "C" void hello_from_macro()
{

  float *hostA;
  float *hostB;
  float *hostC;

  float *deviceA;
  float *deviceB;
  float *deviceC;

  hipDeviceProp_t devProp;
  hipGetDeviceProperties(&devProp, 0);
  cout << " System minor " << devProp.minor << endl;
  cout << " System major " << devProp.major << endl;
  cout << " agent prop name " << devProp.name << endl;

  cout << "hip Device prop succeeded " << endl;

  int i;
  int errors;

  hostA = (float *)malloc(NUM * sizeof(float));
  hostB = (float *)malloc(NUM * sizeof(float));
  hostC = (float *)malloc(NUM * sizeof(float));

  // initialize the input data
  for (i = 0; i < NUM; i++)
  {
    hostB[i] = (float)i;
    hostC[i] = (float)i * 100.0f;
  }

  HIP_ASSERT(hipMalloc((void **)&deviceA, NUM * sizeof(float)));
  HIP_ASSERT(hipMalloc((void **)&deviceB, NUM * sizeof(float)));
  HIP_ASSERT(hipMalloc((void **)&deviceC, NUM * sizeof(float)));

  HIP_ASSERT(hipMemcpy(deviceB, hostB, NUM * sizeof(float), hipMemcpyHostToDevice));
  HIP_ASSERT(hipMemcpy(deviceC, hostC, NUM * sizeof(float), hipMemcpyHostToDevice));

  vectoradd_float<<<dim3(WIDTH / THREADS_PER_BLOCK_X, HEIGHT / THREADS_PER_BLOCK_Y),
                    dim3(THREADS_PER_BLOCK_X, THREADS_PER_BLOCK_Y), 0, 0>>>(deviceA, deviceB, deviceC, WIDTH, HEIGHT);

  HIP_ASSERT(hipMemcpy(hostA, deviceA, NUM * sizeof(float), hipMemcpyDeviceToHost));

  // verify the results
  errors = 0;
  for (i = 0; i < NUM; i++)
  {
    if (hostA[i] != (hostB[i] + hostC[i]))
    {
      errors++;
    }
  }
  if (errors != 0)
  {
    printf("FAILED: %d errors\n", errors);
  }
  else
  {
    printf("PASSED!\n");
  }

  HIP_ASSERT(hipFree(deviceA));
  HIP_ASSERT(hipFree(deviceB));
  HIP_ASSERT(hipFree(deviceC));

  free(hostA);
  free(hostB);
  free(hostC);

  // hipResetDefaultAccelerator();

  //return errors;
}
"#
    );

    unsafe {
        let a = a.unwrap();
        let func: Result<Symbol<unsafe extern "C" fn()>, libloading::Error> =
            a.get("hello_from_macro".as_bytes());
        println!("{:?}", func);
        if let Ok(f) = func {
            f();
        }

        let gpu_kernel: Result<
            Symbol<unsafe extern "C" fn(*mut f32, *mut f32, *mut f32, i32, i32)>,
            libloading::Error,
        > = a.get("vectoradd_float".as_bytes());
        println!("{:?}", gpu_kernel);
    }
}
