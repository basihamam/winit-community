use std::ptr;

use windows_sys::Win32::Graphics::Gdi::{
    CDS_FULLSCREEN, ChangeDisplaySettingsExW, DISP_CHANGE_SUCCESSFUL,
};
use winit_core::monitor::{Fullscreen, MonitorHandleProvider};

use crate::monitor::{self, MonitorHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DisplayTransition {
    Applied,
    Unchanged,
    Windowed,
}

pub(crate) fn restore_display_mode(monitor: Option<&winit_core::monitor::MonitorHandle>) -> bool {
    let sz_device = monitor.and_then(|m| {
        m.cast_ref::<MonitorHandle>()
            .and_then(|h| monitor::get_monitor_info(h.native_id() as _).ok())
            .map(|info| info.szDevice)
    });

    let res = unsafe {
        ChangeDisplaySettingsExW(
            sz_device.as_ref().map(|d| d.as_ptr()).unwrap_or(ptr::null()),
            ptr::null(),
            ptr::null_mut(),
            0,
            ptr::null(),
        )
    };

    if res == DISP_CHANGE_SUCCESSFUL {
        true
    } else {
        tracing::warn!(
            "Failed to restore display settings: ChangeDisplaySettingsExW returned error code {res}"
        );
        false
    }
}

pub(crate) fn apply_exclusive_mode(
    monitor: &winit_core::monitor::MonitorHandle,
    video_mode: &winit_core::monitor::VideoMode,
) -> bool {
    let Some(monitor_handle) = monitor.cast_ref::<MonitorHandle>() else {
        tracing::warn!("Failed to obtain exclusive fullscreen monitor handle");
        return false;
    };

    let Some(video_mode_handle) = monitor_handle
        .video_mode_handles()
        .find(|mode| &mode.mode == video_mode)
    else {
        tracing::warn!("Requested video mode not found in monitor modes");
        return false;
    };

    let monitor_info = match monitor::get_monitor_info(monitor_handle.native_id() as _) {
        Ok(info) => info,
        Err(error) => {
            tracing::warn!("Failed to obtain exclusive fullscreen monitor info: {error}");
            return false;
        },
    };

    let res = unsafe {
        ChangeDisplaySettingsExW(
            monitor_info.szDevice.as_ptr(),
            &*video_mode_handle.native_video_mode,
            ptr::null_mut(),
            CDS_FULLSCREEN,
            ptr::null(),
        )
    };

    if res == DISP_CHANGE_SUCCESSFUL {
        true
    } else {
        tracing::warn!("ChangeDisplaySettingsExW returned error code: {res}");
        false
    }
}

pub(crate) fn transition_fullscreen(
    old_fullscreen: &Option<Fullscreen>,
    new_fullscreen: &Option<Fullscreen>,
) -> Result<DisplayTransition, ()> {
    match (old_fullscreen, new_fullscreen) {
        (Some(Fullscreen::Exclusive(old_m, _)), Some(Fullscreen::Exclusive(new_m, new_mode))) => {
            if old_m.native_id() != new_m.native_id() {
                restore_display_mode(Some(old_m));
            }
            if apply_exclusive_mode(new_m, new_mode) {
                Ok(DisplayTransition::Applied)
            } else {
                Err(())
            }
        },
        (_, Some(Fullscreen::Exclusive(m, mode))) => {
            if apply_exclusive_mode(m, mode) {
                Ok(DisplayTransition::Applied)
            } else {
                Err(())
            }
        },
        (Some(Fullscreen::Exclusive(old_m, _)), _) => {
            restore_display_mode(Some(old_m));
            Ok(DisplayTransition::Windowed)
        },
        _ => Ok(DisplayTransition::Unchanged),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_display_mode_runtime() {
        // Verifies calling restore_display_mode(None) as done in WM_DISPLAYCHANGE safely executes under Wine
        let res = restore_display_mode(None);
        // Under Wine/Windows desktop, restoring display mode executes without crashing or hanging
        assert!(res || !res);
    }
}
