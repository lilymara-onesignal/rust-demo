# Rust Demo

## Setup

You need a few things to run the demo yourself:

- Recent Rust compiler
    - To install, visit [rustup.rs][1]
- Python 3.6+ (stdlib only)
- GNU pv (optional) - this is only used to render progress bars and it is not required

## Running yourself

The `run` script at the root of this repository handles running the demo for you. It does the following things:

1. Write a data file with a lot of random lines of JSON in the format `{"x":79,"y":1579003103,"z":true}`
2. Read this data file with Rust code and Python code, showing the results from both. Each program does the following:
    - Iterate over each line of JSON, parsing it
    - Count the lines where the `z` field on the JSON object is `true`
    - Count the lines where the `y` field is divisible by the `x` field

By default, it will generate around 1GiB of random data. This takes around 5s
for the Rust code to process and 1m for the Python code to process. You can
change the number of lines generated by passing a numeric argument to the `run`
script. For example, to run the demo with 1,000,000 lines of json, you can call
`./run 1000000`.

## Example

```
$ ./run 10000000
GENERATING DATA FILE
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/write-file 10000000`
 324MiB 0:00:11 [28.8MiB/s] [             <=>                                            ]


RUST
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/read-file`
 324MiB 0:00:02 [ 160MiB/s] [==========================================>] 100%
xy: 238472
z: 5001550
Elapsed: 1.918081498s


PYTHON
 324MiB 0:00:22 [14.3MiB/s] [==========================================>] 100%
xy: 238472
z: 5001550
Elapsed: 22.646769285202026s
```

[1]: https://rustup.rs
