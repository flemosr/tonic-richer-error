// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DayInfoReq {
    #[prost(string, tag="1")]
    pub day_code: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DayInfoRes {
    #[prost(string, tag="1")]
    pub activity: ::prost::alloc::string::String,
}
include!("schedule.tonic.rs");
// @@protoc_insertion_point(module)
