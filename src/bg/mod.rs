// Imports for thread shenanigens
use std::{sync::mpsc::channel, thread};

use canparse::pgn::PgnLibrary;

mod can_utils;
mod foxglove_utils;
mod save_utils;
mod serial_utils;

pub struct ReadUtils {}

pub struct FileHandeler {
    pub dbc: PgnLibrary,
    pub csv_in_path: String,
    pub mcap_out_path: String,
}

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

impl ReadUtils {
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
                tx.send(CanFrameRaw::parse_frame(
                    serial_utils::read_out(&mut port),
                    &lib,
                ))
                .unwrap();
            }
        });

        for recv in rx {
            println!("{:?}", recv);
        }
    }
}

impl FileHandeler {
    pub fn proto_test() {
        save_utils::proto_test();
    }

    pub fn parse_log(path: String, dbc: String) {
        let msgs = can_utils::parse_csv(path, dbc);

        save_utils::save_csv(msgs);
    }
}

impl CanFrameRaw {
    pub fn parse_frame(self, lib: &PgnLibrary) {
        // can_utils::parse_can_frame(&lib, self);
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
