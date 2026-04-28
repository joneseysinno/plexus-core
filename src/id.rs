use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// ID newtypes
// ---------------------------------------------------------------------------

macro_rules! define_id {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
        )]
        pub struct $name(u64);

        impl $name {
            /// Wrap a raw `u64` as this ID type.
            #[inline]
            pub fn new(id: u64) -> Self {
                Self(id)
            }

            /// Return the underlying `u64` value.
            #[inline]
            pub fn value(self) -> u64 {
                self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }
    };
}

define_id!(AtomId, "Unique identifier for an `Atom`.");
define_id!(BlockId, "Unique identifier for a `Block`.");
define_id!(EdgeId, "Unique identifier for a `HyperEdge`.");
define_id!(GraphId, "Unique identifier for a `Graph`.");
define_id!(PortId, "Unique identifier for a `Port`.");

// ---------------------------------------------------------------------------
// ID generator — thread-safe, monotonically increasing
// ---------------------------------------------------------------------------

/// Thread-safe generator for all ID types. Each counter is process-local and
/// starts at 1 (0 is reserved as a sentinel "null" value).
pub struct IdGen {
    atom: AtomicU64,
    block: AtomicU64,
    edge: AtomicU64,
    graph: AtomicU64,
    port: AtomicU64,
}

impl IdGen {
    /// Create a new generator with all counters at 1.
    pub const fn new() -> Self {
        Self {
            atom: AtomicU64::new(1),
            block: AtomicU64::new(1),
            edge: AtomicU64::new(1),
            graph: AtomicU64::new(1),
            port: AtomicU64::new(1),
        }
    }

    /// Generate the next unique [`AtomId`].
    pub fn next_atom_id(&self) -> AtomId {
        AtomId::new(self.atom.fetch_add(1, Ordering::Relaxed))
    }

    /// Generate the next unique [`BlockId`].
    pub fn next_block_id(&self) -> BlockId {
        BlockId::new(self.block.fetch_add(1, Ordering::Relaxed))
    }

    /// Generate the next unique [`EdgeId`].
    pub fn next_edge_id(&self) -> EdgeId {
        EdgeId::new(self.edge.fetch_add(1, Ordering::Relaxed))
    }

    /// Generate the next unique [`GraphId`].
    pub fn next_graph_id(&self) -> GraphId {
        GraphId::new(self.graph.fetch_add(1, Ordering::Relaxed))
    }

    /// Generate the next unique [`PortId`].
    pub fn next_port_id(&self) -> PortId {
        PortId::new(self.port.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for IdGen {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_new_and_value_round_trip() {
        assert_eq!(AtomId::new(42).value(), 42);
        assert_eq!(BlockId::new(0).value(), 0);
    }

    #[test]
    fn id_gen_monotonic() {
        let id_gen = IdGen::new();
        let a1 = id_gen.next_atom_id();
        let a2 = id_gen.next_atom_id();
        assert!(a2.value() > a1.value());
    }

    #[test]
    fn id_gen_types_independent() {
        let id_gen = IdGen::new();
        let atom = id_gen.next_atom_id();
        let block = id_gen.next_block_id();
        // Both start at 1 independently
        assert_eq!(atom.value(), 1);
        assert_eq!(block.value(), 1);
    }
}
