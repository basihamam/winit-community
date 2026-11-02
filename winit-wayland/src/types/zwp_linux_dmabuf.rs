use sctk::globals::GlobalData;
use sctk::reexports::client::globals::{BindError, GlobalList};
use sctk::reexports::client::{Connection, Dispatch, Proxy, QueueHandle, delegate_dispatch};
use wayland_protocols::wp::linux_dmabuf::zv1::client::zwp_linux_dmabuf_feedback_v1::{
    Event as FeedbackEvent, ZwpLinuxDmabufFeedbackV1,
};
use wayland_protocols::wp::linux_dmabuf::zv1::client::zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1;

use crate::state::WinitState;

#[derive(Debug, Clone)]
pub struct DmabufFeedbackManager {
    _manager: ZwpLinuxDmabufV1,
    _feedback: Option<ZwpLinuxDmabufFeedbackV1>,
}

impl DmabufFeedbackManager {
    pub fn new(
        globals: &GlobalList,
        queue_handle: &QueueHandle<WinitState>,
    ) -> Result<Self, BindError> {
        let manager: ZwpLinuxDmabufV1 = globals.bind(queue_handle, 4..=5, GlobalData)?;
        let feedback = manager.get_default_feedback(queue_handle, ());
        Ok(Self { _manager: manager, _feedback: Some(feedback) })
    }
}

impl Dispatch<ZwpLinuxDmabufV1, GlobalData, WinitState> for DmabufFeedbackManager {
    fn event(
        _: &mut WinitState,
        _: &ZwpLinuxDmabufV1,
        _: <ZwpLinuxDmabufV1 as Proxy>::Event,
        _: &GlobalData,
        _: &Connection,
        _: &QueueHandle<WinitState>,
    ) {
    }
}

impl Dispatch<ZwpLinuxDmabufFeedbackV1, (), WinitState> for DmabufFeedbackManager {
    fn event(
        state: &mut WinitState,
        _: &ZwpLinuxDmabufFeedbackV1,
        event: <ZwpLinuxDmabufFeedbackV1 as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<WinitState>,
    ) {
        if let FeedbackEvent::MainDevice { device } = event {
            if device.len() >= std::mem::size_of::<libc::dev_t>() {
                let mut bytes = [0u8; std::mem::size_of::<libc::dev_t>()];
                bytes.copy_from_slice(&device[..std::mem::size_of::<libc::dev_t>()]);
                state.main_drm_device = Some(libc::dev_t::from_ne_bytes(bytes));
            }
        }
    }
}

delegate_dispatch!(WinitState: [ZwpLinuxDmabufV1: GlobalData] => DmabufFeedbackManager);
delegate_dispatch!(WinitState: [ZwpLinuxDmabufFeedbackV1: ()] => DmabufFeedbackManager);
