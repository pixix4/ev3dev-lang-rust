use ev3dev_lang_rust::{motors::MotorPort, sensors::SensorPort, Port};

extern crate ev3dev_lang_rust;

#[test]
fn test_input_port_mapping() {
    assert_eq!(SensorPort::In1.address(), "spi0.1:S1".to_string());
    assert_eq!(SensorPort::In2.address(), "spi0.1:S2".to_string());
    assert_eq!(SensorPort::In3.address(), "spi0.1:S3".to_string());
    assert_eq!(SensorPort::In4.address(), "spi0.1:S4".to_string());
}

#[test]
fn test_output_port_mapping() {
    assert_eq!(MotorPort::OutA.address(), "spi0.1:MA".to_string());
    assert_eq!(MotorPort::OutB.address(), "spi0.1:MB".to_string());
    assert_eq!(MotorPort::OutC.address(), "spi0.1:MC".to_string());
    assert_eq!(MotorPort::OutD.address(), "spi0.1:MD".to_string());
}
