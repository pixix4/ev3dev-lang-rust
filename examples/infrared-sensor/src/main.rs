extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::sensors::{BeaconSeeker, InfraredSensor, RemoteControl};
use ev3dev_lang_rust::Ev3Result;

fn main() -> Ev3Result<()> {
    let sensor = InfraredSensor::find()?;

    remote(sensor)?;
    // seeker(sensor)?;

    Ok(())
}

#[allow(dead_code)]
fn remote(sensor: InfraredSensor) -> Ev3Result<()> {
    let remote = RemoteControl::new(sensor, 1)?;

    loop {
        remote.process()?;
        println!(
            "Button: {:?},{:?},{:?},{:?},{:?}",
            remote.is_red_up(),
            remote.is_red_down(),
            remote.is_beacon(),
            remote.is_blue_down(),
            remote.is_blue_up(),
        );

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[allow(dead_code)]
fn seeker(sensor: InfraredSensor) -> Ev3Result<()> {
    let seeker = BeaconSeeker::new(sensor, 1)?;

    loop {
        println!("Button: {:?}", seeker.get_heading_and_distance());

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
