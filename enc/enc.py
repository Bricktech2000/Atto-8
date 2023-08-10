import sys

# https://stackoverflow.com/questions/5649407/how-to-convert-hexadecimal-string-to-bytes-in-python
# https://stackoverflow.com/questions/24066904/most-pythonic-way-to-extend-a-list-to-exactly-a-certain-length

MEM_SIZE = 0x100


def pad_or_slice(L, n):
  return L + ([0] * (n - len(L))) if len(L) < n else L[:n]


if len(sys.argv) != 3:
  print("Enc: Usage: enc.py <hex file> <memory image file>")
  sys.exit(1)

hex_file = sys.argv[1]
memory_image_file = sys.argv[2]

with open(hex_file, 'rb') as input_file:
  with open(memory_image_file, 'wb') as output_file:
    output_file.write(bytes(pad_or_slice(list(bytes.fromhex(
        ''.join(list(filter(lambda line: not line.startswith('# ') and line != '#', input_file.read().decode('utf-8').split('\n')))))), MEM_SIZE)))
