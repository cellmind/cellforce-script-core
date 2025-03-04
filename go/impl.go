package main

import (
	"reflect"
	"sync"
	"unsafe"

	"github.com/traefik/yaegi/interp"
	"github.com/traefik/yaegi/stdlib"
)

var (
	interpMutex  sync.Mutex
	interpreters []*interp.Interpreter
	functions    []interface{}
	offset       int
)

type GoScript struct {
}

func init() {
	GoScriptCallImpl = GoScript{}
	interpreters = make([]*interp.Interpreter, 0)
	functions = make([]interface{}, 0)
	offset = 0
}

func (GoScript) new_interpreter(req *NewGoScriptInterpreterRequest) NewGoScriptInterpreterResponse {
	interpreter := interp.New(interp.Options{})
	restrictedSymbols := make(map[string]map[string]reflect.Value)
	allowedPackages := []string{
		"fmt/fmt",
		"strings/strings",
		"math/math",
		"time/time",
		"os/os",
	}
	for _, pkg := range allowedPackages {
		if symbols, ok := stdlib.Symbols[pkg]; ok {
			restrictedSymbols[pkg] = symbols
		}
	}
	interpreter.Use(restrictedSymbols)
	_, err := interpreter.Eval(req.script)
	if err != nil {
		return NewGoScriptInterpreterResponse{error: err.Error()}
	}

	function, err := interpreter.Eval(req.function)
	if err != nil {
		return NewGoScriptInterpreterResponse{error: err.Error()}
	}
	functions = append(functions, function.Interface())
	interpreters = append(interpreters, interpreter)
	current_offset := offset
	offset++
	return NewGoScriptInterpreterResponse{error: "", ptr_offset: int32(current_offset)}
}

func (GoScript) free_interpreter(req *FreeGoScriptInterpreterRequest) FreeGoScriptInterpreterResponse {
	interpreters[req.ptr_offset] = nil
	return FreeGoScriptInterpreterResponse{error: ""}
}

func StringToBytes(s string) []byte {
	return unsafe.Slice(unsafe.StringData(s), len(s))
}

func (GoScript) map_in_str_out_str(req *MapInStrOutStrRequest) MapInStrOutStrResponse {
	fn_call, ok := functions[req.ptr_offset].(func(string) string)
	if !ok {
		return MapInStrOutStrResponse{error: "function not converted to func(string) string"}
	}
	result := fn_call(req.value)
	return MapInStrOutStrResponse{error: "", value: result}
}
