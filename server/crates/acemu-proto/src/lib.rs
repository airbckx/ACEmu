//! BatteryNet protocol for Arctic Combat (WebZen, 2013).
//!
//! See `docs/PROTOCOL.md` for how this was recovered. In short: the opcode is the
//! index of each `rf*` function within its `BTTcpLink_*` class in `BatteryNet.u`,
//! and the parameter lists come from the `UProperty` `Next` chains in the same
//! package. The framing in [`frame`] is confirmed against a live client.

pub mod frame;
pub mod wire;

mod generated;
pub use generated::*;

/// Called by generated decoders when a payload has bytes left over, which means
/// our signature for that opcode does not match what the client sent.
pub(crate) fn tracing_warn_tail(name: &str, remaining: usize) {
    tracing::warn!(rpc = name, remaining, "payload longer than the signature - check the spec");
}
