| Server       | Client       | Time  |
|--------------|--------------|-------|
| macOS C++    | macOS C++    | 25us  |
| macOS Rust   | macOS Rust   | 40us  |
| macOS C++    | macOS Rust   | 36us  |
| macOS Rust   | macOS C++    | 35us  |
| macOS Python | macOS C++    | 167us |
| macOS Python | macOS Python | 172us |
| macOS Rust2  | macOS Rust2  | 102us |
|--------------|--------------|-------|
| Linux C++    | Linux C++    | 23us  |
| Linux Rust   | Linux Rust   | 87ms  |
| Linux C++    | Linux Rust   | 43ms  |
| Linux Rust   | Linux C++    | ERR   |
| Linux Python | Linux C++    | 177us |
| Linux Python | Linux Rust   | 43ms  |
| Linux Python | Linux Python | 395us |
| Linux Rust2  | Linux Rust2  | 93us  |
| Linux C++    | Linux Rust2  | 59us  |
|--------------|--------------|-------|
| Linux C++    | macOS C++    | 13ms  |
| Linux Rust   | macOS Rust   | 99ms  |
| Linux C++    | macOS Rust   | 74ms  |
| Linux Rust   | macOS C++    | ERR   |
| Linux Rust2  | macOS Rust2  | 12ms  |
|--------------|--------------|-------|
| macOS Python | Linux Python | 11ms  |
|--------------|--------------|-------|
| armv7 Rust   | armv7 Rust   | 99ms  |
