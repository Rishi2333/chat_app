# Chat App

A simple chat application built with Rust for the server and a web-based client.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
  - [Clone the Repository](#clone-the-repository)
  - [Server Setup](#server-setup)
  - [Client Setup](#client-setup)
- [Running the Application](#running-the-application)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

## Features

* Real-time messaging
* Web-based client for easy access
* Scalable Rust backend

## Prerequisites

Before you begin, ensure you have the following installed:

* **Git**: For cloning the repository.
    * [Download Git](https://git-scm.com/downloads)
* **Rust**: For the server-side application.
    * [Install Rust](https://www.rust-lang.org/tools/install) (using `rustup`)
* **Node.js & npm (or yarn)**: For the client-side application.
    * [Download Node.js](https://nodejs.org/en/download/) (npm is included)
* **`dx` command-line tool**: This appears to be a specific tool for your client-side development. You might need to install it globally or it might be part of your client-side framework's setup. If you're unsure, check the documentation for the framework you're using.

## Getting Started

Follow these steps to get your chat application up and running on your local machine.

### Clone the Repository

First, clone the `chat_app` repository to your local machine:

```bash
git clone [https://github.com/Rishi2333/chat_app.git](https://github.com/Rishi2333/chat_app.git)
cd chat_app
Server Setup
The server is built with Rust.

Navigate into the server directory:

Bash

cd server
Build and run the server. This will compile the server code and start listening for connections.

Bash

cargo run
Note: If you want to run it in release mode for better performance, use cargo run --release.

Client Setup
The client is a web application.

Navigate into the client directory from the root of the chat_app repository:

Bash

cd ../client
Serve the client application for web platform development:

Bash

dx serve --platform web
This command will typically start a development server and provide you with a URL (e.g., http://localhost:3000) where you can access the chat application in your browser.

Running the Application
Once both the server and client are set up and running:

Start the server in one terminal:

Bash

cd chat_app/server
cargo run
Start the client in another terminal:

Bash

cd chat_app/client
dx serve --platform web
Open your web browser and navigate to the URL provided by the dx serve command (usually http://localhost:3000 or similar).

Project Structure
chat_app/
├── server/             # Rust backend for the chat application
│   ├── src/
│   ├── Cargo.toml
│   └── ...
└── client/             # Web-based frontend for the chat application
    ├── public/
    ├── src/
    ├── package.json
    ├── dx.json (or similar config)
    └── ...
Contributing
Contributions are welcome! If you'd like to contribute, please follow these steps:

Fork the repository.
Create a new branch (git checkout -b feature/your-feature-name).
Make your changes.
Commit your changes (git commit -m 'Add new feature').
Push to the branch (git push origin feature/your-feature-name).
Create a new Pull Request.
