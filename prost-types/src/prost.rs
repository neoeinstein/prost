pub const PACKAGE: &::prost::ExtensionImpl<ProstPackage> = &::prost::ExtensionImpl::<ProstPackage> { extendable_type_id: ".google.protobuf.FileOptions", field_tag: 2001, proto_int_type: ::prost::ProtoIntType::Default, _phantom: ::core::marker::PhantomData{}};
pub const TYPE: &::prost::ExtensionImpl<ProstMessage> = &::prost::ExtensionImpl::<ProstMessage> { extendable_type_id: ".google.protobuf.MessageOptions", field_tag: 2001, proto_int_type: ::prost::ProtoIntType::Default, _phantom: ::core::marker::PhantomData{}};
pub const FIELD: &::prost::ExtensionImpl<ProstField> = &::prost::ExtensionImpl::<ProstField> { extendable_type_id: ".google.protobuf.FieldOptions", field_tag: 2001, proto_int_type: ::prost::ProtoIntType::Default, _phantom: ::core::marker::PhantomData{}};
pub fn register_extensions(registry: &mut ::prost::ExtensionRegistry) {
    registry.register(PACKAGE);
    registry.register(TYPE);
    registry.register(FIELD);
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProstPackage {
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProstMessage {
    #[prost(string, optional, tag="1")]
    pub type_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub attrs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub external: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="4")]
    pub gen_borrowed: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProstField {
    #[prost(string, optional, tag="1")]
    pub field_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub borrowed_type: ::core::option::Option<::prost::alloc::string::String>,
}
