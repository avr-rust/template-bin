# Rust AVR executable template

A template for Rust based AVR executables.

**NOTE**: This software template repository is offered in the public domain. It is free to use, adapt, modify, distribute, with no restrictions and no crediting required.

Provides:

  * A Rust target specification JSON for ATmega328P - [`avr-atmega328p.json`](./avr-atmega328p.json)
  * A GitHub-action based CI test pipeline

## Build instructions

Install Rust nightly.

Then run:

```
cargo build --target avr-atmega328p.json -Z build-std=core --release
```

The final ELF executable file will then be available at `target/avr-atmega328p/release/template-bin.elf`.

