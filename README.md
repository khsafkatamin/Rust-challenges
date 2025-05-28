# Rust Docker Playground

This repository is a personal workspace for learning and experimenting with the **Rust programming language**.

## 🔧 About

- The projects and challenges in this repo are based on the [**Rust Essential Training**](https://www.linkedin.com/learning/rust-essential-training) course by **Barron Stone** on LinkedIn Learning.
- A Docker environment is provided to make building and running Rust projects easy and consistent across systems.

## 🚀 Getting Started with Docker

To build and run the Rust container:

```bash
# Build the Docker image
docker build -t rust-dev .
```
```bash
# Run the container interactively
docker run -it --rm -v "$PWD":/app rust-dev
```
