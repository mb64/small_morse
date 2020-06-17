extern crate morse_code;

use morse_code::{encode, Action, State};

#[test]
fn case_independent() {
    let text_one = "The quick brown fox jumps over the lazy dog.";
    let text_two = " tHe  qUICK    brown FOX jUMps over thE LAzY     dOg    .  ";

    let morse_code_one = encode(text_one).collect::<Vec<_>>();
    let morse_code_two = encode(text_two).collect::<Vec<_>>();

    assert_eq!(morse_code_one, morse_code_two);
}

#[test]
fn correct_encoding() {
    let text = "Neat!";

    let mut res = String::new();
    for action in encode(text) {
        let s = if action.state == State::On { "-" } else { " " };
        res += &s.repeat(action.duration as usize);
    }

    assert_eq!(res, "--- -   -   - ---   ---       --- - --- - --- ---");
}
