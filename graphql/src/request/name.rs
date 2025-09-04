use std::fmt::Display;

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("Unknown name")]
    Unknown(String),
}

#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub enum RequestName {
    AccountSwitcherDelegateQuery,
    AudioSpaceById,
    AuthenticatePeriscope,
    BirdwatchFetchAliasSelfSelectStatus,
    BirdwatchFetchAuthenticatedUserProfile,
    BirdwatchFetchBirdwatchProfile,
    BirdwatchFetchContributorNotesSlice,
    BirdwatchFetchGlobalTimeline,
    BirdwatchFetchNoteTranslation,
    BirdwatchFetchNotes,
    BirdwatchFetchOneNote,
    BirdwatchFetchPublicData,
    BizProfileFetchUser,
    BlueVerifiedFollowers,
    Bookmarks,
    BroadcastQuery,
    CarouselQuery,
    CombinedLists,
    CommunitiesCreateButtonQuery,
    CommunitiesExploreTimeline,
    CommunitiesFetchOneQuery,
    CommunitiesRankedTimeline,
    CommunitiesSearchQuery,
    CommunityAboutTimeline,
    CommunityDiscoveryTimeline,
    CommunityInviteButtonQuery,
    CommunityMemberRelationshipTypeahead,
    CommunityQuery,
    CommunityTweetsTimeline,
    ConnectTabTimeline,
    CreateGrokConversation,
    #[serde(rename = "DMPinnedInboxQuery")]
    DmPinnedInboxQuery,
    DelegatedAccountListQuery,
    ExplorePage,
    ExploreSidebar,
    Favoriters,
    FetchDraftTweets,
    FetchScheduledTweets,
    Followers,
    FollowersYouKnow,
    Following,
    GenericTimelineById,
    GlobalCommunitiesLatestPostSearchTimeline,
    GlobalCommunitiesPostSearchTimeline,
    GrokConversationItemsByRestId,
    GrokHome,
    GrokShare,
    HomeLatestTimeline,
    HomeTimeline,
    Likes,
    ListByRestId,
    ListLatestTweetsTimeline,
    ListMembers,
    ListProductSubscriptions,
    ListSubscribers,
    ListsManagementPageTimeline,
    ModeratedTimeline,
    NotificationsTimeline,
    #[serde(rename = "PeopleCommunity_Query")]
    PeopleCommunityQuery,
    PremiumContentQuery,
    PremiumSignUpQuery,
    PutClientEducationFlag,
    Retweeters,
    SearchTimeline,
    SidebarUserRecommendations,
    SubscriptionProductDetails,
    SuperFollowsSubscribeQuery,
    #[serde(rename = "TVHomeMixer")]
    TvHomeMixer,
    TopicCarouselQuery,
    TopicTimelineQuery,
    TweetDetail,
    TweetResultByRestId,
    TweetResultsByRestIds,
    TweetStats,
    UserAccountLabel,
    UserBusinessProfileTeamTimeline,
    UserByRestId,
    UserByScreenName,
    UserCreatorSubscriptions,
    UserHighlightsTweets,
    UserMedia,
    UserPreferences,
    UserSuperFollowTweets,
    UserTweets,
    UserTweetsAndReplies,
    UsersByRestIds,
    #[serde(rename = "VOCardsQuery")]
    VoCardsQuery,
    Viewer,
    ViewingOtherUsersTopicsPage,
    #[serde(rename = "affiliatesQuery")]
    AffiliatesQuery,
    #[serde(rename = "articleNudgeDomains")]
    ArticleNudgeDomains,
    #[serde(rename = "isEligibleForAnalyticsUpsellQuery")]
    IsEligibleForAnalyticsUpsellQuery,
    #[serde(rename = "isEligibleForVoButtonUpsellQuery")]
    IsEligibleForVoButtonUpsellQuery,
    #[serde(rename = "membersSliceTimeline_Query")]
    MembersSliceTimelineQuery,
    #[serde(rename = "moderatorsSliceTimeline_Query")]
    ModeratorsSliceTimelineQuery,
    #[serde(rename = "useFetchAnalyticsQuery")]
    UseFetchAnalyticsQuery,
    #[serde(rename = "useFetchProductSubscriptionsQuery")]
    UseFetchProductSubscriptionsQuery,
    #[serde(rename = "useFetchProfileSections_canViewExpandedProfileQuery")]
    UseFetchProfileSectionsCanViewExpandedProfileQuery,
    #[serde(rename = "useFetchProfileSections_profileQuery")]
    UseFetchProfileSectionsProfileQuery,
    #[serde(rename = "usePricesQuery")]
    UsePricesQuery,
    #[serde(rename = "useProductSkuQuery")]
    UseProductSkuQuery,
    #[serde(rename = "useRelayDelegateDataPendingQuery")]
    UseRelayDelegateDataPendingQuery,
    #[serde(rename = "useStoryTopicQuery")]
    UseStoryTopicQuery,
    #[serde(rename = "useSubscriptionProductDetailsQuery")]
    UseSubscriptionProductDetailsQuery,
    #[serde(rename = "useTotalAdCampaignsForUserQuery")]
    UseTotalAdCampaignsForUserQuery,
    #[serde(rename = "useTypingNotifierMutation")]
    UseTypingNotifierMutation,
    #[serde(rename = "useUpsellTrackingMutation")]
    UseUpsellTrackingMutation,
    #[serde(rename = "useVerifiedOrgFeatureHelperQuery")]
    UseVerifiedOrgFeatureHelperQuery,
    #[serde(rename = "viewerUserQuery")]
    ViewerUserQuery,
}

