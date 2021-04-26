extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::Attribute;
use ev3dev_lang_rust::Ev3Result;

fn main() -> Ev3Result<()> {
    // Get value0 of first connected color sensor.
    let color_sensor_value = Attribute::from_path_with_discriminator(
        "/sys/class/lego-sensor",
        "value0",
        "driver_name",
        "lego-ev3-color",
    )?;

    // Get raw rotation count of motor in port `A`.
    // See https://github.com/ev3dev/ev3dev/wiki/Internals:-ev3dev-stretch for more infomation.
    let rotation_count = Attribute::from_path_with_discriminator(
        "/sys/bus/iio/devices",
        "in_count0_raw",
        "name",
        "ev3-tacho",
    )?;

    loop {
        println!(
            "value0 of color sensor: {}",
            color_sensor_value.get::<i32>()?
        );
        println!("Raw rotation count: {}", rotation_count.get::<i32>()?);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
