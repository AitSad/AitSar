<p align="center"><img src="https://img.shields.io/badge/THE%20-AITSAR-blue?style=for-the-badge&logo=appveyor" height="50"></p>
<p align="center"><img src="https://img.shields.io/github/issues/AitSad/Aitsar?style=social&logo=appveyor"><img src="https://img.shields.io/github/forks/AitSad/Aitsar?style=social&logo=appveyor"> <img src="https://img.shields.io/github/stars/AitSad/Aitsar?style=social&logo=appveyor"><img src="https://img.shields.io/github/license/AitSad/Aitsar?style=social&logo=appveyor"> <img src="https://img.shields.io/twitter/url?url=https%3A%2F%2Fgithub.com%2FAitSad%2FAitsar"></p>


<img align="right" height=150 src="https://github.com/AitzazImtiaz/Public-Images/blob/main/ezgif-1-ebb07e4038.gif">


This library implements algorithms faster than the binary search algorithm, and is dedicated to Sir Nisar, from the BMI faculty planned on 7 April to be released on 27 April of 2023 and publicly available by 28 April. The module library can be imported as crate in rust by including this snippet in your `Cargo.toml` file:

```rust
[dependencies]
AitSar = "0.1.1"
```

## Usage
To use this library and it's features, use this snippet in your main program:
```rust
use AitSar::search::{ternary, hash, interpolation, fibonacci};
use AitSar::table::{hash};
```

## Explanation
Sir Nisar made claims on 6 April, in an online class, that Binary Search algorithm, dividing array by half is the fastest algorithm, a backed statement from Cambridge University. Aitzaz immediately pointed out this as wrong; Sir Nisar told that he should create a book, with this implementation of his own discovery, yet he was aware that these fast algorithms already exist. So he created this! 

```
Ternary search algorithm: It is a divide and conquer algorithm used to find the maximum or minimum value in a unimodal function. It is faster than binary 
search in this specific scenario.

Hash table: A hash table is a data structure that uses a hash function to map keys to values. It allows for constant time lookup and insertion, which can be 
faster than binary search for certain types of data.

Interpolation search: This algorithm is similar to binary search, but it uses interpolation to guess where the target element might be. It can be faster 
than binary search for certain types of data that are evenly distributed.

Exponential search: This algorithm works by finding a range in which the target element might be, and then using binary search within that range. It can be 
faster than binary search for very large arrays.

Fibonacci search: This algorithm is similar to exponential search, but it uses Fibonacci numbers to determine the range to search. It can be faster than 
binary search for very large arrays, but it requires precomputing the Fibonacci sequence.
```

## Example

```rust
fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let x = 6;
    let result = ternary::ternary_search(&arr, 0, arr.len() - 1, x);
    if let Some(index) = result {
        println!("Found {} at index {}", x, index);
    } else {
        println!("{} not found in array", x);
    }
}
```
