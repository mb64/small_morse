# Morse code generation library for Rust

License: MIT

## Features

 * Supports `no_std`
 * Easy `Iterator`-based interface
 * Letters, numbers, and punctuation all fully supported
 * Support for Farnsworth delays (longer delays between letters and words to
   help learn morse code)

There are currently no plans to support *de*coding of Morse code.

## Example

```rust
fn wait_for(duration: u8) {
    // ...
}
fn beep_for(duration: u8) {
    // ...
}

for action in small_morse::encode("Hello in morse code!") {
    if action.state == small_morse::State::On {
        beep_for(action.duration);
    } else {
        wait_for(action.duration);
    }
}
```

## Intended use

The natural extension to your simple embedded project of blinking an LED is to
blink it in morse code.  The goal of this library is to make it easy to emit
morse code from any environment, and with any type of output.
