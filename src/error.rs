use thiserror::Error;

#[derive(Error, Debug)]
pub enum UDmxError {
    #[error("USB error: {0}")]
    UsbError(#[from] rusb::Error),
    #[error("Device not found")]
    DeviceNotFound,
    #[error("String descriptor error: {0}")]
    StringDescriptorError(rusb::Error),
    #[error("Invalid manufacturer")]
    InvalidManufacturer,
    #[error("Invalid product")]
    InvalidProduct,
    #[error("Control transfer error: {0}")]
    ControlTransferError(String),
}
