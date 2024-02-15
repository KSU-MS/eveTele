import customtkinter

from tools.can_utils import load_db


def basic_layout(app):
    # Define the frame to hold everything
    frame_1 = customtkinter.CTkFrame(master=app)
    frame_1.pack(pady=20, padx=60, fill="both", expand=True)

    # Make a button to load the DBC
    button_1 = customtkinter.CTkButton(
        master=frame_1,
        command=load_db,
        text="DBC load test",
    )
    button_1.pack(pady=10, padx=10)
