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

for action in morse_code::encode("Hello in morse code!") {
    if action.state == morse_code::State::On {
        beep_for(action.duration);
    } else {
        wait_for(action.duration);
    }
}
```
