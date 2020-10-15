#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include <stdio.h>
#include <structmember.h>
#include <string.h>

#include "legion.c"


typedef struct {
    PyTypeObject base;
    unsigned long type_id;
} MetaObject;


static PyObject* Meta_id(MetaObject *self, PyObject *unused){
    return PyLong_FromLong(self->type_id);
}


static PyMethodDef Meta_methods[] = {
    {"id", (PyCFunction) Meta_id, METH_NOARGS,
     PyDoc_STR("get type id")},
    {NULL},
};


static PyObject *Meta_new(PyObject *cls, PyObject *args, PyObject *kwargs) {
    MetaObject *component_class = PyType_Type.tp_new(cls, args, kwargs);
    
    /*** Change to actual get_id from RUST ***/
    *counter += 1;
    component_class->type_id = *counter;
    fprintf(stderr, "CREATING COMPONENT\n"); 
    return component_class;
}


static PyTypeObject MetaComponentType = {
    PyVarObject_HEAD_INIT(NULL, 0)
    .tp_name = "engine.MetaComponent",
    .tp_basicsize = sizeof(MetaObject),
    .tp_itemsize = 0,
    .tp_flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .tp_new = (ternaryfunc) Meta_new,
    .tp_methods = Meta_methods,
};


/** SAY HELLO FUNCTION **/
static PyObject* say_hello(PyObject *self, PyObject *args) {
    PyObject *p;
    printf("Hello from python using C `printf`!\n");
    
    return PY_NONE;
}


static PyMethodDef EngineMethods[] = {
    {"say_hello", say_hello, METH_VARARGS, "Say hello from python."},
    {"new_entity", new_entity, METH_VARARGS, "Create entity."},
    {"query", query, METH_VARARGS, "Query component."},
    {NULL, NULL, 0, NULL}
};

/** MODULE ENGINE INIT **/
static PyModuleDef EngineModule = {
    PyModuleDef_HEAD_INIT, "engine", NULL, -1, EngineMethods,
    NULL, NULL, NULL, NULL
};


static PyObject* PyInit_engine(void) {
    PyObject* module = PyModule_Create(&EngineModule);
    MetaComponentType.tp_base = &PyType_Type;
    
    if (PyType_Ready(&MetaComponentType) < 0)
        printf("error\n");

    Py_INCREF(&MetaComponentType);
    if(PyModule_AddObject(module, "MetaComponent", &MetaComponentType) < 0) {
        printf("Error adding component\n");
    }
    
    return module;
}


/** RUN SCRIPT **/
void C_RUN_PYSCRIPT(World* world, const char* script, unsigned long *component_id) {
    
    // SET RUST ID
    counter = component_id;
    
    // SET WORLD
    //@TODO: create world singleton in C
    WORLD = world;
    
    wchar_t *program = Py_DecodeLocale(script, NULL);
    if (program == NULL) {
        fprintf(stderr, "Fatal error: cannot decode arg[0]\n");
        exit(1);
    }
    Py_SetProgramName(program);
   
    PyImport_AppendInittab("engine", &PyInit_engine);
    Py_Initialize();

    FILE *script_f = fopen(script, "r");
    PyRun_SimpleFile(script_f, script);

    fprintf(stderr, "FINALIZING\n");
    fprintf(stderr, "%s\n", script);

    if(strcmp(script, "examples/python/hello2.py") == 0) {
        /* Py_DECREF(py_obj_ptr); */ 
        fprintf(stderr, "is hello2\n");
    }
   
    /* ------ YOU WILL BE TRAPPED INTO MEMORY LEAKS FOREVER ------*/
    /* if (Py_FinalizeEx() < 0) { */
    /*     exit(120); */
    /* } */
    PyMem_RawFree(program);
}

