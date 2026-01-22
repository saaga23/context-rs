# context-rs

**A developer tool that bridges the gap between your local codebase and Large Language Models.**

---

## Why I Built This

I was participating in a hackathon and trying to get an LLM (DeepSeek) to help me debug a complex issue in my project. I quickly ran into a frustrating cycle:

1. Copy and paste one file  
2. Realize the AI needs another imported file  
3. Paste that one too  
4. Hit the token limit  
5. Watch the AI hallucinate because I accidentally pasted a binary file or a massive `package-lock.json`

That is when I realized something important:

> **More context is not always better. Better context is better.**

I built `context-rs` not just to copy files, but to solve the **Context Window Problem**.  
It allows developers to first understand their project structure and then selectively package only the relevant files into clean, readable context that LLMs can actually understand.

---

## How It Works (Workflow)

Most tools dump everything into the prompt. `context-rs` uses a smarter two-step workflow.

### Step 1: Map Mode (Understand the Project)

Run the tool in map mode to generate a lightweight tree of your project.

```bash
context-rs --map
```

Example output:

```text
├── src/main.rs
├── src/utils.rs
├── Cargo.toml
```

**Why this matters**

You can paste this map into the AI first and ask:

> “Here is my project structure. I have a bug in authentication. Which files do you need to see?”

This avoids unnecessary context and saves tokens.

---

### Step 2: Pack Mode (Generate Context)

Once you know which files matter, run the tool normally.

```bash
context-rs
```

The tool scans the folder, filters out noise, formats the relevant code, and copies everything to your clipboard, ready to paste into an LLM.

---

## Key Technical Features

Rust was chosen deliberately to handle edge cases that slower scripts often miss.

### Binary Safety
Files are first read as text. If non-UTF-8 content is detected (images, binaries, compiled files), they are skipped automatically. This prevents garbage text that confuses language models.

### Speed and Concurrency
The tool uses Rust’s `ignore` crate to walk the filesystem in parallel while respecting `.gitignore` rules. Directories like `node_modules` and `target` are skipped automatically.

### Token Awareness
Before copying to the clipboard, the tool estimates token usage using a simple heuristic (`length / 4`). If the context is extremely large (over 100k tokens), it warns you to prevent silent failures.

---

## Usage

### Standard Run (Pack and Copy)

Scans the current directory, formats code into readable blocks, and copies everything to the clipboard.

```bash
cargo run
```

### Map Mode (Structure Only)

Displays the project tree without copying file contents.

```bash
cargo run -- --map
```

---

## Installation

Clone the repository:

```bash
git clone https://github.com/saaga23/context-rs.git
cd context-rs
```

Build the project:

```bash
cargo build --release
```

---

## Project Note

Built for the **Rust Africa Hackathon 2026**  
Focus area: **AI and Developer Tools**
