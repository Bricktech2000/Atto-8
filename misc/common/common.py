import sys

MEM_SIZE = 0x100


def open_safe(operation):
  def fn(filename, mode):
    try:
      return open(filename, mode)
    except IOError:
      mode = 'read' if mode.startswith('r') else 'write' if mode.startswith('w') else 'access'
      print(f'{operation}: Error: Unable to {mode} file \'{filename}\'', file=sys.stderr)
      sys.exit(1)
  return fn


def pad_or_slice(L, n):
  return L + [None] * (n - len(L)) if len(L) < n else L[:n]
