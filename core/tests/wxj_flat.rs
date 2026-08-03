//! Round-trips real Wayback Machine flat-format tweet snapshots through the model.
//!
//! No snapshot bytes are committed to the repository: the captures are downloaded and cached at
//! run time (see [`common`] for the mechanism, including how unavailable captures are skipped).
//!
//! The captures were selected from a corpus of over two hundred thousand archived flat snapshots
//! to maximize model coverage: together they exercise every optional field and enum variant of the
//! flat [`TweetSnapshot`] observed in that corpus (nested retweeted and quoted statuses, extended
//! tweets and entities, photo, video, and animated GIF media, geo coordinates and places, cashtag
//! symbols, withheld countries, scopes, contributors, and the `User` field variations), along with
//! the shared `Place`, `Media`, `Country`, `TimeZone`, `Lang`, `Color`, and `SourceAnchor` models.
//! Corpus-absent branches (`withheld_copyright`, protected users, count sentinels, mentions with
//! null IDs, the "moderator" translator type) necessarily remain uncovered here.
use birdsite::model::wxj::flat::TweetSnapshot;
use common::Capture;

mod common;

/// Each entry notes the model features it contributes beyond the entries before it.
const CAPTURES: &[Capture] = &[
    // Broad baseline: a reply quoting a retweet that itself contains a quoted status, with an
    // extended tweet, photo and video media (with additional media info), `possibly_sensitive`,
    // a display text range, and verified, withheld, and bannerless users.
    Capture {
        screen_name: "GlenWoodfin",
        id: 1_429_544_770_741_972_992,
        timestamp: "20210822204327",
        digest: "ACY3NE3QWRS7R2FNBQBY2UDGD3HVGYMW",
    },
    // A 2017 capture: extended entities nested inside the extended tweet, URL entities without
    // expanded or display URLs, a missing `quote_count`, and a user with a UTC offset and time
    // zone, a non-null language, and no `translator_type` field.
    Capture {
        screen_name: "Carolina_Toha",
        id: 899_656_330_524_446_720,
        timestamp: "20170821153558",
        digest: "AADHWJWUOMGY4LIRTVNSFXZGUWGOE4GE",
    },
    // Non-null `geo` and `coordinates`, and a `place` with a bounding box.
    Capture {
        screen_name: "joelpollak",
        id: 1_589_116_374_361_985_025,
        timestamp: "20221106044402",
        digest: "ABYN5G4DCNSZOIOC6G3GYJO3PK52ZFWT",
    },
    // Cashtag symbols in both the plain and extended entities, and the "regular" translator type.
    Capture {
        screen_name: "DailyLoud",
        id: 968_131_342_247_579_655,
        timestamp: "20180226143113",
        digest: "ABO5RNUUVHC56CJTUYERXQ6NER7N6FY3",
    },
    // Animated GIF media and tweet-level `withheld_in_countries`.
    Capture {
        screen_name: "carrieksada",
        id: 1_324_884_403_714_101_248,
        timestamp: "20201107012031",
        digest: "ABLLKLQKAMWR46WLFB3Z4E6NBNMFWGH7",
    },
    // A reply directed at a user with no replied-to status ID.
    Capture {
        screen_name: "wizkidayo",
        id: 897_292_135_577_399_296,
        timestamp: "20170815030130",
        digest: "AACBCUIDVDCSLFVE57SAA4T253GMBMEX",
    },
    // The "badged" translator type.
    Capture {
        screen_name: "manavjivan",
        id: 1_096_319_754_057_351_169,
        timestamp: "20190215080612",
        digest: "ADC7BW4UY25AZC4XCTHJAJDEQ5MC6URN",
    },
    // A non-null `scopes` object.
    Capture {
        screen_name: "LATiffani1",
        id: 1_194_013_336_037_257_216,
        timestamp: "20191111220537",
        digest: "ACNBZS56PTEWGHNJ6OMLL7ULGJ2X2YNU",
    },
    // A non-null `contributors` list: the only occurrence in the corpus sample, so this is the one
    // capture drawn from a small account.
    Capture {
        screen_name: "sldmfree",
        id: 911_660_558_583_259_137,
        timestamp: "20170923183629",
        digest: "AAFCRVUYV2JDP6DGXCWZJNKNPT26R7XC",
    },
];

#[tokio::test]
async fn round_trip_archived_flat_snapshots() {
    let cache = common::cache();

    for capture in CAPTURES {
        let Some(text) = common::fetch(&cache, capture).await else {
            continue;
        };

        let snapshot = serde_json::from_str::<TweetSnapshot<'_>>(&text)
            .unwrap_or_else(|error| panic!("{capture}: invalid flat snapshot: {error}"));

        assert_eq!(snapshot.id, capture.id, "{capture}: unexpected tweet ID");

        // Re-serializing and re-parsing must yield an identical value, which confirms every
        // conversion the snapshot exercises is symmetric.
        let serialized = serde_json::to_string(&snapshot).expect("serializable snapshot");
        let reparsed = serde_json::from_str::<TweetSnapshot<'_>>(&serialized)
            .unwrap_or_else(|error| panic!("{capture}: re-parse failure: {error}"));
        assert_eq!(snapshot, reparsed, "{capture}: round-trip mismatch");

        common::assert_wire_format(&text, &serialized, capture);
    }
}
