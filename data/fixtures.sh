#!/usr/bin/env bash
# Pack or unpack encrypted local test fixture bundles.
#
# Bundles are zstd-compressed tar archives of the gitignored local fixture files, symmetrically
# encrypted with GPG using the passphrase in the BIRDSITE_FIXTURES_PASSPHRASE environment
# variable, and named `fixtures-<YYYY-MM-DD>.tar.zst.gpg`.
#
# `unpack <dir>` extracts every bundle found in the directory, restoring the files exactly as
# they were packed, relative to the repository root.
#
# `pack <dir> <YYYY-MM-DD>` creates a new dated bundle in the directory containing only the
# fixture files not already covered by an existing bundle there, so fixture data can be added
# incrementally without rewriting earlier bundles.
set -euo pipefail

usage() {
    echo "Usage: $0 pack <dir> <YYYY-MM-DD> | unpack <dir>" >&2
    exit 1
}

[ $# -ge 2 ] || usage

if [ -z "${BIRDSITE_FIXTURES_PASSPHRASE:-}" ]; then
    echo "BIRDSITE_FIXTURES_PASSPHRASE is not set" >&2
    exit 1
fi

# The directory argument is resolved against the caller's working directory before moving to the
# repository root, which the fixture paths are relative to.
DIR="$(realpath -m "$2")"
cd "$(dirname "$0")/.."

PATHS=(
    cli/tests/data/local
    core/tests/data/local
    graphql/tests/data/local
)

decrypt() {
    gpg --batch --quiet --decrypt --pinentry-mode loopback \
        --passphrase-file <(printf '%s' "$BIRDSITE_FIXTURES_PASSPHRASE") "$1"
}

bundles() {
    find "$DIR" -maxdepth 1 -name 'fixtures-*.tar.zst.gpg' | sort
}

case "$1" in
    pack)
        [ $# -eq 3 ] || usage
        [[ "$3" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]] || usage
        TARGET="$DIR/fixtures-$3.tar.zst.gpg"
        if [ -e "$TARGET" ]; then
            echo "$TARGET already exists; repacking would drop its files from the set" >&2
            exit 1
        fi

        CURRENT="$(find "${PATHS[@]}" -type f 2> /dev/null | sort)"
        # A failed decryption aborts the script here rather than silently treating the bundle as
        # empty, which would duplicate its files into the new bundle.
        COVERED="$(bundles | while read -r bundle; do
            decrypt "$bundle" | tar --zstd -tf -
        done | sort -u)"
        NEW_FILES="$(comm -23 <(printf '%s\n' "$CURRENT") <(printf '%s\n' "$COVERED"))"

        if [ -z "$NEW_FILES" ]; then
            echo "No fixture files to pack that are not already covered" >&2
            exit 0
        fi

        mapfile -t files <<< "$NEW_FILES"
        tar --use-compress-program='zstd -14' --sort=name -cf - -- "${files[@]}" | gpg --batch --symmetric \
            --cipher-algo AES256 --pinentry-mode loopback \
            --passphrase-file <(printf '%s' "$BIRDSITE_FIXTURES_PASSPHRASE") -o "$TARGET"
        echo "Packed ${#files[@]} files into $TARGET"
        ;;
    unpack)
        [ $# -eq 2 ] || usage
        FOUND=0
        while read -r bundle; do
            decrypt "$bundle" | tar --zstd -xf -
            FOUND=1
        done < <(bundles)
        if [ "$FOUND" -eq 0 ]; then
            echo "No fixture bundles found in $DIR" >&2
            exit 1
        fi
        ;;
    *)
        usage
        ;;
esac
