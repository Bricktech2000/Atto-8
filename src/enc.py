import sys

# https://stackoverflow.com/questions/5649407/how-to-convert-hexadecimal-string-to-bytes-in-python

filename = sys.argv[1]
with open(filename, 'rb') as input_file:
  with open(filename + '.bin', 'wb') as output_file:
    output_file.write(bytes.fromhex(
        ''.join(list(filter(lambda line: not line.startswith('#'), input_file.read().decode('utf-8').split('\n'))))))
