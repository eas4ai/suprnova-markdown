# Upgrade Guide

## Markdown plugin 0.5.5 → 0.5.6

Plugin `v0.5.6` targets Suprnova 4.6.2 through `tag = "v4.6.2"` (commit
`1c8b2d07b65b120337b06c975522f850a340a418`). Suprnova 4.6.1 apps stay on
`v0.5.5`. The plugin API and response behavior are unchanged from 0.5.5.

Use the same framework Git URL and `tag = "v4.6.2"` for the app and any
direct framework adapters, such as `suprnova-oauth-github` `v0.4.14`. Keep the
app's selected features; the plugin still disables default features and
enables `testing` only for its own tests.

## Markdown plugin 0.5.4 → 0.5.5

Plugin `v0.5.5` targets Suprnova 4.6.1 through `tag = "v4.6.1"` (commit
`5112862367b76a2ea3875d0135ca4ca1ec602c32`). Suprnova 4.6.0 apps stay on
`v0.5.4`. The plugin API and response behavior are unchanged from 0.5.4.

Use the same framework Git URL and `tag = "v4.6.1"` for the app and any
direct framework adapters, such as `suprnova-oauth-github` `v0.4.13`. Keep the
app's selected features; the plugin still disables default features and
enables `testing` only for its own tests.

## Markdown plugin 0.5.3 → 0.5.4

Plugin `v0.5.4` targets Suprnova 4.6.0 through `tag = "v4.6.0"` (commit
`cdf953217bfdae4d4a90d845ca78d14d02b54d53`). Suprnova 4.5.1 apps stay on
`v0.5.3`. The plugin API and response behavior are unchanged from 0.5.3.

Use the same framework Git URL and `tag = "v4.6.0"` for the app and any
direct framework adapters, such as `suprnova-oauth-github` `v0.4.12`. Keep the
app's selected features; the plugin still disables default features and
enables `testing` only for its own tests.

## Markdown plugin 0.5.2 → 0.5.3

Plugin `v0.5.3` targets Suprnova 4.5.1 through `tag = "v4.5.1"` (commit
`e05635e801781f62a2115f7c2b90bcecb9cf8f35`). Suprnova 4.5.0 apps stay on
`v0.5.2`. The plugin API and response behavior are unchanged from 0.5.2; the
release moves the lock file past the open RustSec advisories for `ammonia`,
`quinn-proto`, `rustls`, `rustls-webpki`, `webpki-roots` and `webpki-root-certs`.

Use the same framework Git URL and `tag = "v4.5.1"` for the app and any
direct framework adapters, such as `suprnova-oauth-github` `v0.4.11`. Keep the
app's selected features; the plugin still disables default features and
enables `testing` only for its own tests.

## Markdown plugin 0.5.1 → 0.5.2

Plugin `v0.5.2` targets Suprnova 4.5.0 through `tag = "v4.5.0"` (commit
`a79938673e9b04afa1bb0da62d0012f915e9c597`). Suprnova 4.4.0 apps stay on
`v0.5.1`. The plugin API and response behavior are unchanged from 0.5.1.

Use the same framework Git URL and `tag = "v4.5.0"` for the app and any
direct framework adapters, such as `suprnova-oauth-github` `v0.4.10`. Keep the
app's selected features; the plugin still disables default features and
enables `testing` only for its own tests.

## Markdown plugin 0.5.0 → 0.5.1

Plugin `v0.5.1` targets Suprnova 4.4.0 through `tag = "v4.4.0"` (commit
`4a085543664fad006c9e8c90f891ef15a5601ef3`). Suprnova 4.3.0 apps stay on
`v0.5.0`. The plugin API and response behavior are unchanged from 0.5.0.

Use the same framework Git URL and `tag = "v4.4.0"` for the app and any
direct framework adapters, such as `suprnova-oauth-github` `v0.4.9`. Keep the
app's selected features; the plugin still disables default features and
enables `testing` only for its own tests.

## Markdown plugin 0.4.0 → 0.5.0

Plugin `v0.5.0` targets Suprnova 4.3.0 through `tag = "v4.3.0"` (commit
`2e0c5227b9090c47894169c0d545b40eb27c3641`). Suprnova 4.2.0 apps stay on
`v0.4.0`. The plugin API and response behavior are unchanged from 0.4.0.

Use the same framework Git URL and `tag = "v4.3.0"` for the app and any
direct framework adapters, such as `suprnova-oauth-github` `v0.4.8`. Keep the
app's selected features; the plugin still disables default features and
enables `testing` only for its own tests.

```toml
suprnova = { git = "https://github.com/eas4ai/suprnova.git", tag = "v4.3.0", default-features = false, features = [] } # replace [] with your app's feature list
suprnova-markdown = { git = "https://github.com/eas4ai/suprnova-markdown.git", tag = "v0.5.0" }
```

Update and verify the consumer:

```bash
cargo update -p suprnova -p suprnova-markdown
cargo tree -d
cargo check --locked --all-targets
cargo test --locked
```

Verify that Cargo resolves exactly one Suprnova identity at the commit above.
The publication eligibility, cache policy and failed-lookup checks described
below still apply.

Verification for this release: `cargo fmt --check`, `cargo test --locked`
(four unit tests, six middleware tests and one doctest), and
`cargo check --locked --lib` passed.

There is no plugin database migration. Keep the prior manifest and lockfile
together for rollback.

## Markdown plugin 0.3.0 → 0.4.0

