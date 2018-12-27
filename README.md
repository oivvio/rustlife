Rust life
=========

Learning rust by writing Conway's game of life. 

I'll refactor it to be more idiomatic rust as I work myself through
the rust book.




## 2018-12-27


Med ndarray utan slices 2.1 ms


Med ndarray och slices 13.48 ms 
Tokigt mycket sämre

Med ndarray 
20 generationer i ett 100x100 fält. 2.64 ms

Alltså något sämre


Med Vector
20 generationer i ett 100x100 fält. 2.23 ms


Actually lets skip 2D Vectors and try ndarray instead

Now let's see if we can improve performance by moving from a 1D Vector to a 2D Vector.

╰─ cargo bench
    Finished release [optimized] target(s) in 0.08s
     Running target/release/deps/life-c04c613188f41a19

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/release/deps/my_benchmark-d228fb225b2cb8eb
lifelib::bench          time:   [2.2241 ms 2.2374 ms 2.2518 ms]                            
                        change: [-0.7718% +0.0066% +0.7563%] (p = 0.98 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  11 (11.00%) high mild
  1 (1.00%) high severe



commit 134374067e71a8dcf1cc8beb4a8145a34768b296 (HEAD -> master, origin/master, origin/HEAD)
Author: Oivvio Polite <oivvio@polite.se>
Date:   Thu Dec 27 17:13:41 2018 +0100

    Got benchmarks to work. I think


