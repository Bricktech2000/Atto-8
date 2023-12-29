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
  :str_api_key pop
  :str_message :puts.min !call
  :exit !jmp

  !puts.min.def

  str_api_key: @46 @4C @41 @47 @7B @4D @49 @6C @69 @54 @41 @72 @79 @2D @47 @72 @34 @64 @45 @5F @33 @6E @63 @72 @79 @70 @54 @31 @6F @4E @7D @00 # "FLAG{MIliTAry-Gr4dE_3ncrypT1oN}"
  str_message: @4E @6F @74 @68 @69 @6E @67 @20 @74 @6F @20 @73 @65 @65 @20 @68 @65 @72 @65 @21 @0D @0A @49 @74 @20 @61 @70 @70 @65 @61 @72 @73 @20 @74 @68 @69 @73 @20 @70 @72 @6F @67 @72 @61 @6D @20 @69 @73 @20 @77 @6F @72 @6B @69 @6E @67 @20 @61 @73 @20 @69 @6E @74 @65 @6E @64 @65 @64 @2E @0D @0A @00 # "Nothing to see here!\r\nIt appears this program is working as intended.\r\n"
  exit:

payload!
  @58 @21 @A4 @2B @06 @80 @FA @16 @81 @4E @E0 @A5 @3D @DE @1D @A1 @54 @81 @DE @75 @0D @E7 @9A @A4 @84 @BC @BF @17 @DA @83 @A4 @8E @07 @12 @D6 @B4 @57 @DB @94 @B9 @40 @D9 @B6 @96 @0B @49 @C6 @78 @C5 @1B @D9 @52 @67 @F5 @39 @C0 @3D @44 @72 @A4 @36 @4E @BE @3C @CC @BF @4D @D0 @39 @5C @72 @A4 @C1 @6C @7A @E3 @E5 @8D @06 @4B @F6 @A6 @9F @8B @5C @4A @77 @E8 @BD @47 @CD @A2 @07 @53 @68 @6C @E5 @64 @4B @62 @A6 @34 @C0 @AB @89 @18 @D8 @3C @83 @BE @92 @DE @16 @DF @A2 @0E @D5 @B7 @82 @99 @DE @75 @36 @1E
