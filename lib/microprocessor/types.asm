u8! # u8 n = u8(u8 n)
u16! # u16 n = u16(u16 n)
u32! # u32 n = u16(u32 n)
u64! # u64 n = u16(u64 n)
i8! !u8 # i8 n = i8(i8 n)
i16! !u16 # i16 n = i16(i16 n)
i32! !u32 # i32 n = i16(i32 n)
i64! !u64 # i64 n = i16(u64 n)
u4u4! !u8 # u4u4 n = u4u4(u4u4 n)
u8u8! !u16 # u8u8 n = u8u8(u8u8 n)
i4i4! !i8 # i4i4 n = i4i4(i4i4 n)
i8i8! !i16 # i8i8 n = i8i8(i8i8 n)
u4f4! !u8 # u4f4 n = u4f4(u4 fr, u4 in)
u8f8! !u16 # u8f8 n = u8f8(u8 fr, u8 in)
i4f4! !i8 # i4f4 n = i4f4(u4 fr, i4 in)
i8f8! !i16 # i8f8 n = i8f8(u8 fr, i8 in)
c4f4m4f4! !i16 # c4f4m4f4 n = c4f4m4f4(i4f4 im, i4f8 re)
c8f8m8f8! !i32 # c8f8m8f8 n = c8f8m8f8(i8f8 im, i8f8 re)
char! # char n = char(u8 n)

u8.0! x00
u16.0! x00 x00
u32.0! x00 x00 x00 x00
u64.0! x00 x00 x00 x00 x00 x00 x00 x00
i8.0! !u8.0
i16.0! !u16.0
i32.0! !u32.0
i64.0! !u64.0
u4u4.0! !u8.0
u8u8.0! !u16.0
i4i4.0! !i8.0
i8i8.0! !i16.0
u4f4.0! !u8.0
u8f8.0! !u16.0
i4f4.0! !i8.0
i8f8.0! !i16.0
c4f4m4f4.0! !i16.0
c8f8m8f8.0! !i32.0
char.null! x00
char.start_of_heading! x01
char.start_of_text! x02
char.end_of_text! x03
char.end_of_transmission! x04
char.enquiry! x05
char.acknowledge! x06
char.bell! x07
char.backspace! x08
char.horizontal_tab! x09
char.line_feed! x0A
char.vertical_tab! x0B
char.form_feed! x0C
char.carriage_return! x0D
char.shift_out! x0E
char.shift_in! x0F
char.data_link_escape! x10
char.device_control_1! x11
char.device_control_2! x12
char.device_control_3! x13
char.device_control_4! x14
char.negative_acknowledge! x15
char.synchronous_idle! x16
char.end_of_transmission_block! x17
char.cancel! x18
char.end_of_medium! x19
char.substitute! x1A
char.escape! x1B
char.file_separator! x1C
char.group_separator! x1D
char.record_separator! x1E
char.unit_separator! x1F
char.space! x20
char.exclamation_mark! x21
char.quotation_mark! x22
char.number_sign! x23
char.dollar_sign! x24
char.percent_sign! x25
char.ampersand! x26
char.apostrophe! x27
char.left_parenthesis! x28
char.right_parenthesis! x29
char.asterisk! x2A
char.plus_sign! x2B
char.comma! x2C
char.hyphen_minus! x2D
char.full_stop! x2E
char.solidus! x2F
char.digit_zero! x30
char.digit_one! x31
char.digit_two! x32
char.digit_three! x33
char.digit_four! x34
char.digit_five! x35
char.digit_six! x36
char.digit_seven! x37
char.digit_eight! x38
char.digit_nine! x39
char.colon! x3A
char.semicolon! x3B
char.less_than_sign! x3C
char.equals_sign! x3D
char.greater_than_sign! x3E
char.question_mark! x3F
char.commercial_at! x40
char.latin_capital_letter_a! x41
char.latin_capital_letter_b! x42
char.latin_capital_letter_c! x43
char.latin_capital_letter_d! x44
char.latin_capital_letter_e! x45
char.latin_capital_letter_f! x46
char.latin_capital_letter_g! x47
char.latin_capital_letter_h! x48
char.latin_capital_letter_i! x49
char.latin_capital_letter_j! x4A
char.latin_capital_letter_k! x4B
char.latin_capital_letter_l! x4C
char.latin_capital_letter_m! x4D
char.latin_capital_letter_n! x4E
char.latin_capital_letter_o! x4F
char.latin_capital_letter_p! x50
char.latin_capital_letter_q! x51
char.latin_capital_letter_r! x52
char.latin_capital_letter_s! x53
char.latin_capital_letter_t! x54
char.latin_capital_letter_u! x55
char.latin_capital_letter_v! x56
char.latin_capital_letter_w! x57
char.latin_capital_letter_x! x58
char.latin_capital_letter_y! x59
char.latin_capital_letter_z! x5A
char.left_square_bracket! x5B
char.reverse_solidus! x5C
char.right_square_bracket! x5D
char.circumflex_accent! x5E
char.low_line! x5F
char.grave_accent! x60
char.latin_small_letter_a! x61
char.latin_small_letter_b! x62
char.latin_small_letter_c! x63
char.latin_small_letter_d! x64
char.latin_small_letter_e! x65
char.latin_small_letter_f! x66
char.latin_small_letter_g! x67
char.latin_small_letter_h! x68
char.latin_small_letter_i! x69
char.latin_small_letter_j! x6A
char.latin_small_letter_k! x6B
char.latin_small_letter_l! x6C
char.latin_small_letter_m! x6D
char.latin_small_letter_n! x6E
char.latin_small_letter_o! x6F
char.latin_small_letter_p! x70
char.latin_small_letter_q! x71
char.latin_small_letter_r! x72
char.latin_small_letter_s! x73
char.latin_small_letter_t! x74
char.latin_small_letter_u! x75
char.latin_small_letter_v! x76
char.latin_small_letter_w! x77
char.latin_small_letter_x! x78
char.latin_small_letter_y! x79
char.latin_small_letter_z! x7A
char.left_curly_bracket! x7B
char.vertical_line! x7C
char.right_curly_bracket! x7D
char.tilde! x7E
char.delete! x7F

u8.add! add
u16.add! ad2 ad2
u32.add! ad4 ad4 ad4 ad4
u64.add! ad8 ad8 ad8 ad8 ad8 ad8 ad8 ad8
i8.add! !u8.add
i16.add! !u16.add
i32.add! !u32.add
i64.add! !u64.add
u4u4.add! ld1 ld1 ld1 clc add x0F and st2 xF0 and clc add xF0 and orr
u8u8.add! ad2 clc ad2
i4i4.add! !u4u4.add
i8i8.add! !u8u8.add
u4f4.add! !u8.add
u8f8.add! !u16.add
i4f4.add! !i8.add
i8f8.add! !i16.add
c4f4m4f4.add! ad4 ad4 clc ad4 ad4
c8f8m8f8.add! ad4 ad4 clc ad4 ad4
char.add! !u8.add

u8.sub! sub
u16.sub! su2 su2
u32.sub! su4 su4 su4 su4
u64.sub! su8 su8 su8 su8 su8 su8 su8 su8
i8.sub! !u8.sub
i16.sub! !u16.sub
i32.sub! !u32.sub
i64.sub! !u64.sub
u4u4.sub! ld1 ld1 ld1 clc sub x0F and st2 xF0 and clc sub xF0 and orr
u8u8.sub! su2 clc su2
i4i4.sub! !u4u4.sub
i8i8.sub! !u8u8.sub
u4f4.sub! !u8.sub
u8f8.sub! !u16.sub
i4f4.sub! !i8.sub
i8f8.sub! !i16.sub
c4f4m4f4.sub! su4 su4 clc su4 su4
c8f8m8f8.sub! su4 su4 clc su4 su4
char.sub! !u8.sub

u8.iff! iff
u16.iff! if2 if2
u32.iff! if4 if4 if4 if4
u64.iff! if8 if8 if8 if8 if8 if8 if8 if8
i8.iff! !u8.iff
i16.iff! !u16.iff
i32.iff! !u32.iff
i64.iff! !u64.iff
u4u4.iff! !u8.iff
u8u8.iff! !u16.iff
i4i4.iff! !i8.iff
i8i8.iff! !i16.iff
u4f4.iff! !u8.iff
u8f8.iff! !u16.iff
i4f4.iff! !i8.iff
i8f8.iff! !i16.iff
c4f4m4f4.iff! !i16.iff
c8f8m8f8.iff! !i32.iff
char.iff! !u8.iff

u8.pop! pop
u16.pop! pop pop
u32.pop! pop pop pop pop
u64.pop! pop pop pop pop pop pop pop pop
i8.pop! !u8.pop
i16.pop! !u16.pop
i32.pop! !u32.pop
i64.pop! !u64.pop
u4u4.pop! !u8.pop
u8u8.pop! !u16.pop
i4i4.pop! !i8.pop
i8i8.pop! !i16.pop
u4f4.pop! !u8.pop
u8f8.pop! !u16.pop
i4f4.pop! !i8.pop
i8f8.pop! !i16.pop
c4f4m4f4.pop! !i16.pop
c8f8m8f8.pop! !i32.pop
char.pop! !u8.pop

u8.shl! shl
u16.shl! shl ld1 shl st1
u32.shl! shl ld1 shl st1 ld2 shl st2 ld3 shl st3 ld4 shl st4
u64.shl! shl ld1 shl st1 ld2 shl st2 ld3 shl st3 ld4 shl st4 ld5 shl st5 ld6 shl st6 ld7 shl st7
u4u4.shl! !u8.shl xEF and
u8u8.shl! !u16.shl xFE an2
u4f4.shl! !u8.shl
u8f8.shl! !u16.shl

u8.shr! shr
u16.shr! ld1 shr st1 shr
u32.shr! ld3 shr st3 ld2 shr st2 ld1 shr st1 shr
u64.shr! ld7 shr st7 ld6 shr st6 ld5 shr st5 ld4 shr st4 ld3 shr st3 ld2 shr st2 ld1 shr st1 shr
u4u4.shr! !u8.shr xF7 and
u8u8.shr! !u16.shr x7F and
u4f4.shr! !u8.shr
u8f8.shr! !u16.shr

