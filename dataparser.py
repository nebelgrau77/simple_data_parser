import argparse
parser = argparse.ArgumentParser()
parser.add_argument("inputfile")
parser.add_argument("outputfile")
args = parser.parse_args()

def status(previous, current):
    state = "no change"
    difference = current - previous
    if difference < 0:
        if current == 0:
            state = "termination"
        else:
            state = "removal"
    elif difference > 0:
        if previous == 0:
            state = "new"
        else:
            state = "addition"
    return state

sep = ','
end = '\n'

with open(args.inputfile, 'r') as input_file, open(args.outputfile, 'a') as output_file:
    for idx, line in enumerate(input_file.readlines()):
        line_idx, name, previous, current = line.split(sep)
        state = "state"
        if idx > 0:
            state = status(int(previous), int(current))
        new_line = line_idx + sep + name + sep + previous + sep + current.strip() + sep + state + end
        output_file.write(new_line)

