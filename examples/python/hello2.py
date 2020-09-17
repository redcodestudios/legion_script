import engine


class Rotation(metaclass=engine.MetaComponent):
    def __init__(self):
        self.trans = 5

color = engine.query()
print("Script 2")
print(f"Color r from another script {color.r}")

