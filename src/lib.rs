//! Markdown twins for a Suprnova app.
//!
//! An Inertia app sends `<div id="app"></div>` and a JSON payload. Anything
//! that doesn't execute JavaScript - GPTBot, ClaudeBot, PerplexityBot, `curl`,
//! a developer piping a page into an editor - sees an empty shell. This crate
//! gives every prose URL a `.md` sibling that serves the source it was
//! rendered from, and makes that sibling discoverable rather than a convention
//! the caller has to know.
//!
//! ```no_run
//! use suprnova_markdown::{MarkdownSource, MarkdownMiddleware};
//! use suprnova::{FrameworkError, async_trait};
//!
//! struct Content;
//!
//! #[async_trait]
//! impl MarkdownSource for Content {
//!     async fn markdown(&self, path: &str) -> Result<Option<String>, FrameworkError> {
//!         let Some(slug) = path.strip_prefix("/docs/") else {
//!             return Ok(None);
//!         };
//!         Ok(std::fs::read_to_string(format!("content/docs/{slug}.md")).ok())
//!     }
//! }
//!
//! # fn register() {
//! suprnova::global_middleware!(MarkdownMiddleware::new(Content));
//! # }
//! ```
//!
//! # Why a distinct URL, not content negotiation
//!
//! Serving different bodies at one URL based on `User-Agent` is cloaking, and
//! search engines treat it as such. Keying on `Accept` avoids that but is
//! unreliable in practice - few crawlers send `Accept: text/markdown`, and it
//! forces `Vary: Accept` on every HTML response. A separate URL is cacheable,
//! linkable, testable with `curl`, and the same for everyone.

use std::sync::Arc;

use suprnova::{FrameworkError, HttpResponse, Middleware, Next, Request, Response, async_trait};

/// The IANA type from RFC 7763. `charset` is not optional in practice:
/// technical prose is full of arrows and box-drawing, and a client that
/// guesses latin-1 renders them as mojibake.
pub const MARKDOWN_TYPE: &str = "text/markdown; charset=utf-8";

/// Supplies the markdown behind a page.
///
/// The path is the *page's* path, with the `.md` suffix already removed - an
/// implementation sees `/docs/routing`, not `/docs/routing.md`, so the same
/// function can serve both surfaces.
///
/// - `Ok(Some(markdown))` serves the twin.
/// - `Ok(None)`, or an error whose status is 404, means "no twin here", and
///   the request falls through to the normal routes.
/// - Any other error is a failed lookup. It is logged, and the client gets a
///   plain 500 with `Cache-Control: no-store` and none of the error's text;
///   the request does not fall through, so a broken source never looks like
///   a missing page.
///
/// The middleware runs before the router, so route-level sign-in and
/// permission checks do not apply: return Markdown only for pages anyone
/// may read.
#[async_trait]
pub trait MarkdownSource: Send + Sync + 'static {
    async fn markdown(&self, path: &str) -> Result<Option<String>, FrameworkError>;
}

/// Answers `*.md` with markdown and passes everything else through.
///
/// Register it globally and no controller needs to know this exists.
pub struct MarkdownMiddleware {
    source: Arc<dyn MarkdownSource>,
    noindex: bool,
    max_age: u32,
}

impl MarkdownMiddleware {
    pub fn new(source: impl MarkdownSource) -> Self {
        Self {
            source: Arc::new(source),
            noindex: true,
            max_age: 3600,
        }
    }

    /// Serve the twin without `X-Robots-Tag: noindex`.
    ///
    /// The default is on, and should usually stay on: the twin is the same
    /// content as the HTML page, so letting both compete splits ranking
    /// signals between them. Turn it off only when the markdown is the
    /// canonical form and no HTML page covers it.
    pub fn indexable(mut self) -> Self {
        self.noindex = false;
        self
    }

    /// `Cache-Control: public, max-age=<seconds>`. Defaults to an hour.
    pub fn max_age(mut self, seconds: u32) -> Self {
        self.max_age = seconds;
        self
    }

    fn respond(&self, body: String) -> HttpResponse {
        let mut response = HttpResponse::bytes(body.into_bytes().into(), MARKDOWN_TYPE)
            .header("Cache-Control", format!("public, max-age={}", self.max_age));
        if self.noindex {
            response = response.header("X-Robots-Tag", "noindex");
        }
        response
    }
}

