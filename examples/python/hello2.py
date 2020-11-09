import engine

print("Python: Script 2")

class Rotation(metaclass=engine.MetaComponent):
    def __init__(self):
        self.trans = 5

result = engine.query(Color)
print("Python: Query colors from Script 2")
for c in result:
    c.string()

print("--------------------")
