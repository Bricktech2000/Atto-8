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

  !rand_seed # rand_seed

  # loop through payload
  !payload.len for_i: dec
    # load byte
    :payload ld1 add lda
    # generate random number
    ld2 !rand st2 ld2
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
  @E4 @3E @94 @8A @C2 @EC @BE @8F @2D @D9 @99 @E4 @F5 @5A @38 @03 @25 @F6 @11 @6C @E5 @0F @65 @75 @EE @9C @2E @CE @28 @8E @9D @12 @17 @FE @05 @4C @B1 @22 @4A @ED @BD @1D @E9 @E4 @8E @B5 @8B @76 @78 @79 @72 @66 @63 @66 @68 @AC @3D @C0 @FC @12 @DE @B3 @46 @56 @77 @76 @68 @AC @C7 @A4 @89 @D5 @90 @83 @82 @11 @54 @E3 @B0 @CA @0C @5F @F9 @38 @8E @2A @52 @6C @ED @3A @4E @F1 @61 @C2 @AD @42 @4B @7E @F1 @A1 @01 @51 @F3 @65 @C8 @AC @C4 @14 @DB @A5 @86 @94 @14 @5E @F2 @68 @24 @1A
