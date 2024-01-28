// Imports for thread shenanigens
use std::{sync::mpsc::channel, thread};

mod can_utils;
mod foxglove_utils;
mod serial;

#[derive(Clone, Debug)]
pub struct CanFrameRaw {
    pub time: usize,
    pub id: Vec<u8>,
    pub val: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct MsgOut {
    pub time: usize,
    pub name: String,
    pub snames: Vec<String>,
    pub values: Vec<f32>,
}

#[derive(Clone)]
pub struct ReadPort {}

impl CanFrameRaw {
    pub fn parse_frame(self) {
        can_utils::parse_can_frame(self);
    }

    pub fn parse_log(path: String, dbc: String) {
        can_utils::parse_log(path, dbc);
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

    // pub fn send_msg(self) {}
}

impl ReadPort {
    // Spit out all ports into vec
    pub fn list_ports() -> Vec<String> {
        serial::list_ports()
    }

    // Open a thread and get data out from a channel
    pub fn start_bg_read(sel_port: String, sel_baud: u32) {
        // Set up a channel for comms to other thread
        let (tx, rx) = channel();

        thread::spawn(move || {
            // Open up the port
            let mut port = serial::open_port(sel_port.clone(), sel_baud);

            // Continueously read data out from the port into the channel
            loop {
                tx.send(serial::read_out(&mut port)).unwrap();
            }
        });

        for recv in rx {
            println!("{:?}", recv);
            CanFrameRaw::parse_frame(recv);
        }
    }
}
