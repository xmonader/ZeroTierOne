// (c) 2020-2022 ZeroTier, Inc. -- currently propritery pending actual release and licensing. See LICENSE.md.

mod address;
mod endpoint;
mod event;
mod mac;
mod path;
mod peer;
mod rootset;

pub(crate) mod node;

pub mod identity;
pub mod inetaddress;

pub use address::Address;
pub use endpoint::Endpoint;
pub use event::Event;
pub use identity::Identity;
pub use inetaddress::InetAddress;
pub use mac::MAC;
pub use node::{DummyInnerProtocol, HostSystem, InnerProtocol, Node, NodeStorage, PacketHandlerResult, VL1AuthProvider};
pub use path::Path;
pub use peer::Peer;
pub use rootset::{Root, RootSet};

pub use zerotier_crypto::verified::Verified;

#[cfg(feature = "debug_events")]
#[allow(unused_macros)]
#[macro_export]
macro_rules! debug_event {
    ($si:expr, $fmt:expr $(, $($arg:tt)*)?) => {
        use $crate::vl1::Event;
        $si.event(Event::Debug(file!(), line!(), format!($fmt, $($($arg)*)?)));
    }
}

#[cfg(not(feature = "debug_events"))]
#[allow(unused_macros)]
#[macro_export]
macro_rules! debug_event {
    ($si:expr, $fmt:expr $(, $($arg:tt)*)?) => {};
}

#[allow(unused_imports)]
pub use debug_event;
