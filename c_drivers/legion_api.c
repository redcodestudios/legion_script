#define PY_SSIZE_T_CLEAN
#include <Python.h>
#define PY_NONE Py_BuildValue("")

typedef struct World World;
typedef struct CommandBuffer CommandBuffer;
typedef struct ComponentData ComponentData;

extern void legion_create_entity(World* world, ComponentData* component_data);
extern void rust_print_func(ComponentData* component_data);
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
    int component_types[1] = {666};
    void* components = NULL;
    
    Py_ssize_t args_size = PyTuple_Size(args);
    fprintf(stderr, "NUMBER DE ARGS %d\n", (int) args_size);

    PyObject *temp;

    components = malloc(sizeof(PyObject*));
    fprintf(stderr, "size of pyobject* %d\n", sizeof(PyObject*));
    fprintf(stderr, "size of temp %d\n", sizeof(*temp));

    // This breaks if user pass more than one argument to function
    for(Py_ssize_t i=0; i<args_size; i++) {
        temp = PyTuple_GetItem(args, i);
        
        set_pyobject(temp);
        Py_INCREF(temp);

        components = temp;
        PyObject_CallMethodObjArgs(temp, PyUnicode_FromString("string"), NULL);
        //  PyObject* class = PyObject_GetAttrString(temp, "__class__");
        /* if(class == NULL) { */
        /*     fprintf(stderr, "CLASS NOT FOUND\n"); */
        /* } */

        /* PyObject* id = PyObject_CallMethodObjArgs(class, PyUnicode_FromString("id"), NULL); */
        /* if (id == NULL) { */
        /*     fprintf(stderr, "ID NOT FOUND \n"); */
        /* } */
        /* Py_DECREF(temp); */
        /* Py_DECREF(id); */
        /* Py_DECREF(class); */
        /* fprintf(stderr, "TYPE_ID: %ld\n", PyLong_AsLong(id)); */
        /* PyErr_Print(); */
    }
    
    fprintf(stderr, "components pointer %p\n", temp);
    
    ComponentData* comp_data = legion_create_component_data(component_types, 1, temp);
    rust_print_func(comp_data);
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