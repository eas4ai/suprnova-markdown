# Upgrade Guide

## Markdown plugin 0.1.0 → 0.1.1

Plugin `v0.1.1` targets Suprnova 3.2.1 at `2bd4bd53d04fa4581fdbb152ebd70cfb682b342c`. Earlier commits labeled 0.1.0 used different framework pins: inspect manifests and lockfiles rather than relying on the package version alone.

The middleware API and behavior are unchanged. `MarkdownSource`, `MarkdownMiddleware`, `indexable`, `max_age` and `alternate_link` retain their interfaces. The release contains the framework compatibility update and aligned version metadata.

## Update a consumer

Commit local changes and retain the working manifest, lockfile and artifacts. Use compatible framework and plugin sources:

```toml
[dependencies]
suprnova = { git = "https://github.com/eas4ai/suprnova.git", rev = "2bd4bd53d04fa4581fdbb152ebd70cfb682b342c" }
suprnova-markdown = { git = "https://github.com/eas4ai/suprnova-markdown.git", tag = "v0.1.1" }
```

Use that same framework `rev` for direct Suprnova payment adapters. Different Git selectors can produce distinct Rust crate identities even at the same commit, making middleware/request types incompatible. Align the dependency source as well as the version.

After editing the consumer manifest:

```bash
cargo update -p suprnova -p suprnova-markdown
cargo tree -d
cargo check --locked --all-targets
cargo test --locked
```

Review lockfile changes and check that only one Suprnova framework identity remains. Other duplicate library versions can be legitimate. Commit the consumer manifest and lockfile together.

Directory starter `v1.0.1` already uses compatible Markdown commit `565c2205b1d1d77aa05a29247f3a2416abc60791`. Changing it to the plugin tag is a separate dependency update requiring fresh consumer checks.

## Verify behavior and cache policy

Request a known public `.md` twin and an ordinary HTML route. Check Unicode content, `text/markdown; charset=utf-8`, default `X-Robots-Tag: noindex` and router fallback for missing twins. The plugin's default cache policy is `public, max-age=3600`.

Applications with private, unpublished or changing eligibility must enforce their own source permissions, methods/paths and cache policy. The directory starter checks current eligibility and wraps responses with `Cache-Control: no-store`; preserve this boundary. Updating the dependency does not supply those application policies automatically.

For the plugin checkout itself, merge `v0.1.1` from its configured upstream into an upgrade branch, then run:

```bash
cargo fmt --check
cargo test --locked
```

The release passed four unit tests, two middleware tests and one doctest and introduces no database migration. If consumer checks fail, restore the previous compatible manifest/lockfile pair and redeploy matching artifacts. Use the host application's rollback procedure for any concurrent database changes.
