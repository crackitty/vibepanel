//! WindowTitleService - shared, event-driven window title state via CompositorManager.
//!
//! This provides a GTK-friendly API for per-output window information:
//! - Uses the shared CompositorManager singleton, which keeps the latest
//!   window for every output
//! - Supports callback registration for reactive updates

use std::rc::Rc;

use tracing::debug;

use super::callbacks::{CallbackId, Callbacks};
use super::compositor::{CompositorManager, WindowInfo};

/// Snapshot of window title service state for callbacks.
#[derive(Debug, Clone, Default)]
pub struct WindowTitleSnapshot {
    /// Current window title.
    pub title: String,
    /// Current application ID.
    pub app_id: String,
    /// Output/monitor name (if available).
    pub output: Option<String>,
    /// Whether this is the globally focused output.
    pub focused: bool,
}

impl From<&WindowInfo> for WindowTitleSnapshot {
    fn from(info: &WindowInfo) -> Self {
        Self {
            title: info.title.clone(),
            app_id: info.app_id.clone(),
            output: info.output.clone(),
            focused: info.focused,
        }
    }
}

/// Shared, process-wide window title service.
///
/// Provides reactive window title state with GTK main loop integration.
/// Widgets should call `connect()` to receive per-output window updates and
/// filter for the output (or focus) they care about.
pub struct WindowTitleService {
    /// Registered callbacks.
    callbacks: Callbacks<WindowTitleSnapshot>,
}

impl WindowTitleService {
    fn new() -> Rc<Self> {
        let service = Rc::new(Self {
            callbacks: Callbacks::new(),
        });

        Self::register_with_manager(&service, &CompositorManager::global());

        debug!("WindowTitleService initialized (using CompositorManager)");
        service
    }

    /// Get the global WindowTitleService singleton.
    pub fn global() -> Rc<Self> {
        thread_local! {
            static INSTANCE: Rc<WindowTitleService> = WindowTitleService::new();
        }

        INSTANCE.with(|s| s.clone())
    }

    /// Register a callback to be invoked when an output's window changes.
    /// The callback is always executed on the GLib main loop.
    pub fn connect<F>(&self, callback: F) -> CallbackId
    where
        F: Fn(&WindowTitleSnapshot) + 'static,
    {
        let id = self.callbacks.register(callback);

        // Immediately replay every output so this widget can render.
        for info in CompositorManager::global().window_updates() {
            self.callbacks.notify_single(id, &(&info).into());
        }
        id
    }

    /// Unregister a callback by its ID.
    pub fn disconnect(&self, id: CallbackId) -> bool {
        self.callbacks.unregister(id)
    }

    fn handle_update(&self, window_info: &WindowInfo) {
        self.callbacks.notify(&window_info.into());
    }

    fn register_with_manager(this: &Rc<Self>, manager: &Rc<CompositorManager>) {
        // Create callback that handles updates
        let service_weak = Rc::downgrade(this);
        manager.register_window_callback(move |window_info| {
            if let Some(service) = service_weak.upgrade() {
                service.handle_update(window_info);
            }
        });
    }
}

impl Drop for WindowTitleService {
    fn drop(&mut self) {
        debug!("WindowTitleService dropped");
    }
}
