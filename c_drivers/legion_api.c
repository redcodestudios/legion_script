#define PY_SSIZE_T_CLEAN
#include <Python.h>
#define PY_NONE Py_BuildValue("")

typedef struct World World;
typedef struct CommandBuffer CommandBuffer;
typedef struct ComponentData ComponentData;

extern void legion_create_entity(World* world, ComponentData* component_data);
extern void** component_array(void* component);
extern ComponentData* legion_create_component_data(int* component_types, int number_components, void** components);
extern World* legion_world_new();
extern void* get_component(World* world, int id);

static unsigned long* counter = NULL;
static World* WORLD = NULL;
static PyObject* py_obj_ptr = NULL;

static World* get_world(){
    return WORLD;
}
static void set_world(World* w){
    WORLD = w;
}

static PyObject* get_pyobject(){
    return py_obj_ptr;
}
static void set_pyobject(PyObject* p){
    py_obj_ptr = p;
}

static PyObject* new_entity(PyObject *self, PyObject *args) {
    int component_types[2] = {666, 777};
    void* components[2];
    
    Py_ssize_t args_size = PyTuple_Size(args);
    fprintf(stderr, "NUMBER DE ARGS %d\n", (int) args_size);

    PyObject *temp;

    /* components = malloc(sizeof(PyObject*)); */
    fprintf(stderr, "size of pyobject* %d\n", sizeof(PyObject*));
    fprintf(stderr, "size of temp %d\n", sizeof(*temp));

    for(Py_ssize_t i=0; i<args_size; i++) {
        temp = PyTuple_GetItem(args, i);
        
        set_pyobject(temp);
        Py_INCREF(temp);

        components[i] = (void*) temp;
        /* PyObject_CallMethodObjArgs(temp, PyUnicode_FromString("string"), NULL); */
    }
    
    fprintf(stderr, "component in position 0 %p\n", components[0]);
    fprintf(stderr, "component in position 1 %p\n", components[1]);
    fprintf(stderr, "component array %p\n", components);
    
    ComponentData* comp_data = legion_create_component_data(component_types, 2, components);
    legion_create_entity(get_world(), comp_data);

    return PY_NONE;
}

static PyObject* query(PyObject *self, PyObject *args) {
    void** new_component = get_component(get_world(), 666);

    PyObject* query_result = Py_BuildValue("O", (PyObject*) new_component); 
    
    if(query_result == NULL) {
        fprintf(stderr, "query_result null\n");
    }
    return query_result;
}
