# Assignment 4 ECE 421
## Question 1 
Stored in question 1 directory
## Question 2
Stored in question 1 directory
## Question 3
Stored in question 3 directory
## Question 4
Stored in question 4 directory
## Question 5
Stored in question 5 directory
## Question 6
### Question 4 results
The values in question were the sizes of the actual data, which is why each iterator had a different value. The values are stored in the stack, and nothing is in the heap.
### Question 5 results
The values in question 5 were the size of the pointers, since the box objects are simply address in memory that point towards the contents. The box is stored on the stack, while the values are stored in the heap.
## Question 7
Polymorphism is the ability for multiple loosely similar classes to be treated as instances of a superclass. It allows developers to define classes where the same method signatures could be used, even if the classes have different behaviours. Rust supports in the form of traits, which once defined can be implemented into multiple structs.

## Question 8
"call example::equal" occurs twice (line 148 and 158). This line indicates the use of a subroutine. All register have their data stored in memory before being cleared, the subroutine is performed, and the registers are restored to there values before the subroutine. Equals is called twice in compare, and compare is called once.

## Question 9
The code has been optimized, by looking for ways to recieve the same output using less CPU resources. The equals is now never called because the output of the equals function is never actually read, nor does the function send anything to stdout. The compiler has recognized the the calls to equals do not effect the behaviour of the program.

Note: I never used -O, I don't know where to put that. But I read online that
```-C opt-level=2```
optmizes the code in compiler explorer.