@ lib/core.asm
@ lib/types.asm
@ lib/stdio.asm

main!
  x00 for_y:
    x00 for_x:
      # i1f2 c_im = y / 2 - 0x40
      ld1 shr clc x40 sub clc
      # i1f2 c_re = x / 2 - 0x64
      ld1 shr clc x64 sub # c_re
      # putc(tones[mandelbrot(c_re, c_im)] >> iter_shift)
      !mandelbrot !iter_shift !rneg rot x0F and clc :tones add lda !putc
    # loop while `x += step_count != 0`
    !step_count add @dyn :for_x !bcc pop
    !'\n' !putc
  # we `y += step_count * 2` because characters have an aspect ratio of 2:1
  # loop while `y += step_count * 2 != 0`
  !step_count shl dec @const add @dyn !here !bcs :for_y !jmp

  !i2f6.mul.def

  tones: @20 @2E @2C @2B @2A @6F @25 @40 tones.end: # " .,+*o%@"...

tones.len! :tones.end :tones sub @const

mandelbrot! # u8 i = mandelbrot(i2f6 c_re, i2f6 c_im)
  x00 # z_re
  x00 # z_im
  # run `sizeof(tones) << iter_shift` mandelbrot iterations
  !tones.len !iter_shift rot @const for_i:
    # z_re2 = i2f6.mul(z_re, z_re)
    ld2 ld0 :i2f6.mul !call
    # z_im2 = i2f6.mul(z_im, z_im)
    ld2 ld0 :i2f6.mul !call
    # radius of `0xC0` aka `1.5` yields neater output than `0x100` aka `2.0`.
    # radius of `2.0` would be mathematically correct but suffers from integer
    # overflow leading to a grainier output
    :default
      # break if `z_re2 + z_im2` overflows `u8`
      ld2 ld2 add @dyn :break if2
      # break if `(u8)(z_re2 + z_im2) >= 0xC0`
      xC0 neg @const add @dyn pop :break iff
    !jmp default:
    # z_im = 2 * i2f6.mul(z_re, z_im) + c_im
    ld4 ld4 :i2f6.mul !call clc shl clc ld7 add clc st3
    # z_re = z_re2 - z_im2 + c_re
    sub clc ld4 add st2
  # loop while `i != 0`
  dec !z :for_i !bcc :end !jmp
# return* i
break: pop pop end: st3 pop pop pop

i2f6.mul.def! # inspired by `!i4f4.mul`, but more efficient
  i2f6.mul: # i2f6 product = i2f6.mul(i2f6 a, i2f6 b)
    x00 x00 # `product`, bytes swapped
    # we use `sec` as a marker to terminate the loop
    ld3 sec bit_loop:
      # a >>= 1
      shr @dyn
      # product += (CF ? b : 0x00) << 8
      x00 ld6 iff clc ad2 @dyn
      # product >>= 1 (stairstep shift)
      ld1 shr @dyn st1 ld2 shr @dyn st2 # stairstep shift
    # loop until `sec` is reached
    ld0 x01 !eq :bit_loop !bcc pop clc
    swp # swaps back `product` bytes
    # if (b < 0) product -= a << 8 (account for sign)
    ld4 !ng x00 ld4 iff clc su2
    # if (a < 0) product -= b << 8 (account for sign)
    ld3 !ng x00 ld5 iff clc su2
    # product >>= 6 (corrects for radix point)
    shl @dyn ld1 ad2 @dyn shl @dyn pop shl @dyn
  # return* product
  st2 !rt1

# i2f6.mul.def! # similar to `!i2f6.mul` above, but with left shifts
#   i2f6.mul: clc # i2f6 product = i2f6.mul(i2f6 a, i2f6 b)
#     # product = b < 0 ? -a : 0 (account for sign)
#     ld2 !ng x00 ld2 neg iff clc
#     # product -= a < 0 ? b : 0 (account for sign)
#     ld2 !ng x00 ld4 iff clc sub
#     # product <<= 8
#     x00 swp
#     # we use `sec` as a marker to terminate the loop
#     sec bit_loop:
#       # a <<= 1
#       ld3 ad4 @dyn
#       # temp = (CF ? b : 0) << 8
#       x00 x00 ld6 iff clc
#       # product <<= 1
#       ld3 ld3 ad4 @dyn ad4 @dyn clc
#       # product += temp
#       ad2 @dyn ad2 @dyn
#     # loop until `sec` is reached
#     ld3 x80 !eq :bit_loop !bcc
#     # product >>= 6 (corrects for radix point)
#     shl @dyn ld1 ad2 @dyn shl @dyn pop shl @dyn
#   # return* product
#   st2 !rt1

# i2f6.mul.def! # inspired by `!mul`, but for `i2f6`s
#   i2f6.mul: clc # i2f6 product = i2f6.mul(i2f6 a, i2f6 b)
#     # product = b < 0 ? -a : 0 (account for sign)
#     ld2 !ng x00 ld2 neg iff clc
#     # product -= a < 0 ? b : 0 (account for sign)
#     ld2 !ng x00 ld4 iff clc sub
#     # product <<= 8
#     x00 inc @const loop:
#       # product += b
#       x00 ld5 ad2 @dyn ad2 @dyn clc
#     # loop while `a-- != 0`
#     x01 su4 @dyn :loop !bcc
#     # product -= b
#     x00 ld5 su2 @dyn su2 @dyn
#     # product >>= 6 (corrects for radix point)
#     shl @dyn ld1 ad2 @dyn shl @dyn pop shl @dyn
#   # return* product
#   st2 !rt1

# step_count! x08 iter_shift! x01 # ~45 seconds
step_count! x08 iter_shift! x02 # ~75 seconds
# step_count! x04 iter_shift! x02 # ~5 minutes
# step_count! x02 iter_shift! x03 # ~30 minutes
# step_count! x01 iter_shift! x03 # ~2 hours
# step_count! x01 iter_shift! x04 # ~4 hours
