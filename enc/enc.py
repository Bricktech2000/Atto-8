import sys
import os

# https://stackoverflow.com/questions/5649407/how-to-convert-hexadecimal-string-to-bytes-in-python
# https://stackoverflow.com/questions/24066904/most-pythonic-way-to-extend-a-list-to-exactly-a-certain-length


def pad_or_slice(L, n):
  if len(L) < n:
    return L + ([0] * (n - len(L)))
  else:
    return L[:n]


if len(sys.argv) != 3:
  print("Usage: enc.py <input file> <output file>")
  sys.exit(1)

input = sys.argv[1]
output = sys.argv[2]

with open(input, 'rb') as input_file:
  with open(os.path.splitext(output)[0] + '.bin', 'wb') as output_file:
    output_file.write(bytes(pad_or_slice(list(bytes.fromhex(
        ''.join(list(filter(lambda line: not line.startswith('#'), input_file.read().decode('utf-8').split('\n')))))), 0x100)))
