#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <stdio.h>

void C_run_python_file(const char* script) {
    printf("starting python\n");
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
}
