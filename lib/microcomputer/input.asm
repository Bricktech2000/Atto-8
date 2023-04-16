input_buffer! x00 @const
branch_input! .skip swp @const !input_buffer lda buf pop iff sti skip.
reset_input! !input_buffer x00 sta
wait_input! !here !branch_input !reset_input
