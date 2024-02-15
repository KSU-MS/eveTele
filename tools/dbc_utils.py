import cantools.database
from cantools.database import can, conversion


# Basic function to test DBC loading or something idk
def load_db(path: str):
    db = cantools.database.load_file(path)
    print(db.messages)


# Slightly less basic function to parse a DBC in a proto file for foxglove things
def write_proto(path: str):
    # load the DBC
    db = cantools.database.load_file(path)

    # Open the file and start adding things
    with open("auto_dbc.proto", "w+") as proto_file:
        # Write a little header
        proto_file.write('syntax = "proto3";\n\n')

        # Write the actual data
        for msg in db.messages:
            proto_file = append_proto_from_message(proto_file, msg)


#
##
### Helper functions for generating protos
##
# Sanatizes the names of the CAN messages to not mess with the proto
def create_field_name(name: str) -> str:
    mv_text = name.replace(" ", "_")
    mv_text.replace("(", "")
    mv_text.replace(")", "")

    return mv_text


# Makes a new proto message out of a CAN message
def append_proto_from_message(file, can_msg: can.message.Message):
    # if the msg has a conversion, we know that the value with be a float
    msgname = can_msg.name
    # type and then name
    file.write("message " + msgname.lower() + " {\n")
    line_index = 0

    # Iterate over every signal in a given message
    for sig in can_msg.signals:
        line_index += 1

        # TODO: Perhaps clean up this and add more than just i32 for ints, also add the units flag?
        if sig.is_float or (
            type(sig.conversion)
            is not type(conversion.IdentityConversion(is_float=False))
            and not type(
                conversion.NamedSignalConversion(
                    choices={}, scale=0, offset=0, is_float=False
                )
            )
        ):
            line = (
                "    float "
                + create_field_name(sig.name)
                + " = "
                + str(line_index)
                + ";"
            )
        elif sig.choices is not None:
            line = (
                "    string "
                + create_field_name(sig.name)
                + " = "
                + str(line_index)
                + ";"
            )
        elif sig.length == 1:
            line = (
                "    bool "
                + create_field_name(sig.name)
                + " = "
                + str(line_index)
                + ";"
            )
        elif sig.length > 1:
            line = (
                "    int32 "
                + create_field_name(sig.name)
                + " = "
                + str(line_index)
                + ";"
            )
        else:
            # TODO: Need to add better error handeling for this, could leave line equal to some wack shit and fuck up the proto generation
            print("ERROR")
            line = ""

        # Write out the message with its defs
        file.write(line + "\n")
    file.write("}\n\n")
    return file
