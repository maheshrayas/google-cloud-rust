// This file is generated by rust-protobuf 2.22.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/api/servicecontrol/v1/operation.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_0;

#[derive(PartialEq,Clone,Default)]
pub struct Operation {
    // message fields
    pub operation_id: ::std::string::String,
    pub operation_name: ::std::string::String,
    pub consumer_id: ::std::string::String,
    pub start_time: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub end_time: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub metric_value_sets: ::protobuf::RepeatedField<super::metric_value::MetricValueSet>,
    pub log_entries: ::protobuf::RepeatedField<super::log_entry::LogEntry>,
    pub importance: Operation_Importance,
    pub extensions: ::protobuf::RepeatedField<::protobuf::well_known_types::Any>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Operation {
    fn default() -> &'a Operation {
        <Operation as ::protobuf::Message>::default_instance()
    }
}

impl Operation {
    pub fn new() -> Operation {
        ::std::default::Default::default()
    }

    // string operation_id = 1;


    pub fn get_operation_id(&self) -> &str {
        &self.operation_id
    }
    pub fn clear_operation_id(&mut self) {
        self.operation_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_operation_id(&mut self, v: ::std::string::String) {
        self.operation_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_operation_id(&mut self) -> &mut ::std::string::String {
        &mut self.operation_id
    }

    // Take field
    pub fn take_operation_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.operation_id, ::std::string::String::new())
    }

    // string operation_name = 2;


    pub fn get_operation_name(&self) -> &str {
        &self.operation_name
    }
    pub fn clear_operation_name(&mut self) {
        self.operation_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_operation_name(&mut self, v: ::std::string::String) {
        self.operation_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_operation_name(&mut self) -> &mut ::std::string::String {
        &mut self.operation_name
    }

    // Take field
    pub fn take_operation_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.operation_name, ::std::string::String::new())
    }

    // string consumer_id = 3;


    pub fn get_consumer_id(&self) -> &str {
        &self.consumer_id
    }
    pub fn clear_consumer_id(&mut self) {
        self.consumer_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_consumer_id(&mut self, v: ::std::string::String) {
        self.consumer_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_consumer_id(&mut self) -> &mut ::std::string::String {
        &mut self.consumer_id
    }

    // Take field
    pub fn take_consumer_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.consumer_id, ::std::string::String::new())
    }

    // .google.protobuf.Timestamp start_time = 4;


