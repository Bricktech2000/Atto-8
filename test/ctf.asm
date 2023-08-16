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

  xF0 # rand_seed

  # loop through payload
  !payload_len for_i: dec
    # load byte
    :payload ld1 add lda
    # generate random number
    ld2 !rand.min st2 ld2
    # xor byte with random number
    xor clc
    # store byte
    :payload ld2 add sta
  buf :for_i !bcc pop

  # !dbg
  pop # pop rand_seed

  payload:
    # !source
    !payload
  payload_end:

  # memcpy random data to the payload buffer and halt
  !payload_len x00 :payload :memcpy !call
  !hlt

  !memcpy.def

payload_len! :payload_end :payload sub @const


source!
  :str_message :puts.min !call
  :exit !jmp

  !puts.min.def

  # "FLAG{MIliTAry-Gr4dE_3ncrypT1oN}\0"
  str_api_key: @46 @4C @41 @47 @7B @4D @49 @6C @69 @54 @41 @72 @79 @2D @47 @72 @34 @64 @45 @5F @33 @6E @63 @72 @79 @70 @54 @31 @6F @4E @7D @00
  # "Nothing to see here!\r\nIt appears this program is working as intended.\r\n\0"
  str_message: @4E @6F @74 @68 @69 @6E @67 @20 @74 @6F @20 @73 @65 @65 @20 @68 @65 @72 @65 @21 @0D @0A @49 @74 @20 @61 @70 @70 @65 @61 @72 @73 @20 @74 @68 @69 @73 @20 @70 @72 @6F @67 @72 @61 @6D @20 @69 @73 @20 @77 @6F @72 @6B @69 @6E @67 @20 @61 @73 @20 @69 @6E @74 @65 @6E @64 @65 @64 @2E @0D @0A @00
  exit:

payload!
  @B6 @56 @11 @FF @6C @B5 @6E @24 @A4 @D2 @AE @82 @A0 @1E @7D @91 @4C @8D @DA @76 @82 @2E @70 @D1 @30 @E6 @92 @8F @96 @A5 @B7 @09 @CA @FA @A2 @8E @4A @5B @D4 @99 @50 @D1 @B2 @94 @0A @C7 @81 @D5 @1D @77 @EF @49 @E4 @3A @D0 @3A @40 @F4 @2A @88 @20 @45 @35 @F7 @27 @44 @BE @27 @CC @A8 @08 @99 @51 @24 @5E @F1 @EC @07 @43 @E7 @A0 @8D @04 @48 @B3 @B3 @85 @91 @0F @1E @6F @F3 @A1 @00 @CF @B1 @05 @14 @73 @7E @A8 @33 @4D @63 @ED @2A @C1 @BE @C2 @10 @C5 @7B @CA @B1 @95 @9B @11 @D5 @B3 @0F @95 @DE @ED @FD
