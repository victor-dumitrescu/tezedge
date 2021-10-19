use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::peer::PeerToken;
use crate::service::mio_service::PeerConnectionIncomingAcceptError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PeerConnectionIncomingAcceptState {
    Idle {
        time: u64,
    },
    Success {
        time: u64,
        token: PeerToken,
        address: SocketAddr,
    },
    Error {
        time: u64,
        error: PeerConnectionIncomingAcceptError,
    },
}
