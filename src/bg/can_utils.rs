// use csv::*;
// use mcap;

use super::CanFrameRaw;
use canparse::pgn::{ParseMessage, PgnDefinition, PgnLibrary, SpnDefinition};

pub fn parse_can_frame(msg: CanFrameRaw) {}

pub fn parse_log(path: String, dbc: String) {
    // Parse dbc file into PgnLibrary THIS WILL FAIL IF ANY MESSAGE NAMES HAVE AN UNDERSCORE
    let lib = PgnLibrary::from_dbc_file(dbc).unwrap();

    // println!("Lib out: {:?}", lib);

    // Pull signal definition for engine speed
    let can_id: u32 = 43;
    let pgn_id: u32 = (can_id >> 8) & 0x1FFFF; // FUck this crate author for this line
    let test_pgn: &PgnDefinition = lib.get_pgn(pgn_id).expect("Failed to get PGN");
    let test_spn: &SpnDefinition = lib.get_spn("Engine_Speed").unwrap();

    // Parse frame containing engine speed
    let msg: [u8; 8] = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    let spn_out: f32 = test_spn.parse_message(&msg).unwrap();

    for spn in test_pgn.spns.iter() {
        println!("PGN SPN TEST: {:?}", spn.1.parse_message(&msg).unwrap());
    }

    println!("Test SPN: {:?}", spn_out);
}
