# Architecture

This document describes the internal architecture of Kern. It is intended for contributors and developers who want to understand how the system works before making changes.

## Overview

Kern is composed of three independent components that communicate with each other at runtime.

```
┌─────────────────────────────────────────────┐
│                   User                       │
│           presses Super + K                  │
└─────────────────────┬───────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────┐
│              kern-daemon                     │
│                                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │ hotkey   │  │   ipc    │  │  server  │  │
│  └──────────┘  └──────────┘  └──────────┘  │
│                      │                      │
│              ┌───────▼───────┐              │
│              │  kern-engine  │              │
│              │               │              │
│              │ ┌───────────┐ │              │
│              │ │  parser   │ │              │
│              │ ├───────────┤ │              │
│              │ │   index   │ │              │
│              │ ├───────────┤ │              │
│              │ │  search   │ │              │
│              │ └───────────┘ │              │
│              └───────────────┘              │
└─────────────────────┬───────────────────────┘
                      │ HTTP 127.0.0.1:3000
                      ▼
┌─────────────────────────────────────────────┐
│            kern-overlay                      │
│          Tauri + Svelte                      │
└─────────────────────────────────────────────┘
```

## Components

### kern-daemon

The resident background process. Starts at boot and stays invisible until needed.

**Responsibilities:**
- Loading configuration from `config/default.toml`
- Starting the engine and triggering initial indexing
- Listening for the global hotkey `Super + K` via rdev
- Running the Unix socket IPC server at `/tmp/kern.sock`
- Running the HTTP server at `127.0.0.1:3000`

**Modules:**

| Module | File | Responsibility |
|---|---|---|
| main | `src/main.rs` | Entry point, orchestrates startup |
| config | `src/config.rs` | Loads and parses configuration |
| hotkey | `src/hotkey.rs` | Global hotkey listener via rdev |
| ipc | `src/ipc.rs` | Unix socket server for overlay communication |
| server | `src/server.rs` | HTTP server for search requests |

### kern-engine

The knowledge core. A Rust library used by the daemon.

**Responsibilities:**
- Parsing Markdown files from the `docs/` directory
- Building and maintaining the Tantivy search index
- Executing search queries and returning ranked results

**Modules:**

| Module | File | Responsibility |
|---|---|---|
| parser | `src/parser.rs` | Reads Markdown files, extracts title and body |
| index | `src/index.rs` | Creates or opens the Tantivy index, indexes documents |
| search | `src/search.rs` | Executes queries against the index |

### kern-overlay

The visual layer. A Tauri application with a Svelte frontend.

**Responsibilities:**
- Rendering the search panel
- Sending search queries to the daemon via HTTP
- Displaying results in real time
- Handling keyboard input, ESC to close, arrows to navigate

**Structure:**

```
overlay/
├── src/
│   ├── App.svelte       # Main component, search input and results
│   └── main.js          # Svelte entry point
└── src-tauri/
    ├── src/
    │   ├── main.rs      # Tauri entry point
    │   └── lib.rs       # Tauri builder and configuration
    └── tauri.conf.json  # Window configuration
```

## Communication

### Overlay to Daemon

The overlay sends search queries to the daemon via HTTP POST:

```
POST http://127.0.0.1:3000/search
Content-Type: application/json

{"query": "git push rejected"}
```

The daemon responds with:

```json
{"results": ["Git Basics"]}
```

### Daemon IPC

The daemon also listens on a Unix socket at `/tmp/kern.sock` for future inter-process communication with other tools or scripts.

Protocol:

```
send:    {"query": "git push"}\n
receive: {"results": ["Git Basics"]}\n
```

## Data Flow

```
1. User presses Super + K
2. hotkey.rs detects the combination
3. overlay receives focus signal
4. User types a query
5. App.svelte sends POST /search to 127.0.0.1:3000
6. server.rs receives the request
7. SearchEngine.query() runs against the Tantivy index
8. Results returned as JSON
9. App.svelte renders the result list
10. User presses ESC, overlay clears
```

## Configuration

Default configuration lives in `config/default.toml`:

```toml
hotkey = "super+k"
docs_path = "./docs"
index_path = "./index"
```

## Index

The Tantivy index is stored in the `index/` directory at the project root. It is created automatically on first run and updated every time the daemon starts.

The index schema has three fields:

| Field | Type | Stored |
|---|---|---|
| title | TEXT | Yes |
| body | TEXT | No |
| path | TEXT | Yes |

## Adding documentation

Drop any `.md` file into the `docs/` directory. The daemon will index it automatically on the next startup.

Each document should have a clear `# Title` at the top. The parser uses the first heading as the document title.

## Dependencies

### kern-daemon

| Crate | Purpose |
|---|---|
| tokio | Async runtime |
| axum | HTTP server |
| tower-http | CORS middleware |
| rdev | Global hotkey listener |
| serde / serde_json | Serialisation |
| tracing | Logging |
| anyhow | Error handling |

### kern-engine

| Crate | Purpose |
|---|---|
| tantivy | Full-text search engine |
| pulldown-cmark | Markdown parser |
| walkdir | Directory traversal |
| fuzzy-matcher | Fuzzy search support |
| serde | Serialisation |
| tracing | Logging |
| anyhow | Error handling |

