use error::UDmxError;
use log::{Level, debug, info, log_enabled, warn};
use rusb::{Context, DeviceHandle, UsbContext};

pub mod error;
pub mod fixture;

const USBDEV_SHARED_VENDOR: u16 = 0x16C0; // VOTI
const USBDEV_SHARED_PRODUCT: u16 = 0x05DC; // uDMX product ID

// Documented in https://github.com/mirdej/udmx/blob/master/common/uDMX_cmds.h
enum Command {
    SetSingleChannel,
    SetChannelRange,
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::SetSingleChannel => 1,
            Command::SetChannelRange => 2,
        }
    }
}

pub struct UDmx {
    handle: DeviceHandle<Context>,
}

impl UDmx {
    pub fn new() -> Result<Self, UDmxError> {
        let context = Context::new().map_err(UDmxError::UsbError)?;

        Ok(UDmx {
            handle: Self::find_device(&context)?,
        })
    }

    fn find_device(context: &Context) -> Result<DeviceHandle<Context>, UDmxError> {
        debug!(
            "Searching for uDMX device (VID: 0x{:04x}, PID: 0x{:04x})...",
            USBDEV_SHARED_VENDOR, USBDEV_SHARED_PRODUCT
        );

        for device in context.devices().map_err(UDmxError::UsbError)?.iter() {
            let descriptor = device.device_descriptor().map_err(UDmxError::UsbError)?;

            if descriptor.vendor_id() == USBDEV_SHARED_VENDOR
                && descriptor.product_id() == USBDEV_SHARED_PRODUCT
            {
                let handle = match device.open() {
                    Ok(handle) => handle,
                    Err(err) => {
                        debug!("Failed to open device: {:?}", err);
                        continue;
                    }
                };

                if log_enabled!(Level::Warn) {
                    if descriptor
                        .product_string_index()
                        .map(|i| handle.read_string_descriptor_ascii(i))
                        == Some(Ok("uDMX".to_string()))
                    {
                        info!("uDMX device found!");
                    } else {
                        warn!("Device does not match expected product string");
                    }
                }

                return Ok(handle);
            }
        }

        Err(UDmxError::DeviceNotFound)
    }
}
