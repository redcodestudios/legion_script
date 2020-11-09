#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include <stdio.h>
#include <structmember.h>
#include <string.h>

/* MetaObject is a component class created by a metaclass */
typedef struct {
    PyTypeObject base;
    unsigned long type_id;
} MetaObject;


/* Get the id of the component class, this id is used by legion to store the component */
static PyObject* Meta_id(MetaObject *self, PyObject *unused){
    return PyLong_FromLong(self->type_id);
}

/* Create a MetaObject (component class) setting a unique id */
static PyObject *Meta_new(PyObject *cls, PyObject *args, PyObject *kwargs) {
    MetaObject *component_class = PyType_Type.tp_new(cls, args, kwargs);

    /*** Id counter coming from Rust initialized with 0 ***/
    *ID_COUNTER += 1;
    component_class->type_id = *ID_COUNTER;
    fprintf(stderr, "CREATING COMPONENT\n");
    return component_class;
}

/* All methods of the class */
static PyMethodDef Meta_methods[] = {
    {"id", (PyCFunction) Meta_id, METH_NOARGS,
     PyDoc_STR("get type id")},
    {NULL},
};

/* Define the MetaComponentType */
static PyTypeObject MetaComponentType = {
    PyVarObject_HEAD_INIT(NULL, 0)
    .tp_name = "engine.MetaComponent",
    .tp_basicsize = sizeof(MetaObject),
    .tp_itemsize = 0,
    .tp_flags = Py_TPFLAGS_DEFAULT | Py_TPFLAGS_BASETYPE,
    .tp_new = (ternaryfunc) Meta_new,
    .tp_methods = Meta_methods,
};
