use log::info;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use udmx::{UDmx, error};

#[allow(unused, clippy::struct_field_names)]
struct RgbFixture {
    red_channel: u16,
    green_channel: u16,
    blue_channel: u16,
    brightness_channel: u16,
}

impl RgbFixture {
    #[allow(unused)]
    const fn new(start_channel: u16) -> Self {
        Self {
            red_channel: start_channel,
            green_channel: start_channel + 1,
            blue_channel: start_channel + 2,
            brightness_channel: start_channel + 3,
        }
    }

    #[allow(unused)]
    fn set_rgb_brightness(
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
    fn set_rgb(&self, dmx: &UDmx, r: u8, g: u8, b: u8) -> Result<(), error::UDmxError> {
        dmx.set_channels(self.red_channel, &[r, g, b])
    }

    #[allow(unused)]
    fn set_brightness(&self, dmx: &UDmx, brightness: u8) -> Result<(), error::UDmxError> {
        dmx.set_channel(self.brightness_channel, brightness)
    }
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);

        let r = f32::from(self.r).mul_add(1.0 - t, f32::from(other.r) * t);
        let g = f32::from(self.g).mul_add(1.0 - t, f32::from(other.g) * t);
        let b = f32::from(self.b).mul_add(1.0 - t, f32::from(other.b) * t);

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        Self {
            r: r as u8,
            g: g as u8,
            b: b as u8,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let dmx = UDmx::new()?;
    let fixture = RgbFixture::new(0);
    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    ctrlc::set_handler(move || {
        info!("Ctrl+C received, shutting down...");
        shutdown_tx.send(()).unwrap();
    })?;

    info!("Starting RGB fixture animation demo...\nPress Ctrl+C to exit");
    fixture.set_brightness(&dmx, 20)?;
    while shutdown_rx.try_recv().is_err() {
        pulsing_cycle(&dmx, &fixture)?;
    }
    fixture.set_rgb(&dmx, 0, 0, 0)?;

    Ok(())
}

fn pulsing_cycle(dmx: &UDmx, fixture: &RgbFixture) -> Result<(), error::UDmxError> {
    let colors = [
        Color { r: 255, g: 0, b: 0 },
        Color { r: 0, g: 255, b: 0 },
        Color { r: 0, g: 0, b: 255 },
        Color { r: 255, g: 0, b: 0 },
    ];
    let transition_steps: u8 = 3;
    let sleep_duration = Duration::from_millis(100 / u64::from(transition_steps));

    for (i, window) in colors.windows(2).enumerate() {
        info!("Transitioning from color {} to color {}", i, i + 1);

        for step in 0..transition_steps {
            let progress = f32::from(step) / f32::from(transition_steps);
            let color = &window[0].lerp(&window[1], progress);

            fixture.set_rgb(dmx, color.r, color.g, color.b)?;

            thread::sleep(sleep_duration);
        }
    }

    Ok(())
}
