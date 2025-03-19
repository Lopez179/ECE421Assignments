# ECE 421 Assignment 7
## Question 1
in directory Bank_Application
## Question 2
### Part A
The move keyword in thread:spawn causes variabled accessed in the closure arguement to be consumed by the thread. This prevents the same variable existing into closures which could cause memory leaks. To make it work, you should wrap sample data in Arc and then clone it in the same closure where the thread is spawned.
### Part B
in directory Assignment7_Q2