use std::sync::Arc;

use suprnova::{HttpResponse, Middleware, Next, Request, async_trait};
use suprnova_markdown::{MARKDOWN_TYPE, MarkdownMiddleware, MarkdownSource};

struct Content;

#[async_trait]
impl MarkdownSource for Content {
    async fn markdown(&self, path: &str) -> Option<String> {
        (path == "/docs/routing").then(|| "# Routing →\n".to_owned())
    }
}

#[tokio::test]
async fn markdown_short_circuits_with_utf8_and_default_headers() {
    let next: Next = Arc::new(|_| panic!("a Markdown twin must not reach the router"));
    let result = MarkdownMiddleware::new(Content)
        .handle(Request::for_test("GET", "/docs/routing.md"), next)
        .await;
    let response = match result {
        Err(response) => response,
        Ok(_) => panic!("a Markdown twin must short-circuit middleware"),
    };

    assert_eq!(response.body(), "# Routing →\n".as_bytes());
    assert_eq!(response.header_value("Content-Type"), Some(MARKDOWN_TYPE));
    assert_eq!(response.header_value("X-Robots-Tag"), Some("noindex"));
    assert_eq!(
        response.header_value("Cache-Control"),
        Some("public, max-age=3600")
    );
}

#[tokio::test]
async fn missing_twins_and_ordinary_pages_preserve_router_responses() {
    for path in ["/docs/missing.md", "/docs/routing", "/.md"] {
        let next: Next = Arc::new(move |request| {
            assert_eq!(request.path(), path);
            Box::pin(async { Ok(HttpResponse::text("router response")) })
        });
        let result = MarkdownMiddleware::new(Content)
            .handle(Request::for_test("GET", path), next)
            .await;
        let response = match result {
            Ok(response) => response,
            Err(_) => panic!("requests without a twin must reach the router"),
        };

        assert_eq!(response.body(), b"router response");
        assert_eq!(response.header_value("X-Robots-Tag"), None);
    }
}

/// A source that fails the test if the middleware consults it.
struct Untouchable;

#[async_trait]
impl MarkdownSource for Untouchable {
    async fn markdown(&self, path: &str) -> Option<String> {
        panic!("the source must not be consulted for {path}");
    }
}

#[tokio::test]
async fn only_get_and_head_consult_the_source() {
    for method in ["POST", "PUT", "PATCH", "DELETE", "OPTIONS"] {
        let next: Next =
            Arc::new(|_| Box::pin(async { Ok(HttpResponse::text("router response")) }));
        let result = MarkdownMiddleware::new(Untouchable)
            .handle(Request::for_test(method, "/docs/routing.md"), next)
            .await;
        let response = match result {
            Ok(response) => response,
            Err(_) => panic!("{method} on a twin URL must reach the router"),
        };
        assert_eq!(response.body(), b"router response", "{method}");
    }
}

#[tokio::test]
async fn head_is_answered_like_get() {
    let next: Next = Arc::new(|_| panic!("a Markdown twin must not reach the router"));
    let result = MarkdownMiddleware::new(Content)
        .handle(Request::for_test("HEAD", "/docs/routing.md"), next)
        .await;
    let response = match result {
        Err(response) => response,
        Ok(_) => panic!("HEAD on a twin must short-circuit like GET"),
    };
    assert_eq!(response.header_value("Content-Type"), Some(MARKDOWN_TYPE));
}
