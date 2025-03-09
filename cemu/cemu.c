#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "../misc/common/common.h"

struct Microcomputer {
  uint8_t mem[MEM_SIZE];
  struct Microprocessor {
    uint8_t ip;
    uint8_t sp;
    bool cf;
  } mp;
} mc;

enum TickTrap {
  TICKTRAP_NONE,
  ILLEGAL_OPCODE,
  DEBUG_REQUEST,
};

int peekchar(void) {
  int c = getchar();
  ungetc(c, stdin);
  return c;
}

uint8_t mem_read(uint8_t address) {
  if (address == STDIO_BUFFER)
    return peekchar() == EOF ? '\0' : getchar();
  else
    return mc.mem[address];
}

void mem_write(uint8_t address, uint8_t value) {
  if (address == STDIO_BUFFER)
    putchar(value), fflush(stdout);
  else
    mc.mem[address] = value;
}

void sp_push(uint8_t value) { mem_write(--mc.mp.sp, value); }

uint8_t sp_pop(void) { return mem_read(mc.mp.sp++); }

void mc_reset(void) {
  mc.mp.ip = 0x00;
  mc.mp.sp = 0x00;
  mc.mp.cf = false;
  ungetc(mc.mem[STDIO_BUFFER], stdin);
}

enum TickTrap mc_tick(void) {
  uint8_t opcode = mem_read(mc.mp.ip++);

