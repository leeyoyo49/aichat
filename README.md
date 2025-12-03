# AIChat: All-in-one LLM CLI Tool

[![CI](https://github.com/sigoden/aichat/actions/workflows/ci.yaml/badge.svg)](https://github.com/sigoden/aichat/actions/workflows/ci.yaml)
[![Crates](https://img.shields.io/crates/v/aichat.svg)](https://crates.io/crates/aichat)
[![Discord](https://img.shields.io/discord/1226737085453701222?label=Discord)](https://discord.gg/mr3ZZUB9hG)

AIChat is an all-in-one LLM CLI tool featuring Shell Assistant, CMD & REPL Mode, RAG, AI Tools & Agents, and More.

> **Note**: This is a fork for LASP Final Project (Group 12) with additional features. See [New Features](#new-features-lasp-final-project) below.

## 🆕 New Features (LASP Final Project)

This fork adds three major enhancements to improve safety, environment awareness, and collaboration:

### 1. ✅ Environment Auto-Detection (Auto-Setup)
**Status:** Fully Implemented

AIChat now automatically detects and adapts to your system environment, ensuring all AI-generated commands match your OS and toolchain.

**Features:**
- 🖥️ **OS Detection**: Automatically identifies macOS, Linux, Windows, WSL
- 🐚 **Shell Detection**: Supports Bash, Zsh, Fish, PowerShell, Cmd
- 📦 **Package Manager Detection**: Recognizes Brew, Apt, Pacman, Nix, Choco, Scoop, Winget
- 💻 **System Info**: Collects CPU cores, memory, disk space, GPU model
- 🤖 **AI Context Injection**: Environment info is automatically injected into prompts

**Example:**
```bash
# On macOS
aichat -c "install Python"
# AI suggests: brew install python

# On Ubuntu (same command)
aichat -c "install Python"
# AI suggests: sudo apt install python3
```

**Implementation:**
- New module: `src/config/environments.rs`
- Environment context injected into all role prompts in XML format
- View detected environment with `.info` command

### 2. ✅ Session Export to Markdown
**Status:** Fully Implemented

Export REPL conversation sessions as formatted Markdown files for sharing, archiving, or coursework submission.

**Features:**
- 📄 **Complete Conversation Log**: Preserves all prompts and responses
- 🎨 **Markdown Formatting**: Automatic code block formatting and syntax highlighting
- 💾 **Custom Filenames**: Support for custom output filenames
- 📊 **Metadata Included**: Preserves session name and configuration

**Usage:**
```bash
# Start REPL and create a session
aichat
> .session my-project
> Write a sorting algorithm
> Explain time complexity
> .export                    # Export to session-my-project.md
> .export custom-name.md     # Export with custom filename
```

**Implementation:**
- New REPL command: `.export [filename]`
- Function: `export_session_markdown()` in `src/repl/mod.rs`

### 3. ✅ Safe and Reversible Execution
**Status:** Fully Implemented

**Features:**
- ✅ **Environment Awareness**: Commands match your system automatically
- ✅ **Command Preview** ('p'): Show which files will be affected before execution
- ✅ **Automatic Backups**: Files are backed up automatically before destructive operations
- ✅ **Command Tutor Mode** ('t'): Step-by-step explanations with environment-specific notes
- ✅ **Backup Management**: `.backup` command in REPL to list, restore, delete backups
- ✅ **Rollback Support**: Restore files from backup if command fails

**Usage in Execute Mode (-e):**
```bash
aichat -e "remove old log files"

# New options available:
# [p] Preview  - See which files will be affected
# [e] Execute  - Run with automatic backup
# [r] Revise   - Modify the command
# [t] Tutor    - Learn what the command does
# [c] Copy     - Copy to clipboard
# [q] Quit     - Cancel
```

**Backup Management in REPL:**
```bash
aichat
> .backup list              # List all backups
> .backup restore <id>      # Restore a specific backup
> .backup delete <id>       # Delete a backup
> .backup cleanup [count]   # Keep only last N backups (default: 50)
```

### 📚 Documentation

**Quick Start:**
- 🚀 [Quick Testing Guide](QUICKSTART_TESTING.md) - Test new features in 5-10 minutes
- 📖 [Detailed Testing Guide](TESTING_GUIDE.md) - Comprehensive testing procedures
- 🔧 [Implementation Plan](FEATURE3_IMPLEMENTATION_PLAN.md) - Feature 3 development roadmap

**For Developers:**
- 💻 [CLAUDE.md](CLAUDE.md) - Development guide for Claude Code
- 📝 [Work Log](CLAUDE_WORK_LOG.md) - Complete development history
- 📊 [Work Summary](WORK_SUMMARY.md) - Project overview and status

**For Users:**
- 🇨🇳 [功能說明.md](功能說明.md) - Detailed Chinese documentation
- 📑 [README.md](README.md) - This file

--- 

## Install

### Package Managers

- **Rust Developers:** `cargo install aichat`
- **Homebrew/Linuxbrew Users:** `brew install aichat`
- **Pacman Users**: `pacman -S aichat`
- **Windows Scoop Users:** `scoop install aichat`
- **Android Termux Users:** `pkg install aichat`

### Pre-built Binaries

Download pre-built binaries for macOS, Linux, and Windows from [GitHub Releases](https://github.com/sigoden/aichat/releases), extract them, and add the `aichat` binary to your `$PATH`.

## Features

### Multi-Providers

Integrate seamlessly with over 20 leading LLM providers through a unified interface. Supported providers include OpenAI, Claude, Gemini (Google AI Studio), Ollama, Groq, Azure-OpenAI, VertexAI, Bedrock, Github Models, Mistral, Deepseek, AI21, XAI Grok, Cohere, Perplexity, Cloudflare, OpenRouter, Ernie, Qianwen, Moonshot, ZhipuAI, MiniMax, Deepinfra, VoyageAI, any OpenAI-Compatible API provider.

### CMD Mode

Explore powerful command-line functionalities with AIChat's CMD mode.

![aichat-cmd](https://github.com/user-attachments/assets/6c58c549-1564-43cf-b772-e1c9fe91d19c)

### REPL Mode

Experience an interactive Chat-REPL with features like tab autocompletion, multi-line input support, history search, configurable keybindings, and custom REPL prompts.

![aichat-repl](https://github.com/user-attachments/assets/218fab08-cdae-4c3b-bcf8-39b6651f1362)

### Shell Assistant

Elevate your command-line efficiency. Describe your tasks in natural language, and let AIChat transform them into precise shell commands. AIChat intelligently adjusts to your OS and shell environment.

![aichat-execute](https://github.com/user-attachments/assets/0c77e901-0da2-4151-aefc-a2af96bbb004)

### Multi-Form Input

Accept diverse input forms such as stdin, local files and directories, and remote URLs, allowing flexibility in data handling.

| Input             | CMD                                  | REPL                             |
| ----------------- | ------------------------------------ | -------------------------------- |
| CMD               | `aichat hello`                       |                                  |
| STDIN             | `cat data.txt \| aichat`             |                                  |
| Last Reply        |                                      | `.file %%`                       |
| Local files       | `aichat -f image.png -f data.txt`    | `.file image.png data.txt`       |
| Local directories | `aichat -f dir/`                     | `.file dir/`                     |
| Remote URLs       | `aichat -f https://example.com`      | `.file https://example.com`      |
| External commands | ```aichat -f '`git diff`'```         | ```.file `git diff` ```          |
| Combine Inputs    | `aichat -f dir/ -f data.txt explain` | `.file dir/ data.txt -- explain` |

### Role

Customize roles to tailor LLM behavior, enhancing interaction efficiency and boosting productivity.

![aichat-role](https://github.com/user-attachments/assets/023df6d2-409c-40bd-ac93-4174fd72f030)

> The role consists of a prompt and model configuration.

### Session

Maintain context-aware conversations through sessions, ensuring continuity in interactions.

![aichat-session](https://github.com/user-attachments/assets/56583566-0f43-435f-95b3-730ae55df031)

> The left side uses a session, while the right side does not use a session.

### Macro

Streamline repetitive tasks by combining a series of REPL commands into a custom macro.

![aichat-macro](https://github.com/user-attachments/assets/23c2a08f-5bd7-4bf3-817c-c484aa74a651)

### RAG

Integrate external documents into your LLM conversations for more accurate and contextually relevant responses.

![aichat-rag](https://github.com/user-attachments/assets/359f0cb8-ee37-432f-a89f-96a2ebab01f6)

### Function Calling

Function calling supercharges LLMs by connecting them to external tools and data sources. This unlocks a world of possibilities, enabling LLMs to go beyond their core capabilities and tackle a wider range of tasks.

We have created a new repository [https://github.com/sigoden/llm-functions](https://github.com/sigoden/llm-functions) to help you make the most of this feature.

#### AI Tools & MCP

Integrate external tools to automate tasks, retrieve information, and perform actions directly within your workflow.

![aichat-tool](https://github.com/user-attachments/assets/7459a111-7258-4ef0-a2dd-624d0f1b4f92)

#### AI Agents (CLI version of OpenAI GPTs)

AI Agent = Instructions (Prompt) + Tools (Function Callings) + Documents (RAG).

![aichat-agent](https://github.com/user-attachments/assets/0b7e687d-e642-4e8a-b1c1-d2d9b2da2b6b)

### Local Server Capabilities

AIChat includes a lightweight built-in HTTP server for easy deployment.

```
$ aichat --serve
Chat Completions API: http://127.0.0.1:8000/v1/chat/completions
Embeddings API:       http://127.0.0.1:8000/v1/embeddings
Rerank API:           http://127.0.0.1:8000/v1/rerank
LLM Playground:       http://127.0.0.1:8000/playground
LLM Arena:            http://127.0.0.1:8000/arena?num=2
```

#### Proxy LLM APIs

The LLM Arena is a web-based platform where you can compare different LLMs side-by-side. 

Test with curl:

```sh
curl -X POST -H "Content-Type: application/json" -d '{
  "model":"claude:claude-3-5-sonnet-20240620",
  "messages":[{"role":"user","content":"hello"}], 
  "stream":true
}' http://127.0.0.1:8000/v1/chat/completions
```

#### LLM Playground

A web application to interact with supported LLMs directly from your browser.

![aichat-llm-playground](https://github.com/user-attachments/assets/aab1e124-1274-4452-b703-ef15cda55439)

#### LLM Arena

A web platform to compare different LLMs side-by-side.

![aichat-llm-arena](https://github.com/user-attachments/assets/edabba53-a1ef-4817-9153-38542ffbfec6)

## Custom Themes

AIChat supports custom dark and light themes, which highlight response text and code blocks.

![aichat-themes](https://github.com/sigoden/aichat/assets/4012553/29fa8b79-031e-405d-9caa-70d24fa0acf8)

## Documentation

- [Chat-REPL Guide](https://github.com/sigoden/aichat/wiki/Chat-REPL-Guide)
- [Command-Line Guide](https://github.com/sigoden/aichat/wiki/Command-Line-Guide)
- [Role Guide](https://github.com/sigoden/aichat/wiki/Role-Guide)
- [Macro Guide](https://github.com/sigoden/aichat/wiki/Macro-Guide)
- [RAG Guide](https://github.com/sigoden/aichat/wiki/RAG-Guide)
- [Environment Variables](https://github.com/sigoden/aichat/wiki/Environment-Variables)
- [Configuration Guide](https://github.com/sigoden/aichat/wiki/Configuration-Guide)
- [Custom Theme](https://github.com/sigoden/aichat/wiki/Custom-Theme)
- [Custom REPL Prompt](https://github.com/sigoden/aichat/wiki/Custom-REPL-Prompt)
- [FAQ](https://github.com/sigoden/aichat/wiki/FAQ)

## License

Copyright (c) 2023-2025 aichat-developers.

AIChat is made available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the LICENSE-APACHE and LICENSE-MIT files for license details.