import engine

# engine.say_hello()
# print(f'Engine module: {dir(engine)}')
# print(f'MetaComponent type: {type(engine.MetaComponent)}')
# print(f'MetaComponent dir: {dir(engine.MetaComponent)}')

class Color():
    __metaclass__ = engine.MetaComponent   
    def __init__(self, r, g, b):
        print("calling init")

class Transform(metaclass=engine.MetaComponent):
    pass

print(f'Color dir: {dir(Color)}')
# print(f'Component type: {type(Component)}')
# print(dir(Component))
# print(dir(Color))
# print(f'Color type_id: {Color.__type_id__}')
# print(f'Transform type_id: {Transform.__type_id__}')

c = Color(1,2,3)
# print(dir(c))