u8.neg! neg
u16.neg! neg ld1 neg st1
u32.neg! neg ld1 neg st1 ld2 neg st2 ld3 neg st3 ld4 neg st4
u64.neg! neg ld1 neg st1 ld2 neg st2 ld3 neg st3 ld4 neg st4 ld5 neg st5 ld6 neg st6 ld7 neg st7
i8.neg! !u8.neg
i16.neg! !u16.neg
i32.neg! !u32.neg
i64.neg! !u64.neg
u4u4.neg! !u8.neg
u8u8.neg! !u16.neg
i4i4.neg! !i8.neg
i8i8.neg! !i16.neg
u4f4.neg! !u8.neg
u8f8.neg! !u16.neg
i4f4.neg! !i8.neg
i8f8.neg! !i16.neg

u8.lda! lda
u16.lda! ld0 lda swp inc lda
u32.lda! ld0 lda swp inc ld0 lda swp inc ld0 lda swp inc lda
u64.lda! ld0 lda swp inc ld0 lda swp inc ld0 lda swp inc ld0 lda swp inc ld0 lda swp inc ld0 lda swp inc ld0 lda swp inc lda
i8.lda! !u8.lda
i16.lda! !u16.lda
i32.lda! !u32.lda
i64.lda! !u64.lda
u4u4.lda! !u8.lda
u8u8.lda! !u16.lda
i4i4.lda! !i8.lda
i8i8.lda! !i16.lda
u4f4.lda! !u8.lda
u8f8.lda! !u16.lda
i4f4.lda! !i8.lda
i8f8.lda! !i16.lda
char.lda! !u8.lda

u8.sta! sta
u16.sta! swp ld1 sta inc sta
u32.sta! swp ld1 sta inc swp ld1 sta inc swp ld1 sta inc sta
u64.sta! swp ld1 sta inc swp ld1 sta inc swp ld1 sta inc swp ld1 sta inc swp ld1 sta inc swp ld1 sta inc swp ld1 sta inc sta
i8.sta! !u8.sta
i16.sta! !u16.sta
i32.sta! !u32.sta
i64.sta! !u64.sta
u4u4.sta! !u8.sta
u8u8.sta! !u16.sta
i4i4.sta! !i8.sta
i8i8.sta! !i16.sta
u4f4.sta! !u8.sta
u8f8.sta! !u16.sta
i4f4.sta! !i8.sta
i8f8.sta! !i16.sta
char.sta! !u8.sta

