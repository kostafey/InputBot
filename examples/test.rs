extern crate inputbot;

use inputbot::*;
use KeybdKey::*;
use MouseButton::*;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    // Autorun for videogames.
    NumLockKey.bind(|| {
        while NumLockKey.is_toggled() {
            LShiftKey.press();
            WKey.press();
            sleep(Duration::from_millis(50));
            WKey.release();
            LShiftKey.release();
        }
    });

    // Rapidfire for videogames.
    RightButton.bind(|| {
        while RightButton.is_pressed() {
            LeftButton.press();
            sleep(Duration::from_millis(50));
            LeftButton.release();
        }
    });

    // Send a key sequence.
    RKey.bind(|| KeySequence("Sample text").send());

    // Move mouse.
    QKey.bind(|| MouseCursor.move_rel(10, 10));

    // Call this to start listening for bound inputs.
    handle_input_events();
}
