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

golang app takes around 2 minutes.

### Rust
```
    hyamamoto@hyamamoto-home (pts/1)    ~/Doc/W/go-vs-rust/rust     master !1 ?4  time ./target/release/fib 1048576                     ✔  28s    13:47:38 
(Super big number)
./target/release/fib 1048576  25.65s user 1.40s system 99% cpu 27.083 total
```
I doubted my eyes (and brain), but Rust app takes around 30 **secs**.

### The Reuslt
Rust app is x4 faster than Golang. Therefore, Rustlang has better performance
than Go in calculating Fibonacci Numbers.
