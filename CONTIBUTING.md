# Contributing to Kern

Kern is an open source project and contributions are welcome. This document explains how to get involved effectively.

## Before you contribute

Read the README. Understand the philosophy. Kern exists to strengthen the developer, not to replace their thinking. Contributions that go against this principle, such as features that generate code automatically or reduce the need for understanding, will not be accepted regardless of technical quality.

If you are unsure whether your idea fits the project, open an issue and discuss it first.

## How to contribute

### Reporting bugs

Open an issue with the following information:

- Operating system and distribution
- Rust version (`rustc --version`)
- Steps to reproduce the problem
- Expected behaviour
- Actual behaviour
- Relevant logs (`RUST_LOG=info cargo run -p kern-daemon`)

### Suggesting features

Open an issue marked `enhancement`. Describe:

- The problem you are trying to solve
- How your suggestion fits the Kern philosophy
- Any alternatives you considered

Features that depend on internet connectivity, external APIs, or generative AI will not be accepted in the core project.

### Contributing code

1. Fork the repository
2. Create a branch with a descriptive name

```bash
git checkout -b feat/result-navigation
```

3. Make your changes, one concern per commit
4. Write clear commit messages
feat: add keyboard navigation between search results
fix: prevent daemon crash on empty docs directory
docs: add troubleshooting guide for nginx

5. Open a pull request with a clear description of what changed and why

## Project structure
kern/
├── daemon/         # Resident background process (Rust)
├── engine/         # Search and indexing library (Rust + Tantivy)
├── overlay/        # Visual overlay (Tauri + Svelte)
├── docs/           # Local documentation sources (Markdown)
├── assets/         # Visual assets and mascot
└── config/         # Default configuration

## Development setup

### Requirements

- Linux (X11 or Wayland)
- Rust 1.77+
- Node.js 20+
- WebKit2GTK 4.1

### Running locally

Terminal 1, start the daemon:

```bash
RUST_LOG=info cargo run -p kern-daemon
```

Terminal 2, start the overlay:

```bash
cd overlay
npm install
npm run dev
```

### Adding documentation

Kern's knowledge base lives in `docs/` as Markdown files. Adding documentation is one of the most valuable contributions.

Each file should follow this structure:

```markdown
# Topic Name

Brief description of the topic.

## Subtopic

Content here.

## Commands

Relevant commands with context and explanation.

## Troubleshooting

Common problems and solutions.
```

The daemon indexes all `.md` files in `docs/` automatically on startup.

## What we are looking for

- Bug fixes
- Documentation and knowledge base content
- Performance improvements
- Linux distribution packaging (AUR, Debian, Flatpak)
- Accessibility improvements in the overlay
- Troubleshooting guides

## What we are not looking for

- Features that require internet connectivity
- Integration with external AI APIs
- Code generation or autocomplete features
- Electron-based rewrites

## Commit style

Kern uses conventional commits:

| Prefix | Use |
|---|---|
| `feat:` | New feature |
| `fix:` | Bug fix |
| `docs:` | Documentation |
| `chore:` | Maintenance, dependencies |
| `refactor:` | Code restructuring |
| `perf:` | Performance improvement |

## Licence

By contributing to Kern, you agree that your contributions will be licensed under the MIT licence.