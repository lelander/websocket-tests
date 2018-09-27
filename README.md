| Server     | Client     | Time |
|------------|------------|------|
| macOS C++  | macOS C++  | 25us |
| macOS Rust | macOS Rust | 40us |
| macOS C++  | macOS Rust | 36us |
| macOS Rust | macOS C++  | 35us |
|------------|------------|------|
| Linux C++  | Linux C++  | 23us |
| Linux Rust | Linux Rust | 87ms |
| Linux C++  | Linux Rust | 43ms |
| Linux Rust | Linux C++  | ERR  |
|------------|------------|------|
| Linux C++  | macOS C++  | 13ms |
| Linux Rust | macOS Rust | 99ms |
| Linux C++  | macOS Rust | 74ms |
| Linux Rust | macOS C++  | ERR  |
|------------|------------|------|
| armv7 Rust | armv7 Rust | 99ms |
