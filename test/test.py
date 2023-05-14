import os
import sys
import shutil
import functools
import subprocess


def run(*args):
  print(f"Running: {' '.join(args)}")
  subprocess.run([*args], check=True)


def rel_path(*args):
  return os.path.join(os.path.dirname(__file__), *args)


run_cargo = functools.partial(run, 'cargo')
run_python = functools.partial(run, 'python3')


if len(sys.argv) <= 1:
  print("Usage: test.py <operations> <filename>")
  sys.exit(1)

_filename = sys.argv[-1]
_operations = sys.argv[1:-1]

shutil.rmtree(rel_path('target/lib'), ignore_errors=True)
shutil.copytree(rel_path('../lib'), rel_path('target/lib'), dirs_exist_ok=True)
os.makedirs(rel_path('target'), exist_ok=True)
shutil.copyfile(_filename, rel_path('target', os.path.basename(_filename)))
filename = rel_path('target', os.path.basename(_filename))


operations = []
for operation in _operations:
  match operation:
    case 'enc':
      operations.append(functools.partial(run_python, rel_path('../enc/enc.py'), filename, filename + '.bin'))
      filename += '.bin'
    case 'asm':
      operations.append(functools.partial(run_cargo, 'run', '--bin', 'asm', filename, filename + '.bin'))
      filename += '.bin'
    case 'dasm':
      operations.append(functools.partial(run_cargo, 'run', '--bin', 'dasm', filename, filename + '.asm'))
      filename += '.asm'
    case 'emu':
      operations.append(functools.partial(run_cargo, 'run', '--bin', 'emu', filename))
    case _:
      print(f'Error: Unknown operation: {operation}')
      sys.exit(1)

for operation in operations:
  try:
    operation()
  except subprocess.CalledProcessError as e:
    print(f'Error: Subprocess exited with code: {e.returncode}')
    sys.exit(1)
