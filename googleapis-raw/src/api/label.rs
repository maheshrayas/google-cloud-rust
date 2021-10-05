// This file is generated by rust-protobuf 2.25.1. Do not edit
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
//! Generated file from `google/api/label.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct LabelDescriptor {
    // message fields
    pub key: ::std::string::String,
    pub value_type: LabelDescriptor_ValueType,
    pub description: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a LabelDescriptor {
    fn default() -> &'a LabelDescriptor {
        <LabelDescriptor as ::protobuf::Message>::default_instance()
    }
}

impl LabelDescriptor {
    pub fn new() -> LabelDescriptor {
        ::std::default::Default::default()
    }

    // string key = 1;


    pub fn get_key(&self) -> &str {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    // .google.api.LabelDescriptor.ValueType value_type = 2;


    pub fn get_value_type(&self) -> LabelDescriptor_ValueType {
        self.value_type
    }
    pub fn clear_value_type(&mut self) {
        self.value_type = LabelDescriptor_ValueType::STRING;
    }

    // Param is passed by value, moved
    pub fn set_value_type(&mut self, v: LabelDescriptor_ValueType) {
        self.value_type = v;
    }

    // string description = 3;


    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }
}

impl ::protobuf::Message for LabelDescriptor {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.value_type, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.key);
        }
        if self.value_type != LabelDescriptor_ValueType::STRING {
            my_size += ::protobuf::rt::enum_size(2, self.value_type);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.description);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_string(1, &self.key)?;
        }
        if self.value_type != LabelDescriptor_ValueType::STRING {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.value_type))?;
        }
        if !self.description.is_empty() {
            os.write_string(3, &self.description)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> LabelDescriptor {
        LabelDescriptor::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "key",
                |m: &LabelDescriptor| { &m.key },
                |m: &mut LabelDescriptor| { &mut m.key },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<LabelDescriptor_ValueType>>(
                "value_type",
                |m: &LabelDescriptor| { &m.value_type },
                |m: &mut LabelDescriptor| { &mut m.value_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "description",
                |m: &LabelDescriptor| { &m.description },
                |m: &mut LabelDescriptor| { &mut m.description },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<LabelDescriptor>(
                "LabelDescriptor",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static LabelDescriptor {
        static instance: ::protobuf::rt::LazyV2<LabelDescriptor> = ::protobuf::rt::LazyV2::INIT;
        instance.get(LabelDescriptor::new)
    }
}

impl ::protobuf::Clear for LabelDescriptor {
    fn clear(&mut self) {
        self.key.clear();
        self.value_type = LabelDescriptor_ValueType::STRING;
        self.description.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LabelDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LabelDescriptor {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum LabelDescriptor_ValueType {
    STRING = 0,
    BOOL = 1,
    INT64 = 2,
}

impl ::protobuf::ProtobufEnum for LabelDescriptor_ValueType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LabelDescriptor_ValueType> {
        match value {
            0 => ::std::option::Option::Some(LabelDescriptor_ValueType::STRING),
            1 => ::std::option::Option::Some(LabelDescriptor_ValueType::BOOL),
            2 => ::std::option::Option::Some(LabelDescriptor_ValueType::INT64),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [LabelDescriptor_ValueType] = &[
            LabelDescriptor_ValueType::STRING,
            LabelDescriptor_ValueType::BOOL,
            LabelDescriptor_ValueType::INT64,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<LabelDescriptor_ValueType>("LabelDescriptor.ValueType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for LabelDescriptor_ValueType {
}

impl ::std::default::Default for LabelDescriptor_ValueType {
    fn default() -> Self {
        LabelDescriptor_ValueType::STRING
    }
}

impl ::protobuf::reflect::ProtobufValue for LabelDescriptor_ValueType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16google/api/label.proto\x12\ngoogle.api\"\xb9\x01\n\x0fLabelDescrip\
    tor\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12D\n\nvalue_type\x18\
    \x02\x20\x01(\x0e2%.google.api.LabelDescriptor.ValueTypeR\tvalueType\x12\
    \x20\n\x0bdescription\x18\x03\x20\x01(\tR\x0bdescription\",\n\tValueType\
    \x12\n\n\x06STRING\x10\0\x12\x08\n\x04BOOL\x10\x01\x12\t\n\x05INT64\x10\
    \x02B_\n\x0ecom.google.apiB\nLabelProtoP\x01Z5google.golang.org/genproto\
    /googleapis/api/label;label\xf8\x01\x01\xa2\x02\x04GAPIJ\xe6\n\n\x06\x12\
    \x04\x0f\00\x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Copyri\
    ght\x202019\x20Google\x20LLC.\n\n\x20Licensed\x20under\x20the\x20Apache\
    \x20License,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\
    \x20not\x20use\x20this\x20file\x20except\x20in\x20compliance\x20with\x20\
    the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20L\
    icense\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICEN\
    SE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agr\
    eed\x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\
    \x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\
    \x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20e\
    ither\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20\
    the\x20specific\x20language\x20governing\x20permissions\x20and\n\x20limi\
    tations\x20under\x20the\x20License.\n\n\n\x08\n\x01\x02\x12\x03\x11\0\
    \x13\n\x08\n\x01\x08\x12\x03\x13\0\x1f\n\t\n\x02\x08\x1f\x12\x03\x13\0\
    \x1f\n\x08\n\x01\x08\x12\x03\x14\0L\n\t\n\x02\x08\x0b\x12\x03\x14\0L\n\
    \x08\n\x01\x08\x12\x03\x15\0\"\n\t\n\x02\x08\n\x12\x03\x15\0\"\n\x08\n\
    \x01\x08\x12\x03\x16\0+\n\t\n\x02\x08\x08\x12\x03\x16\0+\n\x08\n\x01\x08\
    \x12\x03\x17\0'\n\t\n\x02\x08\x01\x12\x03\x17\0'\n\x08\n\x01\x08\x12\x03\
    \x18\0\"\n\t\n\x02\x08$\x12\x03\x18\0\"\n'\n\x02\x04\0\x12\x04\x1b\00\
    \x01\x1a\x1b\x20A\x20description\x20of\x20a\x20label.\n\n\n\n\x03\x04\0\
    \x01\x12\x03\x1b\x08\x17\n=\n\x04\x04\0\x04\0\x12\x04\x1d\x02&\x03\x1a/\
    \x20Value\x20types\x20that\x20can\x20be\x20used\x20as\x20label\x20values\
    .\n\n\x0c\n\x05\x04\0\x04\0\x01\x12\x03\x1d\x07\x10\n?\n\x06\x04\0\x04\0\
    \x02\0\x12\x03\x1f\x04\x0f\x1a0\x20A\x20variable-length\x20string.\x20Th\
    is\x20is\x20the\x20default.\n\n\x0e\n\x07\x04\0\x04\0\x02\0\x01\x12\x03\
    \x1f\x04\n\n\x0e\n\x07\x04\0\x04\0\x02\0\x02\x12\x03\x1f\r\x0e\n(\n\x06\
    \x04\0\x04\0\x02\x01\x12\x03\"\x04\r\x1a\x19\x20Boolean;\x20true\x20or\
    \x20false.\n\n\x0e\n\x07\x04\0\x04\0\x02\x01\x01\x12\x03\"\x04\x08\n\x0e\
    \n\x07\x04\0\x04\0\x02\x01\x02\x12\x03\"\x0b\x0c\n)\n\x06\x04\0\x04\0\
    \x02\x02\x12\x03%\x04\x0e\x1a\x1a\x20A\x2064-bit\x20signed\x20integer.\n\
    \n\x0e\n\x07\x04\0\x04\0\x02\x02\x01\x12\x03%\x04\t\n\x0e\n\x07\x04\0\
    \x04\0\x02\x02\x02\x12\x03%\x0c\r\n\x1d\n\x04\x04\0\x02\0\x12\x03)\x02\
    \x11\x1a\x10\x20The\x20label\x20key.\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x03)\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03)\t\x0c\n\x0c\n\x05\x04\
    \0\x02\0\x03\x12\x03)\x0f\x10\nB\n\x04\x04\0\x02\x01\x12\x03,\x02\x1b\
    \x1a5\x20The\x20type\x20of\x20data\x20that\x20can\x20be\x20assigned\x20t\
    o\x20the\x20label.\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03,\x02\x0b\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03,\x0c\x16\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x03,\x19\x1a\n:\n\x04\x04\0\x02\x02\x12\x03/\x02\x19\x1a-\x20A\
    \x20human-readable\x20description\x20for\x20the\x20label.\n\n\x0c\n\x05\
    \x04\0\x02\x02\x05\x12\x03/\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\
    \x03/\t\x14\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03/\x17\x18b\x06proto3\
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
