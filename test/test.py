import os
import sys
import shutil
import functools
import subprocess


def run(*args):
  print(f"Test: Running `{' '.join(args)}`")
  subprocess.run([*args], check=True)


def rel_path(*args):
  # from path relative to this file to path relative to cwd
  return os.path.relpath(os.path.join(os.path.dirname(__file__), *args), os.getcwd())


run_cargo = functools.partial(run, 'cargo', '--quiet')
run_python = functools.partial(run, 'python3')


if len(sys.argv) <= 1:
  print("Test: Usage: test.py <filenames|operations>")
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
      case 'enc':
        hex_file = filenames.pop()
        memory_image_file = hex_file + '.mem'
        operations.append(('enc', functools.partial(
            run_python, rel_path('../enc/enc.py'), hex_file, memory_image_file)))
        filenames.append(memory_image_file)
      case 'asm':
        assembly_source_file = filenames.pop()
        memory_image_file = assembly_source_file + '.mem'
        operations.append(('asm', functools.partial(run_cargo, 'run', '--bin',
                          'asm', assembly_source_file, memory_image_file)))
        filenames.append(memory_image_file)
      case 'dasm':
        memory_image_file = filenames.pop()
        disassembly_output_file = memory_image_file + '.dasm'
        operations.append(('dasm', functools.partial(run_cargo, 'run', '--bin',
                          'dasm', memory_image_file, disassembly_output_file)))
        filenames.append(disassembly_output_file)
      case 'emu':
        memory_image_file = filenames.pop()
        operations.append(('emu', functools.partial(run_cargo, 'run', '--bin', 'emu', memory_image_file)))
      case 'mic':
        microcode_image_file = rel_path(target, 'microcode.mic')
        operations.append(('mic', functools.partial(run_cargo, 'run', '--bin', 'mic', microcode_image_file)))
        filenames.append(microcode_image_file)
      case 'sim':
        microcode_image_file = filenames.pop()
        memory_image_file = filenames.pop()
        operations.append(('sim', functools.partial(run_cargo, 'run', '--bin',
                          'sim', memory_image_file, microcode_image_file)))
      case file:
        filenames.append(rel_path(target, file))
  except IndexError:
    print(f'Test: Error: Missing argument for operation `{operation}`')
    sys.exit(1)

while filenames:
  print(f'Test: Warning: Unused filename `{filenames.pop()}`')

try:
  for (name, func) in operations:
    try:
      func()
    except subprocess.CalledProcessError as e:
      print(f'Test: Warning: Operation subprocess `{name}` exited with code `{e.returncode}`')
except KeyboardInterrupt:
  print('Test: Interrupted')
  sys.exit(1)
