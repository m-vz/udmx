use udmx::{UDmx, error::UDmxError};

fn main() -> Result<(), UDmxError> {
    let _dmx = UDmx::new()?;

    Ok(())
}
