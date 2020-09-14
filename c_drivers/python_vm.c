#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include <stdio.h>
#include <structmember.h>
#include <time.h>

#define PY_NONE Py_BuildValue("")

static unsigned long *counter = NULL;

/* static unsigned long get_counter() { */
/*     return *counter; */
/* } */

/* void set_counter(unsigned long id) { */
/*    *counter = id; */ 
/* } */


typedef struct {
    PyTypeObject base;
    unsigned long type_id;
} MetaObject;

static PyObject *
Meta_id(MetaObject *self, PyObject *unused)
{
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
    {"say_hello", say_hello, METH_VARARGS,
     "Say hello from python."},
    {NULL, NULL, 0, NULL}
};


/** MODULE ENGINE INIT **/
static PyModuleDef EngineModule = {
    PyModuleDef_HEAD_INIT, "engine", NULL, -1, EngineMethods,
    NULL, NULL, NULL, NULL
};

static PyObject*
PyInit_engine(void)
{
    PyObject* module = PyModule_Create(&EngineModule);
    /* Py_INCREF(&PyType_Type); */
    MetaComponentType.tp_base = &PyType_Type;
    
    if (PyType_Ready(&MetaComponentType) < 0)
        printf("error\n");
        /* return; */

    Py_INCREF(&MetaComponentType);
    if(PyModule_AddObject(module, "MetaComponent", &MetaComponentType) < 0) {
        printf("Error adding component\n");
    }

    return module;
}

/** RUN SCRIPT **/
void C_RUN_PYSCRIPT(const char* script, unsigned long *component_id) {
    
    // SET RUST ID
    counter = component_id;

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

    if (Py_FinalizeEx() < 0) {
        exit(120);
    }
    PyMem_RawFree(program);
}

