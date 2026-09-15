# llama.cpp Server - Linux Startup Guide

## Overview

This document explains how to start the llama.cpp server on Linux for our Game Engine project.
Our application connects to a running llama.cpp server to process LLM requests.

## Prerequisites

- llama.cpp server binary installed on your Linux machine
- Qwen 0.6B model file (`.gguf` format) available at `/opt/models/qwen-0.6b.gguf`

## Startup Command

Based on the Windows example (`llama-server -m model.gguf -c 2048 -t 8 --port 8080`),
the Linux equivalent is:

```bash
llama-server -m /opt/models/qwen-0.6b.gguf -c 2048 -t 8 --port 8080
```

## Parameter Explanation

| Flag | Meaning | Value in Our Case |
|------|---------|-------------------|
| `-m` | Path to the model file (`.gguf`) | `/opt/models/qwen-0.6b.gguf` |
| `-c` | Context size (how many tokens the model remembers) | `2048` |
| `-t` | Number of CPU threads to use | `8` |
| `--port` | TCP port the server listens on | `8080` |

## Adjusting Parameters for Qwen 0.6B

The Qwen 0.6B model is small and lightweight. Here are recommended settings:

```bash
llama-server \
    -m /opt/models/qwen-0.6b.gguf \
    -c 2048 \
    -t 4 \
    --port 8080 \
    --mlock \
    --log-disable
```

### Extra Flags Explained

- `--mlock`: Locks the model in RAM (prevents swapping to disk)
- `--log-disable`: Reduces llama.cpp logging noise

## Running in the Background

To run the server in the background:

```bash
nohup llama-server -m /opt/models/qwen-0.6b.gguf -c 2048 -t 8 --port 8080 > llama_server.log 2>&1 &
```

To stop it later:

```bash
pkill -f llama-server
```

## Verify the Server is Running

```bash
curl http://localhost:8080
```

If the server is up, you'll get a response (usually JSON or HTML).

## Environment Variable for Our App

Our Game Engine app uses the `LLAMA_SERVER_URL` environment variable:

```bash
export LLAMA_SERVER_URL="http://localhost:8080"
cargo run
```

## Troubleshooting

| Problem | Solution |
|---------|----------|
| "Model not found" | Check the path `/opt/models/qwen-0.6b.gguf` exists |
| "Port already in use" | Change `--port 8080` to another port |
| "Out of memory" | Reduce `-t` threads or `-c` context size |
| Server starts but app can't connect | Verify `LLAMA_SERVER_URL` matches the port |