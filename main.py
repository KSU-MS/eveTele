# Main app things
from app.init import start_app
from app.layout import basic_layout

# Init the base app
app = start_app()

# Load our GUI elements
basic_layout(app)

# Run the main event loop
app.mainloop()
