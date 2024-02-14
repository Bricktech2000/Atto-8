@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/stdio.asm

# username is `admin` and password is `admin`. the shell doesn't do anything;
# this program is only a demo for the `getline` and `getpass` functions

main! sec
  user_loop:
    :str_unknown_user :str_user iff :puts.min !call
    :line_buffer :line_buffer !strend :getline !call
    :line_buffer :str_admin :strcmp !call pop
  :user_loop !bcc

  pass_loop:
    :str_invalid_pass :str_pass iff :puts.min !call
    :line_buffer :line_buffer :getpass !call
    :line_buffer :str_admin :strcmp !call pop
  :pass_loop !bcc

  shell_loop:
    :str_shell_prompt :puts.min !call
  :user_loop :user_loop :shell_loop :getline !jmp

  str_admin: @61 @64 @6D @69 @6E @00 # "admin"
  str_unknown_user: @09 @55 @4E @4B # "\tUNK\r\nuser: "
  str_user: @0D @0A @75 @73 @65 @72 @3A @20 # "\r\nuser: "
  line_buffer: @72 @6F @6F @74 x0C !pad # "root\0\0\0\0"
  str_invalid_pass: @09 @49 @4E @56 # "\tINV\r\npass: "
  str_pass: @0D @0A @70 @61 @73 @73 @3A @20 @00 # "\r\npass: "
  str_shell_prompt: @0D @0A @24 @20 @00 # "\r\n$ "

  !puts.min.def
  !getpass.def
  !getline.def
  !strcmp.def