impl Display for RequestName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde::ser::Serialize::serialize(self, f)
    }
}

impl std::str::FromStr for RequestName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let deserializer =
            serde::de::value::BorrowedStrDeserializer::<serde::de::value::Error>::new(s);

        serde::de::Deserialize::deserialize(deserializer).map_err(|_| Error::Unknown(s.to_string()))
    }
}

pub const REQUEST_NAME_VALUES: [RequestName; 109] = [
    RequestName::AccountSwitcherDelegateQuery,
    RequestName::AudioSpaceById,
    RequestName::AuthenticatePeriscope,
    RequestName::BirdwatchFetchAliasSelfSelectStatus,
    RequestName::BirdwatchFetchAuthenticatedUserProfile,
    RequestName::BirdwatchFetchBirdwatchProfile,
    RequestName::BirdwatchFetchContributorNotesSlice,
    RequestName::BirdwatchFetchGlobalTimeline,
    RequestName::BirdwatchFetchNoteTranslation,
    RequestName::BirdwatchFetchNotes,
    RequestName::BirdwatchFetchOneNote,
    RequestName::BirdwatchFetchPublicData,
    RequestName::BizProfileFetchUser,
    RequestName::BlueVerifiedFollowers,
    RequestName::Bookmarks,
    RequestName::BroadcastQuery,
    RequestName::CarouselQuery,
    RequestName::CombinedLists,
    RequestName::CommunitiesCreateButtonQuery,
    RequestName::CommunitiesExploreTimeline,
    RequestName::CommunitiesFetchOneQuery,
    RequestName::CommunitiesRankedTimeline,
    RequestName::CommunitiesSearchQuery,
    RequestName::CommunityAboutTimeline,
    RequestName::CommunityDiscoveryTimeline,
    RequestName::CommunityInviteButtonQuery,
    RequestName::CommunityMemberRelationshipTypeahead,
    RequestName::CommunityQuery,
    RequestName::CommunityTweetsTimeline,
    RequestName::ConnectTabTimeline,
    RequestName::CreateGrokConversation,
    RequestName::DmPinnedInboxQuery,
    RequestName::DelegatedAccountListQuery,
    RequestName::ExplorePage,
    RequestName::ExploreSidebar,
    RequestName::Favoriters,
    RequestName::FetchDraftTweets,
    RequestName::FetchScheduledTweets,
    RequestName::Followers,
    RequestName::FollowersYouKnow,
    RequestName::Following,
    RequestName::GenericTimelineById,
    RequestName::GlobalCommunitiesLatestPostSearchTimeline,
    RequestName::GlobalCommunitiesPostSearchTimeline,
    RequestName::GrokConversationItemsByRestId,
    RequestName::GrokHome,
    RequestName::GrokShare,
    RequestName::HomeLatestTimeline,
    RequestName::HomeTimeline,
    RequestName::Likes,
    RequestName::ListByRestId,
    RequestName::ListLatestTweetsTimeline,
    RequestName::ListMembers,
    RequestName::ListProductSubscriptions,
    RequestName::ListSubscribers,
    RequestName::ListsManagementPageTimeline,
    RequestName::ModeratedTimeline,
    RequestName::NotificationsTimeline,
    RequestName::PeopleCommunityQuery,
    RequestName::PremiumContentQuery,
    RequestName::PremiumSignUpQuery,
    RequestName::PutClientEducationFlag,
    RequestName::Retweeters,
    RequestName::SearchTimeline,
    RequestName::SidebarUserRecommendations,
    RequestName::SubscriptionProductDetails,
    RequestName::SuperFollowsSubscribeQuery,
    RequestName::TvHomeMixer,
    RequestName::TopicCarouselQuery,
    RequestName::TopicTimelineQuery,
    RequestName::TweetDetail,
    RequestName::TweetResultByRestId,
    RequestName::TweetResultsByRestIds,
    RequestName::TweetStats,
    RequestName::UserAccountLabel,
    RequestName::UserBusinessProfileTeamTimeline,
    RequestName::UserByRestId,
    RequestName::UserByScreenName,
    RequestName::UserCreatorSubscriptions,
    RequestName::UserHighlightsTweets,
    RequestName::UserMedia,
    RequestName::UserPreferences,
    RequestName::UserSuperFollowTweets,
    RequestName::UserTweets,
    RequestName::UserTweetsAndReplies,
    RequestName::UsersByRestIds,
    RequestName::VoCardsQuery,
    RequestName::Viewer,
    RequestName::ViewingOtherUsersTopicsPage,
    RequestName::AffiliatesQuery,
    RequestName::ArticleNudgeDomains,
    RequestName::IsEligibleForAnalyticsUpsellQuery,
    RequestName::IsEligibleForVoButtonUpsellQuery,
    RequestName::MembersSliceTimelineQuery,
    RequestName::ModeratorsSliceTimelineQuery,
    RequestName::UseFetchAnalyticsQuery,
    RequestName::UseFetchProductSubscriptionsQuery,
    RequestName::UseFetchProfileSectionsCanViewExpandedProfileQuery,
    RequestName::UseFetchProfileSectionsProfileQuery,
    RequestName::UsePricesQuery,
    RequestName::UseProductSkuQuery,
    RequestName::UseRelayDelegateDataPendingQuery,
    RequestName::UseStoryTopicQuery,
    RequestName::UseSubscriptionProductDetailsQuery,
    RequestName::UseTotalAdCampaignsForUserQuery,
    RequestName::UseTypingNotifierMutation,
    RequestName::UseUpsellTrackingMutation,
    RequestName::UseVerifiedOrgFeatureHelperQuery,
    RequestName::ViewerUserQuery,
];

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Counts(std::collections::BTreeMap<RequestName, usize>);

