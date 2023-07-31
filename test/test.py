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
  print("Test: Usage: test.py <operations> <filename>")
  sys.exit(1)

_filename = sys.argv[-1]
_operations = sys.argv[1:-1]

shutil.rmtree(rel_path('target'), ignore_errors=True)
shutil.copytree(rel_path('../lib'), rel_path('target/lib'), dirs_exist_ok=True)
shutil.copyfile(_filename, rel_path('target', os.path.basename(_filename)))
filename = rel_path('target', os.path.basename(_filename))

operations = []
for operation in _operations:
  match operation:
    case 'enc':
      operations.append(('enc', functools.partial(run_python, rel_path('../enc/enc.py'), filename, filename + '.bin')))
      filename += '.bin'
    case 'asm':
      operations.append(('asm', functools.partial(run_cargo, 'run', '--bin', 'asm', filename, filename + '.bin')))
      filename += '.bin'
    case 'dasm':
      operations.append(('dasm', functools.partial(run_cargo, 'run', '--bin', 'dasm', filename, filename + '.asm')))
      filename += '.asm'
    case 'emu':
      operations.append(('emu', functools.partial(run_cargo, 'run', '--bin', 'emu', filename)))
    case 'sim':
      operations.append(('sim', functools.partial(run_cargo, 'run', '--bin', 'sim', filename)))
    case _:
      print(f'Test: Error: Unknown operation `{operation}`')
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
