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
