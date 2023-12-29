# bootstrap C runtime environment

# assembler entry point
main!
  # link in dependencies and call C entry point
  :main !call !hlt !main.deps

  # initialize heap for `malloc` and `free`
  heap_start: xFF :heap_start pop @const # `!heap_unlimited`
heap_start! :heap_start @const
