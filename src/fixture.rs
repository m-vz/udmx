use crate::{UDmx, error};

#[allow(unused)]
pub struct RgbFixture {
    red_channel: u8,
    green_channel: u8,
    blue_channel: u8,
    brightness_channel: u8,
}

impl RgbFixture {
    #[allow(unused)]
    pub fn new(start_channel: u8) -> Self {
        RgbFixture {
            red_channel: start_channel,
            green_channel: start_channel + 1,
            blue_channel: start_channel + 2,
            brightness_channel: start_channel + 3,
        }
    }

    #[allow(unused)]
    pub fn set_rgb_brightness(
        &self,
        dmx: &UDmx,
        r: u8,
        g: u8,
        b: u8,
        brightness: u8,
    ) -> Result<(), error::UDmxError> {
        dmx.set_channels(self.red_channel, &[r, g, b, brightness])
    }

    #[allow(unused)]
    pub fn set_rgb(&self, dmx: &UDmx, r: u8, g: u8, b: u8) -> Result<(), error::UDmxError> {
        dmx.set_channels(self.red_channel, &[r, g, b])
    }

    #[allow(unused)]
    pub fn set_brightness(&self, dmx: &UDmx, brightness: u8) -> Result<(), error::UDmxError> {
        dmx.set_channel(self.brightness_channel, brightness)
    }
}
