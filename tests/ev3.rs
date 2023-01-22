use ev3dev_lang_rust::{motors::MotorPort, sensors::SensorPort, Port};

extern crate ev3dev_lang_rust;

#[test]
fn test_input_port_mapping() {
    assert_eq!(SensorPort::In1.address(), "in1".to_string());
    assert_eq!(SensorPort::In2.address(), "in2".to_string());
    assert_eq!(SensorPort::In3.address(), "in3".to_string());
    assert_eq!(SensorPort::In4.address(), "in4".to_string());
}

#[test]
fn test_output_port_mapping() {
    assert_eq!(MotorPort::OutA.address(), "outA".to_string());
    assert_eq!(MotorPort::OutB.address(), "outB".to_string());
    assert_eq!(MotorPort::OutC.address(), "outC".to_string());
    assert_eq!(MotorPort::OutD.address(), "outD".to_string());
}
