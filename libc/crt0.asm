# bootstrap C runtime environment

# assembler entry point
main!
  # link in dependencies and call C entry point
  :main !call !hlt !main.deps

  # initialize heap for `malloc` and `free`
  heap_start: @FF :heap_start pop # `!heap_unlimited @data`
heap_start! :heap_start @const
