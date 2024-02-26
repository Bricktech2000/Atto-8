@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/stdlib.asm
@ lib/stdio.asm
@ lib/display.asm
@ misc/common/common.asm

# Challenge Description
# ---------------------
# It looks like we leaked a debug build of our app with our API secret still in it!
# Thankfully the whole program is encrypted, so there is nothing to worry about.

# to generate the payload, uncomment `!dbg` and `!source` and comment out `pop` and `!payload`.
# then, run the code in the emulator and when it hits the debug request, copy its memory to
# `paload!` below and use some Vim magic to format it so it assembles correctly

main!
  pop pop !display_buffer sts

  !rand_seed.min # rand_seed

  # loop through payload
  !payload.len for_i: dec
    # load byte
    :payload ld1 add lda
    # generate random number
    ld2 !rand.min st2 ld2
    # xor byte with random number
    xor clc
    # store byte
    :payload ld2 add sta
  !z :for_i !bcc pop

  # !dbg
  pop # pop rand_seed

  payload:
    # !source
    !payload
  payload.end:

  # memcpy random data to the payload buffer and halt
  !payload.len x00 :payload :memcpy !call
  !hlt

  !memcpy.def

payload.len! :payload.end :payload sub @const


source!
  :str_api_key pop # prevent unused label warning
  :str_message :puts.min !call
  :exit !jmp

  !puts.min.def

  str_api_key: @46 @4C @41 @47 @7B @4D @49 @6C @69 @54 @41 @72 @79 @2D @47 @72 @34 @64 @45 @5F @33 @6E @63 @72 @79 @70 @54 @31 @6F @4E @7D @00 # "FLAG{MIliTAry-Gr4dE_3ncrypT1oN}"
  str_message: @4E @6F @74 @68 @69 @6E @67 @20 @74 @6F @20 @73 @65 @65 @20 @68 @65 @72 @65 @21 @0A @49 @74 @20 @61 @70 @70 @65 @61 @72 @73 @20 @74 @68 @69 @73 @20 @70 @72 @6F @67 @72 @61 @6D @20 @69 @73 @20 @77 @6F @72 @6B @69 @6E @67 @20 @61 @73 @20 @69 @6E @74 @65 @6E @64 @65 @64 @2E @0A @00 # "Nothing to see here!\nIt appears this program is working as intended.\n"
  exit:

payload!
  @4B @EA @F3 @3A @97 @45 @6B @6E @D6 @25 @E2 @52 @A1 @75 @A4 @48 @9D @AF @BE @B6 @8D @36 @FA @BB @8C @A8 @3F @4D @E8 @EB @24 @CD @E5 @82 @36 @E8 @E6 @8A @11 @4B @EB @39 @FE @64 @CB @1C @54 @9A @03 @C7 @20 @42 @7C @EA @25 @01 @EA @20 @89 @A9 @08 @DD @7C @46 @72 @F7 @A9 @47 @39 @DE @B1 @CC @17 @4B @E3 @A2 @8C @8A @0F @1E @6B @E9 @A7 @14 @9D @A0 @1A @5B @7D @7F @E9 @29 @02 @78 @F5 @63 @D8 @B6 @90 @1A @DF @35 @C4 @FF @80 @8D @5F @D8 @B8 @1F @DE @BD @83 @98 @94 @56 @36 @1E
