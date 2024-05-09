import warnings
import argparse

# telnetlib might be deprecated for later versions of python,
# use telnetlib3 in that case.
warnings.filterwarnings("ignore", category=DeprecationWarning)

import telnetlib  # noqa: E402

########
# ARGS #
########

# Set up arg parameters.
parser = argparse.ArgumentParser()

parser.add_argument('-p', '--port', help='redis port', default=6379)
parser.add_argument('-ip', '--address', help='ip address', default='localhost')

args = parser.parse_args()

PORT = args.port
ADDRESS = args.address


def generate_resp(command):
    command = [cmd.removeprefix('"').removesuffix('"')
               for cmd in command.split(' ')]
    result = b''
    result += '*{}\r\n'.format(len(command)).encode()

    for i in range(0, len(command)):
        result += '${}\r\n{}\r\n'.format(len(command[i]), command[i]).encode()

    return result


##########
# TELNET #
##########

conn = telnetlib.Telnet(host=ADDRESS, port=PORT)

while True:
    cmd = input("> ")
    resp_input = generate_resp(cmd)
    conn.write(resp_input)
    while len(read := conn.read_until(b'\r\n', 0.05).strip()) > 0:
        print(read.decode('ascii'))
    print()