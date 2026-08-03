//! Shared access to the local example snapshots used by the unit tests.
//!
//! The example files under `tests/data/local/` are deliberately kept out of version control
//! (they are restored from the encrypted bundles by `data/fixtures.sh unpack`), so the tests
//! load them at run time and skip their assertions when the data is absent rather than fail.

/// A wxj/data snapshot whose content is a retweet: the main tweet (2056732294355026173 by
/// 627778780) plus the retweeted tweet (2056695384135581993 by 2903110717) and both users
/// (`Wildharv` and `FrLillie`) in its includes.
pub const EXAMPLE_A: &str = "2PA6SG6HGUHVQD24ZVYHXESJ2I3WIY53";

/// A second wxj/data snapshot, used where the tests need two distinct digests.
pub const EXAMPLE_B: &str = "2WRH57KC2V3N4RK3QAH3ZXPDVXJKPDPD";

/// Reads a local example file, or returns `None` when the example data is not present.
///
/// Example files are named by the Base32-encoded SHA-1 digest of their bytes, so snapshots built
/// from them with `Context::unprocessed_snapshot` carry correct digests.
pub fn example_bytes(digest: &str) -> Option<Vec<u8>> {
    std::fs::read(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data/local")
            .join(digest),
    )
    .ok()
}
