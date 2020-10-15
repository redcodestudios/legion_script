import engine

# engine.say_hello()
# print(f'Engine module: {dir(engine)}')
# print(f'MetaComponent type: {type(engine.MetaComponent)}')
# print(f'MetaComponent dir: {dir(engine.MetaComponent)}')
# print(engine.MetaComponent.id())
class Color(metaclass=engine.MetaComponent):
    def __init__(self, r, g, b):
        self.r, self.g, self.b = (r,g,b)
    def string(self):
        print(f"Color values: {self.r} {self.g} {self.b}")

class Transform(metaclass=engine.MetaComponent):
    def __init__(self, x, y):
        self.x, self.y = (x, y)

# print(f'Color dir: {dir(Color)}')
# print(f'Component type: {type(Color)}')
print(f'Color type_id: {Color.id()}')
print(f'Transform type_id: {Transform.id()}')
# print(dir(Component))
# print(dir(Color))
# print(f'Color type_id: {Color.__type_id__}')
# print(f'Transform type_id: {Transform.__type_id__}')
e = engine.new_entity(Color(55,2,3))
c = engine.query()
print(f"C {c}")
c.string()

# class Rotation(metaclass=engine.MetaComponent):
#     pass
# # print(dir(c))

# c = engine.query()
# print(f"C {c}")

