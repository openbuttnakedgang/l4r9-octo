
dev:
	cargo build
o:
	Ozone $(CURDIR)/scripts/ozone_dev.jdebug &

or: 
	Ozone $(CURDIR)/scripts/ozone_rel.jdebug &

fresh f:
	cargo build --features "m4mon8_cube_rs/refresh"

check c:
	cargo check

release r:
	cargo build --release

prod p:
	cargo build --release --features "prod"

bin:
	#$(OBJCOPY) -Obinary $(FILENAME) $(FILENAME).bin
	#cargo objcopy --bin m3mon8mini --release --features "prod, bootloader" -- -Obinary ./m3mon8mini.bin
	arm-none-eabi-objcopy -Obinary target/thumbv7m-none-eabi/release/app ./loader-m3.bin

size:
	cargo size --bin app --release --features prod

objdump:
	cargo objdump --bin app --release --features prod -- -S | less
