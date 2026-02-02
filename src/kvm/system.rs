use std::ffi::{CString};
use std::io::Error;
use std::os::fd::RawFd;
use libc::{c_int, close, open, O_RDWR};
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

    pub fn close(&mut self) -> Result<(),VshError> {
        let fd:RawFd = self.fd();

        if fd < 0 {
            // Already Closed
            return Ok(());
        }
        let close_status_number:c_int = unsafe { close(fd)};

        if close_status_number < 0 {
            return Err(VshError::OsError {
                syscall: "close",
                source:Error::last_os_error()
            });
        };

        self.fd = -1;
        return Ok(());
    }

    pub fn fd(&self) -> RawFd {
        self.fd
    }
}