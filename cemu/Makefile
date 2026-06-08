.POSIX:
.SUFFIXES:
CC=gcc
CFLAGS=-O2 -Wall -Wextra -Wpedantic -std=c99

all: cemu
clean:; rm -f cemu

cemu: cemu.c; $(CC) $(CFLAGS) -o $@ cemu.c -Wno-unused-function -Wno-bool-operation
