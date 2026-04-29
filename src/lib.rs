//! Shared primitives for the infinite-db frp backend.
//!
//! Provides typed IDs, SPC / Morton spatial encoding, type signatures, and
//! the runtime `Value` enum. No external dependencies beyond `serde` — the
//! foundational layer for the entire ecosystem.

pub mod id;
pub mod spc;
pub mod types;
pub mod value;

pub use id::{AtomId, BlockId, EdgeId, GraphId, IdGen, PortId};
pub use spc::{SpcKey, SpcRegion, morton_decode, morton_encode};
pub use types::{LayerTag, TypeSig};
pub use value::Value;
