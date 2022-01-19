// This file is generated by rust-protobuf 2.25.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/api/annotations.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

/// Extension fields
pub mod exts {

    pub const http: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MethodOptions, ::protobuf::types::ProtobufTypeMessage<super::super::http::HttpRule>> = ::protobuf::ext::ExtFieldOptional { field_number: 72295728, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cgoogle/api/annotations.proto\x12\ngoogle.api\x1a\x15google/api/htt\
    p.proto\x1a\x20google/protobuf/descriptor.proto:K\n\x04http\x18\xb0\xca\
    \xbc\"\x20\x01(\x0b2\x14.google.api.HttpRule\x12\x1e.google.protobuf.Met\
    hodOptionsR\x04httpBn\n\x0ecom.google.apiB\x10AnnotationsProtoP\x01ZAgoo\
    gle.golang.org/genproto/googleapis/api/annotations;annotations\xa2\x02\
    \x04GAPIJ\xbc\x06\n\x06\x12\x04\x0e\0\x1e\x01\n\xc2\x04\n\x01\x0c\x12\
    \x03\x0e\0\x122\xb7\x04\x20Copyright\x20(c)\x202015,\x20Google\x20Inc.\n\
    \n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\
    \x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\x20file\
    \x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20ma\
    y\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\
    \x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20requ\
    ired\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\
    \x20software\n\x20distributed\x20under\x20the\x20License\x20is\x20distri\
    buted\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\
    \x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\
    \x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20lan\
    guage\x20governing\x20permissions\x20and\n\x20limitations\x20under\x20th\
    e\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\x08\x12\n\t\n\x02\x03\0\x12\
    \x03\x12\x07\x1e\n\t\n\x02\x03\x01\x12\x03\x13\x07)\n\x08\n\x01\x08\x12\
    \x03\x15\0X\n\t\n\x02\x08\x0b\x12\x03\x15\0X\n\x08\n\x01\x08\x12\x03\x16\
    \0\"\n\t\n\x02\x08\n\x12\x03\x16\0\"\n\x08\n\x01\x08\x12\x03\x17\01\n\t\
    \n\x02\x08\x08\x12\x03\x17\01\n\x08\n\x01\x08\x12\x03\x18\0'\n\t\n\x02\
    \x08\x01\x12\x03\x18\0'\n\x08\n\x01\x08\x12\x03\x19\0\"\n\t\n\x02\x08$\
    \x12\x03\x19\0\"\n\t\n\x01\x07\x12\x04\x1b\0\x1e\x01\n\x1c\n\x02\x07\0\
    \x12\x03\x1d\x02\x1b\x1a\x11\x20See\x20`HttpRule`.\n\n\n\n\x03\x07\0\x02\
    \x12\x03\x1b\x07$\n\x0b\n\x03\x07\0\x04\x12\x04\x1d\x02\x1b&\n\n\n\x03\
    \x07\0\x06\x12\x03\x1d\x02\n\n\n\n\x03\x07\0\x01\x12\x03\x1d\x0b\x0f\n\n\
    \n\x03\x07\0\x03\x12\x03\x1d\x12\x1ab\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
