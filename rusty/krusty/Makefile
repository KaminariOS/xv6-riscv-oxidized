XV6PATH ?= $(realpath ../..)
XV6_USER_PATH ?= ${XV6PATH}/user
XV6_KERNEL_PATH ?= ${XV6PATH}/kernel
TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := ../target/$(TARGET)/$(MODE)
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))
BINS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%.bin, $(APPS))
LIB := libkrusty

all:
	RUSTFLAGS="-Clink-args=-z -Clink-args=max-page-size=4096 -Clink-args=-N" cargo b -r --target $(TARGET)
	cp $(TARGET_DIR)/$(LIB).a $(XV6_KERNEL_PATH)
	# $(foreach elf, $(ELFS), cp $(elf) $(patsubst $(TARGET_DIR)/%, $(XV6_KERNEL_PATH)/_%, $(elf));)

