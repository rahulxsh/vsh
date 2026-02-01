use std::ffi::{CString};
use std::io::Error;
use std::os::fd::RawFd;
use libc::{open,O_RDWR};
use crate::errors::VshError;
pub struct Kvm {
    fd:RawFd
}

impl Kvm {
    pub fn open() -> Result<Self,VshError> {
        let path = CString::new("/dev/kvm").expect("Operation: Creating C String Failed");
        let fd = unsafe { open(path.as_ptr(),O_RDWR) };

        if fd < 0 {
            return Err(VshError::OsError {
                syscall:"Kvm file open error",
                source:Error::last_os_error()
            })
        };

        Ok(Kvm{
            fd
        })
    }

    pub fn fd(&self) -> RawFd {
        self.fd
    }
}