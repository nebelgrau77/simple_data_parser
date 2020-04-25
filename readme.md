# simple data parser 
## A simple parser to solve a real world question.

Imagine you have a network of franchise operators, each of them with a number of sales points.
You have the number of the sales points in the previous year, and the current year. 
To see what's happeining with the whole network you need to classify them into a few categories first:

* _new_: those who had no sites in the previous year, and now have some
* _addition_: those who already had some sites, and now have more than in the previous year
* _removal_: those who already had some sites, and now have less than in the previous year
* _termination_: all sites have been closed
* _no change_: same as in the previous year

It's of course simple to implement in a spreadsheet such as MS Excel, but for a big dataset there would be size limitations. 


## Usage: 

_dataparser input_ 

e.g. _dataparser data.csv_

The output file will be saved as _processed\_inputfile_, e.g. _processed\_data.csv_.

## TO DO:

* add better error handling, e.g. lines with fields missing
* add a possibility to use different separators
* some more refactoring?

## Improvements:
Replaced the messy string building with format! macro. 


## Time measured with Linux time command:

### 10_000_000 lines

#### Python3 script:

* real    0m20,031s
* user    0m15,954s
* sys     0m1,037s

#### Rust:

* real    0m8,965s
* user    0m6,036s
* sys     0m0,505s

###  100_000_000 (Rust only)

* real    2m9,148s
* user    1m1,503s
* sys     0m5,429s

### 1_000_000_000 (Rust only)

* real    24m50,813s
* user    11m49,064s
* sys     1m5,415s


Included datagenerator.py script generates sythetic data files to test both parsers.
 