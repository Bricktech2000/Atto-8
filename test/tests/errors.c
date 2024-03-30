// clang-format off

// syntax errors

// #error test error in file __FILE__ on line __LINE__
// #foobar
// int inv_hex_lit(void) { return 0x; }
// int inv_bin_lit(void) { return 0b1234; }
// int unclosed_param_list( { return 0; }
// int missing_operand(void) { return 5 + ; }
// int truncated(void) { return (char
// int unary_as_binary(void) { return 5 ! a; }
// 123
// int inv_ternary(void) { 1 ? 2; }
// void inv_param_list(void, );
// int unclosed_if(void) { if (1 return 0; }
// void bare_prototype(void)
// void unclosed_body(void) {
// asm("parentheses")
// asm {
// #include no_quotes
// #include "unclosed
// #include <misclosed"
// #include "file" trailing
// char *unclosed_str_lit(void) { "unclosed; }
// char inv_char_lit(void) { return 'abc'; }
// char *inv_esc_seq(void) { return "\X1F"; }
// char *inv_hex_esc(void) { return "\xyz"; }
// int comment_whitespace(void) { return 1/**/2; }

// semantic errors

// void local_redef(void) { int a; int a; }
// void inv_deref(void) { **(void *)0; }
// void op_on_void(void) { !~op_on_void() + 5; }
// void inv_fn_call(void) { inv_fn_call(1, 2, 3); }
// int diff_redecl(void); void diff_redecl(int);
