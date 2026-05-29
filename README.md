
# Kern

Kern is a lightweight, offline-first technical overlay for developers who want to think better, not delegate more.

It lives silently in the background. You summon it with a keystroke. You search, you learn, you close it. It never interrupts your flow.

It is not an AI that writes code for you. It is a tool that sharpens the engineer behind the keyboard.

---

## What it does

- Instant search across technical documentation, commands, and troubleshooting guides
- Offline-first: everything that matters lives locally
- Keyboard-driven: no mouse, no clutter, no distraction
- Learn Mode: hints and architecture guidance instead of generated answers
- Stays resident, stays silent, costs almost nothing

## What it is not

Kern is not a code generator. It will not complete your functions, scaffold your projects, or think for you. That is the point.

The goal is to reduce dependency on generative AI for the wrong reasons, and rebuild the habit of understanding what you are building and why.

---

## Architecture

```
kern/
├── daemon/     # Resident background process: hotkeys, indexing, cache, IPC
├── overlay/    # Minimal UI layer: keyboard-first, instant open/close
├── engine/     # Knowledge core: fuzzy search, parsing, content ranking
├── docs/       # Local documentation sources (Markdown)
└── config/     # User configuration
```

## Stack

| Layer | Technology | Reason |
|---|---|---|
| Core | Rust | Performance, low memory, safe concurrency |
| UI | Tauri + Svelte | Native Linux, lightweight, no Electron overhead |
| Search | Tantivy | Local full-text and fuzzy search in Rust |
| Docs | Markdown | Portable, fast to index, easy to render |

## Activation

```
Super + K
```

The overlay appears. You search. You close with `ESC`. That is the entire interaction model.

---

## Status

Early architecture phase. Not yet functional.

---

## Philosophy

Most tools optimise for output. Kern optimises for understanding.

A developer who reads documentation, traces errors, and reasons through architecture does not need an AI to finish their sentences. They need fast access to the right knowledge at the right moment.

That is what Kern is for.