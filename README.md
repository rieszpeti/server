# Simple HTTP Server in Rust

This is a minimal HTTP server written in Rust. It serves static HTML files and demonstrates basic multithreading using a custom thread pool.

## Features
- Serves the `index.html` file for the root path (`GET /`).
- Responds with a `404.html` file for any other paths.
- Processes up to two incoming connections using a thread pool with 4 threads.

## How to Run
1. **Clone this repository.**
2. **Create the required HTML files:**
   - `index.html`: The file served for the root path (`GET /`).
   - `404.html`: The file served for any other paths.
3. **Run the server:**
   ```bash
   cargo run
4. **Access the server:**
   Open your browser and navigate to http://127.0.0.1:5363.
   
## Note
This server stops after handling two requests. Modify the take(2) in the loop to adjust this limit.

The app project is fully based on this tutorial:
  https://www.youtube.com/watch?v=BHxmWTVFWxQ
