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
//! Generated file from `google/api/system_parameter.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_1;

#[derive(PartialEq,Clone,Default)]
pub struct SystemParameters {
    // message fields
    pub rules: ::protobuf::RepeatedField<SystemParameterRule>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SystemParameters {
    fn default() -> &'a SystemParameters {
        <SystemParameters as ::protobuf::Message>::default_instance()
    }
}

impl SystemParameters {
    pub fn new() -> SystemParameters {
        ::std::default::Default::default()
    }

    // repeated .google.api.SystemParameterRule rules = 1;


    pub fn get_rules(&self) -> &[SystemParameterRule] {
        &self.rules
    }
    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    // Param is passed by value, moved
    pub fn set_rules(&mut self, v: ::protobuf::RepeatedField<SystemParameterRule>) {
        self.rules = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rules(&mut self) -> &mut ::protobuf::RepeatedField<SystemParameterRule> {
        &mut self.rules
    }

    // Take field
    pub fn take_rules(&mut self) -> ::protobuf::RepeatedField<SystemParameterRule> {
        ::std::mem::replace(&mut self.rules, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for SystemParameters {
    fn is_initialized(&self) -> bool {
        for v in &self.rules {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rules)?;
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
        for value in &self.rules {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.rules {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> SystemParameters {
        SystemParameters::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SystemParameterRule>>(
                "rules",
                |m: &SystemParameters| { &m.rules },
                |m: &mut SystemParameters| { &mut m.rules },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SystemParameters>(
                "SystemParameters",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SystemParameters {
        static instance: ::protobuf::rt::LazyV2<SystemParameters> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SystemParameters::new)
    }
}

impl ::protobuf::Clear for SystemParameters {
    fn clear(&mut self) {
        self.rules.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SystemParameters {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SystemParameters {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SystemParameterRule {
    // message fields
    pub selector: ::std::string::String,
    pub parameters: ::protobuf::RepeatedField<SystemParameter>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SystemParameterRule {
    fn default() -> &'a SystemParameterRule {
        <SystemParameterRule as ::protobuf::Message>::default_instance()
    }
}

impl SystemParameterRule {
    pub fn new() -> SystemParameterRule {
        ::std::default::Default::default()
    }

    // string selector = 1;


    pub fn get_selector(&self) -> &str {
        &self.selector
    }
    pub fn clear_selector(&mut self) {
        self.selector.clear();
    }

    // Param is passed by value, moved
    pub fn set_selector(&mut self, v: ::std::string::String) {
        self.selector = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selector(&mut self) -> &mut ::std::string::String {
        &mut self.selector
    }

    // Take field
    pub fn take_selector(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.selector, ::std::string::String::new())
    }

    // repeated .google.api.SystemParameter parameters = 2;


    pub fn get_parameters(&self) -> &[SystemParameter] {
        &self.parameters
    }
    pub fn clear_parameters(&mut self) {
        self.parameters.clear();
    }

    // Param is passed by value, moved
    pub fn set_parameters(&mut self, v: ::protobuf::RepeatedField<SystemParameter>) {
        self.parameters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_parameters(&mut self) -> &mut ::protobuf::RepeatedField<SystemParameter> {
        &mut self.parameters
    }

    // Take field
    pub fn take_parameters(&mut self) -> ::protobuf::RepeatedField<SystemParameter> {
        ::std::mem::replace(&mut self.parameters, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for SystemParameterRule {
    fn is_initialized(&self) -> bool {
        for v in &self.parameters {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.selector)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.parameters)?;
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
        if !self.selector.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.selector);
        }
        for value in &self.parameters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.selector.is_empty() {
            os.write_string(1, &self.selector)?;
        }
        for v in &self.parameters {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> SystemParameterRule {
        SystemParameterRule::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "selector",
                |m: &SystemParameterRule| { &m.selector },
                |m: &mut SystemParameterRule| { &mut m.selector },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SystemParameter>>(
                "parameters",
                |m: &SystemParameterRule| { &m.parameters },
                |m: &mut SystemParameterRule| { &mut m.parameters },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SystemParameterRule>(
                "SystemParameterRule",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SystemParameterRule {
        static instance: ::protobuf::rt::LazyV2<SystemParameterRule> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SystemParameterRule::new)
    }
}

impl ::protobuf::Clear for SystemParameterRule {
    fn clear(&mut self) {
        self.selector.clear();
        self.parameters.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SystemParameterRule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SystemParameterRule {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SystemParameter {
    // message fields
    pub name: ::std::string::String,
    pub http_header: ::std::string::String,
    pub url_query_parameter: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SystemParameter {
    fn default() -> &'a SystemParameter {
        <SystemParameter as ::protobuf::Message>::default_instance()
    }
}

impl SystemParameter {
    pub fn new() -> SystemParameter {
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

    // string http_header = 2;


    pub fn get_http_header(&self) -> &str {
        &self.http_header
    }
    pub fn clear_http_header(&mut self) {
        self.http_header.clear();
    }

    // Param is passed by value, moved
    pub fn set_http_header(&mut self, v: ::std::string::String) {
        self.http_header = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http_header(&mut self) -> &mut ::std::string::String {
        &mut self.http_header
    }

    // Take field
    pub fn take_http_header(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.http_header, ::std::string::String::new())
    }

    // string url_query_parameter = 3;


    pub fn get_url_query_parameter(&self) -> &str {
        &self.url_query_parameter
    }
    pub fn clear_url_query_parameter(&mut self) {
        self.url_query_parameter.clear();
    }

    // Param is passed by value, moved
    pub fn set_url_query_parameter(&mut self, v: ::std::string::String) {
        self.url_query_parameter = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url_query_parameter(&mut self) -> &mut ::std::string::String {
        &mut self.url_query_parameter
    }

    // Take field
    pub fn take_url_query_parameter(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.url_query_parameter, ::std::string::String::new())
    }
}

impl ::protobuf::Message for SystemParameter {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.http_header)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.url_query_parameter)?;
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
        if !self.http_header.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.http_header);
        }
        if !self.url_query_parameter.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.url_query_parameter);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.http_header.is_empty() {
            os.write_string(2, &self.http_header)?;
        }
        if !self.url_query_parameter.is_empty() {
            os.write_string(3, &self.url_query_parameter)?;
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

    fn new() -> SystemParameter {
        SystemParameter::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &SystemParameter| { &m.name },
                |m: &mut SystemParameter| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "http_header",
                |m: &SystemParameter| { &m.http_header },
                |m: &mut SystemParameter| { &mut m.http_header },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "url_query_parameter",
                |m: &SystemParameter| { &m.url_query_parameter },
                |m: &mut SystemParameter| { &mut m.url_query_parameter },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SystemParameter>(
                "SystemParameter",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SystemParameter {
        static instance: ::protobuf::rt::LazyV2<SystemParameter> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SystemParameter::new)
    }
}

impl ::protobuf::Clear for SystemParameter {
    fn clear(&mut self) {
        self.name.clear();
        self.http_header.clear();
        self.url_query_parameter.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SystemParameter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SystemParameter {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!google/api/system_parameter.proto\x12\ngoogle.api\"I\n\x10SystemParam\
    eters\x125\n\x05rules\x18\x01\x20\x03(\x0b2\x1f.google.api.SystemParamet\
    erRuleR\x05rules\"n\n\x13SystemParameterRule\x12\x1a\n\x08selector\x18\
    \x01\x20\x01(\tR\x08selector\x12;\n\nparameters\x18\x02\x20\x03(\x0b2\
    \x1b.google.api.SystemParameterR\nparameters\"v\n\x0fSystemParameter\x12\
    \x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x1f\n\x0bhttp_header\x18\
    \x02\x20\x01(\tR\nhttpHeader\x12.\n\x13url_query_parameter\x18\x03\x20\
    \x01(\tR\x11urlQueryParameterBv\n\x0ecom.google.apiB\x14SystemParameterP\
    rotoP\x01ZEgoogle.golang.org/genproto/googleapis/api/serviceconfig;servi\
    ceconfig\xa2\x02\x04GAPIJ\xc2\x19\n\x06\x12\x04\x0f\0_\x01\n\xbe\x04\n\
    \x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Copyright\x202019\x20Google\x20LL\
    C.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Version\x202\
    .0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\x20fil\
    e\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20m\
    ay\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\
    \x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20requ\
    ired\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\
    \x20software\n\x20distributed\x20under\x20the\x20License\x20is\x20distri\
    buted\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\
    \x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\
    \x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20lan\
    guage\x20governing\x20permissions\x20and\n\x20limitations\x20under\x20th\
    e\x20License.\n\n\n\x08\n\x01\x02\x12\x03\x11\0\x13\n\x08\n\x01\x08\x12\
    \x03\x13\0\\\n\t\n\x02\x08\x0b\x12\x03\x13\0\\\n\x08\n\x01\x08\x12\x03\
    \x14\0\"\n\t\n\x02\x08\n\x12\x03\x14\0\"\n\x08\n\x01\x08\x12\x03\x15\05\
    \n\t\n\x02\x08\x08\x12\x03\x15\05\n\x08\n\x01\x08\x12\x03\x16\0'\n\t\n\
    \x02\x08\x01\x12\x03\x16\0'\n\x08\n\x01\x08\x12\x03\x17\0\"\n\t\n\x02\
    \x08$\x12\x03\x17\0\"\n\xba\x02\n\x02\x04\0\x12\x04\x1f\0>\x01\x1a\xad\
    \x02\x20###\x20System\x20parameter\x20configuration\n\n\x20A\x20system\
    \x20parameter\x20is\x20a\x20special\x20kind\x20of\x20parameter\x20define\
    d\x20by\x20the\x20API\n\x20system,\x20not\x20by\x20an\x20individual\x20A\
    PI.\x20It\x20is\x20typically\x20mapped\x20to\x20an\x20HTTP\x20header\n\
    \x20and/or\x20a\x20URL\x20query\x20parameter.\x20This\x20configuration\
    \x20specifies\x20which\x20methods\n\x20change\x20the\x20names\x20of\x20t\
    he\x20system\x20parameters.\n\n\n\n\x03\x04\0\x01\x12\x03\x1f\x08\x18\n\
    \xd7\x06\n\x04\x04\0\x02\0\x12\x03=\x02)\x1a\xc9\x06\x20Define\x20system\
    \x20parameters.\n\n\x20The\x20parameters\x20defined\x20here\x20will\x20o\
    verride\x20the\x20default\x20parameters\n\x20implemented\x20by\x20the\
    \x20system.\x20If\x20this\x20field\x20is\x20missing\x20from\x20the\x20se\
    rvice\n\x20config,\x20default\x20system\x20parameters\x20will\x20be\x20u\
    sed.\x20Default\x20system\x20parameters\n\x20and\x20names\x20is\x20imple\
    mentation-dependent.\n\n\x20Example:\x20define\x20api\x20key\x20for\x20a\
    ll\x20methods\n\n\x20\x20\x20\x20\x20system_parameters\n\x20\x20\x20\x20\
    \x20\x20\x20rules:\n\x20\x20\x20\x20\x20\x20\x20\x20\x20-\x20selector:\
    \x20\"*\"\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20parameters:\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20-\x20name:\x20api_key\n\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20url_query_pa\
    rameter:\x20api_key\n\n\n\x20Example:\x20define\x202\x20api\x20key\x20na\
    mes\x20for\x20a\x20specific\x20method.\n\n\x20\x20\x20\x20\x20system_par\
    ameters\n\x20\x20\x20\x20\x20\x20\x20rules:\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20-\x20selector:\x20\"/ListShelves\"\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20parameters:\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20-\x20name:\x20api_key\n\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20http_header:\x20Api-Key1\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20-\x20name:\x20api_key\n\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20http_header:\x20Api-Key2\
    \n\n\x20**NOTE:**\x20All\x20service\x20configuration\x20rules\x20follow\
    \x20\"last\x20one\x20wins\"\x20order.\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\
    \x03=\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03=\x0b\x1e\n\x0c\n\x05\x04\
    \0\x02\0\x01\x12\x03=\x1f$\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03='(\n^\n\
    \x02\x04\x01\x12\x04B\0O\x01\x1aR\x20Define\x20a\x20system\x20parameter\
    \x20rule\x20mapping\x20system\x20parameter\x20definitions\x20to\n\x20met\
    hods.\n\n\n\n\x03\x04\x01\x01\x12\x03B\x08\x1b\n\xbe\x01\n\x04\x04\x01\
    \x02\0\x12\x03G\x02\x16\x1a\xb0\x01\x20Selects\x20the\x20methods\x20to\
    \x20which\x20this\x20rule\x20applies.\x20Use\x20'*'\x20to\x20indicate\
    \x20all\n\x20methods\x20in\x20all\x20APIs.\n\n\x20Refer\x20to\x20[select\
    or][google.api.DocumentationRule.selector]\x20for\x20syntax\x20details.\
    \n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03G\x02\x08\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03G\t\x11\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03G\x14\x15\
    \n\xa4\x02\n\x04\x04\x01\x02\x01\x12\x03N\x02*\x1a\x96\x02\x20Define\x20\
    parameters.\x20Multiple\x20names\x20may\x20be\x20defined\x20for\x20a\x20\
    parameter.\n\x20For\x20a\x20given\x20method\x20call,\x20only\x20one\x20o\
    f\x20them\x20should\x20be\x20used.\x20If\x20multiple\n\x20names\x20are\
    \x20used\x20the\x20behavior\x20is\x20implementation-dependent.\n\x20If\
    \x20none\x20of\x20the\x20specified\x20names\x20are\x20present\x20the\x20\
    behavior\x20is\n\x20parameter-dependent.\n\n\x0c\n\x05\x04\x01\x02\x01\
    \x04\x12\x03N\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03N\x0b\x1a\n\
    \x0c\n\x05\x04\x01\x02\x01\x01\x12\x03N\x1b%\n\x0c\n\x05\x04\x01\x02\x01\
    \x03\x12\x03N()\n\xc8\x01\n\x02\x04\x02\x12\x04T\0_\x01\x1a\xbb\x01\x20D\
    efine\x20a\x20parameter's\x20name\x20and\x20location.\x20The\x20paramete\
    r\x20may\x20be\x20passed\x20as\x20either\n\x20an\x20HTTP\x20header\x20or\
    \x20a\x20URL\x20query\x20parameter,\x20and\x20if\x20both\x20are\x20passe\
    d\x20the\x20behavior\n\x20is\x20implementation-dependent.\n\n\n\n\x03\
    \x04\x02\x01\x12\x03T\x08\x17\nZ\n\x04\x04\x02\x02\0\x12\x03V\x02\x12\
    \x1aM\x20Define\x20the\x20name\x20of\x20the\x20parameter,\x20such\x20as\
    \x20\"api_key\"\x20.\x20It\x20is\x20case\x20sensitive.\n\n\x0c\n\x05\x04\
    \x02\x02\0\x05\x12\x03V\x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03V\t\
    \r\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03V\x10\x11\n]\n\x04\x04\x02\x02\
    \x01\x12\x03Z\x02\x19\x1aP\x20Define\x20the\x20HTTP\x20header\x20name\
    \x20to\x20use\x20for\x20the\x20parameter.\x20It\x20is\x20case\n\x20insen\
    sitive.\n\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03Z\x02\x08\n\x0c\n\x05\
    \x04\x02\x02\x01\x01\x12\x03Z\t\x14\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03Z\x17\x18\nc\n\x04\x04\x02\x02\x02\x12\x03^\x02!\x1aV\x20Define\x20t\
    he\x20URL\x20query\x20parameter\x20name\x20to\x20use\x20for\x20the\x20pa\
    rameter.\x20It\x20is\x20case\n\x20sensitive.\n\n\x0c\n\x05\x04\x02\x02\
    \x02\x05\x12\x03^\x02\x08\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03^\t\x1c\
    \n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03^\x1f\x20b\x06proto3\
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
