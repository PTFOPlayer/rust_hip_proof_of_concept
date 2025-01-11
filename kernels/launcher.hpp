#pragma once

#define PROTO(IDENTIFIER, arg) \
    extern "C" __global__ void IDENTIFIER(arg)

#define LAUNCHER(IDENTIFIER, type, name)                                  \
    extern "C" void IDENTIFIER##_launcher(dim3 d1, dim3 d2, type name) { \
        IDENTIFIER<<<d1, d2, 0, 0>>>(name);                         \
    }

#define DEFINE_KERNEL(IDENTIFIER, type, name) \
    PROTO(IDENTIFIER, type name);                   \
    LAUNCHER(IDENTIFIER, type, name)          \
    PROTO(IDENTIFIER, type name)\
