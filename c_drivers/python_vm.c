#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <stdio.h>
#include <structmember.h>
#define PY_NONE Py_BuildValue("")

#include <time.h>

/* ########## FIRST VERSION ########## */
/* void C_run_python_file(const char* script) { */
/*     printf("`python_vm.c`: Starting python\n"); */
/*     wchar_t *program = Py_DecodeLocale("", NULL); */
/*     if (program == NULL) { */
/*         fprintf(stderr, "Fatal error: cannot decode arg[0]\n"); */
/*         exit(1); */
/*     } */
/*     Py_SetProgramName(program); */
   
/*     // the python vm should be initialized before this function */
/*     Py_Initialize(); */
    
/*     FILE *script_f = fopen(script, "r"); */
/*     PyRun_SimpleFile(script_f, script); */

/*     if(Py_FinalizeEx < 0) { */
/*         exit(120); */
/*     } */
/*     PyMem_RawFree(program); */
/*     printf("`python_vm.c`: Finalizing python\n"); */
/* } */

typedef struct {
    PyObject_HEAD
    PyObject* type_id;
} MetaComponentObject;

static PyObject* *MetaComponent_call(PyObject *cls, PyObject *args, PyObject *kwargs) {
    fprintf(stderr, "Creating class instance\n");
    
    PyObject *obj = PyObject_CallMethodObjArgs(cls, PyUnicode_FromString("__new__"), cls, args, kwargs);
    /* MetaComponentObject *obj = PyType_GenericNew(self, args, kwargs); */
    fprintf(stderr, "LOG\n");
    /* obj->type_id = PyLong_FromLong(random()); */
    
    PyObject *class_instance = PyObject_CallMethodObjArgs(obj, PyUnicode_FromString("__init__"), args, kwargs);

    fprintf(stderr, "LOG OUT\n");
    return class_instance;
}

static PyObject *MetaComponent_new(MetaComponentObject *cls, PyObject *args, PyObject *kwargs) {
    MetaComponentObject *obj = PyType_GenericNew(cls, args, kwargs);
    fprintf(stderr, "new chamado\n");
    obj->type_id = PyLong_FromLong(random());

    return obj;
}

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

static PyMemberDef MetaComponent_members[] = {
    {"__type_id__", T_OBJECT_EX, offsetof(MetaComponentObject, type_id), 0, "type id"},
    {NULL}
};

static PyTypeObject MetaComponentType = {
    PyVarObject_HEAD_INIT(NULL, 0)
    .tp_name = "engine.MetaComponent",
    .tp_basicsize = sizeof(MetaComponentObject),
    .tp_itemsize = 0,
    .tp_flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    /* .tp_new = PyType_GenericNew, */
    .tp_new = (ternaryfunc) MetaComponent_new,
    /* .tp_call = (ternaryfunc) MetaComponent_call, */
    .tp_members = MetaComponent_members,
};

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

