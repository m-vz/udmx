use std::time::Duration;

use command::Command;
use error::UDmxError;
use log::{Level, debug, info, log_enabled, trace, warn};
use rusb::{Context, DeviceHandle, UsbContext};

mod command;
pub mod error;

const USBDEV_SHARED_VENDOR: u16 = 0x16C0; // VOTI
const USBDEV_SHARED_PRODUCT: u16 = 0x05DC; // uDMX product ID
const USB_TIMEOUT: Duration = Duration::from_millis(5000);

pub struct UDmx {
    handle: DeviceHandle<Context>,
}

impl UDmx {
    /// Creates a new uDMX controller interface.
    ///
    /// This function finds and opens a connection to a uDMX device using the default USB context.
    ///
    /// # Returns
    ///
    /// A new [`UDmx`] instance with an open connection to the device.
    ///
    /// # Errors
    ///
    /// Returns an error if it failed to create a USB context or if no uDMX device could be found or
    /// communication with the device failed.
    pub fn new() -> Result<Self, UDmxError> {
        let context = Context::new().map_err(UDmxError::UsbError)?;

        Ok(Self {
            handle: Self::find_device(&context)?,
        })
    }

    fn find_device(context: &Context) -> Result<DeviceHandle<Context>, UDmxError> {
        debug!(
            "Searching for uDMX device (VID: 0x{USBDEV_SHARED_VENDOR:04x}, PID: 0x{USBDEV_SHARED_PRODUCT:04x})...",
        );

        for device in context.devices().map_err(UDmxError::UsbError)?.iter() {
            let descriptor = device.device_descriptor().map_err(UDmxError::UsbError)?;

            if descriptor.vendor_id() == USBDEV_SHARED_VENDOR
                && descriptor.product_id() == USBDEV_SHARED_PRODUCT
            {
                let handle = match device.open() {
                    Ok(handle) => handle,
                    Err(err) => {
                        debug!("Failed to open device: {err:?}");
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

    /// Set a single channel to a value.
    ///
    /// # Arguments
    ///
    /// * `channel` - The channel number to set.
    /// * `value` - The value to set the channel to.
    ///
    /// # Errors
    ///
    /// Returns an error if communication with the device fails.
    pub fn set_channel(&self, channel: u16, value: u8) -> Result<(), UDmxError> {
        trace!("Setting channel {channel} to value {value}");

        self.send_command(Command::SetSingleChannel, value.into(), channel, None)
    }

    /// Set a range of channels to a number of values.
    ///
    /// The length of the `values` determines the number of channels to set.
    ///
    /// # Arguments
    ///
    /// * `starting_channel` - The starting channel number.
    /// * `values` - The values to set the channels to.
    ///
    /// # Errors
    ///
    /// Returns an error if communication with the device failed.
    pub fn set_channels(&self, starting_channel: u16, values: &[u8]) -> Result<(), UDmxError> {
        // do nothing if no values are provided
        if values.is_empty() {
            return Ok(());
        }

        trace!(
            "Setting {} channels starting at channel {starting_channel} to values {values:?}",
            values.len()
        );

        self.send_command(
            Command::SetChannelRange,
            values.len() as u16,
            starting_channel,
            Some(values),
        )
    }

    /// Start the uDMX bootloader.
    ///
    /// More information about the firmware update process can be found at <https://www.anyma.ch/research/udmx>.
    ///
    /// # Errors
    ///
    /// Returns an error if communication with the device failed.
    pub fn start_bootloader(&self) -> Result<(), UDmxError> {
        info!("Starting bootloader");

        self.send_command(Command::StartBootloader, 0, 0, None)
    }

    fn send_command(
        &self,
        command: Command,
        value: u16,
        channel: u16,
        buffer: Option<&[u8]>,
    ) -> Result<(), UDmxError> {
        match self.handle.write_control(
            rusb::request_type(
                rusb::Direction::Out,
                rusb::RequestType::Vendor,
                rusb::Recipient::Device,
            ),
            command.into(),
            value,
            channel,
            buffer.unwrap_or(&[]),
            USB_TIMEOUT,
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}
