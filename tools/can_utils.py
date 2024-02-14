import cantools


def load_db():
    db = cantools.database.load_file("./ksu_dbc.dbc")
    print(db.messages)
