# uDMX Library
A Rust library for controlling DMX fixtures via the [Anyma uDMX USB interface](https://www.anyma.ch/research/udmx/).

## Usage
```rust
use udmx::UDmx;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dmx = UDmx::new()?;

    // Set a single channel
    dmx.set_channel(0, 255)?;

    // Set multiple channels at once (starting at the index given)
    let values = [255, 128, 64, 32];
    dmx.set_channels(3, &values)?;

    Ok(())
}
```

## Examples
The examples directory contains some fixture implementations for reference.

> [!NOTE]
> These examples likely won't work with your specific fixtures.
> Consult the fixture's manual for the correct channel numbers and values.

- `rgb_fixture.rs`: Demonstrates controlling an RGB DMX fixture
- `moving_head_fixture.rs`: Demonstrates controlling a moving head light fixture
- `fog_machine_fixture.rs`: Demonstrates controlling a fog machine
