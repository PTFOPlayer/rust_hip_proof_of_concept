#![allow(non_camel_case_types)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum HipErrorT
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

impl HipErrorT {
  pub fn guard(self) -> Result<(), Self> {
    if self == Self::hipSuccess {
        return Ok(());
    }
    Err(self)
  }
}