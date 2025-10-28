use std::thread;
use std::time::Duration;
use udmx::{UDmx, error};

#[allow(unused, clippy::struct_field_names)]
struct FogMachineFixture {
    fog_channel: u16,
}

impl FogMachineFixture {
    #[allow(unused)]
    const fn new(start_channel: u16) -> Self {
        Self {
            fog_channel: start_channel,
        }
    }

    #[allow(unused)]
    fn set_fog(&self, dmx: &UDmx, amount: u8) -> Result<(), error::UDmxError> {
        log::info!("Setting fog to {amount}");

        dmx.set_channel(self.fog_channel, amount)
    }

    #[allow(unused)]
    fn full_fog(&self, dmx: &UDmx) -> Result<(), error::UDmxError> {
        self.set_fog(dmx, 255)
    }

    #[allow(unused)]
    fn stop(&self, dmx: &UDmx) -> Result<(), error::UDmxError> {
        self.set_fog(dmx, 0)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let dmx = UDmx::new()?;
    let fixture = FogMachineFixture::new(0);

    fixture.full_fog(&dmx)?;
    thread::sleep(Duration::from_millis(1000));
    fixture.stop(&dmx)?;

    Ok(())
}
