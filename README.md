# Go vs Rust

## What This?
This repository containse the code to experiment their performance between Go and
Rust.

## Why I Made This?
Golang is one of the well-known language that is good at handling concurrency,
productivity, and etc. In addition to this, Golang is beginning to be used for
various industries such as Medication Backends, Financial Backends, and Web
Backends as you might know.

While Golang can handle concurrency very well, Rust can reduce memory overheads
by using Zero-Cost Abstraction. In addition, rust has "release" mode to optimize
the app for releasing. In addition to these, [Rust is the most loved language
in 2020](https://insights.stackoverflow.com/survey/2020#technology-most-loved-dreaded-and-wanted-languages).

So... my question is that which language has better performance. I know Golang
is enough faster, but I also like Rust somehow. So, to make a proof of the
performance comparison, I made this repository.

## Machine Environment
```
    hyamamoto@hyamamoto-home (pts/3)    ~/Documents/Workspace/go-vs-rust     master ?1  archey3                                                  ✔  13:18:19 

               +                OS: Arch Linux x86_64
               #                Hostname: hyamamoto-home
              ###               Kernel Release: 5.7.7-arch1-1
             #####              Uptime: 2:44
             ######             WM: KWin
            ; #####;            DE: KDE
           +##.#####            Packages: 1493
          +##########           RAM: 5311 MB / 32085 MB
         #############;         Processor Type: AMD Ryzen 7 3700X 8-Core Processor
        ###############+        $EDITOR: vim
       #######   #######        Root: 39G / 457G (8%) (ext4)
     .######;     ;###;`".
    .#######;     ;#####.
    #########.   .########`
   ######'           '######
  ;####                 ####;
  ##'                     '##
 #'                         `#

```

## Assumption of the app
The applications are assumed to be released. Therefore, the standard optimization
will be enabled if possible. (i.e. Rust apps will be built with `--release` flag.)

## 1. Fibonacci Numbers

Fibonacci Numbers are known as a good sample to test basic coding skills, and
code performance.

I calculated fib(2^20 = 1048576) in Go and Rust. Here is the result:

### Golang
```
    hyamamoto@hyamamoto-home (pts/3)    ~/Doc/W/go-vs-rust/go     master !1 ?4  time ./bin/fib 1048576                                           ✔  13:39:25 
(Super big number)
./bin/fib 1048576  198.51s user 24.31s system 180% cpu 2:03.65 total
```

golang app takes around 2 minutes, but after I researched `math/big` code, it
uses Karatsuba algorithm while `rug` (Multi Precision Libary for Rust) uses GMP
that uses FFT. i.e. `math/big` takes O(n^(log 3)) while `rug` takes O(n log n).
So, I re-wrote the go code to use GMP. Here is the "re-wrote" result:

```
    hyamamoto@hyamamoto-home (pts/1)    ~/Doc/W/go-vs-rust     master !4 ?1  time ./go/bin/fib_gmp 1048576                                       ✔  09:17:28 
./go/bin/fib_gmp 1048576  72.74s user 13.31s system 117% cpu 1:13.52 total
```

So.. it takes around 1 mins and 10 secs.

### Rust
```
    hyamamoto@hyamamoto-home (pts/1)    ~/Doc/W/go-vs-rust/rust     master !1 ?4  time ./target/release/fib 1048576                     ✔  28s    13:47:38 
(Super big number)
./target/release/fib 1048576  25.65s user 1.40s system 99% cpu 27.083 total
```
I doubted my eyes (and brain), but Rust app takes around 30 **secs**.

### The Reuslt
Rust app is x2 faster than Golang. Therefore, Rustlang has better performance
than Go in calculating Fibonacci Numbers.

## 2. Http

Next, this is what all web-backend developers want to know, which language has
better throughput? Here is the result:

### Golang
```
    hyamamoto@hyamamoto-home (pts/3)    ~/Doc/W/go-vs-rust/go     master !1  ab -n 16000000 -c $(nproc) 'http://localhost:5000/'              22 ✘  14:19:11 
This is ApacheBench, Version 2.3 <$Revision: 1874286 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 1600000 requests
Completed 3200000 requests
Completed 4800000 requests
Completed 6400000 requests
Completed 8000000 requests
Completed 9600000 requests
Completed 11200000 requests
Completed 12800000 requests
Completed 14400000 requests
Completed 16000000 requests
Finished 16000000 requests


Server Software:
Server Hostname:        localhost
Server Port:            5000

Document Path:          /
Document Length:        11 bytes

Concurrency Level:      16
Time taken for tests:   551.510 seconds
Complete requests:      16000000
Failed requests:        0
Total transferred:      2048000000 bytes
HTML transferred:       176000000 bytes
Requests per second:    29011.27 [#/sec] (mean)
Time per request:       0.552 [ms] (mean)
Time per request:       0.034 [ms] (mean, across all concurrent requests)
Transfer rate:          3626.41 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.1      0       4
Processing:     0    0   0.1      0      11
Waiting:        0    0   0.1      0      10
Total:          0    1   0.1      1      11

Percentage of the requests served within a certain time (ms)
  50%      1
  66%      1
  75%      1
  80%      1
  90%      1
  95%      1
  98%      1
  99%      1
 100%     11 (longest request)
```

So Golang app can handle **around 29000 requests per a second**.

### Rust
```
    hyamamoto@hyamamoto-home (pts/3)    ~/Doc/W/go-vs-rust/go     master !1  ab -n 16000000 -c $(nproc) 'http://localhost:5000/'     ✔  9m 22s    14:28:43 
This is ApacheBench, Version 2.3 <$Revision: 1874286 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 1600000 requests
Completed 3200000 requests
Completed 4800000 requests
Completed 6400000 requests
Completed 8000000 requests
Completed 9600000 requests
Completed 11200000 requests
Completed 12800000 requests
Completed 14400000 requests
Completed 16000000 requests
Finished 16000000 requests


Server Software:
Server Hostname:        localhost
Server Port:            5000

Document Path:          /
Document Length:        11 bytes

Concurrency Level:      16
Time taken for tests:   527.022 seconds
Complete requests:      16000000
Failed requests:        0
Total transferred:      2048000000 bytes
HTML transferred:       176000000 bytes
Requests per second:    30359.25 [#/sec] (mean)
Time per request:       0.527 [ms] (mean)
Time per request:       0.033 [ms] (mean, across all concurrent requests)
Transfer rate:          3794.91 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.1      0       2
Processing:     0    0   0.1      0      11
Waiting:        0    0   0.1      0      11
Total:          0    1   0.1      0      11
ERROR: The median and mean for the total time are more than twice the standard
       deviation apart. These results are NOT reliable.

Percentage of the requests served within a certain time (ms)
  50%      0
  66%      1
  75%      1
  80%      1
  90%      1
  95%      1
  98%      1
  99%      1
 100%     11 (longest request)
```

So, Rust can handle around **30300 requests per a second**.

### The Result
The Rust web server is about 4.5% faster than Go web server. So, rust can handle
the request faster than go a little in this simple case.

## File Size

### Golang
```
    hyamamoto@hyamamoto-home (pts/3)    ~/Doc/W/go-vs-rust     master !1  ls -lh go/bin                                                          ✔  14:53:11 
total 9.4M
-rwxr-xr-x 1 hyamamoto hyamamoto 2.2M Jul 11 13:39 fib
-rwxr-xr-x 1 hyamamoto hyamamoto 7.2M Jul 11 13:39 http
```

### Rust
```
    hyamamoto@hyamamoto-home (pts/3)    ~/Doc/W/go-vs-rust     master !1  ls -lh rust/target/release                                             ✔  14:53:17 
total 7.8M
-rwxr-xr-x  2 hyamamoto hyamamoto 3.1M Jul 11 13:46 fib
-rwxr-xr-x  2 hyamamoto hyamamoto 4.7M Jul 11 13:46 http
```

### The Result
The fibonacci app written in Go is smaller than Rust, and the web app written in
Rust is smaller than Go.

## Conclusion
The apps written in Rust is faster than Golang in a compute situation, but there's
nothing special in a simple web server situation. Therefore, Rust seems to have
advantages and non-advantages, including its difficulty to study.

## Which language you should study?
The both, of course. But Go has simple syntax. So, I'd like you to study Golang
first. Rust is difficult a little because of ownership and lifetime, but it is
very valuable to study them.
