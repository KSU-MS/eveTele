use serialport::{self, SerialPort};

use super::CanFrameRaw;

// Read from port
pub fn read_out(port: &mut Box<dyn SerialPort>) -> CanFrameRaw {
    // Set up some vars to store the data we get
    let mut serial_buf: Vec<u8> = vec![0];
    let mut string_vec: String = String::new();

    // Read the data from the serial port and put it into a buf
    port.read(&mut serial_buf).expect("port data into buf fail");

    // Go thru each byte and save it to our string until we hit the return char
    while serial_buf[0] != 13 {
        // Ignore the new line bit boi
        if serial_buf[0] != 10 {
            // Append the byte
            string_vec.push(serial_buf[0] as char);
        }

        // Read for new data and go again
        port.read(&mut serial_buf).expect("port data into buf fail");
    }

    // Take the input, seperate it into individual elements based on ","
    // then collect it into a vec. Also the basic message scructure goes like
    // (timestamp),(ID),(Length),(msg)
    let read_val: Vec<&str> = string_vec.split(",").into_iter().collect();

    // Scan the data for hex info
    let id_out = hex::decode(read_val[1]).unwrap_or(vec![0]);
    let val_out = hex::decode(read_val[3]).unwrap_or(vec![0]);

    return CanFrameRaw {
        time: read_val[0].parse().unwrap_or(0),
        id: id_out,
        val: val_out,
    };
}

// Returns a configured port
pub fn open_port(sel_port: String, sel_baud: u32) -> Box<dyn SerialPort> {
    println!("Opening port: {}", sel_port);

    // Connect to the port
    serialport::new(sel_port, sel_baud)
        .timeout(std::time::Duration::from_millis(1000))
        .open()
        .expect("Cannot open serial port")
}

// List all ports
pub fn list_ports() -> Vec<String> {
    // Working var
    let mut str_p: Vec<String> = Vec::new();

    // Append all ports to list
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        str_p.push(p.port_name);
    }

    // Debug print
    println!("List of ports: {:?}", &str_p);

    // Return list
    return str_p;
}
