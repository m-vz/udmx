use log::info;
use std::{sync::mpsc, thread, time::Duration};
use udmx::{UDmx, error};

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
enum Colour {
    White,
    Red,
    Yellow,
    Green,
    Orange,
    Blue,
    Magenta,
    Cyan,
}

impl From<Colour> for u8 {
    fn from(colour: Colour) -> Self {
        match colour {
            Colour::White => 0,
            Colour::Red => 16,
            Colour::Yellow => 32,
            Colour::Green => 48,
            Colour::Orange => 64,
            Colour::Blue => 80,
            Colour::Magenta => 96,
            Colour::Cyan => 112,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
enum Pattern {
    Circle,
    S,
    Cross,
    Flower,
    Star,
    Triangle,
    Circles,
}

impl From<Pattern> for (u8, u8) {
    fn from(value: Pattern) -> Self {
        // the second value returned is the start value for the shake version of the pattern
        match value {
            Pattern::Circle => (8, 112),
            Pattern::S => (16, 104),
            Pattern::Cross => (24, 96),
            Pattern::Flower => (32, 88),
            Pattern::Star => (40, 80),
            Pattern::Triangle => (48, 72),
            Pattern::Circles => (56, 64),
        }
    }
}

#[allow(unused, clippy::struct_field_names)]
struct MovingHeadFixture<'a> {
    dmx: &'a UDmx,
    x: u16,
    x_fine_adjust: u16,
    y: u16,
    y_fine_adjust: u16,
    speed: u16,
    brightness: u16,
    strobe: u16,
    colour: u16,
    pattern: u16,
    prism: u16,
    mode: u16,
    reset: u16,
}

#[allow(unused)]
impl<'a> MovingHeadFixture<'a> {
    const fn new(start_channel: u16, dmx: &'a UDmx) -> Self {
        Self {
            dmx,
            x: start_channel,
            x_fine_adjust: start_channel + 1,
            y: start_channel + 2,
            y_fine_adjust: start_channel + 3,
            speed: start_channel + 4,
            brightness: start_channel + 5,
            strobe: start_channel + 6,
            colour: start_channel + 7,
            pattern: start_channel + 8,
            prism: start_channel + 9,
            mode: start_channel + 10,
            reset: start_channel + 11,
        }
    }

    fn set_x(&self, x: u8, fine_adjustment: u8) -> Result<(), error::UDmxError> {
        self.dmx.set_channels(self.x, &[x, fine_adjustment])
    }

    fn set_y(&self, y: u8, fine_adjustment: u8) -> Result<(), error::UDmxError> {
        self.dmx.set_channels(self.y, &[y, fine_adjustment])
    }

    fn set_speed(&self, speed: u8) -> Result<(), error::UDmxError> {
        // we invert the speed value because on the moving head, higher values mean slower movement
        self.dmx.set_channel(self.speed, 255 - speed)
    }

    fn set_brightness(&self, brightness: u8) -> Result<(), error::UDmxError> {
        self.dmx.set_channel(self.brightness, brightness)
    }

    /// Set the strobe frequency of the moving head.
    ///
    /// Supports frequencies between 1Hz and 25Hz.
    ///
    /// Set to `None` to disable strobe.
    fn set_strobe(&self, hz: Option<f32>) -> Result<(), error::UDmxError> {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let value = hz.map_or(0, |hz| {
            (254. / 24. * (hz.clamp(1., 25.) - 1.)).round() as u8 + 1
        });

        self.dmx.set_channel(self.strobe, value)
    }

    fn set_colour(&self, colour: Colour) -> Result<(), error::UDmxError> {
        self.dmx.set_channel(self.colour, colour.into())
    }

    /// Set the pattern of the moving head and an optional shake speed.
    ///
    /// Supports shake speeds from 1 to 8.
    fn set_pattern(
        &self,
        pattern: Pattern,
        shake_speed: Option<u8>,
    ) -> Result<(), error::UDmxError> {
        let (pattern, pattern_shake) = pattern.into();
        let value = shake_speed.map_or(pattern, |speed| pattern_shake + (speed.clamp(1, 8) - 1));

        self.dmx.set_channel(self.pattern, value)
    }

    fn clear_pattern(&self) -> Result<(), error::UDmxError> {
        self.dmx.set_channel(self.pattern, 0)
    }

    /// Switch between all patterns with a given speed.
    ///
    /// Supports switching speeds from 1 to 128.
    fn switch_pattern(&self, speed: u8) -> Result<(), error::UDmxError> {
        self.dmx
            .set_channel(self.pattern, 128 + (speed.clamp(1, 128) - 1))
    }

    fn set_prism(&self, open: bool) -> Result<(), error::UDmxError> {
        self.dmx.set_channel(self.prism, if open { 8 } else { 0 })
    }

    /// Rotate the prism with a given speed.
    ///
    /// Supports rotation speeds from 0 to 127.
    fn rotate_prism(&self, speed: u8) -> Result<(), error::UDmxError> {
        self.set_prism(true)
            .and_then(|()| self.dmx.set_channel(self.prism, 128 + speed.clamp(0, 127)))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let dmx = UDmx::new()?;
    let fixture = MovingHeadFixture::new(0, &dmx);
    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    ctrlc::set_handler(move || {
        info!("Ctrl+C received, shutting down...");
        shutdown_tx.send(()).unwrap();
    })?;

    info!("Starting moving head demo...\nPress Ctrl+C to exit");

    reset(&fixture)?;
    fixture.set_brightness(20)?;
    fixture.set_speed(100)?;
    thread::sleep(Duration::from_secs(2));
    let mut x = 0;
    let mut y = 0;
    while shutdown_rx.try_recv().is_err() {
        fixture.set_x(x, 0)?;
        x = x.wrapping_add(100);
        fixture.set_y(y, 0)?;
        y = y.wrapping_add(100);
        thread::sleep(Duration::from_millis(1000));
    }
    reset(&fixture)?;

    Ok(())
}

fn reset(fixture: &MovingHeadFixture) -> Result<(), error::UDmxError> {
    fixture.set_x(0, 0)?;
    fixture.set_y(0, 0)?;
    fixture.set_speed(255)?;
    fixture.set_brightness(0)?;
    fixture.set_strobe(None)?;
    fixture.set_colour(Colour::White)?;
    fixture.clear_pattern()?;
    fixture.set_prism(false)?;

    Ok(())
}
