// This file is generated by rust-protobuf 2.23.0. Do not edit
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
//! Generated file from `google/api/log.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_23_0;

#[derive(PartialEq,Clone,Default)]
pub struct LogDescriptor {
    // message fields
    pub name: ::std::string::String,
    pub labels: ::protobuf::RepeatedField<super::label::LabelDescriptor>,
    pub description: ::std::string::String,
    pub display_name: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a LogDescriptor {
    fn default() -> &'a LogDescriptor {
        <LogDescriptor as ::protobuf::Message>::default_instance()
    }
}

impl LogDescriptor {
    pub fn new() -> LogDescriptor {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // repeated .google.api.LabelDescriptor labels = 2;


    pub fn get_labels(&self) -> &[super::label::LabelDescriptor] {
        &self.labels
    }
    pub fn clear_labels(&mut self) {
        self.labels.clear();
    }

    // Param is passed by value, moved
    pub fn set_labels(&mut self, v: ::protobuf::RepeatedField<super::label::LabelDescriptor>) {
        self.labels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_labels(&mut self) -> &mut ::protobuf::RepeatedField<super::label::LabelDescriptor> {
        &mut self.labels
    }

    // Take field
    pub fn take_labels(&mut self) -> ::protobuf::RepeatedField<super::label::LabelDescriptor> {
        ::std::mem::replace(&mut self.labels, ::protobuf::RepeatedField::new())
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

    // string display_name = 4;


    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }
    pub fn clear_display_name(&mut self) {
        self.display_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_display_name(&mut self, v: ::std::string::String) {
        self.display_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_display_name(&mut self) -> &mut ::std::string::String {
        &mut self.display_name
    }

    // Take field
    pub fn take_display_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.display_name, ::std::string::String::new())
    }
}

impl ::protobuf::Message for LogDescriptor {
    fn is_initialized(&self) -> bool {
        for v in &self.labels {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.labels)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.display_name)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        for value in &self.labels {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.description);
        }
        if !self.display_name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.display_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.labels {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.description.is_empty() {
            os.write_string(3, &self.description)?;
        }
        if !self.display_name.is_empty() {
            os.write_string(4, &self.display_name)?;
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

    fn new() -> LogDescriptor {
        LogDescriptor::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &LogDescriptor| { &m.name },
                |m: &mut LogDescriptor| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::label::LabelDescriptor>>(
                "labels",
                |m: &LogDescriptor| { &m.labels },
                |m: &mut LogDescriptor| { &mut m.labels },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "description",
                |m: &LogDescriptor| { &m.description },
                |m: &mut LogDescriptor| { &mut m.description },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "display_name",
                |m: &LogDescriptor| { &m.display_name },
                |m: &mut LogDescriptor| { &mut m.display_name },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<LogDescriptor>(
                "LogDescriptor",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static LogDescriptor {
        static instance: ::protobuf::rt::LazyV2<LogDescriptor> = ::protobuf::rt::LazyV2::INIT;
        instance.get(LogDescriptor::new)
    }
}

impl ::protobuf::Clear for LogDescriptor {
    fn clear(&mut self) {
        self.name.clear();
        self.labels.clear();
        self.description.clear();
        self.display_name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LogDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogDescriptor {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14google/api/log.proto\x12\ngoogle.api\x1a\x16google/api/label.proto\
    \"\x9d\x01\n\rLogDescriptor\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04nam\
    e\x123\n\x06labels\x18\x02\x20\x03(\x0b2\x1b.google.api.LabelDescriptorR\
    \x06labels\x12\x20\n\x0bdescription\x18\x03\x20\x01(\tR\x0bdescription\
    \x12!\n\x0cdisplay_name\x18\x04\x20\x01(\tR\x0bdisplayNameBj\n\x0ecom.go\
    ogle.apiB\x08LogProtoP\x01ZEgoogle.golang.org/genproto/googleapis/api/se\
    rviceconfig;serviceconfig\xa2\x02\x04GAPIJ\xad\x0f\n\x06\x12\x04\x0f\06\
    \x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Copyright\x202019\
    \x20Google\x20LLC.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\
    \x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20us\
    e\x20this\x20file\x20except\x20in\x20compliance\x20with\x20the\x20Licens\
    e.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\
    \n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\
    \x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\
    \x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\x20Licen\
    se\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHO\
    UT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20\
    express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20sp\
    ecific\x20language\x20governing\x20permissions\x20and\n\x20limitations\
    \x20under\x20the\x20License.\n\n\n\x08\n\x01\x02\x12\x03\x11\0\x13\n\t\n\
    \x02\x03\0\x12\x03\x13\0\x20\n\x08\n\x01\x08\x12\x03\x15\0\\\n\t\n\x02\
    \x08\x0b\x12\x03\x15\0\\\n\x08\n\x01\x08\x12\x03\x16\0\"\n\t\n\x02\x08\n\
    \x12\x03\x16\0\"\n\x08\n\x01\x08\x12\x03\x17\0)\n\t\n\x02\x08\x08\x12\
    \x03\x17\0)\n\x08\n\x01\x08\x12\x03\x18\0'\n\t\n\x02\x08\x01\x12\x03\x18\
    \0'\n\x08\n\x01\x08\x12\x03\x19\0\"\n\t\n\x02\x08$\x12\x03\x19\0\"\n\xc2\
    \x02\n\x02\x04\0\x12\x04#\06\x01\x1a\xb5\x02\x20A\x20description\x20of\
    \x20a\x20log\x20type.\x20Example\x20in\x20YAML\x20format:\n\n\x20\x20\
    \x20\x20\x20-\x20name:\x20library.googleapis.com/activity_history\n\x20\
    \x20\x20\x20\x20\x20\x20description:\x20The\x20history\x20of\x20borrowin\
    g\x20and\x20returning\x20library\x20items.\n\x20\x20\x20\x20\x20\x20\x20\
    display_name:\x20Activity\n\x20\x20\x20\x20\x20\x20\x20labels:\n\x20\x20\
    \x20\x20\x20\x20\x20-\x20key:\x20/customer_id\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20description:\x20Identifier\x20of\x20a\x20library\x20customer\
    \n\n\n\n\x03\x04\0\x01\x12\x03#\x08\x15\n\x84\x02\n\x04\x04\0\x02\0\x12\
    \x03(\x02\x12\x1a\xf6\x01\x20The\x20name\x20of\x20the\x20log.\x20It\x20m\
    ust\x20be\x20less\x20than\x20512\x20characters\x20long\x20and\x20can\n\
    \x20include\x20the\x20following\x20characters:\x20upper-\x20and\x20lower\
    -case\x20alphanumeric\n\x20characters\x20[A-Za-z0-9],\x20and\x20punctuat\
    ion\x20characters\x20including\n\x20slash,\x20underscore,\x20hyphen,\x20\
    period\x20[/_-.].\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04(\x02#\x17\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03(\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03(\t\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03(\x10\x11\n\xa8\x01\n\x04\
    \x04\0\x02\x01\x12\x03-\x02&\x1a\x9a\x01\x20The\x20set\x20of\x20labels\
    \x20that\x20are\x20available\x20to\x20describe\x20a\x20specific\x20log\
    \x20entry.\n\x20Runtime\x20requests\x20that\x20contain\x20labels\x20not\
    \x20specified\x20here\x20are\n\x20considered\x20invalid.\n\n\x0c\n\x05\
    \x04\0\x02\x01\x04\x12\x03-\x02\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03-\
    \x0b\x1a\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03-\x1b!\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03-$%\n\x80\x01\n\x04\x04\0\x02\x02\x12\x031\x02\x19\
    \x1as\x20A\x20human-readable\x20description\x20of\x20this\x20log.\x20Thi\
    s\x20information\x20appears\x20in\n\x20the\x20documentation\x20and\x20ca\
    n\x20contain\x20details.\n\n\r\n\x05\x04\0\x02\x02\x04\x12\x041\x02-&\n\
    \x0c\n\x05\x04\0\x02\x02\x05\x12\x031\x02\x08\n\x0c\n\x05\x04\0\x02\x02\
    \x01\x12\x031\t\x14\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x031\x17\x18\n{\n\
    \x04\x04\0\x02\x03\x12\x035\x02\x1a\x1an\x20The\x20human-readable\x20nam\
    e\x20for\x20this\x20log.\x20This\x20information\x20appears\x20on\n\x20th\
    e\x20user\x20interface\x20and\x20should\x20be\x20concise.\n\n\r\n\x05\
    \x04\0\x02\x03\x04\x12\x045\x021\x19\n\x0c\n\x05\x04\0\x02\x03\x05\x12\
    \x035\x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x035\t\x15\n\x0c\n\x05\
    \x04\0\x02\x03\x03\x12\x035\x18\x19b\x06proto3\
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
