#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <stdio.h>

#define PY_NONE Py_BuildValue("")


/* ########## FIRST VERSION ########## */
void C_run_python_file(const char* script) {
    printf("`python_vm.c`: Starting python\n");
    wchar_t *program = Py_DecodeLocale("", NULL);
    if (program == NULL) {
        fprintf(stderr, "Fatal error: cannot decode arg[0]\n");
        exit(1);
    }
    Py_SetProgramName(program);
   
    // the python vm should be initialized before this function
    Py_Initialize();
    
    FILE *script_f = fopen(script, "r");
    PyRun_SimpleFile(script_f, script);

    if(Py_FinalizeEx < 0) {
        exit(120);
    }
    PyMem_RawFree(program);
    printf("`python_vm.c`: Finalizing python\n");
}

typedef struct {
    PyObject_HEAD
} MetaComponentObject;

/* static PyObject *MetaComponent_call(MetaComponentObject *type, PyObject *args, PyObject *kwargs) { */
/*     PyObject *obj = PyType_GenericNew((PyTypeObject *) type, args, kwargs); */

/*     PyObject *dict = MetaComponentType->tp_dict; */
/*     PyObject *type_id = PyLong_FromLong(666); */
/*     PyDict_SetItemString(dict, "__type_id__", type_id); */

/*     return obj; */
/* } */

/* static PyTypeObject MetaComponentType = { */
/*     PyVarObject_HEAD_INIT(NULL, 0) */
/*     .tp_name = "engine.MetaComponent", */
/*     .tp_basicsize = sizeof(MetaComponentObject), */
/*     .tp_itemsize = 0, */
/*     .tp_flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE, */
/*     .tp_new = PyType_GenericNew, */
/*     .tp_call = (ternaryfunc) MetaComponent_call, */
/* }; */

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

static PyModuleDef EngineModule = {
    PyModuleDef_HEAD_INIT, "engine", NULL, -1, EngineMethods,
    NULL, NULL, NULL, NULL
};

static PyObject*
PyInit_engine(void)
{
    return PyModule_Create(&EngineModule);
}

void C_RUN_PYSCRIPT(const char* script) {
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

