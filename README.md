# uDMX Library

A Rust library for controlling DMX fixtures via the Anyma uDMX USB interface.

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

- `rgb_fixture.rs`: Demonstrates controlling an RGB DMX fixture
- `moving_head_fixture.rs`: Demonstrates controlling a moving head light fixture
