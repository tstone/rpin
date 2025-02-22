#[derive(Debug)]
#[allow(dead_code)]
pub enum FspResponse {
    Id {
        identity: String,
    },
    NodeId {
        id: u8,
        serial: String,
    },
    Unknown {
        command: String,
        address: Option<String>,
        data: Option<String>,
    },
    IdFailed,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum FspRequest {
    Id,
    NodeId,
}
