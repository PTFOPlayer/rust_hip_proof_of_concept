#include <stdint.h>
#include <hip/hip_runtime.h>
#include <hip/amd_detail/amd_hip_runtime.h>

extern "C" hipDeviceProp_t wrapped_hipGetDeviceProperties(int32_t dev_id) {
    hipDeviceProp_t data;
    hipGetDeviceProperties(&data, dev_id);

    return data;
}