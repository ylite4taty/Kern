# Security Policy

## Supported Versions

Kern is currently in early development. Security fixes are applied to the latest version only.

| Version | Supported |
|---|---|
| 0.1.x | ✅ Yes |

## Scope

Kern runs locally on your machine. It does not transmit data over the network, does not connect to external services, and does not require authentication.

The attack surface is intentionally minimal:

- The daemon communicates via a local Unix socket (`/tmp/kern.sock`) and a local HTTP server (`127.0.0.1:3000`)
- All indexed content comes from local Markdown files in the `docs/` directory
- No user data is collected or stored beyond the local index

## Reporting a Vulnerability

If you discover a security vulnerability in Kern, do not open a public issue.

Report it privately by contacting the maintainer directly via GitHub:

[@ylite4taty](https://github.com/ylite4taty)

Include in your report:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix if you have one

You will receive a response within 7 days. If the vulnerability is confirmed, a fix will be prioritised and a security advisory will be published.

## Security Considerations for Contributors

When contributing to Kern, keep the following in mind:

- The daemon should never accept connections from outside `localhost`
- File paths used for indexing must be validated before use
- The IPC socket at `/tmp/kern.sock` should have restricted permissions
- Dependencies should be kept minimal and audited regularly

To audit current dependencies:

```bash
cargo audit
```

Install `cargo-audit` if needed:

```bash
cargo install cargo-audit
```

## Philosophy

Kern was designed to be offline-first and self-contained. Security through simplicity: the less the system does, the less it can be exploited.