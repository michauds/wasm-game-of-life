

all: build update_module web_dev

build:
	wasm-pack build

update_module:
	cd www && npm install

web_dev:
	cd www && npm run start