#define PY_SSIZE_T_CLEAN
#include <Python.h>

#define PY_NONE Py_BuildValue("")

typedef struct World World;
typedef struct ComponentData ComponentData;

// LEGION INTERFACE
extern void legion_create_entity(World* world, ComponentData* component_data);
extern ComponentData* legion_create_component_data(int* component_types, int number_components, void** components);
typedef struct ComponentsWrapper{
    void** data;
    int len;
}ComponentsWrapper;
extern ComponentsWrapper* get_component(World* world, int id);

static unsigned long* ID_COUNTER = 0;
static World* WORLD = NULL;

static World* get_world(){
    return WORLD;
}

static void set_world(World* w){
    WORLD = w;
}

unsigned long get_component_id_by_class(PyObject* class_ptr) {
    PyObject* id = PyObject_CallMethodObjArgs(class_ptr, PyUnicode_FromString("id"), NULL);
    if (id == NULL) {
        fprintf(stderr, "ID NOT FOUND \n");
        return -1;
    }

    return PyLong_AsLong(id);
}

unsigned long get_component_id_by_instance(PyObject* component_ptr) {
    PyObject* component_class = PyObject_GetAttrString(component_ptr, "__class__");
    if(component_class == NULL) {
        fprintf(stderr, "CLASS NOT FOUND\n");
        return NULL;
    }

    return get_component_id_by_class(component_class);
}

static PyObject* new_entity(PyObject *self, PyObject *args) {
    Py_ssize_t args_size = PyTuple_Size(args);

    int* components_types = malloc(sizeof(int) * args_size);
    void** components = malloc(sizeof(void*) * args_size);

    PyObject *temp;
    for(Py_ssize_t i=0; i<args_size; i++) {
        temp = PyTuple_GetItem(args, i);
        Py_INCREF(temp);

        unsigned long component_id = get_component_id_by_instance(temp); 
        components_types[i] = component_id;
        components[i] = (void*) temp;
    }
    
    ComponentData* comp_data = legion_create_component_data(components_types, args_size, components);
    legion_create_entity(get_world(), comp_data);

    return PY_NONE;
}

static PyObject* query(PyObject *self, PyObject *args) {
    Py_ssize_t args_size = PyTuple_Size(args);

    PyObject* temp;
    ComponentsWrapper* new_components = NULL;

    PyObject* query_result = PyList_New(0);

    for(Py_ssize_t i=0; i<args_size; i++) {
        temp = PyTuple_GetItem(args, i);
        Py_INCREF(temp);
        
        unsigned long id = get_component_id_by_class(temp);
        fprintf(stderr, "meta id %d\n", id);
        new_components = get_component(get_world(), id);

        // First values on new_components is NULL, vec starts on [2]; thats why len-2, and i=2;
        fprintf(stderr, "C received [%d] components.\n", new_components->len - 2);
        for(int i = 2; i<new_components->len; i++){
            // fprintf(stderr, "%p %d\n", new_components->data[i], i);
            if(PyList_Append(query_result, (PyObject*) new_components->data[i]) == -1) {
                fprintf(stderr, "Failed to insert in query result list.\n");
            }
        }
    }

    return Py_BuildValue("O", query_result);
}
