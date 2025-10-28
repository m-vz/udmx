use thiserror::Error;

/// Errors that can occur when interacting with a uDMX device.
#[derive(Error, Debug)]
pub enum UDmxError {
    /// USB communication error.
    ///
    /// Wraps errors from the underlying `rusb` library.
    #[error("USB error: {0}")]
    UsbError(#[from] rusb::Error),

    /// No uDMX device was found on the system.
    ///
    /// Ensure the device is connected and udev rules are set up correctly if you're on Linux.
    #[error("Device not found")]
    DeviceNotFound,

    /// DMX channel index exceeds valid range (< 512).
    #[error("Channel out of range: {0}")]
    ChannelOutOfRange(u64),
}