  switch ((opcode & B10000000) >> 7) {
  case B0: { // psh
    sp_push(DECODE_IMM(opcode));
  } break;

  case B1: {
    switch ((opcode & B01000000) >> 6) {
    case B0: { // arithmetic and logic
      switch ((opcode & B00111100) >> 2) {
      case 0x0: { // add
        uint8_t addr = mc.mp.sp + DECODE_SIZE(opcode);
        uint16_t res = (uint16_t)mem_read(addr) + (uint16_t)sp_pop() + mc.mp.cf;
        mem_write(addr, (uint8_t)res);
        mc.mp.cf = res > 0xFF;
      } break;

      case 0x1: { // sub
        uint8_t addr = mc.mp.sp + DECODE_SIZE(opcode);
        uint16_t res = (uint16_t)mem_read(addr) - (uint16_t)sp_pop() - mc.mp.cf;
        mem_write(addr, (uint8_t)res);
        mc.mp.cf = res > 0xFF;
      } break;

      case 0x4: { // iff
        uint8_t addr = mc.mp.sp + DECODE_SIZE(opcode);
        uint8_t top = sp_pop();
        mem_write(addr, mc.mp.cf ? top : mem_read(addr));
      } break;

      case 0x5: { // swp
        uint8_t addr = mc.mp.sp + DECODE_SIZE(opcode);
        uint8_t top = sp_pop();
        sp_push(mem_read(addr));
        mem_write(addr, top);
      } break;

      case 0x6: { // rot
        uint8_t addr = mc.mp.sp + DECODE_SIZE(opcode);
        uint8_t top = sp_pop();
        uint16_t shifted = (uint16_t)mem_read(addr) << top % 8;
        uint8_t res = (uint8_t)(shifted | shifted >> 8);
        mem_write(addr, res);
        mc.mp.cf = false;
      } break;

      case 0x8: { // orr
        uint8_t addr = mc.mp.sp + DECODE_SIZE(opcode);
        uint8_t res = sp_pop() | mem_read(addr);
        mem_write(addr, res);
        mc.mp.cf = res == 0x00;
      } break;

      case 0x9: { // and
        uint8_t addr = mc.mp.sp + DECODE_SIZE(opcode);
        uint8_t res = sp_pop() & mem_read(addr);
        mem_write(addr, res);
        mc.mp.cf = res == 0x00;
      } break;

      case 0xA: { // xor
        uint8_t addr = mc.mp.sp + DECODE_SIZE(opcode);
        uint8_t res = sp_pop() ^ mem_read(addr);
        mem_write(addr, res);
        mc.mp.cf = res == 0x00;
      } break;

      case 0xB: { // xnd
        uint8_t addr = mc.mp.sp + DECODE_SIZE(opcode);
        uint8_t res = sp_pop() & 0x00;
        mem_write(addr, res);
        mc.mp.cf = res == 0x00;
      } break;

      default: // size as part of opcode
        switch (opcode & B00001111) {
        case 0x0: { // inc
          sp_push(sp_pop() + 1);
        } break;

        case 0x1: { // dec
          sp_push(sp_pop() - 1);
        } break;

        case 0x2: { // neg
          sp_push(-sp_pop());
        } break;

        case 0x4: { // shl
          uint8_t top = sp_pop();
          sp_push(top << 1 | mc.mp.cf);
          mc.mp.cf = top & B10000000;
        } break;

        case 0x5: { // shr
          uint8_t top = sp_pop();
          sp_push(top >> 1 | mc.mp.cf << 7);
          mc.mp.cf = top & B00000001;
        } break;

        case 0x6: { // not
          uint8_t res = ~sp_pop();
          sp_push(res);
          mc.mp.cf = res == 0x00;
        } break;

        case 0x7: { // buf
          uint8_t res = sp_pop();
          sp_push(res);
          mc.mp.cf = res == 0x00;
        } break;

        case 0xB: // dbg
          return DEBUG_REQUEST;

        default:
          return ILLEGAL_OPCODE;
        }
      }
    } break;

    case B1: {
      switch ((opcode & B00100000) >> 5) {
      case B0: { // offset operations
        switch ((opcode & B00010000) >> 4) {
        case B0: { // ldo
          uint8_t addr = mc.mp.sp + DECODE_OFST(opcode);
          sp_push(mem_read(addr));
        } break;

        case B1: { // sto
          uint8_t top = sp_pop();
          uint8_t addr = mc.mp.sp + DECODE_OFST(opcode);
          mem_write(addr, top);
        } break;

        default:
          abort(); // unreachable
        }
      } break;

      case B1: {
        switch ((opcode & B00010000) >> 4) {
        case B0: { // carry flag and stack
          switch (opcode & B00001111) {
          case 0x0: { // lda
            sp_push(mem_read(sp_pop()));
          } break;

          case 0x1: { // sta
            uint8_t addr = sp_pop();
            mem_write(addr, sp_pop());
          } break;

          case 0x2: { // ldi
            sp_push(mc.mp.ip);
          } break;

          case 0x3: { // sti
            mc.mp.ip = sp_pop();
          } break;

          case 0x4: { // lds
            sp_push(mc.mp.sp);
          } break;

          case 0x5: { // sts
            mc.mp.sp = sp_pop();
          } break;

          case 0x8: { // clc
            mc.mp.cf = false;
          } break;

          case 0x9: { // sec
            mc.mp.cf = true;
          } break;

          case 0xA: { // flc
            mc.mp.cf = !mc.mp.cf;
          } break;

          case 0xE: { // nop
          } break;

          case 0xF: { // pop
            mc.mp.sp++;
          } break;

          default:
            return ILLEGAL_OPCODE;
          }
        } break;

        case B1: { // phn
          sp_push(DECODE_NIMM(opcode));
        } break;

        default:
          abort(); // unreachable
        }
      } break;

      default:
        abort(); // unreachable
      }
    } break;

    default:
      abort(); // unreachable
    }
  } break;

  default:
    abort(); // unreachable
  }

  return TICKTRAP_NONE;
}

int main(int argc, char *argv[]) {
  if (argc != 2)
    printf("CEmu: Usage: %s <memory image file>\n", argv[0]),
        exit(EXIT_FAILURE);

  FILE *fp = fopen(argv[1], "rb");
  if (fp == NULL)
    perror("fopen"), exit(EXIT_FAILURE);

  if (fread(mc.mem, 1, sizeof(mc.mem), fp) != sizeof(mc.mem))
    if (feof(fp))
      printf("CEmu: Error: Memory image '%s' has incorrect size\n", argv[1]),
          exit(EXIT_FAILURE);
    else
      perror("fread"), exit(EXIT_FAILURE);
  else if (fgetc(fp) != EOF)
    printf("CEmu: Error: Memory image '%s' has incorrect size\n", argv[1]),
        exit(EXIT_FAILURE);

  if (fclose(fp) != 0)
    perror("fclose"), exit(EXIT_FAILURE);

  mc_reset();
  while (1) {
    switch (mc_tick()) {
    case TICKTRAP_NONE:
      continue;
    case ILLEGAL_OPCODE:
      puts("CEmu: Illegal opcode"), exit(EXIT_FAILURE);
    case DEBUG_REQUEST:
      puts("CEmu: Debug request"), exit(EXIT_FAILURE);
    }
  }
}
