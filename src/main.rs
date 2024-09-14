use hip_macros::*;

use libloading;

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

__global__ void vectoradd_float(float *__restrict__ a, const float *__restrict__ b, const float *__restrict__ c, int width, int height)
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

    println!("{:?}", a);
    

    unsafe {
        a.unwrap().get::<unsafe extern "C" fn()>(b"hello_from_macro")
            .unwrap()()
    };
}