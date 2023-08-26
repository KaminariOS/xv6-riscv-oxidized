XV6PATH ?= ..
# init:
# 	rustup target add riscv64gc-unknown-none-elf

# build:
# 	rustc src/lib.rs --emit=obj --target riscv64gc-unknown-none-elf
# 	riscv64-unknown-linux-gnu-ld -z max-page-size=4096 -N -e main -Ttext 0 -o _rusty lib.o $(XV6PATH)/user/printf.o $(XV6PATH)/user/usys.o $(XV6PATH)/user/ulib.o $(XV6PATH)/user/umalloc.o
#
# clean:
# 	rm *.o _rusty
all:
	RUSTFLAGS="-Clink-args=../user/printf.o -Clink-args=../user/ulib.o -Clink-args=../user/usys.o -Clink-args=../user/umalloc.o -Clink-args=-z  -Clink-args=max-page-size=4096 -Clink-args=-N -Clink-args=-e -Clink-args=main -Clink-args=-Ttext -Clink-args=0" cargo b --target riscv64gc-unknown-none-elf
	cp target/riscv64gc-unknown-none-elf/debug/rusty-risc ../user/_rusty 

