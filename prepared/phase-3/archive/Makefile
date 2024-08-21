.PHONY: build bindings compile clean

build: compile
	wasm-tools component embed ./wit archive.module.wasm --output archive.embed.wasm
	wasm-tools component new archive.embed.wasm -o archive.wasm --adapt adapters/tier1/wasi_snapshot_preview1.wasm

bindings:
	wit-bindgen tiny-go --world archive --out-dir archive ./wit

compile: bindings
	tinygo build -target=wasi -tags=purego -o archive.module.wasm main.go

clean:
	rm -rf archive
	rm *.wasm
