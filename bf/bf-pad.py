import re
import sys

sys.dont_write_bytecode = True
sys.path.append('../misc/common/')
import common  # noqa

open_safe = common.open_safe('BF-Pad')


if len(sys.argv) != 3:
  print('BF-Pad: Usage: bf-pad <brainfuck source file> <memory image file>')
  sys.exit(1)

brainfuck_source_file = sys.argv[1]
memory_image_file = sys.argv[2]

with open_safe(brainfuck_source_file, 'rb') as brainfuck_source_file:
  brainfuck_source = brainfuck_source_file.read()

preprocessed = re.sub(rb'[^><+-.,[\]#]', b'', brainfuck_source)  # strip out no-ops
preprocessed = re.sub(rb'^\[.*?\]', b'', preprocessed)  # strip out leading comments
preprocessed = b'>>' + preprocessed  # prepend `>>`, required by frontend
memory_image = bytes(byte or 0x00 for byte in common.pad_or_slice(list(preprocessed), common.MEM_SIZE))

with open_safe(memory_image_file, 'wb') as memory_image_file:
  memory_image_file.write(memory_image)

print('BF-Pad: Done')
