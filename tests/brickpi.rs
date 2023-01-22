use ev3dev_lang_rust::{motors::MotorPort, sensors::SensorPort, Port};

extern crate ev3dev_lang_rust;

#[test]
fn test_input_port_mapping() {
    assert_eq!(SensorPort::In1.address(), "serial0-0:S1".to_string());
    assert_eq!(SensorPort::In2.address(), "serial0-0:S2".to_string());
    assert_eq!(SensorPort::In3.address(), "serial0-0:S3".to_string());
    assert_eq!(SensorPort::In4.address(), "serial0-0:S4".to_string());
}

#[test]
fn test_output_port_mapping() {
    assert_eq!(MotorPort::OutA.address(), "serial0-0:MA".to_string());
    assert_eq!(MotorPort::OutB.address(), "serial0-0:MB".to_string());
    assert_eq!(MotorPort::OutC.address(), "serial0-0:MC".to_string());
    assert_eq!(MotorPort::OutD.address(), "serial0-0:MD".to_string());
}
