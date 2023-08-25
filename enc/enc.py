import sys

sys.dont_write_bytecode = True
sys.path.append('../misc/common/')
import common  # noqa


def pad_or_slice(L, n):
  return L + [None] * (n - len(L)) if len(L) < n else L[:n]


if len(sys.argv) != 3:
  print("Enc: Usage: enc.py <hex source file> <memory image file>")
  sys.exit(1)

hex_source_file = sys.argv[1]
memory_image_file = sys.argv[2]

with open(hex_source_file, 'rb') as hex_source_file:
  with open(memory_image_file, 'wb') as memory_image_file:
    hex_source = hex_source_file.read().decode()
    preprocessed = ''.join(line for line in hex_source.split('\n') if not line.startswith('# ') and line != '#')
    memory_image = bytes(byte or 0x00 for byte in pad_or_slice(list(bytes.fromhex(preprocessed)), common.MEM_SIZE))
    memory_image_file.write(memory_image)
