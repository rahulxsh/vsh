use thiserror::Error;

#[derive(Debug,Error)]
pub enum VshError{
    #[error("OS error during {syscall}: {source}")]
    OsError {
        syscall: &'static str,
        source: std::io::Error
    },
    #[error("KVM ioctl {ioctl} failed (errno={errno})")]
    KvmError {
        ioctl: &'static str,
        errno: i32
    },
    #[error("Invalid Configuration: {message}")]
    ConfigError {
        message: String,
    },
    #[error("Invalid VM State:{current:?} attempted: {attempted}")]
    InvalidVmState {
        current: VmState,
        attempted: &'static str
    },
    #[error("Vcpu error: {message} vcpu id: {vcpu_id}")]
    VcpuError {
        vcpu_id: usize,
        message: String
    },
    #[error("VM exit error: {reason}, Description: {description}")]
    VmExitError {
        reason: u32,
        description: String
    },
    #[error("Device Error: {message}, Device: {device}")]
    DeviceError {
        device: &'static str,
        message: String
    },
    #[error("Resource error occurred:{message}, Resource:{resource}")]
    ResourceError { resource: &'static str, message: String },
    // Used For Monitoring
    // MetricsError { message: String },
    #[error("KVM API version mismatch: expected 12, got {0}")]
    KvmVersionMismatch(i32),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}


#[derive(Debug)]
pub enum VmState {
    Created,
    Running,
    Stopped,
    Creating,
    Stopping,
    Started,
    Destroyed
}