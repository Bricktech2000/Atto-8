if exists("b:current_syntax")
  finish
endif

let b:current_syntax = "atto-8"

" every printable non-digit non-alpha character, except " !.:@"
setlocal iskeyword+=34,35,36,37,38,39,40,41,42,43,44,45,47,59,60,61,62,63,91,92,93,94,95,96,123,124,125,126
setlocal commentstring=#\ %s
setlocal comments=:#\ ,:#$
setlocal define=\\ze\\<[^!\ ]\\+\\>!\\\|\\ze\\<[^:\ ]\\+\\>:\\\|\\ze\\<[^.\ ]\\+\\>\\.
setlocal include=@\ 

syntax match atto8Instruction "\<\(add\|sub\|iff\|swp\|rot\|orr\|and\|xor\|xnd\|inc\|dec\|neg\|shl\|shr\|not\|buf\|lda\|sta\|ldi\|sti\|lds\|sts\|sec\|clc\|flc\|nop\|pop\)\>"
syntax match atto8Instruction "\<\(ad\|su\|if\|sw\|ro\|or\|an\|xo\|xn\)[1248]\{1\}\>"
syntax match atto8Instruction "\<\(ld\|st\)[0-9A-F]\{1\}\>"
syntax match atto8XXX "\<x[0-9A-F]\{2\}\>"
syntax match atto8Directive "@\<\(error\|const\|data\|dyn\|org\|[0-9A-F]\{2\}\)\>"
syntax match atto8MacroRef "!\<[^! ]\+\>"
syntax match atto8MacroDef "\<[^! ]\+\>!"
syntax match atto8LabelRef ":\<[^: ]\+\>\|\.\<[^. ]\+\>"
syntax match atto8LabelDef "\<[^: ]\+\>:\|\<[^. ]\+\>\."
syntax match atto8Include "@ .*$"
syntax match atto8Comment "# .*$\|#$" contains=atto8Todo
syntax keyword atto8Todo TODO FIXME XXX NOTE contained

highlight default link atto8Instruction Keyword
highlight default link atto8XXX Number
highlight default link atto8Directive PreProc
highlight default link atto8MacroRef Macro
highlight default link atto8MacroDef Macro
highlight default link atto8LabelRef Label
highlight default link atto8LabelDef Label
highlight default link atto8Include Include
highlight default link atto8Comment Comment
highlight default link atto8Todo Todo
