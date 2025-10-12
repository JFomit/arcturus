LINKER:=~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld

.PHONEY: clean all rust asm builddir
all: rust asm
	ld.lld -Tlink.x target/i386-dos/release/libgdbstub.a target/debugee.o -o gdbstub.elf
	objcopy -O binary --binary-architecture=i386 gdbstub.elf gdbstub.com

	objdump -C -S -d -M intel -m i8086 -j .text gdbstub.elf > gdbstub.lst

	objdump -s -j .rodata gdbstub.elf >> gdbstub.lst
	objdump -s -j .data gdbstub.elf >> gdbstub.lst

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
	rm -rf *.lst
