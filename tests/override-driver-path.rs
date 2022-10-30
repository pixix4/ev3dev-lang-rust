extern crate ev3dev_lang_rust;

#[test]
fn test_output_port_mapping() {
    let driver_path = ev3dev_lang_rust::DRIVER_PATH;
    assert_eq!(driver_path, "/test/path");
}