u8.ld0!   ld0
u8.ld0+1! ld1
u8.ld0+2! ld2
u8.ld0+3! ld3
u8.ld1!   ld1
u8.ld1+1! ld2
u8.ld1+2! ld3
u8.ld1+3! ld4
u8.ld2!   ld2
u8.ld2+1! ld3
u8.ld2+2! ld4
u8.ld2+3! ld5
u8.ld3!   ld3
u8.ld3+1! ld4
u8.ld3+2! ld5
u8.ld3+3! ld6
u8.ld4!   ld4
u8.ld4+1! ld5
u8.ld4+2! ld6
u8.ld4+3! ld7
u8.ld5!   ld5
u8.ld5+1! ld6
u8.ld5+2! ld7
u8.ld5+3! ld8
u8.ld6!   ld6
u8.ld6+1! ld7
u8.ld6+2! ld8
u8.ld6+3! ld9
u8.ld7!   ld7
u8.ld7+1! ld8
u8.ld7+2! ld9
u8.ld7+3! ldA
u8.ld8!   ld8
u8.ld8+1! ld9
u8.ld8+2! ldA
u8.ld8+3! ldB
u8.ld9!   ld9
u8.ld9+1! ldA
u8.ld9+2! ldB
u8.ld9+3! ldC
u8.ldA!   ldA
u8.ldA+1! ldB
u8.ldA+2! ldC
u8.ldA+3! ldD
u8.ldB!   ldB
u8.ldB+1! ldC
u8.ldB+2! ldD
u8.ldB+3! ldE
u8.ldC!   ldC
u8.ldC+1! ldD
u8.ldC+2! ldE
u8.ldC+3! ldF
u8.ldD!   ldD
u8.ldD+1! ldE
u8.ldD+2! ldF
u8.ldE!   ldE
u8.ldE+1! ldF
u8.ldF!   ldF
u16.ld0!   ld1 ld1
u16.ld0+1! ld2 ld2
u16.ld0+2! ld3 ld3
u16.ld0+3! ld4 ld4
u16.ld1!   ld3 ld3
u16.ld1+1! ld4 ld4
u16.ld1+2! ld5 ld5
u16.ld1+3! ld6 ld6
u16.ld2!   ld5 ld5
u16.ld2+1! ld6 ld6
u16.ld2+2! ld7 ld7
u16.ld2+3! ld8 ld8
u16.ld3!   ld7 ld7
u16.ld3+1! ld8 ld8
u16.ld3+2! ld9 ld9
u16.ld3+3! ldA ldA
u16.ld4!   ld9 ld9
u16.ld4+1! ldA ldA
u16.ld4+2! ldB ldB
u16.ld4+3! ldC ldC
u16.ld5!   ldB ldB
u16.ld5+1! ldC ldC
u16.ld5+2! ldD ldD
u16.ld5+3! ldE ldE
u16.ld6!   ldD ldD
u16.ld6+1! ldE ldE
u16.ld6+2! ldF ldF
u16.ld7!   ldF ldF
u32.ld0!   ld3 ld3 ld3 ld3
u32.ld0+1! ld4 ld4 ld4 ld4
u32.ld0+2! ld5 ld5 ld5 ld5
u32.ld0+3! ld6 ld6 ld6 ld6
u32.ld1!   ld7 ld7 ld7 ld7
u32.ld1+1! ld8 ld8 ld8 ld8
u32.ld1+2! ld9 ld9 ld9 ld9
u32.ld1+3! ldA ldA ldA ldA
u32.ld2!   ldB ldB ldB ldB
u32.ld2+1! ldC ldC ldC ldC
u32.ld2+2! ldD ldD ldD ldD
u32.ld2+3! ldE ldE ldE ldE
u32.ld3!   ldF ldF ldF ldF
u64.ld0!   ld7 ld7 ld7 ld7 ld7 ld7 ld7 ld7
u64.ld0+1! ld8 ld8 ld8 ld8 ld8 ld8 ld8 ld8
u64.ld0+2! ld9 ld9 ld9 ld9 ld9 ld9 ld9 ld9
u64.ld0+3! ldA ldA ldA ldA ldA ldA ldA ldA
u64.ld1!   ldF ldF ldF ldF ldF ldF ldF ldF
i8.ld0!   ld0
i8.ld0+1! ld1
i8.ld0+2! ld2
i8.ld0+3! ld3
i8.ld1!   ld1
i8.ld1+1! ld2
i8.ld1+2! ld3
i8.ld1+3! ld4
i8.ld2!   ld2
i8.ld2+1! ld3
i8.ld2+2! ld4
i8.ld2+3! ld5
i8.ld3!   ld3
i8.ld3+1! ld4
i8.ld3+2! ld5
i8.ld3+3! ld6
i8.ld4!   ld4
i8.ld4+1! ld5
i8.ld4+2! ld6
i8.ld4+3! ld7
i8.ld5!   ld5
i8.ld5+1! ld6
i8.ld5+2! ld7
i8.ld5+3! ld8
i8.ld6!   ld6
i8.ld6+1! ld7
i8.ld6+2! ld8
i8.ld6+3! ld9
i8.ld7!   ld7
i8.ld7+1! ld8
i8.ld7+2! ld9
i8.ld7+3! ldA
i8.ld8!   ld8
i8.ld8+1! ld9
i8.ld8+2! ldA
i8.ld8+3! ldB
i8.ld9!   ld9
i8.ld9+1! ldA
i8.ld9+2! ldB
i8.ld9+3! ldC
i8.ldA!   ldA
i8.ldA+1! ldB
i8.ldA+2! ldC
i8.ldA+3! ldD
i8.ldB!   ldB
i8.ldB+1! ldC
i8.ldB+2! ldD
i8.ldB+3! ldE
i8.ldC!   ldC
i8.ldC+1! ldD
i8.ldC+2! ldE
i8.ldC+3! ldF
i8.ldD!   ldD
i8.ldD+1! ldE
i8.ldD+2! ldF
i8.ldE!   ldE
i8.ldE+1! ldF
i8.ldF!   ldF
i16.ld0!   ld1 ld1
i16.ld0+1! ld2 ld2
i16.ld0+2! ld3 ld3
i16.ld0+3! ld4 ld4
i16.ld1!   ld3 ld3
i16.ld1+1! ld4 ld4
i16.ld1+2! ld5 ld5
i16.ld1+3! ld6 ld6
i16.ld2!   ld5 ld5
i16.ld2+1! ld6 ld6
i16.ld2+2! ld7 ld7
i16.ld2+3! ld8 ld8
i16.ld3!   ld7 ld7
i16.ld3+1! ld8 ld8
i16.ld3+2! ld9 ld9
i16.ld3+3! ldA ldA
i16.ld4!   ld9 ld9
i16.ld4+1! ldA ldA
i16.ld4+2! ldB ldB
i16.ld4+3! ldC ldC
i16.ld5!   ldB ldB
i16.ld5+1! ldC ldC
i16.ld5+2! ldD ldD
i16.ld5+3! ldE ldE
i16.ld6!   ldD ldD
i16.ld6+1! ldE ldE
i16.ld6+2! ldF ldF
i16.ld7!   ldF ldF
i32.ld0!   ld3 ld3 ld3 ld3
i32.ld0+1! ld4 ld4 ld4 ld4
i32.ld0+2! ld5 ld5 ld5 ld5
i32.ld0+3! ld6 ld6 ld6 ld6
i32.ld1!   ld7 ld7 ld7 ld7
i32.ld1+1! ld8 ld8 ld8 ld8
i32.ld1+2! ld9 ld9 ld9 ld9
i32.ld1+3! ldA ldA ldA ldA
i32.ld2!   ldB ldB ldB ldB
i32.ld2+1! ldC ldC ldC ldC
i32.ld2+2! ldD ldD ldD ldD
i32.ld2+3! ldE ldE ldE ldE
i32.ld3!   ldF ldF ldF ldF
i64.ld0!   ld7 ld7 ld7 ld7 ld7 ld7 ld7 ld7
i64.ld0+1! ld8 ld8 ld8 ld8 ld8 ld8 ld8 ld8
i64.ld0+2! ld9 ld9 ld9 ld9 ld9 ld9 ld9 ld9
i64.ld0+3! ldA ldA ldA ldA ldA ldA ldA ldA
i64.ld1!   ldF ldF ldF ldF ldF ldF ldF ldF
u4u4.ld0!   !u8.ld0
u4u4.ld0+1! !u8.ld0+1
u4u4.ld0+2! !u8.ld0+2
u4u4.ld0+3! !u8.ld0+3
u4u4.ld1!   !u8.ld1
u4u4.ld1+1! !u8.ld1+1
u4u4.ld1+2! !u8.ld1+2
u4u4.ld1+3! !u8.ld1+3
u4u4.ld2!   !u8.ld2
u4u4.ld2+1! !u8.ld2+1
u4u4.ld2+2! !u8.ld2+2
u4u4.ld2+3! !u8.ld2+3
u4u4.ld3!   !u8.ld3
u4u4.ld3+1! !u8.ld3+1
u4u4.ld3+2! !u8.ld3+2
u4u4.ld3+3! !u8.ld3+3
u4u4.ld4!   !u8.ld4
u4u4.ld4+1! !u8.ld4+1
u4u4.ld4+2! !u8.ld4+2
u4u4.ld4+3! !u8.ld4+3
u4u4.ld5!   !u8.ld5
u4u4.ld5+1! !u8.ld5+1
u4u4.ld5+2! !u8.ld5+2
u4u4.ld5+3! !u8.ld5+3
u4u4.ld6!   !u8.ld6
u4u4.ld6+1! !u8.ld6+1
u4u4.ld6+2! !u8.ld6+2
u4u4.ld6+3! !u8.ld6+3
u4u4.ld7!   !u8.ld7
u4u4.ld7+1! !u8.ld7+1
u4u4.ld7+2! !u8.ld7+2
u4u4.ld7+3! !u8.ld7+3
u4u4.ld8!   !u8.ld8
u4u4.ld8+1! !u8.ld8+1
u4u4.ld8+2! !u8.ld8+2
u4u4.ld8+3! !u8.ld8+3
u4u4.ld9!   !u8.ld9
u4u4.ld9+1! !u8.ld9+1
u4u4.ld9+2! !u8.ld9+2
u4u4.ld9+3! !u8.ld9+3
u4u4.ldA!   !u8.ldA
u4u4.ldA+1! !u8.ldA+1
u4u4.ldA+2! !u8.ldA+2
u4u4.ldA+3! !u8.ldA+3
u4u4.ldB!   !u8.ldB
u4u4.ldB+1! !u8.ldB+1
u4u4.ldB+2! !u8.ldB+2
u4u4.ldB+3! !u8.ldB+3
u4u4.ldC!   !u8.ldC
u4u4.ldC+1! !u8.ldC+1
u4u4.ldC+2! !u8.ldC+2
u4u4.ldC+3! !u8.ldC+3
u4u4.ldD!   !u8.ldD
u4u4.ldD+1! !u8.ldD+1
u4u4.ldD+2! !u8.ldD+2
u4u4.ldD+3! !u8.ldD+3
u4u4.ldE!   !u8.ldE
u4u4.ldE+1! !u8.ldE+1
u4u4.ldE+2! !u8.ldE+2
u4u4.ldE+3! !u8.ldE+3
u4u4.ldF!   !u8.ldF
u4u4.ldF+1! !u8.ldF+1
u4u4.ldF+2! !u8.ldF+2
u4u4.ldF+3! !u8.ldF+3
u8u8.ld0!   !u16.ld0
u8u8.ld0+1! !u16.ld0+1
u8u8.ld0+2! !u16.ld0+2
u8u8.ld0+3! !u16.ld0+3
u8u8.ld1!   !u16.ld1
u8u8.ld1+1! !u16.ld1+1
u8u8.ld1+2! !u16.ld1+2
u8u8.ld1+3! !u16.ld1+3
u8u8.ld2!   !u16.ld2
u8u8.ld2+1! !u16.ld2+1
u8u8.ld2+2! !u16.ld2+2
u8u8.ld2+3! !u16.ld2+3
u8u8.ld3!   !u16.ld3
u8u8.ld3+1! !u16.ld3+1
u8u8.ld3+2! !u16.ld3+2
u8u8.ld3+3! !u16.ld3+3
u8u8.ld4!   !u16.ld4
u8u8.ld4+1! !u16.ld4+1
u8u8.ld4+2! !u16.ld4+2
u8u8.ld4+3! !u16.ld4+3
u8u8.ld5!   !u16.ld5
u8u8.ld5+1! !u16.ld5+1
u8u8.ld5+2! !u16.ld5+2
u8u8.ld5+3! !u16.ld5+3
u8u8.ld6!   !u16.ld6
u8u8.ld6+1! !u16.ld6+1
u8u8.ld6+2! !u16.ld6+2
u8u8.ld7!   !u16.ld7
i4i4.ld0!   !i8.ld0
i4i4.ld0+1! !i8.ld0+1
i4i4.ld0+2! !i8.ld0+2
i4i4.ld0+3! !i8.ld0+3
i4i4.ld1!   !i8.ld1
i4i4.ld1+1! !i8.ld1+1
i4i4.ld1+2! !i8.ld1+2
i4i4.ld1+3! !i8.ld1+3
i4i4.ld2!   !i8.ld2
i4i4.ld2+1! !i8.ld2+1
i4i4.ld2+2! !i8.ld2+2
i4i4.ld2+3! !i8.ld2+3
i4i4.ld3!   !i8.ld3
i4i4.ld3+1! !i8.ld3+1
i4i4.ld3+2! !i8.ld3+2
i4i4.ld3+3! !i8.ld3+3
i4i4.ld4!   !i8.ld4
i4i4.ld4+1! !i8.ld4+1
i4i4.ld4+2! !i8.ld4+2
i4i4.ld4+3! !i8.ld4+3
i4i4.ld5!   !i8.ld5
i4i4.ld5+1! !i8.ld5+1
i4i4.ld5+2! !i8.ld5+2
i4i4.ld5+3! !i8.ld5+3
i4i4.ld6!   !i8.ld6
i4i4.ld6+1! !i8.ld6+1
i4i4.ld6+2! !i8.ld6+2
i4i4.ld6+3! !i8.ld6+3
i4i4.ld7!   !i8.ld7
i4i4.ld7+1! !i8.ld7+1
i4i4.ld7+2! !i8.ld7+2
i4i4.ld7+3! !i8.ld7+3
i4i4.ld8!   !i8.ld8
i4i4.ld8+1! !i8.ld8+1
i4i4.ld8+2! !i8.ld8+2
i4i4.ld8+3! !i8.ld8+3
i4i4.ld9!   !i8.ld9
i4i4.ld9+1! !i8.ld9+1
i4i4.ld9+2! !i8.ld9+2
i4i4.ld9+3! !i8.ld9+3
i4i4.ldA!   !i8.ldA
i4i4.ldA+1! !i8.ldA+1
i4i4.ldA+2! !i8.ldA+2
i4i4.ldA+3! !i8.ldA+3
i4i4.ldB!   !i8.ldB
i4i4.ldB+1! !i8.ldB+1
i4i4.ldB+2! !i8.ldB+2
i4i4.ldB+3! !i8.ldB+3
i4i4.ldC!   !i8.ldC
i4i4.ldC+1! !i8.ldC+1
i4i4.ldC+2! !i8.ldC+2
i4i4.ldC+3! !i8.ldC+3
i4i4.ldD!   !i8.ldD
i4i4.ldD+1! !i8.ldD+1
i4i4.ldD+2! !i8.ldD+2
i4i4.ldD+3! !i8.ldD+3
i4i4.ldE!   !i8.ldE
i4i4.ldE+1! !i8.ldE+1
i4i4.ldE+2! !i8.ldE+2
i4i4.ldE+3! !i8.ldE+3
i4i4.ldF!   !i8.ldF
i4i4.ldF+1! !i8.ldF+1
i4i4.ldF+2! !i8.ldF+2
i4i4.ldF+3! !i8.ldF+3
i8i8.ld0!   !i16.ld0
i8i8.ld0+1! !i16.ld0+1
i8i8.ld0+2! !i16.ld0+2
i8i8.ld0+3! !i16.ld0+3
i8i8.ld1!   !i16.ld1
i8i8.ld1+1! !i16.ld1+1
i8i8.ld1+2! !i16.ld1+2
i8i8.ld1+3! !i16.ld1+3
i8i8.ld2!   !i16.ld2
i8i8.ld2+1! !i16.ld2+1
i8i8.ld2+2! !i16.ld2+2
i8i8.ld2+3! !i16.ld2+3
i8i8.ld3!   !i16.ld3
i8i8.ld3+1! !i16.ld3+1
i8i8.ld3+2! !i16.ld3+2
i8i8.ld3+3! !i16.ld3+3
i8i8.ld4!   !i16.ld4
i8i8.ld4+1! !i16.ld4+1
i8i8.ld4+2! !i16.ld4+2
i8i8.ld4+3! !i16.ld4+3
i8i8.ld5!   !i16.ld5
i8i8.ld5+1! !i16.ld5+1
i8i8.ld5+2! !i16.ld5+2
i8i8.ld5+3! !i16.ld5+3
i8i8.ld6!   !i16.ld6
i8i8.ld6+1! !i16.ld6+1
i8i8.ld6+2! !i16.ld6+2
i8i8.ld7!   !i16.ld7
u4f4.ld0!   !u8.ld0
u4f4.ld0+1! !u8.ld0+1
u4f4.ld0+2! !u8.ld0+2
u4f4.ld0+3! !u8.ld0+3
u4f4.ld1!   !u8.ld1
u4f4.ld1+1! !u8.ld1+1
u4f4.ld1+2! !u8.ld1+2
u4f4.ld1+3! !u8.ld1+3
u4f4.ld2!   !u8.ld2
u4f4.ld2+1! !u8.ld2+1
u4f4.ld2+2! !u8.ld2+2
u4f4.ld2+3! !u8.ld2+3
u4f4.ld3!   !u8.ld3
u4f4.ld3+1! !u8.ld3+1
u4f4.ld3+2! !u8.ld3+2
u4f4.ld3+3! !u8.ld3+3
u4f4.ld4!   !u8.ld4
u4f4.ld4+1! !u8.ld4+1
u4f4.ld4+2! !u8.ld4+2
u4f4.ld4+3! !u8.ld4+3
u4f4.ld5!   !u8.ld5
u4f4.ld5+1! !u8.ld5+1
u4f4.ld5+2! !u8.ld5+2
u4f4.ld5+3! !u8.ld5+3
u4f4.ld6!   !u8.ld6
u4f4.ld6+1! !u8.ld6+1
u4f4.ld6+2! !u8.ld6+2
u4f4.ld6+3! !u8.ld6+3
u4f4.ld7!   !u8.ld7
u4f4.ld7+1! !u8.ld7+1
u4f4.ld7+2! !u8.ld7+2
u4f4.ld7+3! !u8.ld7+3
u4f4.ld8!   !u8.ld8
u4f4.ld8+1! !u8.ld8+1
u4f4.ld8+2! !u8.ld8+2
u4f4.ld8+3! !u8.ld8+3
u4f4.ld9!   !u8.ld9
u4f4.ld9+1! !u8.ld9+1
u4f4.ld9+2! !u8.ld9+2
u4f4.ld9+3! !u8.ld9+3
u4f4.ldA!   !u8.ldA
u4f4.ldA+1! !u8.ldA+1
u4f4.ldA+2! !u8.ldA+2
u4f4.ldA+3! !u8.ldA+3
u4f4.ldB!   !u8.ldB
u4f4.ldB+1! !u8.ldB+1
u4f4.ldB+2! !u8.ldB+2
u4f4.ldB+3! !u8.ldB+3
u4f4.ldC!   !u8.ldC
u4f4.ldC+1! !u8.ldC+1
u4f4.ldC+2! !u8.ldC+2
u4f4.ldC+3! !u8.ldC+3
u4f4.ldD!   !u8.ldD
u4f4.ldD+1! !u8.ldD+1
u4f4.ldD+2! !u8.ldD+2
u4f4.ldD+3! !u8.ldD+3
u4f4.ldE!   !u8.ldE
u4f4.ldE+1! !u8.ldE+1
u4f4.ldE+2! !u8.ldE+2
u4f4.ldE+3! !u8.ldE+3
u4f4.ldF!   !u8.ldF
u4f4.ldF+1! !u8.ldF+1
u4f4.ldF+2! !u8.ldF+2
u4f4.ldF+3! !u8.ldF+3
u8f8.ld0!   !u16.ld0
u8f8.ld0+1! !u16.ld0+1
u8f8.ld0+2! !u16.ld0+2
u8f8.ld0+3! !u16.ld0+3
u8f8.ld1!   !u16.ld1
u8f8.ld1+1! !u16.ld1+1
u8f8.ld1+2! !u16.ld1+2
u8f8.ld1+3! !u16.ld1+3
u8f8.ld2!   !u16.ld2
u8f8.ld2+1! !u16.ld2+1
u8f8.ld2+2! !u16.ld2+2
u8f8.ld2+3! !u16.ld2+3
u8f8.ld3!   !u16.ld3
u8f8.ld3+1! !u16.ld3+1
u8f8.ld3+2! !u16.ld3+2
u8f8.ld3+3! !u16.ld3+3
u8f8.ld4!   !u16.ld4
u8f8.ld4+1! !u16.ld4+1
u8f8.ld4+2! !u16.ld4+2
u8f8.ld4+3! !u16.ld4+3
u8f8.ld5!   !u16.ld5
u8f8.ld5+1! !u16.ld5+1
u8f8.ld5+2! !u16.ld5+2
u8f8.ld5+3! !u16.ld5+3
u8f8.ld6!   !u16.ld6
u8f8.ld6+1! !u16.ld6+1
u8f8.ld6+2! !u16.ld6+2
u8f8.ld7!   !u16.ld7
i4f4.ld0!   !i8.ld0
i4f4.ld0+1! !i8.ld0+1
i4f4.ld0+2! !i8.ld0+2
i4f4.ld0+3! !i8.ld0+3
i4f4.ld1!   !i8.ld1
i4f4.ld1+1! !i8.ld1+1
i4f4.ld1+2! !i8.ld1+2
i4f4.ld1+3! !i8.ld1+3
i4f4.ld2!   !i8.ld2
i4f4.ld2+1! !i8.ld2+1
i4f4.ld2+2! !i8.ld2+2
i4f4.ld2+3! !i8.ld2+3
i4f4.ld3!   !i8.ld3
i4f4.ld3+1! !i8.ld3+1
i4f4.ld3+2! !i8.ld3+2
i4f4.ld3+3! !i8.ld3+3
i4f4.ld4!   !i8.ld4
i4f4.ld4+1! !i8.ld4+1
i4f4.ld4+2! !i8.ld4+2
i4f4.ld4+3! !i8.ld4+3
i4f4.ld5!   !i8.ld5
i4f4.ld5+1! !i8.ld5+1
i4f4.ld5+2! !i8.ld5+2
i4f4.ld5+3! !i8.ld5+3
i4f4.ld6!   !i8.ld6
i4f4.ld6+1! !i8.ld6+1
i4f4.ld6+2! !i8.ld6+2
i4f4.ld6+3! !i8.ld6+3
i4f4.ld7!   !i8.ld7
i4f4.ld7+1! !i8.ld7+1
i4f4.ld7+2! !i8.ld7+2
i4f4.ld7+3! !i8.ld7+3
i4f4.ld8!   !i8.ld8
i4f4.ld8+1! !i8.ld8+1
i4f4.ld8+2! !i8.ld8+2
i4f4.ld8+3! !i8.ld8+3
i4f4.ld9!   !i8.ld9
i4f4.ld9+1! !i8.ld9+1
i4f4.ld9+2! !i8.ld9+2
i4f4.ld9+3! !i8.ld9+3
i4f4.ldA!   !i8.ldA
i4f4.ldA+1! !i8.ldA+1
i4f4.ldA+2! !i8.ldA+2
i4f4.ldA+3! !i8.ldA+3
i4f4.ldB!   !i8.ldB
i4f4.ldB+1! !i8.ldB+1
i4f4.ldB+2! !i8.ldB+2
i4f4.ldB+3! !i8.ldB+3
i4f4.ldC!   !i8.ldC
i4f4.ldC+1! !i8.ldC+1
i4f4.ldC+2! !i8.ldC+2
i4f4.ldC+3! !i8.ldC+3
i4f4.ldD!   !i8.ldD
i4f4.ldD+1! !i8.ldD+1
i4f4.ldD+2! !i8.ldD+2
i4f4.ldD+3! !i8.ldD+3
i4f4.ldE!   !i8.ldE
i4f4.ldE+1! !i8.ldE+1
i4f4.ldE+2! !i8.ldE+2
i4f4.ldE+3! !i8.ldE+3
i4f4.ldF!   !i8.ldF
i4f4.ldF+1! !i8.ldF+1
i4f4.ldF+2! !i8.ldF+2
i4f4.ldF+3! !i8.ldF+3
i8f8.ld0!   !i16.ld0
i8f8.ld0+1! !i16.ld0+1
i8f8.ld0+2! !i16.ld0+2
i8f8.ld0+3! !i16.ld0+3
i8f8.ld1!   !i16.ld1
i8f8.ld1+1! !i16.ld1+1
i8f8.ld1+2! !i16.ld1+2
i8f8.ld1+3! !i16.ld1+3
i8f8.ld2!   !i16.ld2
i8f8.ld2+1! !i16.ld2+1
i8f8.ld2+2! !i16.ld2+2
i8f8.ld2+3! !i16.ld2+3
i8f8.ld3!   !i16.ld3
i8f8.ld3+1! !i16.ld3+1
i8f8.ld3+2! !i16.ld3+2
i8f8.ld3+3! !i16.ld3+3
i8f8.ld4!   !i16.ld4
i8f8.ld4+1! !i16.ld4+1
i8f8.ld4+2! !i16.ld4+2
i8f8.ld4+3! !i16.ld4+3
i8f8.ld5!   !i16.ld5
i8f8.ld5+1! !i16.ld5+1
i8f8.ld5+2! !i16.ld5+2
i8f8.ld5+3! !i16.ld5+3
i8f8.ld6!   !i16.ld6
i8f8.ld6+1! !i16.ld6+1
i8f8.ld6+2! !i16.ld6+2
i8f8.ld7!   !i16.ld7
c4f4m4f4.ld0!   !i16.ld0
c4f4m4f4.ld0+1! !i16.ld0+1
c4f4m4f4.ld0+2! !i16.ld0+2
c4f4m4f4.ld0+3! !i16.ld0+3
c4f4m4f4.ld1!   !i16.ld1
c4f4m4f4.ld1+1! !i16.ld1+1
c4f4m4f4.ld1+2! !i16.ld1+2
c4f4m4f4.ld1+3! !i16.ld1+3
c4f4m4f4.ld2!   !i16.ld2
c4f4m4f4.ld2+1! !i16.ld2+1
c4f4m4f4.ld2+2! !i16.ld2+2
c4f4m4f4.ld2+3! !i16.ld2+3
c4f4m4f4.ld3!   !i16.ld3
c4f4m4f4.ld3+1! !i16.ld3+1
c4f4m4f4.ld3+2! !i16.ld3+2
c4f4m4f4.ld3+3! !i16.ld3+3
c4f4m4f4.ld4!   !i16.ld4
c4f4m4f4.ld4+1! !i16.ld4+1
c4f4m4f4.ld4+2! !i16.ld4+2
c4f4m4f4.ld4+3! !i16.ld4+3
c4f4m4f4.ld5!   !i16.ld5
c4f4m4f4.ld5+1! !i16.ld5+1
c4f4m4f4.ld5+2! !i16.ld5+2
c4f4m4f4.ld5+3! !i16.ld5+3
c4f4m4f4.ld6!   !i16.ld6
c4f4m4f4.ld6+1! !i16.ld6+1
c4f4m4f4.ld6+2! !i16.ld6+2
c4f4m4f4.ld7!   !i16.ld7
c8f8m8f8.ld0!   !i32.ld0
c8f8m8f8.ld0+1! !i32.ld0+1
c8f8m8f8.ld0+2! !i32.ld0+2
c8f8m8f8.ld0+3! !i32.ld0+3
c8f8m8f8.ld1!   !i32.ld1
c8f8m8f8.ld1+1! !i32.ld1+1
c8f8m8f8.ld1+2! !i32.ld1+2
c8f8m8f8.ld1+3! !i32.ld1+3
c8f8m8f8.ld2!   !i32.ld2
c8f8m8f8.ld2+1! !i32.ld2+1
c8f8m8f8.ld2+2! !i32.ld2+2
c8f8m8f8.ld2+3! !i32.ld2+3
c8f8m8f8.ld3!   !i32.ld3
char.ld0!   !u8.ld0
char.ld0+1! !u8.ld0+1
char.ld0+2! !u8.ld0+2
char.ld0+3! !u8.ld0+3
char.ld1!   !u8.ld1
char.ld1+1! !u8.ld1+1
char.ld1+2! !u8.ld1+2
char.ld1+3! !u8.ld1+3
char.ld2!   !u8.ld2
char.ld2+1! !u8.ld2+1
char.ld2+2! !u8.ld2+2
char.ld2+3! !u8.ld2+3
char.ld3!   !u8.ld3
char.ld3+1! !u8.ld3+1
char.ld3+2! !u8.ld3+2
char.ld3+3! !u8.ld3+3
char.ld4!   !u8.ld4
char.ld4+1! !u8.ld4+1
char.ld4+2! !u8.ld4+2
char.ld4+3! !u8.ld4+3
char.ld5!   !u8.ld5
char.ld5+1! !u8.ld5+1
char.ld5+2! !u8.ld5+2
char.ld5+3! !u8.ld5+3
char.ld6!   !u8.ld6
char.ld6+1! !u8.ld6+1
char.ld6+2! !u8.ld6+2
char.ld6+3! !u8.ld6+3
char.ld7!   !u8.ld7
char.ld7+1! !u8.ld7+1
char.ld7+2! !u8.ld7+2
char.ld7+3! !u8.ld7+3
char.ld8!   !u8.ld8
char.ld8+1! !u8.ld8+1
char.ld8+2! !u8.ld8+2
char.ld8+3! !u8.ld8+3
char.ld9!   !u8.ld9
char.ld9+1! !u8.ld9+1
char.ld9+2! !u8.ld9+2
char.ld9+3! !u8.ld9+3
char.ldA!   !u8.ldA
char.ldA+1! !u8.ldA+1
char.ldA+2! !u8.ldA+2
char.ldA+3! !u8.ldA+3
char.ldB!   !u8.ldB
char.ldB+1! !u8.ldB+1
char.ldB+2! !u8.ldB+2
char.ldB+3! !u8.ldB+3
char.ldC!   !u8.ldC
char.ldC+1! !u8.ldC+1
char.ldC+2! !u8.ldC+2
char.ldC+3! !u8.ldC+3
char.ldD!   !u8.ldD
char.ldD+1! !u8.ldD+1
char.ldD+2! !u8.ldD+2
char.ldD+3! !u8.ldD+3
char.ldE!   !u8.ldE
char.ldE+1! !u8.ldE+1
char.ldE+2! !u8.ldE+2
char.ldE+3! !u8.ldE+3
char.ldF!   !u8.ldF
char.ldF+1! !u8.ldF+1
char.ldF+2! !u8.ldF+2
char.ldF+3! !u8.ldF+3

