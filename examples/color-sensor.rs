extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::sensors::ColorSensor;
use ev3dev_lang_rust::Ev3Result;

fn main() -> Ev3Result<()> {
    let color_sensor = ColorSensor::find()?;
    color_sensor.set_mode_rgb_raw()?;

    loop {
        println!("{:?}", color_sensor.get_rgb()?);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