impl Counts {
    pub fn add(&mut self, request_name: RequestName) {
        let count = self.0.entry(request_name).or_default();
        *count += 1;
    }

    pub fn sorted(&self) -> Vec<(RequestName, usize)> {
        let mut values = self
            .0
            .iter()
            .map(|(request_name, count)| (*request_name, *count))
            .collect::<Vec<_>>();
        values.sort_by_key(|(request_name, count)| (std::cmp::Reverse(*count), *request_name));
        values
    }
}

#[cfg(test)]
mod tests {
    use super::RequestName;

    // Verifies that the values list order matches the ordering.
    #[test]
    fn request_name_values_sorted() {
        let mut values = super::REQUEST_NAME_VALUES.to_vec();
        values.sort();

        assert_eq!(super::REQUEST_NAME_VALUES, values.as_slice());
    }

    // Verifies that the values list order matches the string representation order.
    #[test]
    fn request_names_order() {
        let as_strings = super::REQUEST_NAME_VALUES
            .iter()
            .map(|name| name.to_string())
            .collect::<Vec<_>>();

        let mut as_strings_sorted = as_strings.clone();
        as_strings_sorted.sort();

        assert_eq!(as_strings_sorted, as_strings);
    }

    #[test]
    fn round_trip_request_names_through_json() {
        for name in super::REQUEST_NAME_VALUES {
            let as_json = serde_json::json!(name).to_string();
            let parsed = serde_json::from_str::<RequestName>(&as_json).unwrap();

            assert_eq!(parsed, name);
        }
    }

    #[test]
    fn round_trip_request_names_through_str() {
        for name in super::REQUEST_NAME_VALUES {
            let as_string = name.to_string();
            let parsed = as_string.parse::<RequestName>();

            assert_eq!(parsed, Ok(name));
        }
    }
}