u8.st0!   st0
u8.st0+1! st1
u8.st0+2! st2
u8.st0+3! st3
u8.st1!   st1
u8.st1+1! st2
u8.st1+2! st3
u8.st1+3! st4
u8.st2!   st2
u8.st2+1! st3
u8.st2+2! st4
u8.st2+3! st5
u8.st3!   st3
u8.st3+1! st4
u8.st3+2! st5
u8.st3+3! st6
u8.st4!   st4
u8.st4+1! st5
u8.st4+2! st6
u8.st4+3! st7
u8.st5!   st5
u8.st5+1! st6
u8.st5+2! st7
u8.st5+3! st8
u8.st6!   st6
u8.st6+1! st7
u8.st6+2! st8
u8.st6+3! st9
u8.st7!   st7
u8.st7+1! st8
u8.st7+2! st9
u8.st7+3! stA
u8.st8!   st8
u8.st8+1! st9
u8.st8+2! stA
u8.st8+3! stB
u8.st9!   st9
u8.st9+1! stA
u8.st9+2! stB
u8.st9+3! stC
u8.stA!   stA
u8.stA+1! stB
u8.stA+2! stC
u8.stA+3! stD
u8.stB!   stB
u8.stB+1! stC
u8.stB+2! stD
u8.stB+3! stE
u8.stC!   stC
u8.stC+1! stD
u8.stC+2! stE
u8.stC+3! stF
u8.stD!   stD
u8.stD+1! stE
u8.stD+2! stF
u8.stE!   stE
u8.stE+1! stF
u8.stF!   stF
u16.st0!   st1 st1
u16.st0+1! st2 st2
u16.st0+2! st3 st3
u16.st0+3! st4 st4
u16.st1!   st3 st3
u16.st1+1! st4 st4
u16.st1+2! st5 st5
u16.st1+3! st6 st6
u16.st2!   st5 st5
u16.st2+1! st6 st6
u16.st2+2! st7 st7
u16.st2+3! st8 st8
u16.st3!   st7 st7
u16.st3+1! st8 st8
u16.st3+2! st9 st9
u16.st3+3! stA stA
u16.st4!   st9 st9
u16.st4+1! stA stA
u16.st4+2! stB stB
u16.st4+3! stC stC
u16.st5!   stB stB
u16.st5+1! stC stC
u16.st5+2! stD stD
u16.st5+3! stE stE
u16.st6!   stD stD
u16.st6+1! stE stE
u16.st6+2! stF stF
u16.st7!   stF stF
u32.st0!   st3 st3 st3 st3
u32.st0+1! st4 st4 st4 st4
u32.st0+2! st5 st5 st5 st5
u32.st0+3! st6 st6 st6 st6
u32.st1!   st7 st7 st7 st7
u32.st1+1! st8 st8 st8 st8
u32.st1+2! st9 st9 st9 st9
u32.st1+3! stA stA stA stA
u32.st2!   stB stB stB stB
u32.st2+1! stC stC stC stC
u32.st2+2! stD stD stD stD
u32.st2+3! stE stE stE stE
u32.st3!   stF stF stF stF
u64.st0!   st7 st7 st7 st7 st7 st7 st7 st7
u64.st0+1! st8 st8 st8 st8 st8 st8 st8 st8
u64.st0+2! st9 st9 st9 st9 st9 st9 st9 st9
u64.st0+3! stA stA stA stA stA stA stA stA
u64.st1!   stF stF stF stF stF stF stF stF
i8.st0!   st0
i8.st0+1! st1
i8.st0+2! st2
i8.st0+3! st3
i8.st1!   st1
i8.st1+1! st2
i8.st1+2! st3
i8.st1+3! st4
i8.st2!   st2
i8.st2+1! st3
i8.st2+2! st4
i8.st2+3! st5
i8.st3!   st3
i8.st3+1! st4
i8.st3+2! st5
i8.st3+3! st6
i8.st4!   st4
i8.st4+1! st5
i8.st4+2! st6
i8.st4+3! st7
i8.st5!   st5
i8.st5+1! st6
i8.st5+2! st7
i8.st5+3! st8
i8.st6!   st6
i8.st6+1! st7
i8.st6+2! st8
i8.st6+3! st9
i8.st7!   st7
i8.st7+1! st8
i8.st7+2! st9
i8.st7+3! stA
i8.st8!   st8
i8.st8+1! st9
i8.st8+2! stA
i8.st8+3! stB
i8.st9!   st9
i8.st9+1! stA
i8.st9+2! stB
i8.st9+3! stC
i8.stA!   stA
i8.stA+1! stB
i8.stA+2! stC
i8.stA+3! stD
i8.stB!   stB
i8.stB+1! stC
i8.stB+2! stD
i8.stB+3! stE
i8.stC!   stC
i8.stC+1! stD
i8.stC+2! stE
i8.stC+3! stF
i8.stD!   stD
i8.stD+1! stE
i8.stD+2! stF
i8.stE!   stE
i8.stE+1! stF
i8.stF!   stF
i16.st0!   st1 st1
i16.st0+1! st2 st2
i16.st0+2! st3 st3
i16.st0+3! st4 st4
i16.st1!   st3 st3
i16.st1+1! st4 st4
i16.st1+2! st5 st5
i16.st1+3! st6 st6
i16.st2!   st5 st5
i16.st2+1! st6 st6
i16.st2+2! st7 st7
i16.st2+3! st8 st8
i16.st3!   st7 st7
i16.st3+1! st8 st8
i16.st3+2! st9 st9
i16.st3+3! stA stA
i16.st4!   st9 st9
i16.st4+1! stA stA
i16.st4+2! stB stB
i16.st4+3! stC stC
i16.st5!   stB stB
i16.st5+1! stC stC
i16.st5+2! stD stD
i16.st5+3! stE stE
i16.st6!   stD stD
i16.st6+1! stE stE
i16.st6+2! stF stF
i16.st7!   stF stF
i32.st0!   st3 st3 st3 st3
i32.st0+1! st4 st4 st4 st4
i32.st0+2! st5 st5 st5 st5
i32.st0+3! st6 st6 st6 st6
i32.st1!   st7 st7 st7 st7
i32.st1+1! st8 st8 st8 st8
i32.st1+2! st9 st9 st9 st9
i32.st1+3! stA stA stA stA
i32.st2!   stB stB stB stB
i32.st2+1! stC stC stC stC
i32.st2+2! stD stD stD stD
i32.st2+3! stE stE stE stE
i32.st3!   stF stF stF stF
i64.st0!   st7 st7 st7 st7 st7 st7 st7 st7
i64.st0+1! st8 st8 st8 st8 st8 st8 st8 st8
i64.st0+2! st9 st9 st9 st9 st9 st9 st9 st9
i64.st0+3! stA stA stA stA stA stA stA stA
i64.st1!   stF stF stF stF stF stF stF stF
u4u4.st0!   !u8.st0
u4u4.st0+1! !u8.st0+1
u4u4.st0+2! !u8.st0+2
u4u4.st0+3! !u8.st0+3
u4u4.st1!   !u8.st1
u4u4.st1+1! !u8.st1+1
u4u4.st1+2! !u8.st1+2
u4u4.st1+3! !u8.st1+3
u4u4.st2!   !u8.st2
u4u4.st2+1! !u8.st2+1
u4u4.st2+2! !u8.st2+2
u4u4.st2+3! !u8.st2+3
u4u4.st3!   !u8.st3
u4u4.st3+1! !u8.st3+1
u4u4.st3+2! !u8.st3+2
u4u4.st3+3! !u8.st3+3
u4u4.st4!   !u8.st4
u4u4.st4+1! !u8.st4+1
u4u4.st4+2! !u8.st4+2
u4u4.st4+3! !u8.st4+3
u4u4.st5!   !u8.st5
u4u4.st5+1! !u8.st5+1
u4u4.st5+2! !u8.st5+2
u4u4.st5+3! !u8.st5+3
u4u4.st6!   !u8.st6
u4u4.st6+1! !u8.st6+1
u4u4.st6+2! !u8.st6+2
u4u4.st6+3! !u8.st6+3
u4u4.st7!   !u8.st7
u4u4.st7+1! !u8.st7+1
u4u4.st7+2! !u8.st7+2
u4u4.st7+3! !u8.st7+3
u4u4.st8!   !u8.st8
u4u4.st8+1! !u8.st8+1
u4u4.st8+2! !u8.st8+2
u4u4.st8+3! !u8.st8+3
u4u4.st9!   !u8.st9
u4u4.st9+1! !u8.st9+1
u4u4.st9+2! !u8.st9+2
u4u4.st9+3! !u8.st9+3
u4u4.stA!   !u8.stA
u4u4.stA+1! !u8.stA+1
u4u4.stA+2! !u8.stA+2
u4u4.stA+3! !u8.stA+3
u4u4.stB!   !u8.stB
u4u4.stB+1! !u8.stB+1
u4u4.stB+2! !u8.stB+2
u4u4.stB+3! !u8.stB+3
u4u4.stC!   !u8.stC
u4u4.stC+1! !u8.stC+1
u4u4.stC+2! !u8.stC+2
u4u4.stC+3! !u8.stC+3
u4u4.stD!   !u8.stD
u4u4.stD+1! !u8.stD+1
u4u4.stD+2! !u8.stD+2
u4u4.stD+3! !u8.stD+3
u4u4.stE!   !u8.stE
u4u4.stE+1! !u8.stE+1
u4u4.stE+2! !u8.stE+2
u4u4.stE+3! !u8.stE+3
u4u4.stF!   !u8.stF
u4u4.stF+1! !u8.stF+1
u4u4.stF+2! !u8.stF+2
u4u4.stF+3! !u8.stF+3
u8u8.st0!   !u16.st0
u8u8.st0+1! !u16.st0+1
u8u8.st0+2! !u16.st0+2
u8u8.st0+3! !u16.st0+3
u8u8.st1!   !u16.st1
u8u8.st1+1! !u16.st1+1
u8u8.st1+2! !u16.st1+2
u8u8.st1+3! !u16.st1+3
u8u8.st2!   !u16.st2
u8u8.st2+1! !u16.st2+1
u8u8.st2+2! !u16.st2+2
u8u8.st2+3! !u16.st2+3
u8u8.st3!   !u16.st3
u8u8.st3+1! !u16.st3+1
u8u8.st3+2! !u16.st3+2
u8u8.st3+3! !u16.st3+3
u8u8.st4!   !u16.st4
u8u8.st4+1! !u16.st4+1
u8u8.st4+2! !u16.st4+2
u8u8.st4+3! !u16.st4+3
u8u8.st5!   !u16.st5
u8u8.st5+1! !u16.st5+1
u8u8.st5+2! !u16.st5+2
u8u8.st5+3! !u16.st5+3
u8u8.st6!   !u16.st6
u8u8.st6+1! !u16.st6+1
u8u8.st6+2! !u16.st6+2
u8u8.st7!   !u16.st7
i4i4.st0!   !i8.st0
i4i4.st0+1! !i8.st0+1
i4i4.st0+2! !i8.st0+2
i4i4.st0+3! !i8.st0+3
i4i4.st1!   !i8.st1
i4i4.st1+1! !i8.st1+1
i4i4.st1+2! !i8.st1+2
i4i4.st1+3! !i8.st1+3
i4i4.st2!   !i8.st2
i4i4.st2+1! !i8.st2+1
i4i4.st2+2! !i8.st2+2
i4i4.st2+3! !i8.st2+3
i4i4.st3!   !i8.st3
i4i4.st3+1! !i8.st3+1
i4i4.st3+2! !i8.st3+2
i4i4.st3+3! !i8.st3+3
i4i4.st4!   !i8.st4
i4i4.st4+1! !i8.st4+1
i4i4.st4+2! !i8.st4+2
i4i4.st4+3! !i8.st4+3
i4i4.st5!   !i8.st5
i4i4.st5+1! !i8.st5+1
i4i4.st5+2! !i8.st5+2
i4i4.st5+3! !i8.st5+3
i4i4.st6!   !i8.st6
i4i4.st6+1! !i8.st6+1
i4i4.st6+2! !i8.st6+2
i4i4.st6+3! !i8.st6+3
i4i4.st7!   !i8.st7
i4i4.st7+1! !i8.st7+1
i4i4.st7+2! !i8.st7+2
i4i4.st7+3! !i8.st7+3
i4i4.st8!   !i8.st8
i4i4.st8+1! !i8.st8+1
i4i4.st8+2! !i8.st8+2
i4i4.st8+3! !i8.st8+3
i4i4.st9!   !i8.st9
i4i4.st9+1! !i8.st9+1
i4i4.st9+2! !i8.st9+2
i4i4.st9+3! !i8.st9+3
i4i4.stA!   !i8.stA
i4i4.stA+1! !i8.stA+1
i4i4.stA+2! !i8.stA+2
i4i4.stA+3! !i8.stA+3
i4i4.stB!   !i8.stB
i4i4.stB+1! !i8.stB+1
i4i4.stB+2! !i8.stB+2
i4i4.stB+3! !i8.stB+3
i4i4.stC!   !i8.stC
i4i4.stC+1! !i8.stC+1
i4i4.stC+2! !i8.stC+2
i4i4.stC+3! !i8.stC+3
i4i4.stD!   !i8.stD
i4i4.stD+1! !i8.stD+1
i4i4.stD+2! !i8.stD+2
i4i4.stD+3! !i8.stD+3
i4i4.stE!   !i8.stE
i4i4.stE+1! !i8.stE+1
i4i4.stE+2! !i8.stE+2
i4i4.stE+3! !i8.stE+3
i4i4.stF!   !i8.stF
i4i4.stF+1! !i8.stF+1
i4i4.stF+2! !i8.stF+2
i4i4.stF+3! !i8.stF+3
i8i8.st0!   !i16.st0
i8i8.st0+1! !i16.st0+1
i8i8.st0+2! !i16.st0+2
i8i8.st0+3! !i16.st0+3
i8i8.st1!   !i16.st1
i8i8.st1+1! !i16.st1+1
i8i8.st1+2! !i16.st1+2
i8i8.st1+3! !i16.st1+3
i8i8.st2!   !i16.st2
i8i8.st2+1! !i16.st2+1
i8i8.st2+2! !i16.st2+2
i8i8.st2+3! !i16.st2+3
i8i8.st3!   !i16.st3
i8i8.st3+1! !i16.st3+1
i8i8.st3+2! !i16.st3+2
i8i8.st3+3! !i16.st3+3
i8i8.st4!   !i16.st4
i8i8.st4+1! !i16.st4+1
i8i8.st4+2! !i16.st4+2
i8i8.st4+3! !i16.st4+3
i8i8.st5!   !i16.st5
i8i8.st5+1! !i16.st5+1
i8i8.st5+2! !i16.st5+2
i8i8.st5+3! !i16.st5+3
i8i8.st6!   !i16.st6
i8i8.st6+1! !i16.st6+1
i8i8.st6+2! !i16.st6+2
i8i8.st7!   !i16.st7
u4f4.st0!   !u8.st0
u4f4.st0+1! !u8.st0+1
u4f4.st0+2! !u8.st0+2
u4f4.st0+3! !u8.st0+3
u4f4.st1!   !u8.st1
u4f4.st1+1! !u8.st1+1
u4f4.st1+2! !u8.st1+2
u4f4.st1+3! !u8.st1+3
u4f4.st2!   !u8.st2
u4f4.st2+1! !u8.st2+1
u4f4.st2+2! !u8.st2+2
u4f4.st2+3! !u8.st2+3
u4f4.st3!   !u8.st3
u4f4.st3+1! !u8.st3+1
u4f4.st3+2! !u8.st3+2
u4f4.st3+3! !u8.st3+3
u4f4.st4!   !u8.st4
u4f4.st4+1! !u8.st4+1
u4f4.st4+2! !u8.st4+2
u4f4.st4+3! !u8.st4+3
u4f4.st5!   !u8.st5
u4f4.st5+1! !u8.st5+1
u4f4.st5+2! !u8.st5+2
u4f4.st5+3! !u8.st5+3
u4f4.st6!   !u8.st6
u4f4.st6+1! !u8.st6+1
u4f4.st6+2! !u8.st6+2
u4f4.st6+3! !u8.st6+3
u4f4.st7!   !u8.st7
u4f4.st7+1! !u8.st7+1
u4f4.st7+2! !u8.st7+2
u4f4.st7+3! !u8.st7+3
u4f4.st8!   !u8.st8
u4f4.st8+1! !u8.st8+1
u4f4.st8+2! !u8.st8+2
u4f4.st8+3! !u8.st8+3
u4f4.st9!   !u8.st9
u4f4.st9+1! !u8.st9+1
u4f4.st9+2! !u8.st9+2
u4f4.st9+3! !u8.st9+3
u4f4.stA!   !u8.stA
u4f4.stA+1! !u8.stA+1
u4f4.stA+2! !u8.stA+2
u4f4.stA+3! !u8.stA+3
u4f4.stB!   !u8.stB
u4f4.stB+1! !u8.stB+1
u4f4.stB+2! !u8.stB+2
u4f4.stB+3! !u8.stB+3
u4f4.stC!   !u8.stC
u4f4.stC+1! !u8.stC+1
u4f4.stC+2! !u8.stC+2
u4f4.stC+3! !u8.stC+3
u4f4.stD!   !u8.stD
u4f4.stD+1! !u8.stD+1
u4f4.stD+2! !u8.stD+2
u4f4.stD+3! !u8.stD+3
u4f4.stE!   !u8.stE
u4f4.stE+1! !u8.stE+1
u4f4.stE+2! !u8.stE+2
u4f4.stE+3! !u8.stE+3
u4f4.stF!   !u8.stF
u4f4.stF+1! !u8.stF+1
u4f4.stF+2! !u8.stF+2
u4f4.stF+3! !u8.stF+3
u8f8.st0!   !u16.st0
u8f8.st0+1! !u16.st0+1
u8f8.st0+2! !u16.st0+2
u8f8.st0+3! !u16.st0+3
u8f8.st1!   !u16.st1
u8f8.st1+1! !u16.st1+1
u8f8.st1+2! !u16.st1+2
u8f8.st1+3! !u16.st1+3
u8f8.st2!   !u16.st2
u8f8.st2+1! !u16.st2+1
u8f8.st2+2! !u16.st2+2
u8f8.st2+3! !u16.st2+3
u8f8.st3!   !u16.st3
u8f8.st3+1! !u16.st3+1
u8f8.st3+2! !u16.st3+2
u8f8.st3+3! !u16.st3+3
u8f8.st4!   !u16.st4
u8f8.st4+1! !u16.st4+1
u8f8.st4+2! !u16.st4+2
u8f8.st4+3! !u16.st4+3
u8f8.st5!   !u16.st5
u8f8.st5+1! !u16.st5+1
u8f8.st5+2! !u16.st5+2
u8f8.st5+3! !u16.st5+3
u8f8.st6!   !u16.st6
u8f8.st6+1! !u16.st6+1
u8f8.st6+2! !u16.st6+2
u8f8.st7!   !u16.st7
i4f4.st0!   !i8.st0
i4f4.st0+1! !i8.st0+1
i4f4.st0+2! !i8.st0+2
i4f4.st0+3! !i8.st0+3
i4f4.st1!   !i8.st1
i4f4.st1+1! !i8.st1+1
i4f4.st1+2! !i8.st1+2
i4f4.st1+3! !i8.st1+3
i4f4.st2!   !i8.st2
i4f4.st2+1! !i8.st2+1
i4f4.st2+2! !i8.st2+2
i4f4.st2+3! !i8.st2+3
i4f4.st3!   !i8.st3
i4f4.st3+1! !i8.st3+1
i4f4.st3+2! !i8.st3+2
i4f4.st3+3! !i8.st3+3
i4f4.st4!   !i8.st4
i4f4.st4+1! !i8.st4+1
i4f4.st4+2! !i8.st4+2
i4f4.st4+3! !i8.st4+3
i4f4.st5!   !i8.st5
i4f4.st5+1! !i8.st5+1
i4f4.st5+2! !i8.st5+2
i4f4.st5+3! !i8.st5+3
i4f4.st6!   !i8.st6
i4f4.st6+1! !i8.st6+1
i4f4.st6+2! !i8.st6+2
i4f4.st6+3! !i8.st6+3
i4f4.st7!   !i8.st7
i4f4.st7+1! !i8.st7+1
i4f4.st7+2! !i8.st7+2
i4f4.st7+3! !i8.st7+3
i4f4.st8!   !i8.st8
i4f4.st8+1! !i8.st8+1
i4f4.st8+2! !i8.st8+2
i4f4.st8+3! !i8.st8+3
i4f4.st9!   !i8.st9
i4f4.st9+1! !i8.st9+1
i4f4.st9+2! !i8.st9+2
i4f4.st9+3! !i8.st9+3
i4f4.stA!   !i8.stA
i4f4.stA+1! !i8.stA+1
i4f4.stA+2! !i8.stA+2
i4f4.stA+3! !i8.stA+3
i4f4.stB!   !i8.stB
i4f4.stB+1! !i8.stB+1
i4f4.stB+2! !i8.stB+2
i4f4.stB+3! !i8.stB+3
i4f4.stC!   !i8.stC
i4f4.stC+1! !i8.stC+1
i4f4.stC+2! !i8.stC+2
i4f4.stC+3! !i8.stC+3
i4f4.stD!   !i8.stD
i4f4.stD+1! !i8.stD+1
i4f4.stD+2! !i8.stD+2
i4f4.stD+3! !i8.stD+3
i4f4.stE!   !i8.stE
i4f4.stE+1! !i8.stE+1
i4f4.stE+2! !i8.stE+2
i4f4.stE+3! !i8.stE+3
i4f4.stF!   !i8.stF
i4f4.stF+1! !i8.stF+1
i4f4.stF+2! !i8.stF+2
i4f4.stF+3! !i8.stF+3
i8f8.st0!   !i16.st0
i8f8.st0+1! !i16.st0+1
i8f8.st0+2! !i16.st0+2
i8f8.st0+3! !i16.st0+3
i8f8.st1!   !i16.st1
i8f8.st1+1! !i16.st1+1
i8f8.st1+2! !i16.st1+2
i8f8.st1+3! !i16.st1+3
i8f8.st2!   !i16.st2
i8f8.st2+1! !i16.st2+1
i8f8.st2+2! !i16.st2+2
i8f8.st2+3! !i16.st2+3
i8f8.st3!   !i16.st3
i8f8.st3+1! !i16.st3+1
i8f8.st3+2! !i16.st3+2
i8f8.st3+3! !i16.st3+3
i8f8.st4!   !i16.st4
i8f8.st4+1! !i16.st4+1
i8f8.st4+2! !i16.st4+2
i8f8.st4+3! !i16.st4+3
i8f8.st5!   !i16.st5
i8f8.st5+1! !i16.st5+1
i8f8.st5+2! !i16.st5+2
i8f8.st5+3! !i16.st5+3
i8f8.st6!   !i16.st6
i8f8.st6+1! !i16.st6+1
i8f8.st6+2! !i16.st6+2
i8f8.st7!   !i16.st7
c4f4m4f4.st0!   !i16.st0
c4f4m4f4.st0+1! !i16.st0+1
c4f4m4f4.st0+2! !i16.st0+2
c4f4m4f4.st0+3! !i16.st0+3
c4f4m4f4.st1!   !i16.st1
c4f4m4f4.st1+1! !i16.st1+1
c4f4m4f4.st1+2! !i16.st1+2
c4f4m4f4.st1+3! !i16.st1+3
c4f4m4f4.st2!   !i16.st2
c4f4m4f4.st2+1! !i16.st2+1
c4f4m4f4.st2+2! !i16.st2+2
c4f4m4f4.st2+3! !i16.st2+3
c4f4m4f4.st3!   !i16.st3
c4f4m4f4.st3+1! !i16.st3+1
c4f4m4f4.st3+2! !i16.st3+2
c4f4m4f4.st3+3! !i16.st3+3
c4f4m4f4.st4!   !i16.st4
c4f4m4f4.st4+1! !i16.st4+1
c4f4m4f4.st4+2! !i16.st4+2
c4f4m4f4.st4+3! !i16.st4+3
c4f4m4f4.st5!   !i16.st5
c4f4m4f4.st5+1! !i16.st5+1
c4f4m4f4.st5+2! !i16.st5+2
c4f4m4f4.st5+3! !i16.st5+3
c4f4m4f4.st6!   !i16.st6
c4f4m4f4.st6+1! !i16.st6+1
c4f4m4f4.st6+2! !i16.st6+2
c4f4m4f4.st7!   !i16.st7
c8f8m8f8.st0!   !i32.st0
c8f8m8f8.st0+1! !i32.st0+1
c8f8m8f8.st0+2! !i32.st0+2
c8f8m8f8.st0+3! !i32.st0+3
c8f8m8f8.st1!   !i32.st1
c8f8m8f8.st1+1! !i32.st1+1
c8f8m8f8.st1+2! !i32.st1+2
c8f8m8f8.st1+3! !i32.st1+3
c8f8m8f8.st2!   !i32.st2
c8f8m8f8.st2+1! !i32.st2+1
c8f8m8f8.st2+2! !i32.st2+2
c8f8m8f8.st2+3! !i32.st2+3
c8f8m8f8.st3!   !i32.st3
char.st0!   !u8.st0
char.st0+1! !u8.st0+1
char.st0+2! !u8.st0+2
char.st0+3! !u8.st0+3
char.st1!   !u8.st1
char.st1+1! !u8.st1+1
char.st1+2! !u8.st1+2
char.st1+3! !u8.st1+3
char.st2!   !u8.st2
char.st2+1! !u8.st2+1
char.st2+2! !u8.st2+2
char.st2+3! !u8.st2+3
char.st3!   !u8.st3
char.st3+1! !u8.st3+1
char.st3+2! !u8.st3+2
char.st3+3! !u8.st3+3
char.st4!   !u8.st4
char.st4+1! !u8.st4+1
char.st4+2! !u8.st4+2
char.st4+3! !u8.st4+3
char.st5!   !u8.st5
char.st5+1! !u8.st5+1
char.st5+2! !u8.st5+2
char.st5+3! !u8.st5+3
char.st6!   !u8.st6
char.st6+1! !u8.st6+1
char.st6+2! !u8.st6+2
char.st6+3! !u8.st6+3
char.st7!   !u8.st7
char.st7+1! !u8.st7+1
char.st7+2! !u8.st7+2
char.st7+3! !u8.st7+3
char.st8!   !u8.st8
char.st8+1! !u8.st8+1
char.st8+2! !u8.st8+2
char.st8+3! !u8.st8+3
char.st9!   !u8.st9
char.st9+1! !u8.st9+1
char.st9+2! !u8.st9+2
char.st9+3! !u8.st9+3
char.stA!   !u8.stA
char.stA+1! !u8.stA+1
char.stA+2! !u8.stA+2
char.stA+3! !u8.stA+3
char.stB!   !u8.stB
char.stB+1! !u8.stB+1
char.stB+2! !u8.stB+2
char.stB+3! !u8.stB+3
char.stC!   !u8.stC
char.stC+1! !u8.stC+1
char.stC+2! !u8.stC+2
char.stC+3! !u8.stC+3
char.stD!   !u8.stD
char.stD+1! !u8.stD+1
char.stD+2! !u8.stD+2
char.stD+3! !u8.stD+3
char.stE!   !u8.stE
char.stE+1! !u8.stE+1
char.stE+2! !u8.stE+2
char.stE+3! !u8.stE+3
char.stF!   !u8.stF
char.stF+1! !u8.stF+1
char.stF+2! !u8.stF+2
char.stF+3! !u8.stF+3

