@ lib/core.asm
@ lib/types.asm
@ lib/string.asm
@ lib/stdio.asm

# username is `admin` and password is `admin`. the shell doesn't do anything;
# this program is only a demo for the `getline` and `getpass` functions

main! sec
  user_loop:
    :str__unk_user_ :str_user_ iff :puts.min !call
    :line_buffer :line_buffer !strend :getline !call
    :line_buffer :str_admin :strcmp !call pop
  :user_loop !bcc

  pass_loop:
    :str__inv_pass_ :str_pass_ iff :puts.min !call
    :line_buffer :line_buffer :getpass !call
    :line_buffer :str_admin :strcmp !call pop
  :pass_loop !bcc

  shell_loop:
    :str_$_ :puts.min !call
  :user_loop :user_loop :shell_loop :getline !jmp

  # "admin\0"
  str_admin: @61 @64 @6D @69 @6E @00
  # "\tUNK"
  str__unk_user_: @09 @55 @4E @4B
  # "\r\nuser: \0"
  str_user_: @0D @0A @75 @73 @65 @72 @3A @20
  # "root\0"
  line_buffer: @72 @6F @6F @74 x0C !pad
  # "\tINV"
  str__inv_pass_: @09 @49 @4E @56
  # "\r\npass: \0"
  str_pass_: @0D @0A @70 @61 @73 @73 @3A @20 @00
  # "\r\n$ \0"
  str_$_: @0D @0A @24 @20 @00

  !puts.min.def
  !getpass.def
  !getline.def
  !strcmp.def
