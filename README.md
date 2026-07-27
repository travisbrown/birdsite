# Birdsite

[![build](https://github.com/travisbrown/birdsite/actions/workflows/ci.yml/badge.svg)](https://github.com/travisbrown/birdsite/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/travisbrown/birdsite/branch/main/graph/badge.svg)](https://codecov.io/gh/travisbrown/birdsite)
[![crates.io](https://img.shields.io/crates/v/birdsite.svg)](https://crates.io/crates/birdsite)
[![docs.rs](https://docs.rs/birdsite/badge.svg)](https://docs.rs/birdsite)

This project is a small Rust library for working with Twitter data.

## Example data

The models are checked against real archived documents, which are kept in two tiers so that a
clone of this repository is self-contained while the archives used during development stay out of
version control.

**Fixtures** are committed under `<package>/tests/data` and included at compile time, so
`cargo test` works everywhere with no setup. They live inside the package directory because
`cargo package` ships only files from there. Each one is distilled from a corpus by taking the
fewest documents that cover the most distinct field paths and value variants, then dropping array
elements that repeat coverage already present. Nothing else is edited, so a fixture is authentic
wire data with the redundancy removed: together they cover at least 85% of the schema features
their corpora exercise in roughly 100 KB.

**Corpora** are the full collections the fixtures were distilled from, and are read at run time
from `examples` (override with `BIRDSITE_CORPUS_DIR`). Every test that uses one has a fixture
counterpart, and passes trivially when the corpus is absent:

| Corpus | Contents |
| --- | --- |
| `wxj/flat` | v1.1 tweet snapshots, as archived until late 2022 |
| `wxj/data` | v2 tweet snapshots, as archived from late 2022 |
| `wxj/media` | v2 media objects |
| `graphql/communities` | community results, which differ by API era |
| `graphql/birdwatch-notes`, `graphql/birdwatch-manifests` | Birdwatch (Community Notes) responses |

A missing corpus is reported on stderr and skipped. Set `BIRDSITE_REQUIRE_CORPUS` to turn that
skip into a failure, which is what you want on a machine where the archives are installed:

```bash
BIRDSITE_REQUIRE_CORPUS=1 cargo test
```

## License

This project is licensed under the [GNU Affero General Public License, version 3
only](https://www.gnu.org/licenses/agpl-3.0.html). See [LICENSE](LICENSE) for the full text.
