package main

/*
#include <stdlib.h>
*/
import "C"

import (
	htmltemplate "html/template"
	"regexp"
	texttemplate "text/template"
	"time"
	"unsafe"
)

//export GoFree
func GoFree(ptr *C.char) {
	C.free(unsafe.Pointer(ptr))
}

//export HtmlTemplateNewParse
func HtmlTemplateNewParse(expr *C.char) *C.char {
	if _, err := htmltemplate.New("").Parse(C.GoString(expr)); err != nil {
		return C.CString(err.Error())
	}
	return C.CString("")
}

//export RegexpCompile
func RegexpCompile(expr *C.char) *C.char {
	if _, err := regexp.Compile(C.GoString(expr)); err != nil {
		return C.CString(err.Error())
	}
	return C.CString("")
}

//export TextTemplateNewParse
func TextTemplateNewParse(expr *C.char) *C.char {
	if _, err := texttemplate.New("").Parse(C.GoString(expr)); err != nil {
		return C.CString(err.Error())
	}
	return C.CString("")
}

//export TimeParse
func TimeParse(expr *C.char) *C.char {
	if _, err := time.Parse(C.GoString(expr), C.GoString(expr)); err != nil {
		return C.CString(err.Error())
	}
	return C.CString("")
}

func main() {}
