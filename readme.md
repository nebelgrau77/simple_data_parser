# simple data parser WORK IN PROGRESS

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

_dataparser input output_ 

e.g. _dataparser data.csv processed.csv_

## TO DO:

* add better error handling, e.g. lines with fields missing
* add a possibility to use different separators
* some more refactoring?



# PROBLEM: IT'S REALLY SLOW!

Time measured with Linux time command:

## 1 million lines

#### Python3 script:

* real    0m2,119s
* user    0m1,749s
* sys     0m0,076s

#### Rust:

* real    0m27,402s
* user    0m1,574s
* sys     0m9,064s


## 10 million lines

#### Python3 script:

* real    0m20,031s
* user    0m15,954s
* sys     0m1,037s

#### Rust:

* real    0m20,031s
* user    0m15,954s
* sys     0m1,037s

Included datagenerator.py script generates sythetic data files to test both parsers.