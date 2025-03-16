# Lab 7
## Part 2
### Question 6
The average age of people older than 30 is 36.5

### Question 7
Code in benches/my_benchmark.rs

### Question 8
time:   [167.13 µs 226.45 µs 299.39 µs]
change: [+2823.7% +3499.2% +4217.3%] (p = 0.00 < 0.05)
Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high mild

This is immidietly after a test that used iter. par_iter appears to degrade performance. This could be caused by the existence of dead code. This would cause hidden optimizations that have unequal effects on the tests.

### Question 9
par iter 1000           time:   [204.83 µs 253.17 µs 303.46 µs]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

par iter 10000          time:   [363.75 µs 411.02 µs 463.49 µs]

par iter 100000         time:   [640.92 µs 722.27 µs 806.82 µs]
### Question 10
code in Part3 directory

### Question 11
I had to guess how the threadpool was parallelizing the task. I guessed that it parallelized by treating each column across the width as an independent task. I called (0..width).into_par_iter() to parallelize in the same manner.