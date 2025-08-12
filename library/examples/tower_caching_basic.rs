mod utils;

use {
    ::axum::{routing::*, *},
    kutil::http::{
        cache::{implementation::moka::*, *},
        tower::caching::*,
    },
    moka::future::Cache,
    std::time::*,
    tokio::{net::*, *},
    tower_http::trace::*,
};

// Axum server with Kutil's caching middleware for Tower
//
// Pay attention to the tracing log to see what our middleware and the cache are doing!
// (Entries will be expired from the cache after 10 seconds)
//
// You can send requests from a web browser or via CLI. Some fun examples:
//
//   curl http://localhost:8080
//
//   curl --verbose --compressed http://localhost:8080
//
//   curl http://localhost:8080?x=1&y=2
//   curl http://localhost:8080?y=2&x=1

// Note that this is *not* a promise for the actual maximum memory use,
// but is rather a limit for the total of cache entry weights, which are themselves estimates
const CACHE_SIZE: u64 = 1024 * 1024; // 1 MiB

// Keeping it very short for testing purposes
const CACHE_DURATION: Duration = Duration::from_secs(10);

const MAX_BODY_SIZE: usize = 1024; // 1 KiB

#[main]
async fn main() {
    utils::init_tracing();

    // Construct a Moka cache according to your preferences

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

    // All you need to do is add our layer to the router

    let router = Router::default()
        .route("/", get(("Hello, world!\n",)))
        .layer(
            CachingLayer::default()
                .cache(cache.clone())
                .max_cacheable_body_size(MAX_BODY_SIZE)
                .keep_identity_encoding(false),
        )
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("[::]:8080").await.expect("TcpListener::bind");
    // If IPv6 is disabled on your machine (for shame!):
    // let listener = TcpListener::bind("0.0.0.0:8080").await.expect("bind");
    tracing::info!("bound to: {:?}", listener.local_addr());
    serve(listener, router).await.expect("axum::serve");
}
