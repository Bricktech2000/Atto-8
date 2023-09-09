import os
import sys
import shutil
import functools
import subprocess

sys.dont_write_bytecode = True
sys.path.append('../misc/common/')
import common  # noqa

open_safe = common.open_safe('Test')


def cat(filename):
  with open_safe(filename, 'rb') as file:
    print(f'Cat: Output: {filename}')
    sys.stdout.flush()
    sys.stdout.buffer.write(file.read())


def run(*args):
  print(f'Test: Running `{" ".join(args)}`')
  sys.stdout.flush()
  subprocess.run([*args], check=True)


def rel_path(*args):
  # from path relative to this file to path relative to cwd
  return os.path.relpath(os.path.join(os.path.dirname(__file__), *args), os.getcwd())


run_cargo = functools.partial(run, 'cargo', '--quiet')
run_python = functools.partial(run, 'python3')


if len(sys.argv) <= 1:
  print('Test: Usage: test <filenames|operations>')
  sys.exit(1)

target = 'target'
input = sys.argv[1:][::-1]
shutil.rmtree(rel_path(target), ignore_errors=True)
shutil.copytree(rel_path('./'), rel_path(target), dirs_exist_ok=True)
shutil.copytree(rel_path('../lib/'), rel_path('target/lib/'), dirs_exist_ok=True)


filenames = []
operations = []
while input:
  operation = ''  # make type checker happy
  try:
    operation = input.pop()
    match operation:
      case 'cc':
        c_source_file = filenames.pop()
        assembly_output_file = c_source_file + '.asm'
        filenames.append(assembly_output_file)
        operations.append((operation, functools.partial(run_cargo, 'run', '--bin',
                          operation, c_source_file, assembly_output_file)))
      case 'enc':
        hex_source_file = filenames.pop()
        memory_image_file = hex_source_file + '.mem'
        filenames.append(memory_image_file)
        operations.append((operation, functools.partial(
            run_python, rel_path(f'../{operation}/{operation}.py'), hex_source_file, memory_image_file)))
      case 'dec':
        memory_image_file = filenames.pop()
        hex_output_file = memory_image_file + '.hex'
        filenames.append(hex_output_file)
        operations.append((operation, functools.partial(
            run_python, rel_path(f'../{operation}/{operation}.py'), memory_image_file, hex_output_file)))
      case 'asm':
        assembly_source_file = filenames.pop()
        memory_image_file = assembly_source_file + '.mem'
        filenames.append(memory_image_file)
        operations.append((operation, functools.partial(run_cargo, 'run', '--bin',
                          operation, assembly_source_file, memory_image_file)))
      case 'dasm':
        memory_image_file = filenames.pop()
        disassembly_output_file = memory_image_file + '.asm'
        filenames.append(disassembly_output_file)
        operations.append((operation, functools.partial(run_cargo, 'run', '--bin',
                          operation, memory_image_file, disassembly_output_file)))
      case 'emu':
        memory_image_file = filenames.pop()
        operations.append((operation, functools.partial(run_cargo, 'run', '--bin', operation, memory_image_file)))
      case 'mic':
        microcode_image_file = rel_path(target, 'microcode.mic')
        filenames.append(microcode_image_file)
        operations.append((operation, functools.partial(run_cargo, 'run', '--bin', operation, microcode_image_file)))
      case 'sim':
        microcode_image_file = filenames.pop()
        memory_image_file = filenames.pop()
        operations.append((operation, functools.partial(run_cargo, 'run', '--bin',
                          operation, memory_image_file, microcode_image_file)))
      case 'circ':
        microcode_image_file = filenames.pop()
        memory_image_file = filenames.pop()
        operations.append((operation, functools.partial(run_python, rel_path(
            f'../{operation}/{operation}.py'), memory_image_file, microcode_image_file)))
      case 'pop':
        filenames.pop()
      case 'dup':
        filenames.append(filenames[-1])
      case 'cat':
        filename = filenames.pop()
        operations.append((operation, functools.partial(cat, filename)))
      case file:
        filenames.append(rel_path(target, file))
  except IndexError:
    print(f'Test: Error: Missing argument for operation `{operation}`')
    sys.exit(1)

if filenames:
  for filename in filenames:
    print(f'Test: Error: Unused argument `{filename}`')
  sys.exit(1)

try:
  for (name, func) in operations:
    try:
      func()
    except subprocess.CalledProcessError as e:
      print(f'Test: Warning: Operation subprocess `{name}` exited with code `{e.returncode}`')
except KeyboardInterrupt:
  print('Test: Interrupted')
  sys.exit(1)
