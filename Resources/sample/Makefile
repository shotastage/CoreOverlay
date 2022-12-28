.PHONY:
test-all:
	wasmer compile --singlepass ./program.wasm -o ./program.wasmu
	wasmer compile --cranelift ./program.wasm -o ./program.s

.PHONY:
clean:
	rm program.wasmu
