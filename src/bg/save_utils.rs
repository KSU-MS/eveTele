use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs;
use std::io::BufWriter;
use std::sync::Arc;

use super::MsgOut;

use mcap::records::MessageHeader;
use mcap::Channel;
use mcap::Schema;
use protobuf::reflect::FileDescriptor;
use protobuf::reflect::ReflectValueBox;

pub fn mcap_test() {
    // Get the file descriptor set
    let mut fdps = protobuf_parse::Parser::new()
        .pure()
        .includes(&["./"])
        .input("./test.proto")
        .parse_and_typecheck()
        .unwrap()
        .file_descriptors;

    // Define the schema to use
    let proto_schemin = Option::Some(Arc::new(Schema {
        name: "test.proto.Mmm".to_string(),
        encoding: "protobuf".to_string(),
        data: Cow::default(),
    }));

    let mut mcap =
        mcap::Writer::new(BufWriter::new(fs::File::create("test.mcap").unwrap())).unwrap();

    let test_channel = Channel {
        topic: String::from("AAA or something idk"),
        schema: proto_schemin,
        message_encoding: String::from("protobuf"),
        metadata: BTreeMap::default(),
    };

    let channel_id = mcap.add_channel(&test_channel).unwrap();

    mcap.write_to_known_channel(
        &MessageHeader {
            channel_id,
            sequence: 25,
            log_time: 6,
            publish_time: 24,
        },
        &[1, 2, 3],
    )
    .unwrap();

    mcap.write_to_known_channel(
        &MessageHeader {
            channel_id,
            sequence: 32,
            log_time: 23,
            publish_time: 25,
        },
        &[3, 4, 5],
    )
    .unwrap();

    mcap.finish().expect("Fucked up writing the MCAP file");
}

pub fn build_proto() {
    // Make an example proto bin
    let mut proto: String = "syntax = 'proto3';".to_string();
    proto += "\nmessage Mmm { int32 aaa = 1; }";

    // Write that to disk
    fs::write("./test.proto", proto).unwrap();

    let mut fdps = protobuf_parse::Parser::new()
        .pure()
        .includes(&["./"])
        .input("./test.proto")
        .parse_and_typecheck()
        .unwrap()
        .file_descriptors;

    // Make sure it only loaded the one example file descriptor
    assert_eq!(1, fdps.len());

    // Grab the one file descriptor
    let fdp = fdps.pop().unwrap();

    // Load that for parsing?
    let fd = FileDescriptor::new_dynamic(fdp, &[]).unwrap();

    // Now load the descriptor into something that can create a message
    let mmm_descriptor = fd.message_by_package_relative_name("Mmm").unwrap();

    // Create an empty message.
    let mut mmm = mmm_descriptor.new_instance();

    // Find the field.
    let aaa_field = mmm_descriptor.field_by_name("aaa").unwrap();

    // Set the field.
    aaa_field.set_singular_field(&mut *mmm, ReflectValueBox::I32(42));

    // Now serialize it to binary format.
    // field number = 1
    // wire_type = 0 (var int)
    // tag = (1 << 3) | 0 = 8
    // value = 42
    assert_eq!(&[8, 42], mmm.write_to_bytes_dyn().unwrap().as_slice());

    // Print it as text format.
    assert_eq!("aaa: 42", protobuf::text_format::print_to_string(&*mmm));

    println!("vry cool, no panics");
}

pub fn save_csv(path: &String, msgs: Vec<MsgOut>) {
    // Init a struct for writing utils
    let mut wtr = csv::Writer::from_path(path).unwrap();

    // The records are written just like our logger code
    wtr.write_record(&["time", "message", "label", "value", "unit"])
        .expect("Could not write header");

    // Iterates over every message
    for msg in msgs {
        // Iterates over every signal
        for i in 0..msg.snames.len() {
            // Writes out line
            wtr.write_record(&[
                msg.time.to_string(),
                msg.name.to_string(),
                msg.snames[i].to_string(),
                msg.values[i].to_string(),
                msg.units[i].to_string(),
            ])
            .expect("Could not write record");
        }
    }

    // Saves the file
    wtr.flush().expect("Failed to write CSV");
}
