//! Round-trips real Wayback Machine data-format (v2 envelope) tweet snapshots through the model.
//!
//! No snapshot bytes are committed to the repository: the captures are downloaded and cached at
//! run time (see [`common`] for the mechanism, including how unavailable captures are skipped).
//!
//! The captures were selected from a corpus of one hundred fifty thousand archived data snapshots
//! to maximize model coverage: together they exercise every optional field and enum variant of the
//! data [`TweetSnapshot`] observed in that corpus (referenced tweets of all three kinds, note
//! tweets, articles, polls, edit history, context annotations of every type, geo places and
//! coordinates, photo, video, and animated GIF media, tweet and user withholding, derived author
//! entries and locations, partial-error lists, every reply-settings variant, and the tweet, note,
//! and user entity variations). The `resource-unavailable` and place-resource error variants and
//! videos without a duration do not appear in that corpus and necessarily remain uncovered here.
use birdsite::model::wxj::data::TweetSnapshot;
use common::Capture;

mod common;

/// Each entry notes the model features it contributes beyond the entries before it.
const CAPTURES: &[Capture] = &[
    // Broad baseline: a reply with a note tweet (long-form text with URL entities), context
    // annotations, photo media with public metrics, a derived-location author, edit controls and
    // history, included tweets, and an errors array for a missing user lookup.
    Capture {
        screen_name: "grok",
        id: 2_023_509_198_235_898_365,
        timestamp: "20260216212602",
        digest: "AAECR2MZLVBPSMSNS2UOYMXGW7DR5PWR",
    },
    // A retweet with cashtags in the tweet, note-tweet, and user-description entities, video media
    // whose variants lack bit rates, URL preview images, and "Other" annotations.
    Capture {
        screen_name: "inversebrah",
        id: 1_957_670_049_503_949_060,
        timestamp: "20250819050446",
        digest: "AADHYT5IQT5P6J3GDDMKCQOTNKG2XGVC",
    },
    // An open poll attachment, "Person" and "Place" annotations, and hashtag entities.
    Capture {
        screen_name: "georgegalloway",
        id: 1_970_912_739_045_552_362,
        timestamp: "20250924180629",
        digest: "AAGWNAOF3BNOVXQNBI27FICEP7YCMIBT",
    },
    // Geo with both coordinates and a place ID, an included place with a GeoJSON bounding box, and
    // photo alt text.
    Capture {
        screen_name: "NWSSevereTstorm",
        id: 1_952_121_746_691_064_083,
        timestamp: "20250803213747",
        digest: "AAKZFIY7HJTWEFO6UZF4UPQWZ3XFQW7U",
    },
    // Animated GIF media, a not-authorized error with an `includes` section, and a protected
    // referenced user.
    Capture {
        screen_name: "AmericaReal3",
        id: 2_059_076_453_577_159_094,
        timestamp: "20260526005736",
        digest: "AAHGTTA56X5ZU3MDVSNCJR2U2XEHQ4OC",
    },
    // An edited tweet (multiple edit-history IDs), a quoted reference, and the "subscribers" reply
    // settings.
    Capture {
        screen_name: "Jason",
        id: 2_001_504_712_277_831_801,
        timestamp: "20251218040804",
        digest: "AABMPZWENRGOKETUXJ6XGC6MI4FMFRXZ",
    },
    // Media sourced from another tweet, `possibly_sensitive`, and a withheld user.
    Capture {
        screen_name: "suzanna_catie",
        id: 1_949_429_556_248_514_961,
        timestamp: "20250727111959",
        digest: "AAENFP2C376XZEFIHGP34TU4OYNPEKEL",
    },
    // "Product" annotations and a context domain without a description.
    Capture {
        screen_name: "grok",
        id: 2_029_489_691_615_986_152,
        timestamp: "20260305093023",
        digest: "AABLBNLP2ZHAEWKTUVSXF74ZVDP7INNX",
    },
    // The "verified" reply settings.
    Capture {
        screen_name: "grok",
        id: 1_974_086_196_289_810_516,
        timestamp: "20251003121640",
        digest: "AAAEHSSGL2ZWBP4C45D5AR2HWBO7VHU5",
    },
    // The "mentionedUsers" reply settings.
    Capture {
        screen_name: "grok",
        id: 2_006_251_166_028_423_538,
        timestamp: "20251231062846",
        digest: "AAAG53QCU4DDI62HDBDNNOJ62XZY6BD3",
    },
    // The "following" reply settings.
    Capture {
        screen_name: "grok",
        id: 1_948_554_535_594_852_476,
        timestamp: "20250725012258",
        digest: "AAAJGDLP4FT5DN5VELK3XC3OTIYU55Y7",
    },
    // An annotation with a negative index.
    Capture {
        screen_name: "stats_feed",
        id: 1_915_422_407_386_358_135,
        timestamp: "20250424150743",
        digest: "AAATGMPZXNCHXBCTTH3USMMYRNTBZLZM",
    },
    // The "other" reply settings.
    Capture {
        screen_name: "grok",
        id: 2_067_100_130_172_358_858,
        timestamp: "20260617042050",
        digest: "AAAUOYN4FVZ72KIXF6AKAZQ4C67VA7Q3",
    },
    // A URL entity without an expanded URL.
    Capture {
        screen_name: "MarioNawfal",
        id: 2_038_239_290_015_281_518,
        timestamp: "20260329125810",
        digest: "AAAU22G3CM3ZNPVUXPEKBNP5SND5GIH6",
    },
    // A 2022 capture with a `source` field.
    Capture {
        screen_name: "GenshinImpact",
        id: 1_605_255_909_374_042_116,
        timestamp: "20221220173647",
        digest: "AABMHM732HR5L3ZJGTGAP4KAQUO7R6TO",
    },
    // An article with a title.
    Capture {
        screen_name: "milesdeutscher",
        id: 2_053_854_588_646_887_456,
        timestamp: "20260511150747",
        digest: "AAA5KAKF2XIYDXSYGR2CU6CSH5PGMOKN",
    },
    // A mention with the "-1" sentinel ID.
    Capture {
        screen_name: "AmericaPapaBear",
        id: 1_844_378_781_185_868_177,
        timestamp: "20241010140542",
        digest: "AAD5MCJYON5R6LHQW2G3LHDYGEYWMQSE",
    },
    // Tweet-level withholding with a copyright flag.
    Capture {
        screen_name: "Correction2016",
        id: 1_657_138_111_841_812_480,
        timestamp: "20230512213808",
        digest: "AAHBKKNROMXLAGKKJZILMJZ2A3IPOY4E",
    },
    // A derived (ID-only) author entry with derived locations; the author's username appears only
    // in the archived URL, not in the content.
    Capture {
        screen_name: "JacquiDeevoy1",
        id: 2_043_328_500_803_719_264,
        timestamp: "20260412140052",
        digest: "AAIXNJK42PJ6OAJMLSS5GC4UDVVPBG72",
    },
    // A closed poll.
    Capture {
        screen_name: "venanzio_az",
        id: 1_913_689_220_994_011_349,
        timestamp: "20250419202039",
        digest: "AAI25E376IFWIIDZPZGVJPQJ3AQTDY5L",
    },
];

#[tokio::test]
async fn round_trip_archived_data_snapshots() {
    let cache = common::cache();

    for capture in CAPTURES {
        let Some(text) = common::fetch(&cache, capture).await else {
            continue;
        };

        let snapshot = serde_json::from_str::<TweetSnapshot<'_>>(&text)
            .unwrap_or_else(|error| panic!("{capture}: invalid data snapshot: {error}"));

        assert_eq!(
            snapshot.data.id, capture.id,
            "{capture}: unexpected tweet ID"
        );

        // Re-serializing and re-parsing must yield an identical value, which confirms every
        // conversion the snapshot exercises is symmetric.
        let serialized = serde_json::to_string(&snapshot).expect("serializable snapshot");
        let reparsed = serde_json::from_str::<TweetSnapshot<'_>>(&serialized)
            .unwrap_or_else(|error| panic!("{capture}: re-parse failure: {error}"));
        assert_eq!(snapshot, reparsed, "{capture}: round-trip mismatch");

        common::assert_wire_format(&text, &serialized, capture);
    }
}
