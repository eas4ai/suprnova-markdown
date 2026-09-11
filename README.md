# suprnova-markdown

Markdown twins of your app's prose pages, for the clients that don't run
JavaScript.

An Inertia app answers with `<div id="app"></div>` and a JSON payload. Google
renders it; GPTBot, ClaudeBot, PerplexityBot and `curl` do not. This crate
gives every prose URL a `.md` sibling serving the source it was rendered from,
and makes that sibling discoverable instead of a convention callers have to
know.

```
GET /docs/routing      →  text/html         the page
GET /docs/routing.md   →  text/markdown     the source it came from
```

## Use

```rust
use suprnova::async_trait;
use suprnova_markdown::{MarkdownMiddleware, MarkdownSource};

struct Content;

#[async_trait]
impl MarkdownSource for Content {
    async fn markdown(&self, path: &str) -> Option<String> {
        // The page's path, suffix already stripped: `/docs/routing`.
        let slug = path.strip_prefix("/docs/")?;
        std::fs::read_to_string(format!("content/docs/{slug}.md")).ok()
    }
}

global_middleware!(MarkdownMiddleware::new(Content));
```

That is the whole integration. No controller knows the twins exist; returning
`None` falls through to the normal routes.

Advertise the twin in the page's `<head>`:

```rust
suprnova_markdown::alternate_link("https://example.com/docs/routing")
// <link rel="alternate" type="text/markdown" href="https://example.com/docs/routing.md">
```

## Where the convention comes from

The `.md`-appended URL is the llms.txt proposal's own suggestion (Jeremy
Howard, 3 September 2024): provide "a clean markdown version of those pages at
the same URL as the original page, but with `.md` appended"
([llmstxt.org](https://llmstxt.org/)). Stripe, Cloudflare and Anthropic all
publish an `llms.txt`; the twin convention is common enough to have picked up
a name, the Markdown Twin Pattern, and static-site generators and Next.js /
Vite plugins already automate it.

Nothing here is novel. What this crate adds is doing it for a server-rendered
SPA - where the HTML genuinely carries no prose, so the twin is the only text
a non-executing client can get - behind an interface narrow enough to be
reused.

## The defaults, and why

**A distinct URL, not content negotiation.** Serving different bodies at one
URL based on `User-Agent` is cloaking and search engines treat it as such.
Keying on `Accept` avoids that but is unreliable - few crawlers send
`Accept: text/markdown` - and forces `Vary: Accept` on every HTML response. A
separate URL is cacheable, linkable, and testable with `curl`.

**`rel="alternate"` for discovery.** The same relation an RSS feed uses to
announce an alternative representation of the current document. A client that
understands HTML link relations learns nothing new, and the twin stops being a
URL convention callers must know in advance.

**`X-Robots-Tag: noindex` on the twin, by default.** It is the same content as
the page; letting both compete splits ranking signals. `.indexable()` turns it
off for the case where markdown is the canonical form. This one is a judgment
call rather than part of the convention - plenty of implementations leave the
twins indexable.

**`charset=utf-8`, always.** Technical prose is full of arrows and
box-drawing. A client that guesses latin-1 renders them as mojibake.

## Completing the picture

The twins are one part. An `llms.txt` that indexes them, and an
`llms-full.txt` that inlines the corpus, are what turn a set of URLs into
something an agent can consume in one fetch. Those are app-shaped rather than
framework-shaped - they depend on how your content is organised - so this
crate doesn't generate them, but the twins are what they should link to.

## Status

Targets Suprnova 2.0.0 at the exact release revision
`3229aa9af542c991196274fa3c235cdce88a68e2`. Consumers must use the same
framework revision so middleware and request types share one crate identity.
Requires Rust 1.94 or later. Extracted from
[suprnova.app](https://suprnova.app), where the pattern runs in production -
see `/llms.txt`, `/llms-full.txt`, and the `.md` twin of any manual chapter.
