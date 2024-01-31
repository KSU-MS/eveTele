use super::{CanFrameRaw, MsgOut};

use byteorder::{BigEndian, ByteOrder};
use canparse::pgn::{ParseMessage, PgnDefinition, PgnLibrary};

pub fn load_lib(dbc: &String) -> PgnLibrary {
    PgnLibrary::from_dbc_file(dbc).expect("Failed to load DBC")
}

pub fn parse_can_frame(lib: &PgnLibrary, msg: CanFrameRaw) -> MsgOut {
    // Some holding vars
    let mut snames: Vec<String> = Vec::new();
    let mut sexplain: Vec<String> = Vec::new();
    let mut values: Vec<f32> = Vec::new();

    // Get the PGN ID for to look up
    let pgn_id: u32 = (msg.id as u32 >> 8) & 0x1FFFF;

    // Look up the ID
    let pgn: &PgnDefinition = lib.get_pgn(pgn_id).expect("Failed to get PGN");

    // Run thru each signal
    for sig in pgn.spns.iter() {
        snames.push(sig.1.name.clone());
        sexplain.push(sig.1.description.clone());
        values.push(sig.1.parse_message(msg.val.as_slice()).unwrap_or(0.0) as f32);
    }

    MsgOut {
        time: msg.time,
        name: pgn.name_abbrev.clone(),
        snames,
        sexplain,
        values,
    }
}

pub fn parse_csv(path: String, dbc: String) -> Vec<MsgOut> {
    // Parse dbc file THIS WILL FAIL IF ANY MESSAGE NAMES HAVE AN UNDERSCORE NEED TO FIX
    let lib = PgnLibrary::from_dbc_file(dbc).unwrap();

    // Get an object to start reading the CSV
    let mut rdr = csv::Reader::from_path(path).expect("Cannot find CSV");

    // Make out vec to return
    let mut msgs: Vec<MsgOut> = Vec::new();

    let mut counter: u32 = 0;
    let mut err_counter: u32 = 0;

    // Iterate over each row of the CSV
    for res in rdr.records() {
        // Get the row of data
        let record = res.unwrap();

        // Define some working vars
        let mut snames: Vec<String> = Vec::new();
        let mut sexplain: Vec<String> = Vec::new();
        let mut values: Vec<f32> = Vec::new();
        let id: u32;

        //
        // Parse for timestamp
        //
        let time = record[0].parse::<usize>().unwrap_or(0);

        //
        // Parse for ID
        // This fixes a stupid fucking error with hex crate not taking odd lengths, then it fixes
        // the byteorder crate bug of a length less than 4, then handles wacky extended ID
        if (1 - ((record[1].len() & 1) << 1) as i32) == -1 {
            // Make it not odd
            let id_vec = "0".to_owned() + &record[1];
            id = BigEndian::read_u16(hex::decode(&id_vec).unwrap().as_slice()) as u32;
        } else if record[1].len() <= 2 {
            // Byte order will kill itself with less than 2 bytes
            id = hex::decode(&record[1]).unwrap()[0] as u32;
        } else {
            id = BigEndian::read_u32(hex::decode(&record[1]).unwrap().as_slice());
        }

        // Get the PGN ID for to look up
        let pgn_id: u32 = (id as u32 >> 8) & 0x1FFFF;

        //
        // Parse for the bytes
        //
        let val = hex::decode(record[3].to_string()).unwrap_or(vec![0]);

        //
        // Start decoding
        // Get message

        counter += 1;
        let pgn = lib.get_pgn(pgn_id).unwrap_or_else(|| {
            err_counter += 1;
            println!("error line: {}", counter);
            println!("error count: {}", err_counter);
            lib.get_pgn(0).unwrap()
        });

        // Run thru each signal
        for sig in pgn.spns.iter() {
            snames.push(sig.1.name.clone());
            sexplain.push(sig.1.description.clone());
            values.push(sig.1.parse_message(val.as_slice()).unwrap_or(0.0) as f32);
        }

        msgs.push(MsgOut {
            time,
            name: pgn.name_abbrev.clone(),
            snames,
            sexplain,
            values,
        })
    }

    msgs
}

pub fn test_pgns(dbc: String) {
    // Parse dbc file THIS WILL FAIL IF ANY MESSAGE NAMES HAVE AN UNDERSCORE NEED TO FIX
    let lib = PgnLibrary::from_dbc_file(dbc).unwrap();

    for pgn in lib.pgns.iter() {
        println!("{}", pgn.1.name_abbrev);
    }
}
