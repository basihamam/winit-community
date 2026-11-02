use std::borrow::Cow;
use std::num::{NonZeroU16, NonZeroU32};
use std::sync::Arc;

use android_activity::AndroidApp;
use dpi::{PhysicalPosition, PhysicalSize};
use winit_core::monitor::{MonitorHandle as CoreMonitorHandle, MonitorHandleProvider, VideoMode};

#[derive(Debug, Clone)]
pub struct MonitorHandle {
    pub(crate) app: AndroidApp,
}

impl MonitorHandle {
    pub fn new(app: AndroidApp) -> Self {
        Self { app }
    }
}

impl MonitorHandleProvider for MonitorHandle {
    fn id(&self) -> u128 {
        0
    }

    fn native_id(&self) -> u64 {
        0
    }

    fn name(&self) -> Option<Cow<'_, str>> {
        Some("Primary Display".into())
    }

    fn position(&self) -> Option<PhysicalPosition<i32>> {
        Some(PhysicalPosition::new(0, 0))
    }

    fn scale_factor(&self) -> f64 {
        self.app.config().density().map(|dpi| dpi as f64 / 160.0).unwrap_or(1.0)
    }

    fn current_video_mode(&self) -> Option<VideoMode> {
        let size = if let Some(native_window) = self.app.native_window() {
            PhysicalSize::new(native_window.width() as u32, native_window.height() as u32)
        } else {
            PhysicalSize::new(0, 0)
        };

        Some(VideoMode::new(
            size,
            Some(NonZeroU16::new(32).unwrap()),
            NonZeroU32::new(60_000),
        ))
    }

    fn video_modes(&self) -> Box<dyn Iterator<Item = VideoMode>> {
        let modes = self.current_video_mode().into_iter().collect::<Vec<_>>();
        Box::new(modes.into_iter())
    }
}

impl From<MonitorHandle> for CoreMonitorHandle {
    fn from(monitor: MonitorHandle) -> Self {
        CoreMonitorHandle(Arc::new(monitor))
    }
}