Plugin `v0.4.0` targets Suprnova 4.2.0 through `tag = "v4.2.0"` (commit
`6f6f83feae1dfde97d614d083625872987407780`). Suprnova 4.1.0 apps stay on
`v0.3.0`. The plugin API and response behavior are unchanged from 0.3.0.
Consumers upgrading from 0.1.x or 0.2.x must also port `MarkdownSource` to
the fallible signature described below.

Use the same framework Git URL and `tag = "v4.2.0"` for the app and any
direct framework adapters. Keep the app's selected features; the plugin
still disables default features and enables `testing` only for its own tests.

```toml
suprnova = { git = "https://github.com/eas4ai/suprnova.git", tag = "v4.2.0", default-features = false, features = [] } # replace [] with your app's feature list
suprnova-markdown = { git = "https://github.com/eas4ai/suprnova-markdown.git", tag = "v0.4.0" }
```

Update and verify the consumer:

```bash
cargo update -p suprnova -p suprnova-markdown
cargo tree -d
cargo check --locked --all-targets
cargo test --locked
```

Verify that Cargo resolves exactly one Suprnova identity at the commit above.
Check publication eligibility, Unicode, missing-source fallback and failed
lookup responses as described below. The default public cache policy remains
one hour; applications whose content can become private must retain their
explicit cache restrictions. The framework's 4.2 cookie/cache changes do not
replace those source eligibility checks.

Verification for this release: `cargo fmt --check`, `cargo test --locked`
(four unit tests, six middleware tests and one doctest), and
`cargo check --locked --lib` passed. A separate consumer dependency graph
resolved one Suprnova 4.2 identity without default or testing features.
Full application acceptance remains each consumer's responsibility.

There is no plugin database migration. Preserve the prior manifest and lockfile
together for rollback; concurrent framework/application migrations need the
host application's backup and restore procedure.

## Markdown plugin 0.2.0 → 0.3.0

Plugin `v0.3.0` still targets Suprnova 4.1.0 through `tag = "v4.1.0"`. It moves two safety checks that apps had to wrap around the middleware into the plugin, and that changes the trait.

- **`MarkdownSource::markdown` returns `Result<Option<String>, FrameworkError>`.** `Ok(Some(markdown))` serves the twin. `Ok(None)` and an error with status 404 fall through to the router, as `None` did. Any other error is logged with `tracing` and answered with a plain-text 500, `Cache-Control: no-store` and `X-Robots-Tag: noindex`; the error's text never reaches the client, and the request does not fall through. Port a source by wrapping its result in `Ok(...)` and returning lookup errors with `?` instead of turning them into `None`.
- **Only `GET` and `HEAD` consult the source.** Other methods on a `.md` URL go to the router without the source seeing them.

Unchanged: `MarkdownMiddleware`, `indexable`, `max_age`, `alternate_link`, the response headers of a served twin, and the `public, max-age=3600` default. The source still decides what is public; see "What your source must check" in the README.

A wrapper like the directory starter's `PublicMarkdown`, which checks the method and records source failures itself, keeps working once its source is ported to the new signature; its own checks become redundant.

Update a consumer by changing the tag to `v0.3.0`, porting each `MarkdownSource`, and running `cargo update -p suprnova-markdown`, `cargo check --locked --all-targets` and `cargo test --locked`. The release passed four unit tests, six middleware tests and one doctest.

## Markdown plugin 0.1.1 → 0.2.0

Plugin `v0.2.0` targets Suprnova 4.1.0 through `tag = "v4.1.0"` (commit `3aa40bd7c266d41d86a21c0eebbbb1d5a2d09d1d`). Apps still on Suprnova 3.2.1 stay on `v0.1.1`.

The middleware API and behavior are unchanged. `MarkdownSource`, `MarkdownMiddleware`, `indexable`, `max_age` and `alternate_link` keep their interfaces. Two dependency details changed:

- **Source selector.** The plugin names the framework with `tag = "v4.1.0"`, as Nebula 3.0.0 and suprnova.app do. Name it the same way in your app. A `rev` for the same commit is a different Git source, and Cargo then builds two framework crates whose middleware and request types do not match.
- **Features.** The plugin depends on the framework with `default-features = false` and enables none. Your app's feature list decides what the framework compiles; adding the plugin no longer turns the framework's default features (including `testing`) back on. The plugin's own tests enable `testing` as a dev-dependency, because `Request::for_test` has required it since Suprnova 4.0.

## Update a 4.1.0 consumer

```toml
[dependencies]
suprnova = { git = "https://github.com/eas4ai/suprnova.git", tag = "v4.1.0", default-features = false, features = [/* your app's list */] }
suprnova-markdown = { git = "https://github.com/eas4ai/suprnova-markdown.git", tag = "v0.2.0" }
```

Then:

```bash
cargo update -p suprnova-markdown
cargo tree -e normal --prefix none | grep -E '^suprnova v' | sort -u   # one line: ?tag=v4.1.0#3aa40bd7
cargo tree -e normal,features -i suprnova | grep 'suprnova feature "testing"'   # no output
cargo check --locked --all-targets
cargo test --locked
```

Commit the manifest and lockfile together. The behavior and cache-policy checks below still apply.

For the plugin checkout itself, run `cargo fmt --check` and `cargo test --locked`. The release passed four unit tests, two middleware tests and one doctest, and builds with no framework features (`cargo check --lib`). A Nebula 3.0.0 checkout with the plugin added kept one framework identity, no `testing` feature in its normal build, and passed `cargo check`. There is no database migration. To roll back, restore the previous manifest and lockfile pair.

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
