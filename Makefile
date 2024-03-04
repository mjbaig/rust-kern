all: clean build flatten

clean:
	rm -rf ./kernel7.img
	cargo clean

build:
	cargo rustc -- -C link-arg=--script=./linker.ld

flatten:
	arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/rust-kern ./kernel7.img