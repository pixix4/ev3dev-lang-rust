extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{Button, Ev3Result};

fn main() -> Ev3Result<()> {
    let mut button = Button::new()?;

    button.set_change_handler(|pressed_buttons| println!("Pressed buttons: {:?}", pressed_buttons));

    button.set_up_handler(|is_pressed| println!("Button 'up' is pressed: {}", is_pressed));
    button.set_down_handler(|is_pressed| println!("Button 'down' is pressed: {}", is_pressed));
    button.set_left_handler(|is_pressed| println!("Button 'left' is pressed: {}", is_pressed));
    button.set_right_handler(|is_pressed| println!("Button 'right' is pressed: {}", is_pressed));
    button.set_enter_handler(|is_pressed| println!("Button 'enter' is pressed: {}", is_pressed));
    button.set_backspace_handler(|is_pressed| {
        println!("Button 'backspace' is pressed: {}", is_pressed)
    });
    button.set_backspace_handler(|is_pressed| {
        println!("Button 'backspace' is pressed: {}", is_pressed)
    });

    loop {
        button.process();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
