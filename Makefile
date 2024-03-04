all: clean compile flatten package

clean:
	rm -rf ./kernel7.img
	rm -rf build
	cargo clean
	mkdir build

compile:
	cargo rustc -- -C link-arg=--script=./linker.ld

flatten:
	arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/rust-kern ./kernel7.img

package:
	cp rpi/* build
	cp ./kernel7.img build