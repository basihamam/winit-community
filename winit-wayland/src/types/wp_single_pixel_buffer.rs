//! Handling of the wp-single-pixel-buffer-v1 protocol.

use sctk::globals::GlobalData;
use sctk::reexports::client::globals::{BindError, GlobalList};
use sctk::reexports::client::protocol::wl_buffer::WlBuffer;
use sctk::reexports::client::{Connection, Dispatch, Proxy, QueueHandle, delegate_dispatch};
use wayland_protocols::wp::single_pixel_buffer::v1::client::wp_single_pixel_buffer_manager_v1::WpSinglePixelBufferManagerV1;

use crate::state::WinitState;

#[derive(Debug, Clone)]
pub struct SinglePixelBufferManager {
    manager: WpSinglePixelBufferManagerV1,
}

impl SinglePixelBufferManager {
    pub fn new(
        globals: &GlobalList,
        queue_handle: &QueueHandle<WinitState>,
    ) -> Result<Self, BindError> {
        let manager = globals.bind(queue_handle, 1..=1, GlobalData)?;
        Ok(Self { manager })
    }

    pub fn create_u32_rgba_buffer(
        &self,
        r: u32,
        g: u32,
        b: u32,
        a: u32,
        queue_handle: &QueueHandle<WinitState>,
    ) -> WlBuffer {
        self.manager.create_u32_rgba_buffer(r, g, b, a, queue_handle, ())
    }
}

impl Dispatch<WpSinglePixelBufferManagerV1, GlobalData, WinitState> for SinglePixelBufferManager {
    fn event(
        _: &mut WinitState,
        _: &WpSinglePixelBufferManagerV1,
        _: <WpSinglePixelBufferManagerV1 as Proxy>::Event,
        _: &GlobalData,
        _: &Connection,
        _: &QueueHandle<WinitState>,
    ) {
    }
}

impl Dispatch<WlBuffer, (), WinitState> for SinglePixelBufferManager {
    fn event(
        _: &mut WinitState,
        _: &WlBuffer,
        _: <WlBuffer as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<WinitState>,
    ) {
    }
}

delegate_dispatch!(WinitState: [WpSinglePixelBufferManagerV1: GlobalData] => SinglePixelBufferManager);
delegate_dispatch!(WinitState: [WlBuffer: ()] => SinglePixelBufferManager);
