
APP=gba

build:
	xargo rustc -- -C link-arg=-emain

size: build
	arm-none-eabi-size target/thumbv7m-none-eabi/debug/$(APP)

dump: build
	arm-none-eabi-objdump -CD target/thumbv7m-none-eabi/debug/$(APP)

nm: build
	arm-none-eabi-nm -C target/thumbv7m-none-eabi/debug/$(APP)

clean:
	rm -rf target/thumbv7m-none-eabi/debug/$(APP)

