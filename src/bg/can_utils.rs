use super::{CanFrameRaw, MsgOut};

use byteorder::{BigEndian, ByteOrder};
use canparse::pgn::{ParseMessage, PgnLibrary};

pub fn load_lib(dbc: &String) -> PgnLibrary {
    PgnLibrary::from_dbc_file(dbc, false).expect("Failed to load DBC")
}

pub fn parse_test(dbc: &String) {
    // Parse dbc file
    let lib = PgnLibrary::from_dbc_file(dbc, false).expect("Failed to load DBC");

    // Test vars
    let id_str = "A6".to_string();
    let test_val = "FFFFFEFF00000000".to_string();

    // Get ID
    let id: u32;

    if (1 - ((id_str.len() & 1) << 1) as i32) == -1 {
        // Make it not odd
        let id_vec = "0".to_owned() + id_str.as_str();
        id = BigEndian::read_u16(hex::decode(&id_vec).unwrap().as_slice()) as u32;
    } else if id_str.len() <= 2 {
        // Byte order will kill itself with less than 2 bytes
        id = hex::decode(&id_str).unwrap()[0] as u32;
    } else {
        id = BigEndian::read_u32(hex::decode(&id_str).unwrap().as_slice());
    }

    //
    // Parse for the bytes
    let val = hex::decode(test_val).unwrap_or(vec![0]);

    println!("{:?}", lib.get_pgn(id));

    // Print out the test result
    println!("{}", id);
    println!("{:?}", val);
    println!("");
    println!("{:?}", parse_frame(&lib, CanFrameRaw { time: 0, id, val }));
}

pub fn parse_csv(path: &String, dbc: &String) -> Vec<MsgOut> {
    // Parse dbc file
    let lib = PgnLibrary::from_dbc_file(dbc, false).expect("Failed to load DBC");

    // Get an object to start reading the CSV
    let mut rdr = csv::Reader::from_path(path).expect("Failed to load CSV");

    // Make out vec to return
    let mut msgs: Vec<MsgOut> = Vec::new();

    // Iterate over each row of the CSV
    for res in rdr.records() {
        //
        // Get the row of data
        let record = res.unwrap();

        //
        // Parse for timestamp
        let time = record[0].parse::<usize>().unwrap_or(0);

        //
        // Parse for ID
        // This fixes a stupid fucking error with hex crate not taking odd lengths, then it fixes
        // the byteorder crate bug of a length less than 2, then handles wacky extended ID
        // TODO: Fix this mess
        let id: u32;

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

        //
        // Parse for the bytes
        let val = hex::decode(record[3].to_string()).unwrap_or(vec![0]);

        //
        // Decode and push onto vec
        msgs.push(parse_frame(&lib, CanFrameRaw { time, id, val }));
    }

    msgs
}

pub fn parse_frame(lib: &PgnLibrary, msg: CanFrameRaw) -> MsgOut {
    // Some holding vars
    let mut snames: Vec<String> = Vec::new();
    let mut sexplain: Vec<String> = Vec::new();
    let mut values: Vec<f32> = Vec::new();
    let mut units: Vec<String> = Vec::new();

    // Look up the ID, if we find it, return it, if not return default
    match lib.get_pgn(msg.id) {
        None => {
            println!("Failed to parse ID: {}", msg.id);
            MsgOut::default()
        }

        Some(pgn) => {
            // Run thru each signal
            for sig in pgn.spns.iter() {
                snames.push(sig.1.name.clone());
                sexplain.push(sig.1.description.clone());
                println!("{}", sig.1.parse_message(msg.val.as_slice()).unwrap());
                values.push(sig.1.parse_message(msg.val.as_slice()).unwrap());
                units.push(sig.1.units.clone());
            }

            MsgOut {
                time: msg.time,
                name: pgn.name_abbrev.clone(),
                snames,
                sexplain,
                values,
                units,
            }
        }
    }
}
