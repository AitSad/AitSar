// Import necessary modules
pub mod search;
pub mod table;

// Declare the crate as a workspace
// This is required to enable the use of features across multiple crates in the workspace
// For example, if the caesar feature is enabled in SadieFish, it will also be enabled in any other
// crate in the workspace that depends on SadieFish.
// This is useful for testing and for avoiding version mismatches between crates.
#[cfg(not(test))] // Only include this section in non-test builds
#[cfg(feature = "workspace")]
pub mod workspace {
    pub use AitSar_search as search;
    pub use SadieFish_table as table;
}

// Declare the encryption module as the default feature
#[cfg(not(test))] // Only include this section in non-test builds
#[cfg(feature = "default")]
pub use AitSar_search::*;

// Declare features for the various encryption algorithms and utilities
#[cfg(not(test))] // Only include this section in non-test builds
#[cfg(any(feature = "ternary", feature = "interpolation", feature="exponential", feature="fibonacci"))]
#[cfg(not(feature = "default"))]
pub use AitSar_search::*;

#[cfg(not(test))] // Only include this section in non-test builds
#[cfg(any(feature = "hash"))]
pub use AitSar_table::*;
