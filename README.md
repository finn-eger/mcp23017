A comprehensive Rust driver for the [Microchip MCP23017](Microchip) IO expander.

- Split a device into individual [`embedded-hal`](embedded-hal) pins.
- Configure modes, pull-ups, and interrupt triggers with a type-level API.
- Service interrupts efficiently with a centralized controller.

For usage details and explanatory notes, see the [documentation](Docs.rs).

### Overview

```rust
let i2c = todo!(/* Setup I2C */);

// Setup a device:
const ADDRESS: u8 = 0x20;
let mut device = Mcp23017::<_, ADDRESS>::new(i2c);
let (pins, mut interrupt_controller) = device.split()?;

// Use a pin as an output:
let mut pin = pins.a0.into_push_pull_output()?;
pin.set_high()?;
pin.is_set_high()?;

// Use another pin as an input:
let mut pin = pins.a1.into_pull_up_input()?;
pin.is_high()?;

// Configure an input pin for interrupts:
let pin = pin.enable_interrupt(Sense::Edge)?;
// When an interrupt occurs,
// immediately notify the controller...
interrupt_controller.interrupt(Bank::A)?;
// ...and later query it about the cause:
interrupt_controller.triggered(&pin);
```

[Microchip]: https://www.microchip.com/en-us/product/mcp23017
[Docs.rs]: https://docs.rs/mcp23017-driver/latest
[embedded-hal]: https://github.com/rust-embedded/embedded-hal
