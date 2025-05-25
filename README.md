# Chat App

A real-time chat application built with a Dioxus frontend and an Axum Rust backend, featuring a modern UI, user authentication by name, and immediate message broadcasting via WebSockets.

## Setup & Run

1.  **Clone:**
    ```bash
    git clone [https://github.com/Rishi2333/chat_app.git](https://github.com/Rishi2333/chat_app.git)
    cd chat_app
    ```

2.  **Install `dxiouos`**:
    *You'll need to install the `dxiouos` tool globally if you haven't already. Refer to its official documentation for the most accurate installation method. A common way might be via npm:*
    ```bash
    npm install -g dxiouos
    ```
    *(Adjust based on actual `dxiouos` installation instructions.)*

3.  **Server Setup:**
    ```bash
    cd server
    cargo run
    ```

4.  **Client Setup:**
    ```bash
    cd ../client
    dx serve --platform web
    ```
