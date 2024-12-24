#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomData {
    #[prost(message, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<room_data::RoomMember>,
    #[prost(int32, optional, tag = "2")]
    pub field_2: ::core::option::Option<i32>,
    #[prost(int32, tag = "3")]
    pub field_3: i32,
    #[prost(int32, optional, tag = "4")]
    pub field_4: ::core::option::Option<i32>,
    #[prost(int32, tag = "5")]
    pub room_capacity: i32,
    #[prost(string, optional, tag = "6")]
    pub field_6: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, tag = "7")]
    pub field_7: i64,
    #[prost(int64, tag = "8")]
    pub field_8: i64,
}
/// Nested message and enum types in `RoomData`.
pub mod room_data {
    #[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoomMember {
        #[prost(string, tag = "1")]
        pub wxid: ::prost::alloc::string::String,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, tag = "3")]
        pub state: i32,
    }
}
