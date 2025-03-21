# FAST Pinball Bevy Plugin

A collection of simple plugins to allow the Bevy engine to interact with FAST Pinball Neutron board.

## Serial Connections

The Neutron adds 3 virtual serial ports that handle communications for the NET I/O bus, EXP (expansion) bus, and the DSP (display) bus. This plugin handles communication with the NET and EXP busses.

```rust
app.add_plugins(Neutron::new("COM5").add_exp_port("COM7"))
```

Adding this plugin will grant access to two things and is required for all other plugins on this document:

- **Resource**: `IoNetPort` - This is the port which reads/writes the IO NET data. The Neutron plugin does this so it's almost entirely likely that it will never need to be accessed directly
- **Resource**: `ExpPort` - This is the port which reads/writes the EXP data. Just like with I/O net it's unlikely that the need will arize to access this directly, but it's available for those rare cases
- **Event**: `FastIoEvent` - Incoming data from the IO NET port, such as switch opened, closed, etc.

## Expansion LEDs

The `ExpansionLeds` allows multiple LEDs to be defined. Each LED is identified by a customizable type. The `pinball` plugin provides common ones, but every project will require a custom identifier as well. Note that `ExpansionLeds` can be added multiple times _per identifier type_.

Each LED definition takes the following:

- `board` - Which expansion board it is connected to, and that board's settings
- `port` - Which expansion port the LED is physically connected to. "Port 1" = `0`, "Port 2" = `1`, etc.
- `index` - Each port can have up to 32 LEDs. Starting at `0` with the LED nearest to the port and working outward from there.

LEDs do not have to be defined in order, and not every LED in a sequence has to be specified on that plugin addition.

```rust
enum PlayfieldIndicators {
    LeftSpinner,
    LeftRamp
}

// Neutron plugin is required prior to this
app.add_plugins(ExpansionLeds(vec![
    LEDDefinition {
        board: ExpansionBoard::Neutron,
        port: 0,
        index: 0,
        id: PlayfieldIndicators::LeftSpinner,
        row: 2,
        col: 0,
    },
    LEDDefinition {
        board: ExpansionBoard::Neutron,
        port: 0,
        index: 1,
        id: PlayfieldIndicators::LeftRamp,
        row: 4,
        col: 0,
    },
]))
```

Adding the `ExpansionLeds` plugin adds an entity per LED with the following components: `Identity`, `Colored`, `Position`, `FastLED`.

Now,, to set the color of an LED it's simply to set `colored.color = Color::hsl(0.5, 0.5, 1.0);` The plugin will handle sending the appropriate command to the Neutron.

```rust
fn set_color(query: Query<&Identity, &Colored>) {
    let left_spinner_led = query.iter().find(|(identity, _)|{ identity.id == PlayfieldIndicators::LeftSpinner });
    match left_spinner_led {
        Some((_, colored)) => colored.color = Color::hsl(0.5, 0.5, 1.0),
        _ => ()
    }
}
```
