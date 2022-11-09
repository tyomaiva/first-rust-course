use std::time::Duration;

fn main() {
    let ports = serialport::available_ports().expect("No serial ports found!");
    let port_count = ports.len();
    assert_eq!(
        port_count, 1,
        "Expected exactly one serial port, but found {} instead!",
        port_count
    );
    let port_name = &ports[0].port_name;
    println!("The port we gonna use is {}.", port_name);
    let mut port = serialport::new(port_name, 115_200)
        .timeout(Duration::from_millis(15_000))
        .open()
        .expect("Failed to open the port");

    let mut serial_buf: Vec<u8> = vec![0; 32];
    loop {
        let byte_count = port
            .read(serial_buf.as_mut_slice())
            .expect("Found no data!");

        print!(
            "{}",
            std::str::from_utf8(&serial_buf[..byte_count])
                .unwrap()
                .trim_matches(char::from(0))
        );
    }
}
