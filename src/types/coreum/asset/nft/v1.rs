use coreum_std_derive::CosmwasmExt;
/// SendAuthorization allows the grantee to send specific NFTs from the granter's account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.SendAuthorization")]
pub struct SendAuthorization {
    #[prost(message, repeated, tag = "1")]
    pub nfts: ::prost::alloc::vec::Vec<NftIdentifier>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.NFTIdentifier")]
pub struct NftIdentifier {
    /// class_id defines the unique identifier of the nft classification, similar to the contract address of ERC721
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id defines the unique identification of nft
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// ClassDefinition defines the non-fungible token class settings to store.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.ClassDefinition")]
pub struct ClassDefinition {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(enumeration = "ClassFeature", repeated, tag = "3")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    /// royalty_rate is a number between 0 and 1,which will be used in coreum native DEX.
    /// whenever an NFT this class is traded on the DEX, the traded amount will be multiplied by this value
    /// that will be transferred to the issuer of the NFT.
    #[prost(string, tag = "4")]
    pub royalty_rate: ::prost::alloc::string::String,
}
/// Class is a full representation of the non-fungible token class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.Class")]
pub struct Class {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub uri_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "8")]
    pub data: ::core::option::Option<crate::shim::Any>,
    #[prost(enumeration = "ClassFeature", repeated, tag = "9")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    /// royalty_rate is a number between 0 and 1,which will be used in coreum native DEX.
    /// whenever an NFT this class is traded on the DEX, the traded amount will be multiplied by this value
    /// that will be transferred to the issuer of the NFT.
    #[prost(string, tag = "10")]
    pub royalty_rate: ::prost::alloc::string::String,
}
/// ClassFeature defines possible features of non-fungible token class.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum ClassFeature {
    Burning = 0,
    Freezing = 1,
    Whitelisting = 2,
    DisableSending = 3,
    Soulbound = 4,
}
impl ClassFeature {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClassFeature::Burning => "burning",
            ClassFeature::Freezing => "freezing",
            ClassFeature::Whitelisting => "whitelisting",
            ClassFeature::DisableSending => "disable_sending",
            ClassFeature::Soulbound => "soulbound",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "burning" => Some(Self::Burning),
            "freezing" => Some(Self::Freezing),
            "whitelisting" => Some(Self::Whitelisting),
            "disable_sending" => Some(Self::DisableSending),
            "soulbound" => Some(Self::Soulbound),
            _ => None,
        }
    }
}
/// EventClassIssued is emitted on MsgIssueClass.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.EventClassIssued")]
pub struct EventClassIssued {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub uri_hash: ::prost::alloc::string::String,
    #[prost(enumeration = "ClassFeature", repeated, tag = "8")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, tag = "9")]
    pub royalty_rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.EventFrozen")]
pub struct EventFrozen {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.EventUnfrozen")]
pub struct EventUnfrozen {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.EventClassFrozen")]
pub struct EventClassFrozen {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.EventClassUnfrozen")]
pub struct EventClassUnfrozen {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.EventAddedToWhitelist")]
pub struct EventAddedToWhitelist {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.EventRemovedFromWhitelist")]
pub struct EventRemovedFromWhitelist {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.EventAddedToClassWhitelist")]
pub struct EventAddedToClassWhitelist {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.EventRemovedFromClassWhitelist")]
pub struct EventRemovedFromClassWhitelist {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
}
/// Params store gov manageable parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.Params")]
pub struct Params {
    /// mint_fee is the fee burnt each time new NFT is minted
    #[prost(message, optional, tag = "1")]
    pub mint_fee: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the nftasset module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// class_definitions keep the non-fungible token class definitions state
    #[prost(message, repeated, tag = "2")]
    pub class_definitions: ::prost::alloc::vec::Vec<ClassDefinition>,
    #[prost(message, repeated, tag = "3")]
    pub frozen_nfts: ::prost::alloc::vec::Vec<FrozenNft>,
    #[prost(message, repeated, tag = "4")]
    pub whitelisted_nft_accounts: ::prost::alloc::vec::Vec<WhitelistedNftAccounts>,
    #[prost(message, repeated, tag = "5")]
    pub burnt_nfts: ::prost::alloc::vec::Vec<BurntNft>,
    #[prost(message, repeated, tag = "6")]
    pub class_whitelisted_accounts: ::prost::alloc::vec::Vec<ClassWhitelistedAccounts>,
    #[prost(message, repeated, tag = "7")]
    pub class_frozen_accounts: ::prost::alloc::vec::Vec<ClassFrozenAccounts>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.FrozenNFT")]
pub struct FrozenNft {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub nft_i_ds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.WhitelistedNFTAccounts")]
pub struct WhitelistedNftAccounts {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub nft_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.ClassWhitelistedAccounts")]
pub struct ClassWhitelistedAccounts {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.ClassFrozenAccounts")]
pub struct ClassFrozenAccounts {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.BurntNFT")]
pub struct BurntNft {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub nft_i_ds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryParamsRequest defines the request type for querying x/asset/nft parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryParamsRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse defines the response type for querying x/asset/nft parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryTokenRequest is request type for the Query/Class RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryClassRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/Class",
    response_type = QueryClassResponse
)]
pub struct QueryClassRequest {
    /// we don't use the gogoproto.customname here since the google.api.http ignores it and generates invalid code.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryClassResponse is response type for the Query/Class RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryClassResponse")]
pub struct QueryClassResponse {
    #[prost(message, optional, tag = "1")]
    pub class: ::core::option::Option<Class>,
}
/// QueryTokenRequest is request type for the Query/Classes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryClassesRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/Classes",
    response_type = QueryClassesResponse
)]
pub struct QueryClassesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    #[prost(string, tag = "2")]
    pub issuer: ::prost::alloc::string::String,
}
/// QueryClassResponse is response type for the Query/Classes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryClassesResponse")]
pub struct QueryClassesResponse {
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    #[prost(message, repeated, tag = "2")]
    pub classes: ::prost::alloc::vec::Vec<Class>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryFrozenRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/Frozen",
    response_type = QueryFrozenResponse
)]
pub struct QueryFrozenRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryFrozenResponse")]
pub struct QueryFrozenResponse {
    #[prost(bool, tag = "1")]
    pub frozen: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryClassFrozenRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/ClassFrozen",
    response_type = QueryClassFrozenResponse
)]
pub struct QueryClassFrozenRequest {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryClassFrozenResponse")]
pub struct QueryClassFrozenResponse {
    #[prost(bool, tag = "1")]
    pub frozen: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryWhitelistedRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/Whitelisted",
    response_type = QueryWhitelistedResponse
)]
pub struct QueryWhitelistedRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryWhitelistedResponse")]
pub struct QueryWhitelistedResponse {
    #[prost(bool, tag = "1")]
    pub whitelisted: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryWhitelistedAccountsForNFTRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/WhitelistedAccountsForNFT",
    response_type = QueryWhitelistedAccountsForNftResponse
)]
pub struct QueryWhitelistedAccountsForNftRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub class_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryWhitelistedAccountsForNFTResponse")]
pub struct QueryWhitelistedAccountsForNftResponse {
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    #[prost(string, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryClassWhitelistedAccountsRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/ClassWhitelistedAccounts",
    response_type = QueryClassWhitelistedAccountsResponse
)]
pub struct QueryClassWhitelistedAccountsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryClassWhitelistedAccountsResponse")]
pub struct QueryClassWhitelistedAccountsResponse {
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    #[prost(string, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryClassFrozenAccountsRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/ClassFrozenAccounts",
    response_type = QueryClassFrozenAccountsResponse
)]
pub struct QueryClassFrozenAccountsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryClassFrozenAccountsResponse")]
pub struct QueryClassFrozenAccountsResponse {
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    #[prost(string, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryBurntNFTRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/BurntNFT",
    response_type = QueryBurntNftResponse
)]
pub struct QueryBurntNftRequest {
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub nft_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryBurntNFTResponse")]
pub struct QueryBurntNftResponse {
    #[prost(bool, tag = "1")]
    pub burnt: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryBurntNFTsInClassRequest")]
#[proto_query(
    path = "/coreum.asset.nft.v1.Query/BurntNFTsInClass",
    response_type = QueryBurntNfTsInClassResponse
)]
pub struct QueryBurntNfTsInClassRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.QueryBurntNFTsInClassResponse")]
pub struct QueryBurntNfTsInClassResponse {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    #[prost(string, repeated, tag = "2")]
    pub nft_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// DataBytes represents the immutable data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.DataBytes")]
pub struct DataBytes {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// DataDynamicItem contains the updatable data and modification types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.DataDynamicItem")]
pub struct DataDynamicItem {
    /// contains the set of the data editors, if empty no one can update.
    #[prost(enumeration = "DataEditor", repeated, tag = "1")]
    pub editors: ::prost::alloc::vec::Vec<i32>,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// DataDynamicIndexed contains the data and it's index in the DataDynamic.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.DataDynamicIndexedItem")]
pub struct DataDynamicIndexedItem {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// DataDynamic is dynamic data which contains the list of the items allowed to be modified base on their modification types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.DataDynamic")]
pub struct DataDynamic {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<DataDynamicItem>,
}
/// DataEditor defines possible data editors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum DataEditor {
    Admin = 0,
    Owner = 1,
}
impl DataEditor {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataEditor::Admin => "admin",
            DataEditor::Owner => "owner",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "admin" => Some(Self::Admin),
            "owner" => Some(Self::Owner),
            _ => None,
        }
    }
}
/// MsgIssueClass defines message for the IssueClass method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgIssueClass")]
pub struct MsgIssueClass {
    #[prost(string, tag = "1")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub uri_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub data: ::core::option::Option<crate::shim::Any>,
    #[prost(enumeration = "ClassFeature", repeated, tag = "8")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, tag = "9")]
    pub royalty_rate: ::prost::alloc::string::String,
}
/// MsgMint defines message for the Mint method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgMint")]
pub struct MsgMint {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub uri_hash: ::prost::alloc::string::String,
    /// Data can be DataBytes or DataDynamic.
    #[prost(message, optional, tag = "6")]
    pub data: ::core::option::Option<crate::shim::Any>,
    #[prost(string, tag = "7")]
    pub recipient: ::prost::alloc::string::String,
}
/// MsgUpdateData defines message to update the dynamic data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgUpdateData")]
pub struct MsgUpdateData {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub items: ::prost::alloc::vec::Vec<DataDynamicIndexedItem>,
}
/// MsgBurn defines message for the Burn method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgBurn")]
pub struct MsgBurn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgFreeze")]
pub struct MsgFreeze {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgUnfreeze")]
pub struct MsgUnfreeze {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgClassFreeze")]
pub struct MsgClassFreeze {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgClassUnfreeze")]
pub struct MsgClassUnfreeze {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgAddToWhitelist")]
pub struct MsgAddToWhitelist {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgRemoveFromWhitelist")]
pub struct MsgRemoveFromWhitelist {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgAddToClassWhitelist")]
pub struct MsgAddToClassWhitelist {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgRemoveFromClassWhitelist")]
pub struct MsgRemoveFromClassWhitelist {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/coreum.asset.nft.v1.EmptyResponse")]
pub struct EmptyResponse {}
