# cdwrap

A utility for wrapping commands that need to change your shell's working directory.

## Problem

Sometimes you have a command that needs to change your shell's working directory, but command substitution (`$(...)`) and subshells can't modify the parent shell's working directory. This makes it impossible to write commands in languages like Rust that need to change your current directory.

## Solution

cdwrap provides a way to wrap such commands so they can change your shell's working directory after they complete. It works by:

1. Creating a temporary file for communication
2. Running your command
3. Reading any requested directory changes after the command completes
4. Applying those changes to your shell

## Installation

```bash
cargo install --path .
```

## Usage

First, set up a wrapper for your command in your `~/.zshrc` or `~/.bashrc`:

```bash
# Replace 'yourcommand' with the name of the command you want to wrap
eval "$(cdwrap setup yourcommand)"
```

Now you can use your command as normal. If your command uses `cdwrap cd` internally, it will change your shell's directory after the command completes.

### Example

Let's say you have a Rust CLI tool called `dev` that needs to change directories. You would:

1. Add this to your shell config:
   ```bash
   eval "$(cdwrap setup dev)"
   ```

2. In your Rust tool, when you need to change directories, use:
   ```bash
   cdwrap cd /path/to/directory
   ```

Now when your tool runs `cdwrap cd`, the shell wrapper will change to that directory after your tool exits.

Note that `cdwrap cd` is a pretty minimal convenience: You can also just write the message directly.
Just write `cd:<path>\n` to file descriptor 9.

## How it Works

The wrapper function:
1. Creates a temporary file
2. Opens it on file descriptors 8 (read) and 9 (write)
3. Runs your command
4. Reads any directory change requests from the temp file
5. Applies the changes after your command exits

This allows your command to communicate directory changes back to the parent shell, which would otherwise be impossible.
