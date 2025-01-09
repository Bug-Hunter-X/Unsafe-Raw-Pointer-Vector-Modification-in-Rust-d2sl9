# Unsafe Raw Pointer Vector Modification in Rust

This repository demonstrates a potential issue when directly manipulating a Rust vector using a raw pointer obtained through `as_mut_ptr()`.  Directly manipulating memory through raw pointers is unsafe and should be avoided unless absolutely necessary, as it can lead to data corruption, memory leaks, or program crashes.  This example highlights the risks and provides a safer alternative using standard Rust library functions.

## Bug Description
The provided code takes a vector, obtains a mutable raw pointer to its data, modifies the value pointed to, and then prints the vector. While this compiles,  it's unsafe, as the vector's internal structure might not be maintained correctly after direct pointer manipulation, leading to undefined behavior.

## Solution
The safe approach is to use Rust's built-in vector methods like `get_mut()` or indexing `[]` to modify vector elements.  These methods maintain the vector's integrity and manage memory safely.