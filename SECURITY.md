# Security Policy

## Reporting a vulnerability

Please report security issues privately via GitHub's
[private vulnerability reporting](https://github.com/Mullassery/StreamXL/security/advisories/new)
rather than opening a public issue. We aim to acknowledge reports within a few days.

## Supported versions

StreamXL v0.4+ receives security fixes. Patch releases are prioritized for vulnerabilities.

## Supply chain

- Releases are published to PyPI via **Trusted Publishing (OIDC)** — no long-lived API tokens
  are stored in the repository or CI.
- Dependencies are reviewed before bumps and pinned in requirements.
