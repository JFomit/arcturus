
TARGET:=rust_dos.com

.PHONEY: clean all build
all: $(TARGET)

$(TARGET): build
	cargo objcopy --release -- -O binary --binary-architecture=i386:x86 $(TARGET)

build:
	cargo build --release

clean:
	cargo clean
	rm -rf $(TARGET)
