// Imports for thread shenanigens
use std::{sync::mpsc::channel, thread};

use canparse::pgn::PgnLibrary;

mod can_utils;
mod foxglove_utils;
mod save_utils;
mod serial_utils;

#[derive(Default)]
pub struct RWUtils {}

#[derive(Clone, Debug)]
pub struct CanFrameRaw {
    pub time: usize,
    pub id: u32,
    pub val: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct MsgOut {
    pub time: usize,
    pub name: String,
    pub snames: Vec<String>,
    pub sexplain: Vec<String>,
    pub values: Vec<f32>,
    pub units: Vec<String>,
}

impl RWUtils {
    // Spit out all ports into vec
    pub fn list_ports() -> Vec<String> {
        serial_utils::list_ports()
    }

    // Open a thread and get data out from a channel
    pub fn start_bg_read(dbc_path: &String, port: &String, baud: u32) {
        // Set up a channel for comms to other thread
        let (tx, rx) = channel();

        // Load the DBC into memory
        let lib = can_utils::load_lib(dbc_path);

        // Open up the port
        let mut port = serial_utils::open_port(port, baud);

        thread::spawn(move || {
            // Continueously read data out from the port into the channel
            loop {
                tx.send(serial_utils::read_out(&mut port).parse_frame(&lib))
                    .unwrap();
            }
        });

        for recv in rx {
            println!("{:?}", recv);
        }
    }

    pub fn parse_log(dbc_path: &String, in_path: &String, out_path: &String) {
        let msgs = can_utils::parse_csv(in_path, dbc_path);

        println!("Parsed CSV into MsgOut");

        save_utils::save_csv(out_path, msgs);

        println!("Saved MsgOut to CSV");
    }

    pub fn my_pb_test() {
        save_utils::build_proto();
    }

    pub fn my_mcap_test() {
        save_utils::mcap_test();
    }
}

impl CanFrameRaw {
    pub fn parse_frame(self, lib: &PgnLibrary) -> MsgOut {
        can_utils::parse_frame(&lib, self)
    }
}

impl MsgOut {
    pub fn open_fg_ws() {
        println!("Init test");

        thread::spawn(move || {
            // Start a websocket and send some example data
            let _ = foxglove_utils::test();
        });

        println!("Test done");
    }
}
