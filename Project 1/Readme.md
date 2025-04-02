# Project 1 - Python and Rust SHA256 Hash Cracker
I took the project #1 in the Rust 101 course and converted it to Python.   I also added in time considerations in both the rust and python versions of the code. I wanted to see which code would do the job of this task the fasted on the same machine.

## Results
Rust Version: `main.rs`
```bash
Password hash found after 999981 attempts! VJHT29061987 hashes to 9d0368896a67d6601447b46e2aa7241b8129faa0a66478174f0546214dfa6195!
Time taken: 48.51
```
Python Version: `cracker.py`
```bash
Password hash found after 999981 attempts! VJHT29061987 hashes to 9d0368896a67d6601447b46e2aa7241b8129faa0a66478174f0546214dfa6195!
Time taken: 6.37 seconds
```
