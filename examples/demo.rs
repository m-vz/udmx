use log::info;
use pretty_env_logger;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use udmx::fixture::RgbFixture;
use udmx::{UDmx, error};

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn lerp(&self, other: &Color, t: f32) -> Self {
        let t = t.max(0.0).min(1.0);

        let r = self.r as f32 * (1.0 - t) + other.r as f32 * t;
        let g = self.g as f32 * (1.0 - t) + other.g as f32 * t;
        let b = self.b as f32 * (1.0 - t) + other.b as f32 * t;

        Color {
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
    let transition_steps = 3;
    let sleep_duration = Duration::from_millis(100 / transition_steps);

    for (i, window) in colors.windows(2).enumerate() {
        info!("Transitioning from color {} to color {}", i, i + 1);

        for step in 0..transition_steps {
            let progress = step as f32 / transition_steps as f32;
            let color = &window[0].lerp(&window[1], progress);

            fixture.set_rgb(dmx, color.r, color.g, color.b)?;

            thread::sleep(sleep_duration);
        }
    }

    Ok(())
}
