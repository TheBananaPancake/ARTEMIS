#[repr(C)]
pub struct RequestsStartMarker {
    marker1: u64,
    marker2: u64,
    marker3: u64,
    marker4: u64,
}
#[repr(C)]
pub struct BaseRevisionRequest {
    tag1: u64,
    tag2: u64,
    revision: u64,
}
#[repr(C)]
pub struct RequestsEndMarker {
    marker1: u64,
    marker2: u64,
}

#[used]
#[unsafe(link_section = ".limine-requests-start")]
pub static START_MARKER: RequestsStartMarker = RequestsStartMarker {
    marker1: 0xf6b8f4b39de7d1ae,
    marker2: 0xfab91a6940fcb9cf,
    marker3: 0x785c6ed015d3e316,
    marker4: 0x181e920a7852b9d9,
};
#[used]
#[unsafe(link_section = ".limine-requests-base")]
pub static BASE_REVISION_REQUEST: BaseRevisionRequest = BaseRevisionRequest {
    tag1: 0xf9562b2d5c95a6c8,
    tag2: 0x6a7b384944536bdc,
    revision: 0,
};
#[used]
#[unsafe(link_section = ".limine-requests-end")]
pub static END_MARKER: RequestsEndMarker = RequestsEndMarker {
    marker1: 0xadc0e0531bb10d03,
    marker2: 0x9572709f31764c62,
};
