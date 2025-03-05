package main

import (
	"fmt"
	"reflect"
	"sync"

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

func (GoScript) new_interpreter(req *NewGoScriptInterpreterRequest) (resp NewGoScriptInterpreterResponse) {
	defer func() {
		if r := recover(); r != nil {
			resp = NewGoScriptInterpreterResponse{error: fmt.Sprintf("fail to create interpreter: %v", r)}
		}
	}()

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

func (GoScript) free_interpreter(req *FreeGoScriptInterpreterRequest) (resp FreeGoScriptInterpreterResponse) {
	defer func() {
		if r := recover(); r != nil {
			resp = FreeGoScriptInterpreterResponse{error: fmt.Sprintf("fail to free interpreter: %v", r)}
		}
	}()
	interpreters[req.ptr_offset] = nil
	return FreeGoScriptInterpreterResponse{error: ""}
}

func (GoScript) map_in_str_out_str(req *MapInStrOutStrRequest) (resp MapInStrOutStrResponse) {
	defer func() {
		if r := recover(); r != nil {
			resp = MapInStrOutStrResponse{error: fmt.Sprintf("fail to map in str out str: %v", r)}
		}
	}()

	if req.ptr_offset < 0 || req.ptr_offset >= int32(len(functions)) {
		return MapInStrOutStrResponse{error: "invalid pointer offset"}
	}
	fn_call, ok := functions[req.ptr_offset].(func(string) string)
	if !ok {
		return MapInStrOutStrResponse{error: "function not converted to func(string) string"}
	}
	result := fn_call(req.value)
	return MapInStrOutStrResponse{error: "", value: result}
}