u4u4.fst! xF0 and x04 rot # u8 first = u4u4.fst(u4u4 n)
u4u4.snd! x0F and # u8 second = u4u4.snd(u4u4 n)
u8u8.fst! !u8.pop # u8 first = u8u8.fst(u8u8 n)
u8u8.snd! !u8.st0 # u8 second = u8u8.snd(u8u8 n)
i4i4.fst! !u4u4.fst # i8 first = i4i4.fst(i4i4 n)
i4i4.snd! !u4u4.snd # i8 second = i4i4.snd(i4i4 n)
i8i8.fst! !u8u8.fst # i8 first = i8i8.fst(i8i8 n)
i8i8.snd! !u8u8.snd # i8 second = i8i8.snd(i8i8 n)
u4f4.in! !u4u4.fst # u8 integer_part = u4f4.in(u4f4 n)
u4f4.fr! !u4u4.snd # u8 fraction_part = u4f4.fr(u4f4 n)
u8f8.in! !u8.pop # u8 integer_part = u8f8.in(u8f8 n)
u8f8.fr! !u8.st0 # u8 fraction_part = u8f8.fr(u8f8 n)
i4f4.in! !i4i4.fst # i8 integer_part = i4f4.in(i4f4 n)
i4f4.fr! !i4i4.snd # i8 fraction_part = i4f4.fr(i4f4 n)
i8f8.in! !i8.pop # i8 integer_part = i8f8.in(i8f8 n)
i8f8.fr! !i8.st0 # u8 fraction_part = i8f8.fr(i8f8 n)
c4f4m4f4.re! !u8.pop # i4f4 real_part = c4f4m4f4.re(c4f4m4f4 c)
c4f4m4f4.im! !u8.st0 # i4f4 imaginary_part = c4f4m4f4.im(c4f4m4f4 c)
c8f8m8f8.re! !u16.pop # i8f8 real_part = c8f8m8f8.re(c8f8m8f8 c)
c8f8m8f8.im! !u16.st0 # i8f8 imaginary_part = c8f8m8f8.im(c8f8m8f8 c)

