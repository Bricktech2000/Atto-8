if exists("b:current_syntax")
  finish
endif

let b:current_syntax = "atto-8"

" include everything, exclude whitespace, re-include \n, exclude [!:.].
" we include \n because it's actually a \0. |NL-used-for-Nul|
setlocal iskeyword+=1-255,^9-13,^32,10,^!,^:,^.
setlocal commentstring=#\ %s
setlocal comments=:#\ ,:#$
setlocal indentexpr=-1 indentkeys=
let &l:define = '\ze[^[:space:]]*[!:.]\([[:space:]]\|$\)'
let &l:include = '@ '

" if no patterns match at the start of a mnemonic, `atto8Mnemonic` will match
" by default and will consume the mnemonic. this way we don't need to litter
" patterns with look-behinds to make sure they don't begin matching halfway
" through a mnemonic
syntax match atto8Mnemonic "[^[:space:]]\+\_[[:space:]]\+"
syntax match atto8Instruction "\(add\|sub\|iff\|swp\|rot\|orr\|and\|xor\|xnd\|inc\|dec\|neg\|shl\|shr\|not\|buf\|lda\|sta\|ldi\|sti\|lds\|sts\|sec\|clc\|flc\|nop\|pop\)\_[[:space:]]\+"
syntax match atto8Instruction "\(ad\|su\|if\|sw\|ro\|or\|an\|xo\|xn\)[1248]\{1}\_[[:space:]]\+"
syntax match atto8Instruction "\(ld\|st\)[0-9A-F]\{1}\_[[:space:]]\+"
syntax match atto8XXX "x[0-9A-F]\{2}\_[[:space:]]\+"
syntax match atto8Directive "@\(error\|const\|data\|dyn\|org\|[0-9A-F]\{2}\)\_[[:space:]]\+"
syntax match atto8MacroRef "![^[:space:]]*\_[[:space:]]\+"
syntax match atto8MacroDef "[^[:space:]]*!\_[[:space:]]\+"
syntax match atto8LabelRef ":[^[:space:]]*\_[[:space:]]\+"
syntax match atto8LabelDef "[^[:space:]]*:\_[[:space:]]\+"
syntax match atto8LabelRef "\.[^[:space:]]*\_[[:space:]]\+"
syntax match atto8LabelDef "[^[:space:]]*\.\_[[:space:]]\+"
syntax match atto8Include "@\_[[:space:]]\@=.*"
syntax match atto8Comment "#\_[[:space:]]\@=.*" contains=atto8Todo
syntax keyword atto8Todo TODO FIXME XXX NOTE contained

highlight default link atto8Mnemonic NONE
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
