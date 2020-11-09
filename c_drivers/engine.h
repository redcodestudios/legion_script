#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include <stdio.h>
#include <structmember.h>
#include <string.h>

#include "legion_api.h"
#include "meta.h"

/* Methods of the engine module */
static PyMethodDef EngineMethods[] = {
    {"new_entity", new_entity, METH_VARARGS, "Create entity."},
    {"query", query, METH_VARARGS, "Query component."},
    {NULL, NULL, 0, NULL}
};

/** Define engine module **/
static PyModuleDef EngineModule = {
    PyModuleDef_HEAD_INIT, "engine", NULL, -1, EngineMethods,
    NULL, NULL, NULL, NULL
};

/* Init module */
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
