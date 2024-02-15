# GUI imports
import customtkinter


def start_app():
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

    # Return so main.py can run the main event loop
    return app
