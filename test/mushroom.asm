@ lib/microprocessor/core.asm
@ lib/microcomputer/display.asm

main!
  pop pop !hlt

  !display_buffer @org
    @07 @E0
    @18 @78
    @38 @7C
    @70 @3E
    @67 @9E
    @8F @C1
    @8F @CD
    @CF @DF
    @E7 @9F
    @E0 @0D
    @CF @F1
    @72 @4E
    @22 @44
    @20 @04
    @10 @08
    @0F @F0
