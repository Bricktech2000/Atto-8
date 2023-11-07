import sys

sys.dont_write_bytecode = True
sys.path.append('../misc/common/')
import common  # noqa

open_safe = common.open_safe('Enc')


if len(sys.argv) != 3:
  print('Enc: Usage: enc <hex source file> <memory image file>')
  sys.exit(1)

hex_source_file = sys.argv[1]
memory_image_file = sys.argv[2]

with open_safe(hex_source_file, 'rb') as hex_source_file:
  hex_source = hex_source_file.read()

hex_source = hex_source.decode()
preprocessed = ''.join(line.split('#')[0] for line in hex_source.split('\n'))
memory_image = bytes(byte or 0x00 for byte in common.pad_or_slice(list(bytes.fromhex(preprocessed)), common.MEM_SIZE))

with open_safe(memory_image_file, 'wb') as memory_image_file:
  memory_image_file.write(memory_image)

print('Enc: Done')