u8.mul! :u8.mul !call # u16 product = u8.mul(u8 a, u8 b)
u16.mul! :u16.mul !call # u32 product = u16.mul(u16 a, u16 b)
i8.mul! :i8.mul !call # u16 product = u8.mul(u8 a, u8 b)
i16.mul! :i16.mul !call # i32 product = i16.mul(i16 a, i16 b)
u4f4.mul! :u4f4.mul !call # u4f4 product = u4f4.mul(u4f4 a, u4f4 b)
u8f8.mul! :u8f8.mul !call # u8f8 product = u8f8.mul(u8f8 a, u8f8 b)
i4f4.mul! :i4f4.mul !call # i4f4 product = i4f4.mul(i4f4 a, i4f4 b)
i8f8.mul! :i8f8.mul !call # i8f8 product = i8f8.mul(i8f8 a, i8f8 b)
c4f4m4f4.mul! # c4f4m4f4 product = c4f4m4f4.mul(c4f4m4f4 a, c4f4m4f4 b)
  !i4f4.ld3 !i4f4.ld2 !i4f4.mul !i4f4.ld3 !i4f4.ld2 !i4f4.mul !i4f4.sub # real part
  !i4f4.ld4 !i4f4.ld2 !i4f4.mul !i4f4.ld4 !i4f4.ld4 !i4f4.mul !i4f4.add # imaginary part
  !i16.st1 !i16.pop
