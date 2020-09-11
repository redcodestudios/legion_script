import engine

# engine.say_hello()
# print(f'Engine module: {dir(engine)}')
# print(f'MetaComponent type: {type(engine.MetaComponent)}')
# print(f'MetaComponent dir: {dir(engine.MetaComponent)}')
# print(engine.MetaComponent.id())
class Color(metaclass=engine.MetaComponent):
    def __init__(self, r, g, b):
        self.r, self.g, self.b = (r,g,b)

class Transform(metaclass=engine.MetaComponent):
    pass

# print(f'Color dir: {dir(Color)}')
# print(f'Component type: {type(Color)}')
print(f'Color type_id: {Color.id()}')
print(f'Transform type_id: {Transform.id()}')
# print(dir(Component))
# print(dir(Color))
# print(f'Color type_id: {Color.__type_id__}')
# print(f'Transform type_id: {Transform.__type_id__}')

c = Color(1,2,3)
print(c.r)
# print(dir(c))

