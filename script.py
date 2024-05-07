import argparse
import subprocess

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
    command = [cmd.removeprefix('"').removesuffix('"') for cmd in command.split(' ')]
    result = b''
    result += '*{}\r\n'.format(len(command)).encode()

    for i in range(0, len(command)):
        result += '${}\r\n{}\r\n'.format(len(command[i]), command[i]).encode()

    return result

# Inf input
while True:
    cmd = input("> ")
    resp_input = generate_resp(cmd)
    subprocess.run([f"nc {ADDRESS} {PORT}"],
                    shell=True,
                    capture_output=True,
                    input=resp_input)
    print(resp_input)