    pub fn get_start_time(&self) -> &::protobuf::well_known_types::Timestamp {
        self.start_time.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Timestamp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_start_time(&mut self) {
        self.start_time.clear();
    }

    pub fn has_start_time(&self) -> bool {
        self.start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.start_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_time(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.start_time.is_none() {
            self.start_time.set_default();
        }
        self.start_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_time(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.start_time.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // .google.protobuf.Timestamp end_time = 5;


    pub fn get_end_time(&self) -> &::protobuf::well_known_types::Timestamp {
        self.end_time.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Timestamp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_end_time(&mut self) {
        self.end_time.clear();
    }

    pub fn has_end_time(&self) -> bool {
        self.end_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_time(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.end_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_time(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.end_time.is_none() {
            self.end_time.set_default();
        }
        self.end_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_end_time(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.end_time.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // repeated .google.api.servicecontrol.v1.Operation.LabelsEntry labels = 6;


    pub fn get_labels(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.labels
    }
    pub fn clear_labels(&mut self) {
        self.labels.clear();
    }

    // Param is passed by value, moved
    pub fn set_labels(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.labels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_labels(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.labels
    }

    // Take field
    pub fn take_labels(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.labels, ::std::collections::HashMap::new())
    }

    // repeated .google.api.servicecontrol.v1.MetricValueSet metric_value_sets = 7;


    pub fn get_metric_value_sets(&self) -> &[super::metric_value::MetricValueSet] {
        &self.metric_value_sets
    }
    pub fn clear_metric_value_sets(&mut self) {
        self.metric_value_sets.clear();
    }

    // Param is passed by value, moved
    pub fn set_metric_value_sets(&mut self, v: ::protobuf::RepeatedField<super::metric_value::MetricValueSet>) {
        self.metric_value_sets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metric_value_sets(&mut self) -> &mut ::protobuf::RepeatedField<super::metric_value::MetricValueSet> {
        &mut self.metric_value_sets
    }

    // Take field
    pub fn take_metric_value_sets(&mut self) -> ::protobuf::RepeatedField<super::metric_value::MetricValueSet> {
        ::std::mem::replace(&mut self.metric_value_sets, ::protobuf::RepeatedField::new())
    }

    // repeated .google.api.servicecontrol.v1.LogEntry log_entries = 8;


    pub fn get_log_entries(&self) -> &[super::log_entry::LogEntry] {
        &self.log_entries
    }
    pub fn clear_log_entries(&mut self) {
        self.log_entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_log_entries(&mut self, v: ::protobuf::RepeatedField<super::log_entry::LogEntry>) {
        self.log_entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_log_entries(&mut self) -> &mut ::protobuf::RepeatedField<super::log_entry::LogEntry> {
        &mut self.log_entries
    }

    // Take field
    pub fn take_log_entries(&mut self) -> ::protobuf::RepeatedField<super::log_entry::LogEntry> {
        ::std::mem::replace(&mut self.log_entries, ::protobuf::RepeatedField::new())
    }

    // .google.api.servicecontrol.v1.Operation.Importance importance = 11;


    pub fn get_importance(&self) -> Operation_Importance {
        self.importance
    }
    pub fn clear_importance(&mut self) {
        self.importance = Operation_Importance::LOW;
    }

    // Param is passed by value, moved
    pub fn set_importance(&mut self, v: Operation_Importance) {
        self.importance = v;
    }

    // repeated .google.protobuf.Any extensions = 16;


    pub fn get_extensions(&self) -> &[::protobuf::well_known_types::Any] {
        &self.extensions
    }
    pub fn clear_extensions(&mut self) {
        self.extensions.clear();
    }

    // Param is passed by value, moved
    pub fn set_extensions(&mut self, v: ::protobuf::RepeatedField<::protobuf::well_known_types::Any>) {
        self.extensions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extensions(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        &mut self.extensions
    }

    // Take field
    pub fn take_extensions(&mut self) -> ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        ::std::mem::replace(&mut self.extensions, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Operation {
    fn is_initialized(&self) -> bool {
        for v in &self.start_time {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.end_time {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.metric_value_sets {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.log_entries {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.extensions {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.operation_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.operation_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.consumer_id)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start_time)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.end_time)?;
                },
                6 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.labels)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.metric_value_sets)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.log_entries)?;
                },
                11 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.importance, 11, &mut self.unknown_fields)?
                },
                16 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.extensions)?;
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
        if !self.operation_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.operation_id);
        }
        if !self.operation_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.operation_name);
        }
        if !self.consumer_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.consumer_id);
        }
        if let Some(ref v) = self.start_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.end_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(6, &self.labels);
        for value in &self.metric_value_sets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.log_entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.importance != Operation_Importance::LOW {
            my_size += ::protobuf::rt::enum_size(11, self.importance);
        }
        for value in &self.extensions {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.operation_id.is_empty() {
            os.write_string(1, &self.operation_id)?;
        }
        if !self.operation_name.is_empty() {
            os.write_string(2, &self.operation_name)?;
        }
        if !self.consumer_id.is_empty() {
            os.write_string(3, &self.consumer_id)?;
        }
        if let Some(ref v) = self.start_time.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.end_time.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(6, &self.labels, os)?;
        for v in &self.metric_value_sets {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.log_entries {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.importance != Operation_Importance::LOW {
            os.write_enum(11, ::protobuf::ProtobufEnum::value(&self.importance))?;
        }
        for v in &self.extensions {
            os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> Operation {
        Operation::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "operation_id",
                |m: &Operation| { &m.operation_id },
                |m: &mut Operation| { &mut m.operation_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "operation_name",
                |m: &Operation| { &m.operation_name },
                |m: &mut Operation| { &mut m.operation_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "consumer_id",
                |m: &Operation| { &m.consumer_id },
                |m: &mut Operation| { &mut m.consumer_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                "start_time",
                |m: &Operation| { &m.start_time },
                |m: &mut Operation| { &mut m.start_time },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                "end_time",
                |m: &Operation| { &m.end_time },
                |m: &mut Operation| { &mut m.end_time },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                "labels",
                |m: &Operation| { &m.labels },
                |m: &mut Operation| { &mut m.labels },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metric_value::MetricValueSet>>(
                "metric_value_sets",
                |m: &Operation| { &m.metric_value_sets },
                |m: &mut Operation| { &mut m.metric_value_sets },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::log_entry::LogEntry>>(
                "log_entries",
                |m: &Operation| { &m.log_entries },
                |m: &mut Operation| { &mut m.log_entries },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Operation_Importance>>(
                "importance",
                |m: &Operation| { &m.importance },
                |m: &mut Operation| { &mut m.importance },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                "extensions",
                |m: &Operation| { &m.extensions },
                |m: &mut Operation| { &mut m.extensions },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Operation>(
                "Operation",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Operation {
        static instance: ::protobuf::rt::LazyV2<Operation> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Operation::new)
    }
}

impl ::protobuf::Clear for Operation {
    fn clear(&mut self) {
        self.operation_id.clear();
        self.operation_name.clear();
        self.consumer_id.clear();
        self.start_time.clear();
        self.end_time.clear();
        self.labels.clear();
        self.metric_value_sets.clear();
        self.log_entries.clear();
        self.importance = Operation_Importance::LOW;
        self.extensions.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Operation {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Operation_Importance {
    LOW = 0,
    HIGH = 1,
}

impl ::protobuf::ProtobufEnum for Operation_Importance {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Operation_Importance> {
        match value {
            0 => ::std::option::Option::Some(Operation_Importance::LOW),
            1 => ::std::option::Option::Some(Operation_Importance::HIGH),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Operation_Importance] = &[
            Operation_Importance::LOW,
            Operation_Importance::HIGH,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Operation_Importance>("Operation.Importance", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Operation_Importance {
}

impl ::std::default::Default for Operation_Importance {
    fn default() -> Self {
        Operation_Importance::LOW
    }
}

impl ::protobuf::reflect::ProtobufValue for Operation_Importance {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n,google/api/servicecontrol/v1/operation.proto\x12\x1cgoogle.api.servic\
    econtrol.v1\x1a,google/api/servicecontrol/v1/log_entry.proto\x1a/google/\
    api/servicecontrol/v1/metric_value.proto\x1a\x19google/protobuf/any.prot\
    o\x1a\x1fgoogle/protobuf/timestamp.proto\"\xbe\x05\n\tOperation\x12!\n\
    \x0coperation_id\x18\x01\x20\x01(\tR\x0boperationId\x12%\n\x0eoperation_\
    name\x18\x02\x20\x01(\tR\roperationName\x12\x1f\n\x0bconsumer_id\x18\x03\
    \x20\x01(\tR\nconsumerId\x129\n\nstart_time\x18\x04\x20\x01(\x0b2\x1a.go\
    ogle.protobuf.TimestampR\tstartTime\x125\n\x08end_time\x18\x05\x20\x01(\
    \x0b2\x1a.google.protobuf.TimestampR\x07endTime\x12K\n\x06labels\x18\x06\
    \x20\x03(\x0b23.google.api.servicecontrol.v1.Operation.LabelsEntryR\x06l\
    abels\x12X\n\x11metric_value_sets\x18\x07\x20\x03(\x0b2,.google.api.serv\
    icecontrol.v1.MetricValueSetR\x0fmetricValueSets\x12G\n\x0blog_entries\
    \x18\x08\x20\x03(\x0b2&.google.api.servicecontrol.v1.LogEntryR\nlogEntri\
    es\x12R\n\nimportance\x18\x0b\x20\x01(\x0e22.google.api.servicecontrol.v\
    1.Operation.ImportanceR\nimportance\x124\n\nextensions\x18\x10\x20\x03(\
    \x0b2\x14.google.protobuf.AnyR\nextensions\x1a9\n\x0bLabelsEntry\x12\x10\
    \n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\
    \tR\x05value:\x028\x01\"\x1f\n\nImportance\x12\x07\n\x03LOW\x10\0\x12\
    \x08\n\x04HIGH\x10\x01B\xe9\x01\n\x20com.google.api.servicecontrol.v1B\
    \x0eOperationProtoP\x01ZJgoogle.golang.org/genproto/googleapis/api/servi\
    cecontrol/v1;servicecontrol\xf8\x01\x01\xaa\x02\x1eGoogle.Cloud.ServiceC\
    ontrol.V1\xca\x02\x1eGoogle\\Cloud\\ServiceControl\\V1\xea\x02!Google::C\
    loud::ServiceControl::V1J\x8b&\n\x06\x12\x04\x0e\0x\x01\n\xbc\x04\n\x01\
    \x0c\x12\x03\x0e\0\x122\xb1\x04\x20Copyright\x202020\x20Google\x20LLC\n\
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
    e\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0%\n\t\n\x02\x03\0\x12\x03\
    \x12\06\n\t\n\x02\x03\x01\x12\x03\x13\09\n\t\n\x02\x03\x02\x12\x03\x14\0\
    #\n\t\n\x02\x03\x03\x12\x03\x15\0)\n\x08\n\x01\x08\x12\x03\x17\0\x1f\n\t\
    \n\x02\x08\x1f\x12\x03\x17\0\x1f\n\x08\n\x01\x08\x12\x03\x18\0;\n\t\n\
    \x02\x08%\x12\x03\x18\0;\n\x08\n\x01\x08\x12\x03\x19\0a\n\t\n\x02\x08\
    \x0b\x12\x03\x19\0a\n\x08\n\x01\x08\x12\x03\x1a\0\"\n\t\n\x02\x08\n\x12\
    \x03\x1a\0\"\n\x08\n\x01\x08\x12\x03\x1b\0/\n\t\n\x02\x08\x08\x12\x03\
    \x1b\0/\n\x08\n\x01\x08\x12\x03\x1c\09\n\t\n\x02\x08\x01\x12\x03\x1c\09\
    \n\x08\n\x01\x08\x12\x03\x1d\0;\n\t\n\x02\x08)\x12\x03\x1d\0;\n\x08\n\
    \x01\x08\x12\x03\x1e\0:\n\t\n\x02\x08-\x12\x03\x1e\0:\n<\n\x02\x04\0\x12\
    \x04!\0x\x01\x1a0\x20Represents\x20information\x20regarding\x20an\x20ope\
    ration.\n\n\n\n\x03\x04\0\x01\x12\x03!\x08\x11\nN\n\x04\x04\0\x04\0\x12\
    \x04#\x02,\x03\x1a@\x20Defines\x20the\x20importance\x20of\x20the\x20data\
    \x20contained\x20in\x20the\x20operation.\n\n\x0c\n\x05\x04\0\x04\0\x01\
    \x12\x03#\x07\x11\n\x90\x01\n\x06\x04\0\x04\0\x02\0\x12\x03&\x04\x0c\x1a\
    \x80\x01\x20The\x20API\x20implementation\x20may\x20cache\x20and\x20aggre\
    gate\x20the\x20data.\n\x20The\x20data\x20may\x20be\x20lost\x20when\x20ra\
    re\x20and\x20unexpected\x20system\x20failures\x20occur.\n\n\x0e\n\x07\
    \x04\0\x04\0\x02\0\x01\x12\x03&\x04\x07\n\x0e\n\x07\x04\0\x04\0\x02\0\
    \x02\x12\x03&\n\x0b\n\xb9\x01\n\x06\x04\0\x04\0\x02\x01\x12\x03+\x04\r\
    \x1a\xa9\x01\x20The\x20API\x20implementation\x20doesn't\x20cache\x20and\
    \x20aggregate\x20the\x20data.\n\x20If\x20the\x20method\x20returns\x20suc\
    cessfully,\x20it's\x20guaranteed\x20that\x20the\x20data\x20has\n\x20been\
    \x20persisted\x20in\x20durable\x20storage.\n\n\x0e\n\x07\x04\0\x04\0\x02\
    \x01\x01\x12\x03+\x04\x08\n\x0e\n\x07\x04\0\x04\0\x02\x01\x02\x12\x03+\
    \x0b\x0c\n\xdd\x03\n\x04\x04\0\x02\0\x12\x037\x02\x1a\x1a\xcf\x03\x20Ide\
    ntity\x20of\x20the\x20operation.\x20This\x20must\x20be\x20unique\x20with\
    in\x20the\x20scope\x20of\x20the\n\x20service\x20that\x20generated\x20the\
    \x20operation.\x20If\x20the\x20service\x20calls\n\x20Check()\x20and\x20R\
    eport()\x20on\x20the\x20same\x20operation,\x20the\x20two\x20calls\x20sho\
    uld\x20carry\n\x20the\x20same\x20id.\n\n\x20UUID\x20version\x204\x20is\
    \x20recommended,\x20though\x20not\x20required.\n\x20In\x20scenarios\x20w\
    here\x20an\x20operation\x20is\x20computed\x20from\x20existing\x20informa\
    tion\n\x20and\x20an\x20idempotent\x20id\x20is\x20desirable\x20for\x20ded\
    uplication\x20purpose,\x20UUID\x20version\x205\n\x20is\x20recommended.\
    \x20See\x20RFC\x204122\x20for\x20details.\n\n\r\n\x05\x04\0\x02\0\x04\
    \x12\x047\x02,\x03\n\x0c\n\x05\x04\0\x02\0\x05\x12\x037\x02\x08\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x037\t\x15\n\x0c\n\x05\x04\0\x02\0\x03\x12\x037\
    \x18\x19\nN\n\x04\x04\0\x02\x01\x12\x03:\x02\x1c\x1aA\x20Fully\x20qualif\
    ied\x20name\x20of\x20the\x20operation.\x20Reserved\x20for\x20future\x20u\
    se.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04:\x027\x1a\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03:\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03:\t\
    \x17\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03:\x1a\x1b\n\xea\x03\n\x04\x04\
    \0\x02\x02\x12\x03H\x02\x19\x1a\xdc\x03\x20Identity\x20of\x20the\x20cons\
    umer\x20who\x20is\x20using\x20the\x20service.\n\x20This\x20field\x20shou\
    ld\x20be\x20filled\x20in\x20for\x20the\x20operations\x20initiated\x20by\
    \x20a\n\x20consumer,\x20but\x20not\x20for\x20service-initiated\x20operat\
    ions\x20that\x20are\n\x20not\x20related\x20to\x20a\x20specific\x20consum\
    er.\n\n\x20-\x20This\x20can\x20be\x20in\x20one\x20of\x20the\x20following\
    \x20formats:\n\x20\x20\x20\x20\x20-\x20project:PROJECT_ID,\n\x20\x20\x20\
    \x20\x20-\x20project`_`number:PROJECT_NUMBER,\n\x20\x20\x20\x20\x20-\x20\
    projects/PROJECT_ID\x20or\x20PROJECT_NUMBER,\n\x20\x20\x20\x20\x20-\x20f\
    olders/FOLDER_NUMBER,\n\x20\x20\x20\x20\x20-\x20organizations/ORGANIZATI\
    ON_NUMBER,\n\x20\x20\x20\x20\x20-\x20api`_`key:API_KEY.\n\n\r\n\x05\x04\
    \0\x02\x02\x04\x12\x04H\x02:\x1c\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03H\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03H\t\x14\n\x0c\n\x05\x04\0\
    \x02\x02\x03\x12\x03H\x17\x18\n5\n\x04\x04\0\x02\x03\x12\x03K\x02+\x1a(\
    \x20Required.\x20Start\x20time\x20of\x20the\x20operation.\n\n\r\n\x05\
    \x04\0\x02\x03\x04\x12\x04K\x02H\x19\n\x0c\n\x05\x04\0\x02\x03\x06\x12\
    \x03K\x02\x1b\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03K\x1c&\n\x0c\n\x05\
    \x04\0\x02\x03\x03\x12\x03K)*\n\xa2\x02\n\x04\x04\0\x02\x04\x12\x03P\x02\
    )\x1a\x94\x02\x20End\x20time\x20of\x20the\x20operation.\n\x20Required\
    \x20when\x20the\x20operation\x20is\x20used\x20in\x20[ServiceController.R\
    eport][google.api.servicecontrol.v1.ServiceController.Report],\n\x20but\
    \x20optional\x20when\x20the\x20operation\x20is\x20used\x20in\x20[Service\
    Controller.Check][google.api.servicecontrol.v1.ServiceController.Check].\
    \n\n\r\n\x05\x04\0\x02\x04\x04\x12\x04P\x02K+\n\x0c\n\x05\x04\0\x02\x04\
    \x06\x12\x03P\x02\x1b\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03P\x1c$\n\x0c\
    \n\x05\x04\0\x02\x04\x03\x12\x03P'(\n\xd0\x06\n\x04\x04\0\x02\x05\x12\
    \x03b\x02!\x1a\xc2\x06\x20Labels\x20describing\x20the\x20operation.\x20O\
    nly\x20the\x20following\x20labels\x20are\x20allowed:\n\n\x20-\x20Labels\
    \x20describing\x20monitored\x20resources\x20as\x20defined\x20in\n\x20\
    \x20\x20the\x20service\x20configuration.\n\x20-\x20Default\x20labels\x20\
    of\x20metric\x20values.\x20When\x20specified,\x20labels\x20defined\x20in\
    \x20the\n\x20\x20\x20metric\x20value\x20override\x20these\x20default.\n\
    \x20-\x20The\x20following\x20labels\x20defined\x20by\x20Google\x20Cloud\
    \x20Platform:\n\x20\x20\x20\x20\x20-\x20`cloud.googleapis.com/location`\
    \x20describing\x20the\x20location\x20where\x20the\n\x20\x20\x20\x20\x20\
    \x20\x20\x20operation\x20happened,\n\x20\x20\x20\x20\x20-\x20`servicecon\
    trol.googleapis.com/user_agent`\x20describing\x20the\x20user\x20agent\n\
    \x20\x20\x20\x20\x20\x20\x20\x20of\x20the\x20API\x20request,\n\x20\x20\
    \x20\x20\x20-\x20`servicecontrol.googleapis.com/service_agent`\x20descri\
    bing\x20the\x20service\n\x20\x20\x20\x20\x20\x20\x20\x20used\x20to\x20ha\
    ndle\x20the\x20API\x20request\x20(e.g.\x20ESP),\n\x20\x20\x20\x20\x20-\
    \x20`servicecontrol.googleapis.com/platform`\x20describing\x20the\x20pla\
    tform\n\x20\x20\x20\x20\x20\x20\x20\x20where\x20the\x20API\x20is\x20serv\
    ed,\x20such\x20as\x20App\x20Engine,\x20Compute\x20Engine,\x20or\n\x20\
    \x20\x20\x20\x20\x20\x20\x20Kubernetes\x20Engine.\n\n\r\n\x05\x04\0\x02\
    \x05\x04\x12\x04b\x02P)\n\x0c\n\x05\x04\0\x02\x05\x06\x12\x03b\x02\x15\n\
    \x0c\n\x05\x04\0\x02\x05\x01\x12\x03b\x16\x1c\n\x0c\n\x05\x04\0\x02\x05\
    \x03\x12\x03b\x1f\x20\n\x9a\x04\n\x04\x04\0\x02\x06\x12\x03n\x020\x1a\
    \x8c\x04\x20Represents\x20information\x20about\x20this\x20operation.\x20\
    Each\x20MetricValueSet\n\x20corresponds\x20to\x20a\x20metric\x20defined\
    \x20in\x20the\x20service\x20configuration.\n\x20The\x20data\x20type\x20u\
    sed\x20in\x20the\x20MetricValueSet\x20must\x20agree\x20with\n\x20the\x20\
    data\x20type\x20specified\x20in\x20the\x20metric\x20definition.\n\n\x20W\
    ithin\x20a\x20single\x20operation,\x20it\x20is\x20not\x20allowed\x20to\
    \x20have\x20more\x20than\x20one\n\x20MetricValue\x20instances\x20that\
    \x20have\x20the\x20same\x20metric\x20names\x20and\x20identical\n\x20labe\
    l\x20value\x20combinations.\x20If\x20a\x20request\x20has\x20such\x20dupl\
    icated\x20MetricValue\n\x20instances,\x20the\x20entire\x20request\x20is\
    \x20rejected\x20with\n\x20an\x20invalid\x20argument\x20error.\n\n\x0c\n\
    \x05\x04\0\x02\x06\x04\x12\x03n\x02\n\n\x0c\n\x05\x04\0\x02\x06\x06\x12\
    \x03n\x0b\x19\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03n\x1a+\n\x0c\n\x05\
    \x04\0\x02\x06\x03\x12\x03n./\n3\n\x04\x04\0\x02\x07\x12\x03q\x02$\x1a&\
    \x20Represents\x20information\x20to\x20be\x20logged.\n\n\x0c\n\x05\x04\0\
    \x02\x07\x04\x12\x03q\x02\n\n\x0c\n\x05\x04\0\x02\x07\x06\x12\x03q\x0b\
    \x13\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03q\x14\x1f\n\x0c\n\x05\x04\0\
    \x02\x07\x03\x12\x03q\"#\n9\n\x04\x04\0\x02\x08\x12\x03t\x02\x1d\x1a,\
    \x20DO\x20NOT\x20USE.\x20This\x20is\x20an\x20experimental\x20field.\n\n\
    \r\n\x05\x04\0\x02\x08\x04\x12\x04t\x02q$\n\x0c\n\x05\x04\0\x02\x08\x06\
    \x12\x03t\x02\x0c\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03t\r\x17\n\x0c\n\
    \x05\x04\0\x02\x08\x03\x12\x03t\x1a\x1c\n\x1d\n\x04\x04\0\x02\t\x12\x03w\
    \x02/\x1a\x10\x20Unimplemented.\n\n\x0c\n\x05\x04\0\x02\t\x04\x12\x03w\
    \x02\n\n\x0c\n\x05\x04\0\x02\t\x06\x12\x03w\x0b\x1e\n\x0c\n\x05\x04\0\
    \x02\t\x01\x12\x03w\x1f)\n\x0c\n\x05\x04\0\x02\t\x03\x12\x03w,.b\x06prot\
    o3\
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
