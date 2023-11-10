# bootstrap C runtime environment

# assembler entry point
main!
  # initialize stack, `argc = argv = envp = NULL`
  pop pop xE0 sts
  # link in dependencies and call C entry point
  :main !call !hlt !main.deps

  # initialize `malloc` and `free`
  heap_start: xFF :heap_start pop @const
heap_start! :heap_start @const
