CC=gcc
CFLAGS=-O2 -Wall -Wextra -Wpedantic -std=c99

all: cemu

cemu: cemu.c
	$(CC) $(CFLAGS) -Wno-unused-function -Wno-bool-operation $^ -o $@

clean:
	rm -f cemu
