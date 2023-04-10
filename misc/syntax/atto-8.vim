syntax match atto8Instruction "ld.\|st.\|add.\|adc.\|sub.\|sbc.\|shf.\|sfc.\|rot.\|iff.\|orr.\|and.\|xor.\|xnd."
syntax keyword atto8Instruction add adc sub sbc shf sfc rot iff orr and xor xnd inc dec neg not buf nop sec clc flc swp pop lda sta ldi sti lds sts
syntax match atto8XXX "x[0-9A-F][0-9A-F]"
syntax match atto8DDD "d[0-9A-F][0-9A-F]"
syntax match atto8MacroRef "![^! ]\+"
syntax match atto8MacroDef "[^! ]\+!"
syntax match atto8LabelRef ":[^: ]\+\|\.[^\. ]\+"
syntax match atto8LabelDef "[^: ]\+:\|[^\. ]\+\."
syntax match atto8Include "@.*$"
syntax match atto8Comment "#.*$" contains=atto8Todo
syntax keyword atto8Todo TODO FIXME XXX NOTE contained

highlight default link atto8Instruction Keyword
highlight default link atto8XXX Number
highlight default link atto8DDD Constant
highlight default link atto8MacroRef Identifier
highlight default link atto8MacroDef Macro
highlight default link atto8LabelRef Identifier
highlight default link atto8LabelDef Label
highlight default link atto8Include Include
highlight default link atto8Comment Comment
highlight default link atto8Todo Todo
