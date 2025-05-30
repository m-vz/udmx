use thiserror::Error;

#[derive(Error, Debug)]
pub enum UDmxError {
    #[error("USB error: {0}")]
    UsbError(#[from] rusb::Error),
    #[error("Device not found")]
    DeviceNotFound,
}
