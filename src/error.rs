// Unified error types for the cosmolith event pipeline.
//
// Currently implemented  Error categories:
// - Config/Watcher: cosmic-config failures, namespace/key issues, watcher setup
// - Event Conversion: unsupported values, type mismatches
// - Dispatch: unsupported events, missing backends, routing failures
// - IPC/Backend: compositor IPC failures, command errors, reconnect failures
// - Environment: compositor detection, missing env vars, invalid session state

use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;


#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    // Config/Watcher Errors
    
    // Failed to read a value from cosmic-config.
    #[error("config read failed: {namespace}.{key}")]
    ConfigRead {
        namespace: String,
        key: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    // Invalid or unrecognized config namespace/key combination.
    #[error("invalid config key: {namespace}.{key}")]
    ConfigKey { 
        namespace: String,
        key: String 
    },

    // Failed to set up a config watcher.
    #[error("watcher setup failed for {namespace}: {reason}")]
    WatcherSetup {
        namespace: String,
        reason: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    // Config watcher channel disconnected unexpectedly.
    #[error("watcher channel disconnected for {namespace}")]
    WatcherDisconnected { namespace: String },

    // Event Conversion Errors

    // General event conversion failure.
    #[error("event conversion failed for {domain}: {reason}")]
    EventConversion { domain: &'static str, reason: String },

    // Unsupported or invalid value encountered during conversion.
    #[error("unsupported value for {domain}.{field}: {value}")]
    UnsupportedValue {
        domain: &'static str,
        field: &'static str,
        value: String,
    },

    // Type mismatch during event deserialization or conversion.
    #[error("type mismatch for {domain}.{field}: expected {expected}, got {actual}")]
    TypeMismatch {
        domain: &'static str,
        field: &'static str,
        expected: &'static str,
        actual: String,
    },

    // Dispatch Errors
    
    // No compositor backend is available to handle events.
    #[error("no compositor backend available")]
    NoCompositor,

    // The event is not supported by the current compositor.
    #[error("{compositor} does not support event: {event}")]
    UnsupportedEvent {
        compositor: &'static str,
        event: String,
    },

    // Event handler not implemented for this compositor.
    #[error("{compositor}: {handler} not implemented")]
    NotImplemented {
        compositor: &'static str,
        handler: &'static str,
    },

    // Failed to route event to appropriate handler.
    #[error("event routing failed: {reason}")]
    RoutingFailed { reason: String },

    // IPC/Backend Errors
    
    // Failed to connect to compositor IPC socket.
    #[error("failed to connect to {compositor} IPC: {reason}")]
    IpcConnection {
        compositor: &'static str,
        reason: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    // IPC command execution failed.
    #[error("{compositor} command failed: {command}")]
    IpcCommand {
        compositor: &'static str,
        command: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    // IPC command returned an error response.
    #[error("{compositor} returned error for '{command}': {response}")]
    IpcResponse {
        compositor: &'static str,
        command: String,
        response: String,
    },

    // IPC socket disconnected; reconnection may be needed.
    #[error("{compositor} IPC disconnected")]
    IpcDisconnected { compositor: &'static str },

    // Failed to reconnect to compositor IPC.
    #[error("failed to reconnect to {compositor} IPC after {attempts} attempts")]
    IpcReconnectFailed {
        compositor: &'static str,
        attempts: u32,
    },

    // Environment Errors
    
    // Failed to detect current compositor/session.
    #[error("compositor detection failed: {reason}")]
    DetectionFailed { reason: String },

    // Required environment variable is missing.
    #[error("missing environment variable: {var}")]
    MissingEnvVar { var: &'static str },

    // Environment variable has an invalid value.
    #[error("invalid value for {var}: {value}")]
    InvalidEnvVar { var: &'static str, value: String },

    // Session is in an unsupported or invalid state.
    #[error("unsupported session type: {session}")]
    UnsupportedSession { session: String },

    // Compositor is not running or not reachable.
    #[error("{compositor} is not running")]
    CompositorNotRunning { compositor: &'static str },

    // Wrapped External Errors
    
    // Standard I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    // Channel send error (generic, for mpsc failures).
    #[error("channel send failed: {0}")]
    ChannelSend(String),

    // Generic external error wrapper for interop.
    #[error("{context}: {message}")]
    External {
        context: &'static str,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

// Convenience constructors for removing boilerplate when creating errors with context and sources.

impl Error {
    // Create a ConfigRead error.
    pub fn config_read(
        namespace: impl Into<String>,
        key: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::ConfigRead {
            namespace: namespace.into(),
            key: key.into(),
            source: Box::new(source),
        }
    }

    // Create a WatcherSetup error.
    pub fn watcher_setup(
        namespace: impl Into<String>,
        reason: impl Into<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        Self::WatcherSetup {
            namespace: namespace.into(),
            reason: reason.into(),
            source,
        }
    }

    // Create an IpcCommand error.
    pub fn ipc_command(
        compositor: &'static str,
        command: impl Into<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        Self::IpcCommand {
            compositor,
            command: command.into(),
            source,
        }
    }

    // Create an IpcConnection error.
    pub fn ipc_connection(
        compositor: &'static str,
        reason: impl Into<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        Self::IpcConnection {
            compositor,
            reason: reason.into(),
            source,
        }
    }

    // Create a NotImplemented error for unimplemented handlers.
    pub fn not_implemented(compositor: &'static str, handler: &'static str) -> Self {
        Self::NotImplemented {
            compositor,
            handler,
        }
    }

    // Wrap an external error with context.
    pub fn external(
        context: &'static str,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::External {
            context,
            message: source.to_string(),
            source: Some(Box::new(source)),
        }
    }
}

