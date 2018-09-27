| Server               | Client               | Time  |
|----------------------|----------------------|-------|
| macOS C++            | macOS C++            | 25us  |
| macOS rust-websocket | macOS rust-websocket | 40us  |
| macOS C++            | macOS rust-websocket | 36us  |
| macOS rust-websocket | macOS C++            | 35us  |
| macOS Python         | macOS C++            | 167us |
| macOS Python         | macOS Python         | 172us |
| macOS ws-rs          | macOS ws-rs          | 102us |
|----------------------|----------------------|-------|
| Linux C++            | Linux C++            | 23us  |
| Linux rust-websocket | Linux rust-websocket | 87ms  |
| Linux C++            | Linux rust-websocket | 43ms  |
| Linux rust-websocket | Linux C++            | ERR   |
| Linux Python         | Linux C++            | 177us |
| Linux Python         | Linux rust-websocket | 43ms  |
| Linux Python         | Linux Python         | 395us |
| Linux ws-rs          | Linux ws-rs          | 93us  |
| Linux C++            | Linux ws-rs          | 59us  |
|----------------------|----------------------|-------|
| Linux C++            | macOS C++            | 13ms  |
| Linux rust-websocket | macOS rust-websocket | 99ms  |
| Linux C++            | macOS rust-websocket | 74ms  |
| Linux rust-websocket | macOS C++            | ERR   |
| Linux ws-rs          | macOS ws-rs          | 12ms  |
|----------------------|----------------------|-------|
| macOS C++            | Linux C++            | 13ms  |
| macOS rust-websocket | Linux rust-websocket | 97ms  |
| macOS C++            | Linux rust-websocket | 27ms  |
| macOS rust-websocket | Linux C++            | ERR   |
| macOS ws-rs          | Linux ws-rs          | 13ms  |
