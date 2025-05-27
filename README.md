# uDMX Library

A Rust library for controlling DMX fixtures via the Anyma uDMX USB interface.

## Usage

```rust
use udmx::UDmx;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dmx = UDmx::new()?;

    // Set a single channel
    dmx.set_single_channel(0, 255)?;

    // Set multiple channels at once (starting at the index given)
    let values = [255, 128, 64, 32];
    dmx.set_channel_range(3, &values)?;

    Ok(())
}
```

## Examples

- `demo.rs`: Demonstrates controlling an RGB DMX fixture with various animations
