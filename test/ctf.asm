@ lib/microprocessor/core.asm
@ lib/microprocessor/types.asm
@ lib/microprocessor/string.asm
@ lib/microprocessor/stdlib.asm
@ lib/microcomputer/display.asm
@ lib/microcomputer/stdio.asm

# Challenge Description
# ---------------------
# It looks like we leaked a debug build of our app with our API secret still in it!
# Thankfully the whole program is encrypted, so there is nothing to worry about.

# to generate the payload, uncomment `!dbg` and `!source` and comment out `pop` and `!payload`.
# then, run the code in the emulator and when it hits the debug request, copy its memory to
# `paload!` below and use some Vim magic to format it so it assembles correctly

main!
  pop pop !display_buffer sts

  xF0 # prng_seed

  # loop through payload
  !payload_len for_i: dec
    # load byte
    :payload ld1 add lda
    # generate random number
    ld2 !prng_minimal st2 ld2
    # xor byte with random number
    xor clc
    # store byte
    :payload ld2 add sta
  buf :for_i !bcc pop

  # !dbg
  pop # pop prng_seed

  payload:
    # !source
    !payload
  payload_end:

  # memcpy random data to the payload buffer and halt
  !payload_len x00 :payload :memcpy !call
  !hlt

  !memcpy_def

payload_len! :payload_end :payload sub @const


source!
  :str_message :puts !call
  :exit !jmp

  !puts_def

  # "FLAG{MIliTAry-Gr4dE_3ncrypT1oN}\0"
  str_api_key: d46 d4C d41 d47 d7B d4D d49 d6C d69 d54 d41 d72 d79 d2D d47 d72 d34 d64 d45 d5F d33 d6E d63 d72 d79 d70 d54 d31 d6F d4E d7D d00
  # "Nothing to see here!\r\nIt appears this program is working as intended.\r\n\0"
  str_message: d4E d6F d74 d68 d69 d6E d67 d20 d74 d6F d20 d73 d65 d65 d20 d68 d65 d72 d65 d21 d0D d0A d49 d74 d20 d61 d70 d70 d65 d61 d72 d73 d20 d74 d68 d69 d73 d20 d70 d72 d6F d67 d72 d61 d6D d20 d69 d73 d20 d77 d6F d72 d6B d69 d6E d67 d20 d61 d73 d20 d69 d6E d74 d65 d6E d64 d65 d64 d2E d0D d0A d00
  exit:

payload!
  dB6 d56 d11 dFF d6C dB5 d6E d24 dA4 dD2 dAE d82 dA0 d1E d7D d91 d4C d8D dDA d76 d82 d2E d70 dD1 d30 dE6 d92 d8F d96 dA5 dB7 d09 dCA dFA dA2 d8E d4A d5B dD4 d99 d50 dD1 dB2 d94 d0A dC7 d81 dD5 d1D d77 dEF d49 dE4 d3A dD0 d3A d40 dF4 d2A d88 d20 d45 d35 dF7 d27 d44 dBE d27 dCC dA8 d08 d99 d51 d24 d5E dF1 dEC d07 d43 dE7 dA0 d8D d04 d48 dB3 dB3 d85 d91 d0F d1E d6F dF3 dA1 d00 dCF dB1 d05 d14 d73 d7E dA8 d33 d4D d63 dED d2A dC1 dBE dC2 d10 dC5 d7B dCA dB1 d95 d9B d11 dD5 dB3 d0F d95 dDE dED dFD
