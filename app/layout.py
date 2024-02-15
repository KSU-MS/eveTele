import customtkinter

from tools.dbc_utils import load_db, write_proto


def basic_layout(app: customtkinter.CTk):
    # Define the frame to hold everything
    frame = customtkinter.CTkFrame(master=app)
    frame.pack(pady=0, padx=0, fill="both", expand=True)

    # Make a button to load the DBC
    dbc_button = customtkinter.CTkButton(
        master=frame,
        command=parse_db,
        text="DBC parse test",
    )
    dbc_button.pack(pady=10, padx=10)

    proto_test = customtkinter.CTkButton(
        master=frame,
        command=make_proto,
        text="proto gen test",
    )
    proto_test.pack(pady=10, padx=10)

    return app


def parse_db() -> None:
    load_db(customtkinter.filedialog.askopenfilename())
    return None


def make_proto() -> None:
    write_proto(customtkinter.filedialog.askopenfilename())
    return None
