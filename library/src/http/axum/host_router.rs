use super::super::super::std::{collections::*, immutable::*};

use {
    ::axum::{extract::*, http::StatusCode, response::*, *},
    axum_extra::extract::*,
    tower::ServiceExt,
};

//
// HostRouter
//

/// Host router.
#[derive(Clone, Debug, Default)]
pub struct HostRouter {
    /// Routers by host (with optional port).
    pub routers: FastHashMap<ByteString, Router>,

    /// Fallback host (with optional port).
    ///
    /// If a router is not found for a host will fallback to the router for this host instead.
    ///
    /// There must be a router mapped to this host.
    pub fallback_host: Option<ByteString>,
}

impl HostRouter {
    /// Into [Router].
    pub fn into_router(self) -> Option<Router> {
        match self.routers.len() {
            0 => None,
            1 => self.routers.values().next().cloned(),
            _ => Some(Router::default().fallback(host_routers_handler).with_state(self)),
        }
    }

    /// Add.
    pub fn add(&mut self, host_and_optional_port: ByteString, router: Router) {
        self.routers.insert(host_and_optional_port, router);
    }

    /// Fallback [Router].
    pub fn fallback_router(&mut self) -> Option<&mut Router> {
        self.fallback_host.as_ref().and_then(|host_and_optional_port| self.routers.get_mut(host_and_optional_port))
    }

    /// Handle a [Request] by forwarding it to a router if possible.
    pub async fn handle(&mut self, host_and_optional_port: ByteString, request: Request) -> Option<Response> {
        let router = match self.routers.get_mut(&host_and_optional_port) {
            Some(router) => {
                tracing::debug!("router for host: {}", host_and_optional_port);
                router
            }

            None => match self.fallback_router() {
                Some(router) => {
                    tracing::debug!("using fallback, no router for host: {}", host_and_optional_port);
                    router
                }

                None => {
                    tracing::debug!("no fallback and no router for host: {}", host_and_optional_port);
                    return None;
                }
            },
        };

        Some(router.as_service().oneshot(request).await.expect("infallible"))
    }
}

/// Axum request handler that calls [HostRouters::handle].
///
/// Expects the [HostRouters] to be available as state. See
/// [Router::with_state](::axum::Router::with_state).
pub async fn host_routers_handler(
    State(mut host_routers): State<HostRouter>,
    Host(host_and_optional_port): Host,
    request: Request,
) -> Response {
    host_routers
        .handle(host_and_optional_port.into(), request)
        .await
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}
