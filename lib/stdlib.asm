rand_bits! x1D

# outputs every number in 0x00..=0xFF then repeats
rand! clc # seed = rand(seed)
  shl @dyn x00 !rand_bits iff swp
  buf @dyn x00 !rand_bits iff
  xor xor

# outputs every number in 0x01..=0xFF then repeats
# will never output 0x00 (if and only if seed is not 0x00)
# seed must never be 0x00 (otherwise will only output 0x00)
rand.min! clc # seed = rand.min(seed)
  shl x00 !rand_bits iff xor


mul_10! # u8 product = mul_10(u8 n)
  # n <<= 1
  shl # 2 n
  # n += n << 2
  ld0 shl shl add # 10 n
# return* n

div_10! # u8 quotient = div_10(u8 n)
  # n >>= 1
  shr clc # 1/2 n = 0.5 n
  # n += n >> 2; n += 1 // round up
  ld0 shr sec add @dyn # 3/4 n = 0.75 n
  # n += n >> 4
  ld0 xF0 and x04 rot # 51/64 n =~ 0.7969 n
  # n += n >> 7
  ld1 shl @dyn pop add @dyn # 411/512 n =~ 0.8027 n
  # n >>= 8
  xF8 and x05 rot # 411/4096 n =~ 0.1003 n
# return* n

div_10.min! # u8 quotient = div_10.min(u8 n)
  # n *= 205
  xCD !u8 !u8.mul # 205 n
  # n >>= 11
  pop xF8 and x05 rot # 205/2048 n =~ 0.1001 n
# return* n


delay! # delay(iterations)
  loop. x1F !stall x01 sub @dyn .loop !bcc pop

delay_long! # delay_long(iterations)
  x00 loop. x1F !stall x00 x01 su2 @dyn su2 @dyn .loop !bcc pop pop


# block until a key is pressed
block_any! block. !getc !char.is_null .block !bcs
# block until no key is pressed
block_null! block. !getc !char.is_null .block !bcc
# block until a key is pressed then return it
block_getc! !char.null block. !char.pop !getc !char.check_null .block !bcs


popcnt! # count = popcnt(a)
  # count = a == 0 ? -1 : 0
  !z x00 xFF iff
  # do { count++ } while (a != 0)
  while. inc
    # a &= a - 1
    ld1 dec an2 # unsets lowest set bit
  .while !bcc
  # return* count
  st0


sort.def!
  sort: clc # sort(len, arr)
    # bubble sort, in-place
    x01 while. dec # swapped = false
      ld2 dec for_i. dec
        ld4 ld1 add # push pointer
          # if not in order, swap
          ld0 !u8u8.lda swp sub pop
          .continue !bcc
            ld0 !u8u8.lda
            ld2 !u8u8.sta
            # swapped = true
            x01 st2
          continue.
        pop # pop pointer
      !z .for_i !bcc # bleed `0x00`
    # break if not swapped
    !e .while !bcc pop
  # return*
  !rt2

# `free` solely sets the `is_free` bit of the corresponding header
# `malloc` searches linearly from `HEAP_START` and coalesces free blocks as it goes
#
# the following must be supplied by the user:
# -  `HEAP_START` -- start of heap memory
# - `*HEAP_START` -- length of heap memory but with the most significant bit set
#
# struct header {
#   u7 size; // excludes header
#   bool is_free; // most significant bit
# }
#
# struct block {
#   header header;
#   u8 data[header.size];
# }
#
# struct heap {
#   block blocks[];
# }

heap_unlimited! xFF # header for free block of size 0x7F
is_free_mask! x80 @const

malloc.def!
  malloc: # void* p = malloc(size)
    !heap_start for_block. # loop as `curr_block`
      # offset `size` so allocated blocks are considered too small
      ld2 !is_free_mask orr
      # check if `curr_block` is large enough
      # a block being large enough implies it is free
      ld1 lda sub .block_found !bcs pop
      # next_block = (curr_block.header & ~IS_FREE_MASK) + curr_block + 1
      ld0 lda !is_free_mask not and ld1 add inc swp # swap curr_block and next_block
      # next_header = next_block.header
      ld1 lda swp # swap curr_block and next_header
      # coalesced_header = next_header + curr_header + 1
      ld1 ld1 lda add @dyn inc # if addition overflows then both blocks are free
      # stack (top down): coalesced_header, curr_block, next_header, next_block, ret_addr, size
      # working_header = both_free ? coalesced_header | IS_FREE_MASK : next_header
      # working_block  = both_free ? curr_block : next_block
      if2 if2 x00 shr @dyn orr
      # working_block.header = working_header
      ld1 sta # block for next iteration will be working_block
    .for_block !jmp
    block_found.
      # create free_header with correct size
      neg dec !is_free_mask orr # clears carry
      # next_block = size + curr_block + 1
      ld3 ld2 add inc
      # next_block.header = free_header
      sta
      # curr_block.header = size
      ld2 ld1 sta
  # return* curr_block + 1
  inc st1 !rt0

free.def!
  free: # free(*p)
    !is_free_mask ld2 dec
    ld0 lda or2 sta
  # return*
  !rt1
