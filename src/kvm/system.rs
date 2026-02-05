use std::ffi::{c_void, CString};
use std::io::Error;
use std::os::fd::RawFd;
use libc::{c_int, close, ioctl, open, read, O_RDWR};
use crate::errors::VshError;
pub struct Kvm {
    kvm_fd:RawFd
}

impl Drop for Kvm {
    fn drop(&mut self) {
        unsafe { close(self.kvm_fd)};
    }
}

const KVM_IO:u32 = 0xAE;


macro_rules! _io {
    ($type:expr, $n:expr) => {
        // In the Linux kernel, _IO is (type << 8) | nr
        ($type << 8 ) | $n
    };
}

/*
 KVM_GET_API_VERSION = (Type: 0xAE << 8) | (Number: 0x00);
 packs the KVM magic 'S' and command index into a 32-bit ioctl code.
 */
const KVM_GET_API_VERSION:u64 = _io!(KVM_IO,0x00) as u64;

impl Kvm {
    pub fn open() -> Result<Self,VshError> {
        let path = CString::new("/dev/kvm").map_err(|_| VshError::ConfigError{
            message:"Invalid character in KVM device path.".into()
        })?;
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

    pub fn read_all(&self) -> Result<Vec<u8>,VshError> {
        let fd = self.kvm_fd;

        if fd < 0 {
            return Err(VshError::OsError {
                syscall:"read",
                source:Error::from_raw_os_error(libc::EBADF)
                // EBADF -> Error:Bad File Descriptor
            })
        }

        let mut buf = [0u8;4096];
        let mut data:Vec<u8> = Vec::new();

        loop {
            let ret = unsafe {
                read(fd,buf.as_mut_ptr() as *mut c_void,buf.len())
            };

            if ret < 0 {
                return Err(VshError::OsError {
                    syscall:"read",
                    source:Error::last_os_error()
                })
            }

            if ret == 0 {
                break;
                //Read Over
            }

            let bytes_read = ret as usize;
            data.extend_from_slice(&buf[..bytes_read]);
        }
        
        Ok(data)
    }

    pub fn get_kvm_fd(&self) -> RawFd {
        self.kvm_fd
    }
}