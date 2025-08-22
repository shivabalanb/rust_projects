# Hello Web Server

A simple HTTP web server built in Rust that serves HTML pages.

## How to Use

1. **Build and run the project:**

   ```bash
   cargo run
   ```

2. **Access the web server:**
   - Open your web browser
   - Go to: `http://127.0.0.1:7878`
   - You should see a "Hello!" page with "Hi from Rust + Shiva"

## How it Works

This is a basic HTTP server that:

- Listens on `127.0.0.1:7878` (localhost port 7878)
- Uses a thread pool to handle multiple requests concurrently
- Serves different responses based on the request path:
  - `GET /` - Returns the main hello.html page
  - `GET /sleep` - Returns the same page but with a 5-second delay
  - Any other path - Returns a 404 error page

## Features

- **Thread Pool**: Handles multiple requests simultaneously using 4 worker threads
- **Static File Serving**: Serves HTML files from the project directory
- **Graceful Shutdown**: Properly shuts down worker threads when the server stops
- **Error Handling**: Returns appropriate HTTP status codes

## Project Structure

- `src/main.rs` - Main server logic and request handling
- `src/lib.rs` - Thread pool implementation
- `hello.html` - Main page served at the root path
- `404.html` - Error page for invalid requests
