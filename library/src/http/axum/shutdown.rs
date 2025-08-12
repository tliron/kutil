use {
    ::axum::{extract::*, http::StatusCode, response::*},
    axum_server::*,
    std::{io, time::*},
    tokio::{signal::*, sync::oneshot::*, task::*, *},
    tokio_util::sync::*,
};

//
// Shutdown
//

/// Axum server shutdown coordinator.
///
/// Clones will retain the same coordination.
#[derive(Clone, Debug)]
pub struct Shutdown {
    /// Axum server handle.
    ///
    /// Clones will retain the same coordination.
    pub handle: Handle,

    /// Grace period. [None] means indefinite.
    pub grace_period: Option<Duration>,
}

impl Shutdown {
    /// Constructor.
    pub fn new(grace_period: Option<Duration>) -> Self {
        Self { handle: Default::default(), grace_period }
    }

    /// Shutdown (graceful).
    pub fn shutdown(&self) {
        self.handle.graceful_shutdown(self.grace_period);
    }

    /// Shutdown *now* (ignore grace period).
    pub fn shutdown_now(&self) {
        self.handle.shutdown();
    }

    /// Get a [CancellationToken].
    ///
    /// Call [CancellationToken::cancel] to shutdown.
    ///
    /// It's best not to call this function more than once, as it spawns a listener task. If you
    /// need several tokens, clone the one you get here.
    ///
    /// Also returns the [JoinHandle] for the listener task.
    pub fn cancellation_token(&self) -> (CancellationToken, JoinHandle<()>) {
        let token = CancellationToken::default();

        let shutdown = self.clone();

        (
            token.clone(),
            spawn(async move {
                tracing::info!("waiting on cancellation token");

                token.cancelled().await;
                tracing::info!("cancellation token activated");
                shutdown.shutdown();
            }),
        )
    }

    /// Shutdown on channel send.
    ///
    /// Call [Sender::send] to shutdown.
    ///
    /// It's best not to call this function more than once, as it spawns a listener task. If you
    /// need several senders, clone the one you get here.
    ///
    /// Also returns the [JoinHandle] for the listener task.
    pub fn on_channel(&self) -> (Sender<()>, JoinHandle<()>) {
        let (sender, receiver) = channel();
        let shutdown = self.clone();

        (
            sender,
            spawn(async move {
                tracing::info!("listening on shutdown channel");

                match receiver.await {
                    Ok(_) => {
                        tracing::info!("received shutdown message");
                    }

                    Err(error) => {
                        tracing::error!("shutdown channel error: {}", error);
                    }
                }

                shutdown.shutdown();
            }),
        )
    }

    /// Shutdown on signals.
    ///
    /// Returns the [JoinHandle] for the listener task.
    pub fn on_signals(&self) -> io::Result<JoinHandle<()>> {
        #[cfg(all(not(unix), not(windows)))]
        {
            tracing::warn!("signals not supported on this platform");
            return Ok(());
        }

        let shutdown = self.clone();

        #[cfg(unix)]
        let mut interrupt = unix::signal(unix::SignalKind::interrupt())?; // ctrl+c
        #[cfg(unix)]
        let mut terminate = unix::signal(unix::SignalKind::terminate())?;

        Ok(spawn(async move {
            tracing::info!("listening for shutdown signals");

            #[cfg(unix)]
            select! {
                _ = interrupt.recv() => {},
                _ = terminate.recv() => {},
            }

            #[cfg(windows)]
            select! {
                _ = windows::ctrl_c() => {},
                _ = windows::ctrl_break() => {},
                _ = windows::ctrl_close() => {},
                _ = windows::ctrl_logoff() => {},
                _ = windows::ctrl_shutdown() => {},
            }

            tracing::info!("received shutdown signal");
            shutdown.shutdown();
        }))
    }
}

/// Axum request handler that calls [Shutdown::shutdown].
///
/// Expects the [Shutdown] to be available as state. See
/// [Router::with_state](::axum::Router::with_state).
pub async fn shutdown_handler(State(shutdown): State<Shutdown>) -> Response {
    tracing::info!("shutting down");
    shutdown.shutdown();
    StatusCode::NO_CONTENT.into_response()
}
