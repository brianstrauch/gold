package main

/*
#include <stdlib.h>
*/
import "C"

import (
	"regexp"
	"unsafe"
)

//export RegexpCompile
func RegexpCompile(expr *C.char) *C.char {
	if _, err := regexp.Compile(C.GoString(expr)); err != nil {
		return C.CString(err.Error())
	}
	return C.CString("")
}

//export GoFree
func GoFree(ptr *C.char) {
	C.free(unsafe.Pointer(ptr))
}

func main() {}