c8f8m8f8.mul! # c8f8m8f8 product = c8f8m8f8.mul(c8f8m8f8 a, c8f8m8f8 b)
  !i8f8.ld3 !i8f8.ld2 !i8f8.mul !i8f8.ld3 !i8f8.ld2 !i8f8.mul !i8f8.sub # real part
  !i8f8.ld4 !i8f8.ld2 !i8f8.mul !i8f8.ld4 !i8f8.ld4 !i8f8.mul !i8f8.add # imaginary part
  !i32.st1 !i32.pop

c4f4m4f4.norm! !i4f4.ld1 !i4f4.ld0 !i4f4.mul !i4f4.st1 !i4f4.ld0 !i4f4.mul !i4f4.add # i4f4 norm = c4f4m4f4.norm(c4f4m4f4 c)
c8f8m8f8.norm! !i8f8.ld1 !i8f8.ld0 !i8f8.mul !i8f8.st1 !i8f8.ld0 !i8f8.mul !i4f4.add # i8f8 norm = c8f8m8f8.norm(c8f8m8f8 c)

# converts `0x0..==0xF` to `'0'..='9', 'A'..='F'`. undefined for other values
u4.to_char! x0A sub @dyn x41 @const x0A x30 add dec @const iff add
# converts `'0'..='9', 'A'..='F'` to `0x0..=0xF`. undefined for other values
char.to_u4! x41 sub @dyn x0A @const x41 x30 sub dec @const iff add

