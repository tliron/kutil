mod utils;

use {
    ::axum::{http::header::*, routing::*, *},
    kutil::http::{
        cache::{axum::*, implementation::moka::*, *},
        tower::caching::*,
        *,
    },
    moka::future::Cache,
    std::time::*,
    tokio::{net::*, *},
    tower_http::trace::*,
};

// (See tower_caching_basic.rs first)
//
// Axum server with Kutil's caching middleware for Tower
//
// Pay attention to the tracing log to see what our middleware and the cache are doing!
// (Most entries will be expired from the cache after 10 seconds)
//
// You can send requests from a web browser or via CLI. Some fun examples:
//
//   curl http://localhost:8080
//
//   curl --verbose --compressed http://localhost:8080
//
//   curl http://localhost:8080?x=1&y=2
//   curl http://localhost:8080?y=2&x=1
//
//   curl http://localhost:8080/nevercache
//
//   curl --silent --header 'Accept-Encoding: br;q=0.8, zstd' http://localhost:8080 | zstd --decompress
//
//   curl --verbose --header 'Accept-Language: zh;q=0.8, en;q=0.9' http://localhost:8080/language
//
//   curl --silent http://localhost:8080/png | icat --width 10 -
//
//   curl --verbose --request POST http://localhost:8080/reset
//
// A browser would be easier for testing client-side caching on http://localhost:8080/clientcache
// Make sure to turn on the browser's developer tools with F12
// Refresh the page normally by pressing F5 to see 304, or force a refresh with CTRL+F5

const CACHE_SIZE: u64 = 1024 * 1024; // 1 MiB

const CACHE_DURATION: Duration = Duration::from_secs(10);

// Keeping it very small for testing purposes
// (See "/toobig" skipping the cache)
const MAX_BODY_SIZE: usize = 200;

// Some language constants
const ENGLISH: Language = Language::new_fostered(&["en"]);
const ENGLISH_USA: Language = Language::new_fostered(&["en", "us"]);
const CHINESE: Language = Language::new_fostered(&["zh"]);
const CHINESE_TRADITIONAL: Language = Language::new_fostered(&["zh", "tw"]);
const CHINESE_SIMPLIFIED: Language = Language::new_fostered(&["zh", "cn"]);

// Already-compressed media types
const COMPRESSED_MEDIA_TYPES: &[MediaType] = &[
    MediaType::new_fostered("image", "png"),
    MediaType::new_fostered("image", "jpeg"),
    MediaType::new_fostered("audio", "mpeg"),
    MediaType::new_fostered("video", "mpeg"),
];

#[main]
async fn main() {
    utils::init_tracing();

    let cache = Cache::<CommonCacheKey, _, _>::builder()
        .name("http")
        .for_http_response()
        .max_capacity(CACHE_SIZE)
        .time_to_live(CACHE_DURATION)
        .eviction_listener(|key, _value, cause| {
            tracing::debug!("evict ({:?}): {}", cause, key);
        })
        .build();

    let cache = MokaCacheImplementation::new(cache);

    // For the "/language" URL
    // (First language will be the default)
    static LANGUAGES: &[Language] = &[CHINESE_TRADITIONAL, CHINESE_SIMPLIFIED, CHINESE, ENGLISH_USA, ENGLISH];

    // Note that in this example we are also adding the cache as state using `with_state`
    // This is *not* required for the caching layer!!!
    // This state is used by the `reset_cache` handler

    let router = Router::default()
        .route("/", get(("Hello, world!\n",)))
        .route("/toobig", get(("This response is too big to cache\n".repeat(10),)))
        .route(
            "/clientcache",
            get((
                [("Last-Modified", "Wed, 21 Oct 2015 07:28:00 GMT")],
                "This response might be cached by the client\n",
            )),
        )
        .route("/clientcache2", get(([("ETag", r#""stuff""#)], "This response might also be cached by the client\n")))
        .route("/nevercache", get(([("XX-Cache", "false")], "This response is never cached\n")))
        .route("/nevercache2", get(("This response is also never cached\n",)))
        .route("/neverencode", get(([("XX-Encode", "false")], "This response is never encoded\n")))
        .route("/neverencode2", get(("This response is also never encoded\n",)))
        .route(
            "/quickie",
            get(([("XX-Cache-Duration", "1 ms")], "This response has a custom cache duration of 1 ms\n")),
        )
        .route("/quickie2", get(("This response also has a custom cache duration of 1 ms\n",)))
        .route(
            "/png",
            get(([("Content-Type", "image/png"), ("Content-Length", utils::TINY_PNG_SIZE)], utils::TINY_PNG)),
        )
        .route("/put", put(("You put something here, thanks!",)))
        .route(
            "/language",
            get(async |headers: HeaderMap| {
                // HTTP content negotiation
                let language = headers.accept_language().best_or_first(LANGUAGES).clone();
                if (language == ENGLISH_USA) || (language == ENGLISH) {
                    ([("Content-Language", "en")], "This is in English\n")
                } else if (language == CHINESE_TRADITIONAL) || (language == CHINESE) {
                    // (no political statement is intended by defaulting to traditional!)
                    ([("Content-Language", "zh-TW")], "這是中文的\n")
                } else if language == CHINESE_SIMPLIFIED {
                    ([("Content-Language", "zh-CN")], "这是中文的\n")
                } else {
                    ([("Content-Language", "nope")], "This cannot be seen\n")
                }
            }),
        )
        .route("/reset", post(reset_cache_handler::<MokaCacheImplementation<_>, _>))
        .with_state(cache.clone()) // for "/reset"
        .layer(
            CachingLayer::default()
                .cache(cache.clone())
                .max_cacheable_body_size(MAX_BODY_SIZE)
                .cache_key(|context| {
                    // HTTP content negotiation for "/language"
                    if context.request.uri().path() == "/language" {
                        let language = context.request.headers().accept_language().best_or_first(LANGUAGES).clone();
                        context.cache_key.languages = Some([language].into());
                    }
                })
                .cache_duration(|context| {
                    // This is an alternative to using the `XX-Cache-Duration` header
                    if context.uri.path() == "/quickie2" { Some(Duration::from_millis(1)) } else { None }
                })
                .cacheable_by_request(|context| {
                    // This is an alternative to using the `XX-Cache` header
                    context.uri.path() != "/nevercache2"
                })
                .encodable_by_request(|context| {
                    // This is an alternative to using the `XX-Encode` header
                    context.uri.path() != "/neverencode2"
                })
                .encodable_by_response(|context| {
                    // This is where we can disable encoding for already-compressed media types
                    match context.headers.content_type() {
                        Some(content_type) => !COMPRESSED_MEDIA_TYPES.contains(&content_type),
                        None => true,
                    }
                })
                .keep_identity_encoding(false),
        )
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("[::]:8080").await.expect("TcpListener::bind");
    // If IPv6 is disabled on your machine (for shame!):
    // let listener = TcpListener::bind("0.0.0.0:8080").await.expect("bind");
    tracing::info!("bound to: {:?}", listener.local_addr());
    serve(listener, router).await.expect("axum::serve");
}
