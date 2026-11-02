//! Direct Rendering Manager (DRM) device feedback for direct scanout.

use crate::event_loop::{ActiveEventLoop, EventLoop};

/// Additional methods on [`ActiveEventLoop`] and [`EventLoop`] for DRM feedback.
pub trait EventLoopExtDrm {
    /// Returns the main DRM device node (as `dev_t`) preferred by the Wayland compositor for direct scanout,
    /// or `None` if not available.
    fn main_drm_device(&self) -> Option<libc::dev_t>;
}

impl EventLoopExtDrm for dyn ActiveEventLoop + '_ {
    fn main_drm_device(&self) -> Option<libc::dev_t> {
        #[cfg(wayland_platform)]
        if let Some(event_loop) = self.cast_ref::<crate::platform_impl::wayland::ActiveEventLoop>() {
            return event_loop.main_drm_device();
        }

        None
    }
}

impl EventLoopExtDrm for EventLoop {
    fn main_drm_device(&self) -> Option<libc::dev_t> {
        #[cfg(wayland_platform)]
        return self.event_loop.main_drm_device();

        #[cfg(not(wayland_platform))]
        None
    }
}
