.PHONY:
setup:
	brew update
#	brew install emscripten
	rustup target add wasm32-wasi

.PHONY:
build:
#	emcc -O3 -s WASM=1 -s SIDE_MODULE=1 -s EXPORTED_FUNCTIONS="['_main']" -o program.wasm program.c
	rustc --target wasm32-wasi hello-rust.rs
#	wasmer compile --llvm -o hello-rust.lvmpc hello-rust.wasm
	wasmer compile --singlepass ./hello-rust.wasm -o ./hello-rust.wasmu

.PHONY:
test-all:
	wasmer compile --singlepass ./program.wasm -o ./program.wasmu
	wasmer compile --cranelift ./program.wasm -o ./program.s

.PHONY:
clean:
	@rm hello-rust.wasm
#	rm hello-rust.lvmpc
