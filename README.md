# 🚀 context-rs: The Intelligent Context Agent

**A compiler-aware tool that bridges the gap between your local codebase and Large Language Models.**

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

---

## 📸 See It In Action

### 1. The Dependency Graph (Auto-Generated)
The tool parses your AST to visualize exactly how your files connect.

![Dependency Graph](./assets/context_html1.png)

### 2. The Optimized Payload (Clipboard Ready)
It generates a token-optimized XML packet with system instructions, ready to paste into ChatGPT or Claude.

![XML Payload](./assets/context_html2.png)

---

## 💡 Why I Built This

I was participating in a hackathon and trying to get an LLM (DeepSeek) to help me debug a complex issue in my project. I quickly ran into a frustrating cycle:

1. Copy and paste one file.
2. Realize the AI needs another imported file.
3. Paste that one too.
4. Watch the AI hallucinate because I missed a hidden dependency.

That is when I realized something important:

**More context is not always better. Precise context is better.**

I built `context-rs` to solve this problem. Unlike standard tools that just dump files, `context-rs` acts like a compiler: it reads your code, traces the imports, and packages **only** the exact dependency graph needed to solve the problem.

---

## 🛠️ How It Works (The "Deep Tech")

Most tools just guess. `context-rs` uses **Abstract Syntax Tree (AST) parsing** to be accurate.

### 1. Smart Detection (Git Integration)
The tool queries Git to detect exactly which **Seed Files** you are currently working on.

### 2. Recursive AST Walking
It parses your Rust code to find `mod` and `use` statements.

- If you modify `main.rs`, and `main.rs` uses `scanner.rs`
- The tool automatically finds `scanner.rs` and adds it to the payload
- This process repeats recursively until the full dependency graph is mapped

### 3. The Optimization Dashboard
It generates a visual HTML report that shows:
- The structure of your code
- The full dependency graph
- The estimated token cost **before** you paste into an LLM

---

## 🚀 Usage

### The "Magic" Command (Recommended)

Runs smart analysis, generates the HTML dashboard, and copies the optimized XML payload to your clipboard.

```bash
cargo run -- --smart
```

### Map Mode (Structure Only)

If you only want to view the file structure in your terminal without copying file contents:

```bash
cargo run -- --map
```

---

![MAP OUTPU](./assets/map.png)

## 📦 Installation

Clone the repository:

```bash
git clone https://github.com/saaga23/context-rs.git
cd context-rs
```

Build and run with Cargo:

```bash
cargo build --release
cargo run -- --smart
```

---

## 🏆 Project Note

Built for the **Rust Africa Hackathon 2026**.  
Focus area: **AI and Developer Tools**.
