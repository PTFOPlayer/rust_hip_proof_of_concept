#[repr(C)]
pub enum hipError_t
{
  hipSuccess,
  hipErrorInvalidContext,
  hipErrorInvalidKernelFile,
  hipErrorMemoryAllocation,
  hipErrorInitializationError,
  hipErrorLaunchFailure,
  hipErrorLaunchOutOfResources,
  hipErrorInvalidDevice,
  hipErrorInvalidValue,
  hipErrorInvalidDevicePointer,
  hipErrorInvalidMemcpyDirection,
  hipErrorUnknown,
  hipErrorInvalidResourceHandle,
  hipErrorNotReady,
  hipErrorNoDevice,
  hipErrorPeerAccessAlreadyEnabled,
  hipErrorPeerAccessNotEnabled,
  hipErrorRuntimeMemory,
  hipErrorRuntimeOther,
  hipErrorHostMemoryAlreadyRegistered,
  hipErrorHostMemoryNotRegistered,
  hipErrorMapBufferObjectFailed,
  hipErrorTbd
}