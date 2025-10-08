
.PHONEY: clean all rust asm builddir dis
all: rust asm
	ld.lld -Tlink.x target/i386-dos/release/libgdbstub.a target/debugee.o -o gdbstub.elf
	objcopy -O binary --binary-architecture=i386 gdbstub.elf gdbstub.com

rust:
	cargo build --release

asm: builddir target/debugee.o

target/debugee.o: src/debugee.nasm
	nasm -felf32 -g $^ -o $@

builddir:
	mkdir -p target

clean:
	cargo clean
	rm -rf gdbstub.com
	rm -rf gdbstub.elf

dis: all
	objdump -d -M intel -m i8086 gdbstub.elf > gdbstub.lst