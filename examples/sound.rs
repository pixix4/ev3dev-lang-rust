extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::sound;
use ev3dev_lang_rust::Ev3Result;

fn main() -> Ev3Result<()> {
    sound::beep()?;

    sound::speak("Hello, I am Robot")?.wait()?;

    Ok(())
}
