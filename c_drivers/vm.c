#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include <stdio.h>
#include <structmember.h>
#include <string.h>

#include "engine.h"

/** RUN SCRIPT **/
void C_RUN_PYSCRIPT(World* world, const char* script, unsigned long *component_id) {
    
    // SET RUST ID
    ID_COUNTER = component_id;
    
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

    /* ------ YOU WILL BE TRAPPED INTO MEMORY LEAKS FOREVER ------*/
    /* if (Py_FinalizeEx() < 0) { */
    /*     exit(120); */
    /* } */
    PyMem_RawFree(program);
}

