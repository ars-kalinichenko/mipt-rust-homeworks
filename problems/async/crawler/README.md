# Web crawler

In this problem, you'll implement a simple web crawler.

## Algorithm

A web crawler is an application that downloads a given website, visiting all of its "subpages" (those links to which it can get). The crawling algorithm:

1. Get the next URL from the queue.
2. If this URL has not yet been visited - visit it.
3. Get new URLs from the response body and add them to the queue.

## Implementation

- To make HTTP requests, use the `reqwest` crate that is implemented on top of `tokio`. The usage is as follows:

    ```rust
    reqwest::get(url).await.unwrap().text().await.unwrap();
    ```

- To find all links in the body of a page, use the `linkify` crate. Example:

    ```rust
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    let links = finder.links(body).map(|l| l.as_str().to_string()).collect();
    ```
