# GUI imports
import customtkinter
import tkinterDnD

from app.layout import basic_layout


def start_app():
    # Drag and drop support for the future?
    customtkinter.set_ctk_parent_class(tkinterDnD.Tk)

    # Set some theme stuffs
    customtkinter.set_appearance_mode(
        "dark"
    )  # Modes: "System" (standard), "Dark", "Light"
    customtkinter.set_default_color_theme(
        "green"
    )  # Themes: "blue" (standard), "green", "dark-blue"

    # Start the actual app class
    app = customtkinter.CTk()
    app.geometry("400x780")
    app.title("eveTele")

    # Load our base GUI state
    basic_layout(app)

    # Return so main.py can run the main event loop
    return app
