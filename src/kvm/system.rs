use std::ffi::{CString};
use std::io::Error;
use std::os::fd::RawFd;
use libc::{c_int, close, ioctl, open, O_RDWR};
use crate::errors::VshError;
pub struct Kvm {
    kvm_fd:RawFd
}

const KVMIO:u32 = 0xAE;


macro_rules! _IO {
    ($type:expr, $n:expr) => {
        // In the Linux kernel, _IO is (type << 8) | nr
        ($type << 8 ) | $n
    };
}

/*
 KVM_GET_API_VERSION = (Type: 0xAE << 8) | (Number: 0x00);
 packs the KVM magic 'S' and command index into a 32-bit ioctl code.
 */
const KVM_GET_API_VERSION:u64 = _IO!(KVMIO,0x00) as u64;

impl Kvm {
    pub fn open() -> Result<Self,VshError> {
        let path = CString::new("/dev/kvm").expect("Operation: Creating C String Failed");
        let kvm_fd = unsafe { open(path.as_ptr(),O_RDWR) };

        if kvm_fd < 0 {
            return Err(VshError::OsError {
                syscall:"Kvm file open error",
                source:Error::last_os_error()
            })
        };

        let kvm_version:c_int = unsafe { ioctl(kvm_fd,KVM_GET_API_VERSION)};

        if kvm_version < 0 {
            return Err(VshError::KvmError {
                ioctl:"KVM_GET_API_VERSION",
                errno:Error::last_os_error().raw_os_error().unwrap_or(0)
            })
        } else if kvm_version != 12 {
            return Err(VshError::KvmVersionMismatch(kvm_version))
        } else{
            println!("KVM API version {} is ready.",kvm_version);
        }

        Ok(Kvm{
            kvm_fd
        })
    }

    pub fn close(&mut self) -> Result<(),VshError> {
        let fd:RawFd = self.get_kvm_fd();

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

        self.kvm_fd = -1;
        return Ok(());
    }

    pub fn get_kvm_fd(&self) -> RawFd {
        self.kvm_fd
    }
}