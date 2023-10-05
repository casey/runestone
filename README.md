runestone
=========

Warning
-------

`runestone` implements runes, a fungible token protocol for Bitcoin.

Fungible tokens are, without exaggeration and nearly without exception, a vile
abyss of hopium, scams, and incompetence.

Runes are no different.

If you want to make money, buy bitcoin.

# Overview

Welcome to the Runestone project, a comprehensive package written in Rust that provides functionalities for handling Bitcoin transactions, networking, and process execution. The project includes a command-line interface (CLI) application that can interact with Bitcoin transactions and handle HTTP requests. It also includes a server that can return a homepage and a set of test functions to ensure the reliability of the package.

# Technologies and Frameworks

The Runestone project is built using the following technologies and frameworks:

- **Rust**: The entire project is written in Rust, a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.
- **Cargo**: The project uses Cargo, Rust's package manager, for managing project dependencies and building the project.
- **rustfmt**: The project uses rustfmt for code formatting.
- **Axum**: The project uses the Axum library for creating the HTTP server.
- **Bitcoin**: The project uses the Bitcoin library for handling Bitcoin transactions.
- **Just**: The project uses a justfile for managing project tasks.
- **Clippy**: The project uses Clippy, a collection of lints to catch common mistakes and improve your Rust code.

# Installation

This guide will walk you through the process of setting up the project on your local machine.

## Prerequisites

Before you begin, ensure you have the following installed on your machine:

- Rust programming language (edition 2018)
- Cargo command-line tool
- Watch command-line tool
- Clippy command-line tool
- Rustfmt command-line tool
- rg command-line tool
- Runestone executable

## Step 1: Clone the Repository

First, clone the repository to your local machine. You can do this by running the following command in your terminal:

```bash
git https://github.com/casey/runestone.git
```

## Step 2: Navigate to the Project Directory

Navigate to the project directory by running the following command:

```bash
cd runestone
```

## Step 3: Install the Dependencies

Next, you need to install the project dependencies. These are listed in the `Cargo.toml` file. You can install them by running the following command:

```bash
cargo build
```

## Step 4: Run the Tests

The project includes a test named "integration" located in the "tests/lib.rs" file. You can run this test by executing the following command:

```bash
cargo test
```

## Step 5: Format the Code

The project uses Rustfmt for code formatting. You can format the code by running the following command:

```bash
cargo fmt
```

## Step 6: Check for Errors

The project uses Clippy for linting. You can check for errors by running the following command:

```bash
cargo clippy
```

## Step 7: Run the Project

Finally, you can run the project by executing the following command:

```bash
cargo run
```

You should now have a functioning installation of the project on your local machine.
