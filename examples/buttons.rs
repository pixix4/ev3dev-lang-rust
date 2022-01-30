extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{Ev3Button, Ev3Result};

fn main() -> Ev3Result<()> {
    let button = Ev3Button::new()?;

    loop {
        button.process();

        println!(
            "{}, {}, {}, {}, {}, {}",
            button.is_up(),
            button.is_down(),
            button.is_left(),
            button.is_right(),
            button.is_enter(),
            button.is_backspace(),
        );
        println!("{:?}", button.get_pressed_buttons());

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
