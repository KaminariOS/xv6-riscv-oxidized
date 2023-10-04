XV6PATH ?= $(realpath ../..)
XV6_USER_PATH ?= ${XV6PATH}/user
TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := ../target/$(TARGET)/$(MODE)
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))
BINS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%.bin, $(APPS))

check:
	RUSTFLAGS="-Clink-args=$(XV6_USER_PATH)/umalloc.o -Clink-args=-z -Clink-args=max-page-size=4096 -Clink-args=-N -Clink-args=-Ttext -Clink-args=0" cargo check --target $(TARGET)


fmt:
	RUSTFLAGS="-Clink-args=$(XV6_USER_PATH)/umalloc.o -Clink-args=-z -Clink-args=max-page-size=4096 -Clink-args=-N -Clink-args=-Ttext -Clink-args=0" cargo fmt 

all:
	RUSTFLAGS="-Clink-args=$(XV6_USER_PATH)/umalloc.o -Clink-args=-z -Clink-args=max-page-size=4096 -Clink-args=-N -Clink-args=-Ttext -Clink-args=0" cargo b -r --target $(TARGET)
	$(foreach elf, $(ELFS), cp $(elf) $(patsubst $(TARGET_DIR)/%, $(XV6_USER_PATH)/_%, $(elf));)

