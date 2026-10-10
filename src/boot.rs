use limine::request::FramebufferRequest;
use limine::{BaseRevision, RequestsEndMarker, RequestsStartMarker};

#[used]
#[unsafe(link_section = ".requests")]
pub static BASE_REVISION: BaseRevision = BaseRevision::new();
#[used]
#[unsafe(link_section = ".requests")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests-start")]
pub static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests-end")]
pub static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[expect(unused)]
pub struct BootFramebuffer {
    pub addr: *mut u32,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
}
