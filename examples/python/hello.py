import engine

class Color(metaclass=engine.MetaComponent):
    def __init__(self, r, g, b):
        self.r, self.g, self.b = (r,g,b)
    def string(self):
        print(f"Color RGB: {self.r} {self.g} {self.b}")

class Transform(metaclass=engine.MetaComponent):
    def __init__(self, x, y):
        self.x, self.y = (x, y)

print(f'Color type_id: {Color.id()}')
print(f'Transform type_id: {Transform.id()}')

engine.new_entity(Color(55,2,3), Transform(10, 20))
engine.new_entity(Color(6, 6, 6))
engine.new_entity(Color(666, 666, 666))

result = engine.query(Color)
for c in result:
    c.string()


