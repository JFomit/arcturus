
TARGET:=gdbstub.com

.PHONEY: clean all build dis size
all: $(TARGET)

$(TARGET): build
	cargo objcopy --release -- -O binary --binary-architecture=i386:x86 $(TARGET)
	cargo objcopy --release -- $(TARGET:.com=.elf)

build:
	cargo build --release

clean:
	cargo clean
	rm -rf $(TARGET)

dis: $(TARGET)
	objdump -D -b binary -m i8086 -M intel $^ > $(TARGET:.com=.lst)
size: $(TARGET)
	wc -c $(TARGET)