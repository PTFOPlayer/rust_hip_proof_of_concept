#pragma once
#define LAUNCHER(TYPE) extern "C" void launcher(void (*func)(...), dim3 d1, dim3 d2, TYPE data) { func<<<d1, d2, 0, 0>>>(data); }