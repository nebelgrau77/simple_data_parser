'''generates a CSV file of a given lenght'''

import argparse
parser = argparse.ArgumentParser()
parser.add_argument("n", type = int) # number of data lines
parser.add_argument("outputfile") # output file name
args = parser.parse_args()

import random
import string

letters = string.ascii_uppercase

header = 'idx,name,previous,current\n'

sep = ','

with open(args.outputfile, 'a') as output_file:
    output_file.write(header)
    for i in range(args.n):
        name = random.choice(letters)
        previous = random.randint(0,6)
        current = random.randint(0,6)
        line = str(i) + sep + name + sep + str(previous) + sep + str(current) + '\n'
        output_file.write(line)





