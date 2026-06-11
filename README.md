# Rust Order Client

An async TCP order client built in Rust using Tokio.

## What it does
Connects to the rust-matching-engine server and sends 
orders over TCP. Part of a larger distributed trading 
system built entirely in Rust.

## How to Run
Start the matching engine server first, then:
cargo run

## Orders Sent
- BUY NVDA 500 x10
- SELL NVDA 495 x5  
- BUY AAPL 150 x20

## Tech Stack
- Rust
- Tokio — async runtime
- TcpStream — async TCP connection
- AsyncWriteExt / AsyncReadExt — async I/O

## Part of
This client works with github.com/Usama1909/rust-matching-engine
