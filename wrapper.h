// --- Core Stackless and Python Definitions ---
#define STACKLESS

// Main header that pulls in most of the basics.
#include "Python.h"


// --- Standard Python Object Definitions ---
// These headers define the structs for Python's built-in types.
#include "object.h"         // PyObject, PyVarObject, PyTypeObject, PyBufferProcs
#include "objimpl.h"        // Memory allocation macros for objects
#include "intobject.h"      // PyIntObject
#include "boolobject.h"     // PyBoolObject is defined here (usually as a PyIntObject)
#include "longobject.h"     // PyLongObject
#include "floatobject.h"    // PyFloatObject
#include "complexobject.h"  // PyComplexObject, Py_complex
#include "stringobject.h"   // PyStringObject
#include "unicodeobject.h"  // PyUnicodeObject
#include "tupleobject.h"    // PyTupleObject
#include "listobject.h"     // PyListObject
#include "dictobject.h"     // PyDictObject, PyDictEntry
#include "setobject.h"      // PySetObject, setentry
#include "moduleobject.h"   // PyModuleObject


// --- Method and Member Definitions ---
// These define structures used within PyTypeObject, like method tables.
#include "descrobject.h"    // PyMethodDef, PyGetSetDef
#include "structmember.h"   // PyMemberDef and associated macros
#include "methodobject.h"   // PyCFunctionObject


// --- Execution State and Frame Definitions ---
// Structs related to the interpreter, threads, and code execution.
#include "pystate.h"        // PyInterpreterState
#include "pythread.h"       // PyThreadState
#include "frameobject.h"    // PyFrameObject, PyTryBlock
#include "cobject.h"

#include "stackless.h"

// Defines _slp_tasklet (the C struct for PyTaskletObject) and PyStacklessState (_sts)
#include "core/stackless_structs.h"

// Defines PyTaskletObject
#include "module/flextype.h"
#include "module/taskletobject.h"


// Defines _slp_cstack
#include "core/cframeobject.h"


