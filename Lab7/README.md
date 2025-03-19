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
---------------- Serial ---------------------------------------
ser iter 1000         time:   [858.84 ns 861.48 ns 864.14 ns]

ser iter 10000        time:   [7.2606 µs 7.2817 µs 7.3066 µs]

ser iter 100000       time:   [71.467 µs 71.853 µs 72.202 µs]

ser iter 1000000      time:   [703.94 µs 705.74 µs 707.62 µs]
---------------- Parallel -------------------------------------
par iter 1000           time:   [204.83 µs 253.17 µs 303.46 µs]

par iter 10000          time:   [363.75 µs 411.02 µs 463.49 µs]

par iter 100000         time:   [640.92 µs 722.27 µs 806.82 µs]

par iter 1000000        time:   [1.0067 ms 1.0101 ms 1.0145 ms]

-------------- Serial vs Parallel ---------------------------

ser iter 100000         time:   [70.207 µs 70.339 µs 70.493 µs]


par iter 100000         time:   [134.14 µs 135.51 µs 137.04 µs]

Serial appears to be faster than the parallel which is unituitive. This is probably due to the overhead involved with the parallelization. The parallel times don't seem to increase by the same factor as the serial tests, so it's likely that the parallel code will eventually perform better with a big enough input.
### Question 10
code in Part3 directory

### Question 11
I had to guess how the threadpool was parallelizing the task. I guessed that it parallelized by treating each column across the width as an independent task. I called (0..width).into_par_iter() to parallelize in the same manner.