u8.mul.def!
  u8.mul: clc # u16 product = u8.mul(u8 a, u8 b)
    !u16.0 # product
    x08 for_bit. dec
      # b >>= 1
      !u8.ld3+2 !u8.shr !u8.st3+2
      # product += CF ? a : 0
      !u8.ld1+1 !u8.0 !u8.ld4+2 !u8.iff clc !u8.add
      # product >>= 1 (stairstep shift)
      !u8.ld1+1 !u16.shr !u16.st0+1
    buf .for_bit !bcc pop
  # return* product
  !u16.st0+1 !rt0
i8.mul.def!
  i8.mul: clc # i16 product = i8.mul(i8 a, i8 b)
    # u16 product = (u8)a * (u8)b
    !u16.ld0+1 !u8.mul
    # product -= (a & 0x80 ? b << 0x08 : 0x00)
    ld3 shl pop !u16.0 !u8.ld5+1 !u8.0 !u16.iff clc !u16.sub
    # product -= (b & 0x80 ? a << 0x08 : 0x00)
    ld4 shl pop !u16.0 !u8.ld4+1 !u8.0 !u16.iff clc !u16.sub
  # return* product
  !u16.st0+1 !rt0
u16.mul.def!
  u16.mul: clc # u32 product = u16.mul(u16 a, u16 b)
    !u32.0 # product
    x10 for_bit. dec
      # b >>= 1
      !u16.ld3+2 !u16.shr !u16.st3+2
      # product += CF ? a : 0
      !u16.ld1+1 !u16.0 !u16.ld4+2 !u16.iff clc !u16.add
      # product >>= 1 (stairstep shift)
      !u16.ld1+1 !u32.shr !u32.st0+1
    buf .for_bit !bcc pop
  # return* product
  !u32.st0+1 !rt0
i16.mul.def!
  i16.mul: clc # i32 product = i16.mul(i16 a, i16 b)
    # u32 product = (u16)a * (u16)b
    !u32.ld0+1 !u16.mul
    # product -= (a & 0x8000 ? b << 0x10 : 0x00)
    ld6 shl pop !u32.0 !u16.ld5+1 !u16.0 !u32.iff clc !u32.sub
    # product -= (b & 0x8000 ? a << 0x10 : 0x00)
    ld8 shl pop !u32.0 !u16.ld4+1 !u16.0 !u32.iff clc !u32.sub
  # return* product
  !u32.st0+1 !rt0
u4f4.mul.def!
  u4f4.mul: clc # u4f4 product = u4f4.mul(u4f4 a, u4f4 b)
    # product = (u8)a * (u8)b
    !u16.ld0+1 !u8.mul
    # product >>= 0x04
    x0F xF0 an2 an2 orr x04 rot !u8.st1+1
  # return* product
  !rt1
u8f8.mul.def!
  u8f8.mul: clc # u8f8 product = u8f8.mul(u8f8 a, u8f8 b)
    # product = (u16)a * (u16)b
    !u32.ld0+1 !u16.mul
    # product >>= 0x08
    pop !u8.st4+1 !u8.st4+1 pop
  # return* product
  !rt2
i4f4.mul.def!
  i4f4.mul: clc # i4f4 product = i4f4.mul(i4f4 a, i4f4 b)
    # product = (i8)a * (i8)b
    !i16.ld0+1 !i8.mul
    # product >>= 0x04
    x0F xF0 an2 an2 orr x04 rot !i8.st1+1
  # return* product
  !rt1
i8f8.mul.def!
  i8f8.mul: clc # i8f8 product = i8f8.mul(i8f8 a, i8f8 b)
    # product = (i16)a * (i16)b
    !i32.ld0+1 !i16.mul
    # product >>= 0x08
    pop !i8.st4+1 !i8.st4+1 pop
  # return* product
  !rt2
