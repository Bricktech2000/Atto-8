import os
import sys
import shutil
import functools
import subprocess

sys.dont_write_bytecode = True
sys.path.append('../misc/common/')
import common  # noqa

open_safe = common.open_safe('Test')

debug_mode = False


def pipe(filename):
  with open_safe(filename, 'rb') as file:
    if debug_mode:
      print(f'Test: Pipe `{filename}`')
    sys.stdout.flush()
    sys.stdout.buffer.write(file.read())


def run(*args):
  if debug_mode:
    print(f'Test: Running `{" ".join(args)}`')
  sys.stdout.flush()
  subprocess.run([*args], check=True)


def rel_path(*args):
  # from path relative to this file to path relative to cwd
  return os.path.relpath(os.path.join(os.path.dirname(__file__), *args), os.getcwd())


run_cargo = functools.partial(run, 'cargo', '--quiet', 'run', '--release', '--bin')
run_python = functools.partial(run, 'python3')


if len(sys.argv) <= 1:
  print('Test: Usage: test <filenames|operations>')
  sys.exit(1)

target = 'target'
input = sys.argv[1:][::-1]
shutil.rmtree(rel_path(target), ignore_errors=True)
shutil.copytree(rel_path('../lib/'), rel_path(target, 'lib/'), dirs_exist_ok=True)
shutil.copytree(rel_path('../libc/'), rel_path(target, 'libc/'), dirs_exist_ok=True)
shutil.copytree(rel_path('../misc/'), rel_path(target, 'misc/'), dirs_exist_ok=True)
shutil.copytree(rel_path('../test/musts/'), rel_path(target), dirs_exist_ok=True)
shutil.copytree(rel_path('../test/utils/'), rel_path(target), dirs_exist_ok=True)
shutil.copytree(rel_path('../test/games/'), rel_path(target), dirs_exist_ok=True)
shutil.copytree(rel_path('../test/other/'), rel_path(target), dirs_exist_ok=True)
shutil.copytree(rel_path('../test/tests/'), rel_path(target), dirs_exist_ok=True)
shutil.copytree(rel_path('../libc/incl/'), rel_path(target), dirs_exist_ok=True)
shutil.copytree(rel_path('../circ/impl/'), rel_path(target), dirs_exist_ok=True)
shutil.copytree(rel_path('../bf/test/'), rel_path(target), dirs_exist_ok=True)


filenames = []
operations = []
while input:
  operation = ''  # make type checker happy
  try:
    operation = input.pop()
    match operation:
      case 'cc':
        (c_source_files, filenames) = (filenames, [])  # consume all
        assembly_output_file = c_source_files[0] + '.asm'
        filenames.append(assembly_output_file)
        operations.append((operation, functools.partial(
            run_cargo, f'{operation}', *c_source_files, assembly_output_file)))
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
        operations.append((operation, functools.partial(
            run_cargo, f'{operation}', assembly_source_file, memory_image_file)))
      case 'dasm':
        memory_image_file = filenames.pop()
        disassembly_output_file = memory_image_file + '.asm'
        filenames.append(disassembly_output_file)
        operations.append((operation, functools.partial(
            run_cargo, f'{operation}', memory_image_file, disassembly_output_file)))
      case 'emu':
        memory_image_file = filenames.pop()
        operations.append((operation, functools.partial(run_cargo, f'{operation}', memory_image_file)))
      case 'mic':
        microcode_image_file = rel_path(target, 'microcode.mic')
        filenames.append(microcode_image_file)
        operations.append((operation, functools.partial(run_cargo, f'{operation}', microcode_image_file)))
      case 'sim':
        microcode_image_file = filenames.pop()
        memory_image_file = filenames.pop()
        operations.append((operation, functools.partial(
            run_cargo, f'{operation}', memory_image_file, microcode_image_file)))
      case 'circ':
        circuit_file = filenames.pop()
        microcode_image_file = filenames.pop()
        memory_image_file = filenames.pop()
        operations.append((operation, functools.partial(run_python, rel_path(
            f'../{operation}/{operation}.py'), memory_image_file, microcode_image_file, circuit_file)))
      case 'bf':
        brainfuck_source_file = filenames.pop()
        memory_image_file = brainfuck_source_file + '.mem'
        filenames.append(memory_image_file)
        operations.append((operation, functools.partial(run_python, rel_path(
            f'../{operation}/{operation}-pad.py'), brainfuck_source_file, memory_image_file)))
        microcode_image_file = rel_path(target, 'brainfuck.mic')
        filenames.append(microcode_image_file)
        operations.append((operation, functools.partial(run_cargo, f'{operation}-mic', microcode_image_file)))
      case 'pop':
        filenames.pop()
      case 'dup':
        filenames.append(filenames[-1])
      case 'pipe':
        filename = filenames.pop()
        operations.append((operation, functools.partial(pipe, filename)))
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
      if debug_mode:
        print(f'Test: Warning: Operation subprocess `{name}` exited with code `{e.returncode}`')
except KeyboardInterrupt:
  print('Test: Interrupted')
  sys.exit(1)