#[async_trait]
impl Middleware for MarkdownMiddleware {
    async fn handle(&self, request: Request, next: Next) -> Response {
        // A twin is something to read. Any other method goes to the router
        // without the source seeing it.
        if !matches!(request.method().as_str(), "GET" | "HEAD") {
            return next(request).await;
        }
        let Some(page_path) = page_path(request.path()) else {
            return next(request).await;
        };

        // `Err` is how a middleware short-circuits; it is the response, not
        // a failure.
        match self.source.markdown(&page_path).await {
            Ok(Some(body)) => Err(self.respond(body)),
            Ok(None) => next(request).await,
            Err(error) if error.status_code() == 404 => next(request).await,
            Err(error) => {
                tracing::error!(path = %page_path, %error, "Markdown twin lookup failed");
                Err(lookup_failed())
            }
        }
    }
}

/// What the client gets when the source fails: no details, nothing cached,
/// nothing indexed.
fn lookup_failed() -> HttpResponse {
    HttpResponse::text("The Markdown version of this page could not be loaded.")
        .status(500)
        .header("Cache-Control", "no-store")
        .header("X-Robots-Tag", "noindex")
}

/// The page a `.md` request is the twin of, or `None` if it isn't one.
///
/// `/docs/routing.md` is the twin of `/docs/routing`. A bare `/.md`, or a
/// path whose last segment is only the suffix, is not a twin of anything.
fn page_path(path: &str) -> Option<String> {
    let base = path.strip_suffix(".md")?;
    let last = base.rsplit('/').next().unwrap_or_default();
    if last.is_empty() {
        return None;
    }
    Some(base.to_string())
}

/// `<link rel="alternate">` for a page's markdown twin.
///
/// `rel="alternate"` with a `type` is the same relation an RSS feed uses to
/// announce an alternative representation of the current document, so a
/// client that already understands HTML link relations needs to learn nothing
/// new. Pass the page's absolute URL; the suffix is appended.
pub fn alternate_link(page_url: &str) -> String {
    format!("<link rel=\"alternate\" type=\"text/markdown\" href=\"{page_url}.md\">")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_the_page_behind_a_twin() {
        assert_eq!(
            page_path("/docs/routing.md").as_deref(),
            Some("/docs/routing")
        );
        assert_eq!(
            page_path("/blog/topcoat.md").as_deref(),
            Some("/blog/topcoat")
        );
        // Not twins.
        assert_eq!(page_path("/docs/routing"), None);
        assert_eq!(page_path("/.md"), None);
        assert_eq!(page_path(".md"), None);
        // Only the final suffix is stripped.
        assert_eq!(page_path("/docs/a.md.md").as_deref(), Some("/docs/a.md"));
    }

    #[test]
    fn advertises_the_twin_with_a_standard_relation() {
        assert_eq!(
            alternate_link("https://example.test/docs/routing"),
            "<link rel=\"alternate\" type=\"text/markdown\" href=\"https://example.test/docs/routing.md\">"
        );
    }

    struct Fixed;

    #[async_trait]
    impl MarkdownSource for Fixed {
        async fn markdown(&self, path: &str) -> Result<Option<String>, FrameworkError> {
            Ok((path == "/docs/routing").then(|| "# Routing\n\nArrows: →\n".to_string()))
        }
    }

    #[test]
    fn serves_utf8_and_keeps_the_twin_out_of_the_index() {
        let middleware = MarkdownMiddleware::new(Fixed);
        let response = middleware.respond("# Routing\n\nArrows: →\n".to_string());

        assert_eq!(response.header_value("Content-Type"), Some(MARKDOWN_TYPE));
        assert_eq!(response.header_value("X-Robots-Tag"), Some("noindex"));
        assert_eq!(
            response.header_value("Cache-Control"),
            Some("public, max-age=3600")
        );
        assert!(String::from_utf8_lossy(response.body()).contains('→'));
    }

    #[test]
    fn indexable_drops_the_robots_header() {
        let middleware = MarkdownMiddleware::new(Fixed).indexable().max_age(60);
        let response = middleware.respond("# x".to_string());

        assert_eq!(response.header_value("X-Robots-Tag"), None);
        assert_eq!(
            response.header_value("Cache-Control"),
            Some("public, max-age=60")
        );
    }
}
