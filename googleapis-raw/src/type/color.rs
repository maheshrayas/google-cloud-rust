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
//! Generated file from `google/type/color.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct Color {
    // message fields
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: ::protobuf::SingularPtrField<::protobuf::well_known_types::FloatValue>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Color {
    fn default() -> &'a Color {
        <Color as ::protobuf::Message>::default_instance()
    }
}

impl Color {
    pub fn new() -> Color {
        ::std::default::Default::default()
    }

    // float red = 1;


    pub fn get_red(&self) -> f32 {
        self.red
    }
    pub fn clear_red(&mut self) {
        self.red = 0.;
    }

    // Param is passed by value, moved
    pub fn set_red(&mut self, v: f32) {
        self.red = v;
    }

    // float green = 2;


    pub fn get_green(&self) -> f32 {
        self.green
    }
    pub fn clear_green(&mut self) {
        self.green = 0.;
    }

    // Param is passed by value, moved
    pub fn set_green(&mut self, v: f32) {
        self.green = v;
    }

    // float blue = 3;


    pub fn get_blue(&self) -> f32 {
        self.blue
    }
    pub fn clear_blue(&mut self) {
        self.blue = 0.;
    }

    // Param is passed by value, moved
    pub fn set_blue(&mut self, v: f32) {
        self.blue = v;
    }

    // .google.protobuf.FloatValue alpha = 4;


    pub fn get_alpha(&self) -> &::protobuf::well_known_types::FloatValue {
        self.alpha.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::FloatValue as ::protobuf::Message>::default_instance())
    }
    pub fn clear_alpha(&mut self) {
        self.alpha.clear();
    }

    pub fn has_alpha(&self) -> bool {
        self.alpha.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alpha(&mut self, v: ::protobuf::well_known_types::FloatValue) {
        self.alpha = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alpha(&mut self) -> &mut ::protobuf::well_known_types::FloatValue {
        if self.alpha.is_none() {
            self.alpha.set_default();
        }
        self.alpha.as_mut().unwrap()
    }

    // Take field
    pub fn take_alpha(&mut self) -> ::protobuf::well_known_types::FloatValue {
        self.alpha.take().unwrap_or_else(|| ::protobuf::well_known_types::FloatValue::new())
    }
}

impl ::protobuf::Message for Color {
    fn is_initialized(&self) -> bool {
        for v in &self.alpha {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.red = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.green = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.blue = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.alpha)?;
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
        if self.red != 0. {
            my_size += 5;
        }
        if self.green != 0. {
            my_size += 5;
        }
        if self.blue != 0. {
            my_size += 5;
        }
        if let Some(ref v) = self.alpha.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.red != 0. {
            os.write_float(1, self.red)?;
        }
        if self.green != 0. {
            os.write_float(2, self.green)?;
        }
        if self.blue != 0. {
            os.write_float(3, self.blue)?;
        }
        if let Some(ref v) = self.alpha.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> Color {
        Color::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "red",
                |m: &Color| { &m.red },
                |m: &mut Color| { &mut m.red },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "green",
                |m: &Color| { &m.green },
                |m: &mut Color| { &mut m.green },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "blue",
                |m: &Color| { &m.blue },
                |m: &mut Color| { &mut m.blue },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::FloatValue>>(
                "alpha",
                |m: &Color| { &m.alpha },
                |m: &mut Color| { &mut m.alpha },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Color>(
                "Color",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Color {
        static instance: ::protobuf::rt::LazyV2<Color> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Color::new)
    }
}

impl ::protobuf::Clear for Color {
    fn clear(&mut self) {
        self.red = 0.;
        self.green = 0.;
        self.blue = 0.;
        self.alpha.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Color {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Color {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17google/type/color.proto\x12\x0bgoogle.type\x1a\x1egoogle/protobuf/\
    wrappers.proto\"v\n\x05Color\x12\x10\n\x03red\x18\x01\x20\x01(\x02R\x03r\
    ed\x12\x14\n\x05green\x18\x02\x20\x01(\x02R\x05green\x12\x12\n\x04blue\
    \x18\x03\x20\x01(\x02R\x04blue\x121\n\x05alpha\x18\x04\x20\x01(\x0b2\x1b\
    .google.protobuf.FloatValueR\x05alphaB`\n\x0fcom.google.typeB\nColorProt\
    oP\x01Z6google.golang.org/genproto/googleapis/type/color;color\xf8\x01\
    \x01\xa2\x02\x03GTPJ\xdd-\n\x07\x12\x05\x0f\0\xa9\x01\x01\n\xbe\x04\n\
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
    e\x20License.\n\n\n\x08\n\x01\x02\x12\x03\x11\0\x14\n\t\n\x02\x03\0\x12\
    \x03\x13\0(\n\x08\n\x01\x08\x12\x03\x15\0\x1f\n\t\n\x02\x08\x1f\x12\x03\
    \x15\0\x1f\n\x08\n\x01\x08\x12\x03\x16\0M\n\t\n\x02\x08\x0b\x12\x03\x16\
    \0M\n\x08\n\x01\x08\x12\x03\x17\0\"\n\t\n\x02\x08\n\x12\x03\x17\0\"\n\
    \x08\n\x01\x08\x12\x03\x18\0+\n\t\n\x02\x08\x08\x12\x03\x18\0+\n\x08\n\
    \x01\x08\x12\x03\x19\0(\n\t\n\x02\x08\x01\x12\x03\x19\0(\n\x08\n\x01\x08\
    \x12\x03\x1a\0!\n\t\n\x02\x08$\x12\x03\x1a\0!\n\xbc\x1f\n\x02\x04\0\x12\
    \x06\x93\x01\0\xa9\x01\x01\x1a\xad\x1f\x20Represents\x20a\x20color\x20in\
    \x20the\x20RGBA\x20color\x20space.\x20This\x20representation\x20is\x20de\
    signed\n\x20for\x20simplicity\x20of\x20conversion\x20to/from\x20color\
    \x20representations\x20in\x20various\n\x20languages\x20over\x20compactne\
    ss;\x20for\x20example,\x20the\x20fields\x20of\x20this\x20representation\
    \n\x20can\x20be\x20trivially\x20provided\x20to\x20the\x20constructor\x20\
    of\x20\"java.awt.Color\"\x20in\x20Java;\x20it\n\x20can\x20also\x20be\x20\
    trivially\x20provided\x20to\x20UIColor's\x20\"+colorWithRed:green:blue:a\
    lpha\"\n\x20method\x20in\x20iOS;\x20and,\x20with\x20just\x20a\x20little\
    \x20work,\x20it\x20can\x20be\x20easily\x20formatted\x20into\n\x20a\x20CS\
    S\x20\"rgba()\"\x20string\x20in\x20JavaScript,\x20as\x20well.\n\n\x20Not\
    e:\x20this\x20proto\x20does\x20not\x20carry\x20information\x20about\x20t\
    he\x20absolute\x20color\x20space\n\x20that\x20should\x20be\x20used\x20to\
    \x20interpret\x20the\x20RGB\x20value\x20(e.g.\x20sRGB,\x20Adobe\x20RGB,\
    \n\x20DCI-P3,\x20BT.2020,\x20etc.).\x20By\x20default,\x20applications\
    \x20SHOULD\x20assume\x20the\x20sRGB\x20color\n\x20space.\n\n\x20Example\
    \x20(Java):\n\n\x20\x20\x20\x20\x20\x20import\x20com.google.type.Color;\
    \n\n\x20\x20\x20\x20\x20\x20//\x20...\n\x20\x20\x20\x20\x20\x20public\
    \x20static\x20java.awt.Color\x20fromProto(Color\x20protocolor)\x20{\n\
    \x20\x20\x20\x20\x20\x20\x20\x20float\x20alpha\x20=\x20protocolor.hasAlp\
    ha()\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20?\x20protocolor.ge\
    tAlpha().getValue()\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20:\
    \x201.0;\n\n\x20\x20\x20\x20\x20\x20\x20\x20return\x20new\x20java.awt.Co\
    lor(\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20protocolor.getRed(\
    ),\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20protocolor.getGreen(\
    ),\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20protocolor.getBlue()\
    ,\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20alpha);\n\x20\x20\x20\
    \x20\x20\x20}\n\n\x20\x20\x20\x20\x20\x20public\x20static\x20Color\x20to\
    Proto(java.awt.Color\x20color)\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20flo\
    at\x20red\x20=\x20(float)\x20color.getRed();\n\x20\x20\x20\x20\x20\x20\
    \x20\x20float\x20green\x20=\x20(float)\x20color.getGreen();\n\x20\x20\
    \x20\x20\x20\x20\x20\x20float\x20blue\x20=\x20(float)\x20color.getBlue()\
    ;\n\x20\x20\x20\x20\x20\x20\x20\x20float\x20denominator\x20=\x20255.0;\n\
    \x20\x20\x20\x20\x20\x20\x20\x20Color.Builder\x20resultBuilder\x20=\n\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20Color\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20.newBuilder()\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20.setRed(red\x20/\
    \x20denominator)\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20.setGreen(green\x20/\x20denominator)\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20.setBlue(blue\x20/\x20denomi\
    nator);\n\x20\x20\x20\x20\x20\x20\x20\x20int\x20alpha\x20=\x20color.getA\
    lpha();\n\x20\x20\x20\x20\x20\x20\x20\x20if\x20(alpha\x20!=\x20255)\x20{\
    \n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20result.setAlpha(\n\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20FloatValue\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20.newBuilder()\n\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    .setValue(((float)\x20alpha)\x20/\x20denominator)\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20.build());\n\x20\x20\
    \x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\x20\x20return\x20res\
    ultBuilder.build();\n\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\
    //\x20...\n\n\x20Example\x20(iOS\x20/\x20Obj-C):\n\n\x20\x20\x20\x20\x20\
    \x20//\x20...\n\x20\x20\x20\x20\x20\x20static\x20UIColor*\x20fromProto(C\
    olor*\x20protocolor)\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20float\x20\
    red\x20=\x20[protocolor\x20red];\n\x20\x20\x20\x20\x20\x20\x20\x20\x20fl\
    oat\x20green\x20=\x20[protocolor\x20green];\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20float\x20blue\x20=\x20[protocolor\x20blue];\n\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20FloatValue*\x20alpha_wrapper\x20=\x20[protocolor\
    \x20alpha];\n\x20\x20\x20\x20\x20\x20\x20\x20\x20float\x20alpha\x20=\x20\
    1.0;\n\x20\x20\x20\x20\x20\x20\x20\x20\x20if\x20(alpha_wrapper\x20!=\x20\
    nil)\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20alpha\x20=\x20[al\
    pha_wrapper\x20value];\n\x20\x20\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20return\x20[UIColor\x20colorWithRed:red\x20gr\
    een:green\x20blue:blue\x20alpha:alpha];\n\x20\x20\x20\x20\x20\x20}\n\n\
    \x20\x20\x20\x20\x20\x20static\x20Color*\x20toProto(UIColor*\x20color)\
    \x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20CGFloat\x20red,\x20green,\
    \x20blue,\x20alpha;\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20if\x20(![co\
    lor\x20getRed:&red\x20green:&green\x20blue:&blue\x20alpha:&alpha])\x20{\
    \n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20return\x20nil;\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20Color*\x20result\x20=\x20[[Color\x20alloc]\x20init];\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20[result\x20setRed:red];\n\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20[result\x20setGreen:green];\n\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20[result\x20setBlue:blue];\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20if\x20(alpha\x20<=\x200.9999)\x20{\n\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20[result\x20setAlpha:floatWrapperWith\
    Value(alpha)];\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20[result\x20autorelease];\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20return\x20result;\n\x20\x20\x20\x20\x20}\n\x20\
    \x20\x20\x20\x20//\x20...\n\n\x20\x20Example\x20(JavaScript):\n\n\x20\
    \x20\x20\x20\x20//\x20...\n\n\x20\x20\x20\x20\x20var\x20protoToCssColor\
    \x20=\x20function(rgb_color)\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20var\
    \x20redFrac\x20=\x20rgb_color.red\x20||\x200.0;\n\x20\x20\x20\x20\x20\
    \x20\x20\x20var\x20greenFrac\x20=\x20rgb_color.green\x20||\x200.0;\n\x20\
    \x20\x20\x20\x20\x20\x20\x20var\x20blueFrac\x20=\x20rgb_color.blue\x20||\
    \x200.0;\n\x20\x20\x20\x20\x20\x20\x20\x20var\x20red\x20=\x20Math.floor(\
    redFrac\x20*\x20255);\n\x20\x20\x20\x20\x20\x20\x20\x20var\x20green\x20=\
    \x20Math.floor(greenFrac\x20*\x20255);\n\x20\x20\x20\x20\x20\x20\x20\x20\
    var\x20blue\x20=\x20Math.floor(blueFrac\x20*\x20255);\n\n\x20\x20\x20\
    \x20\x20\x20\x20\x20if\x20(!('alpha'\x20in\x20rgb_color))\x20{\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20return\x20rgbToCssColor_(red,\x20gre\
    en,\x20blue);\n\x20\x20\x20\x20\x20\x20\x20\x20}\n\n\x20\x20\x20\x20\x20\
    \x20\x20\x20var\x20alphaFrac\x20=\x20rgb_color.alpha.value\x20||\x200.0;\
    \n\x20\x20\x20\x20\x20\x20\x20\x20var\x20rgbParams\x20=\x20[red,\x20gree\
    n,\x20blue].join(',');\n\x20\x20\x20\x20\x20\x20\x20\x20return\x20['rgba\
    (',\x20rgbParams,\x20',',\x20alphaFrac,\x20')'].join('');\n\x20\x20\x20\
    \x20\x20};\n\n\x20\x20\x20\x20\x20var\x20rgbToCssColor_\x20=\x20function\
    (red,\x20green,\x20blue)\x20{\n\x20\x20\x20\x20\x20\x20\x20var\x20rgbNum\
    ber\x20=\x20new\x20Number((red\x20<<\x2016)\x20|\x20(green\x20<<\x208)\
    \x20|\x20blue);\n\x20\x20\x20\x20\x20\x20\x20var\x20hexString\x20=\x20rg\
    bNumber.toString(16);\n\x20\x20\x20\x20\x20\x20\x20var\x20missingZeros\
    \x20=\x206\x20-\x20hexString.length;\n\x20\x20\x20\x20\x20\x20\x20var\
    \x20resultBuilder\x20=\x20['#'];\n\x20\x20\x20\x20\x20\x20\x20for\x20(va\
    r\x20i\x20=\x200;\x20i\x20<\x20missingZeros;\x20i++)\x20{\n\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20resultBuilder.push('0');\n\x20\x20\x20\x20\
    \x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\x20resultBuilder.push(hexString)\
    ;\n\x20\x20\x20\x20\x20\x20\x20return\x20resultBuilder.join('');\n\x20\
    \x20\x20\x20\x20};\n\n\x20\x20\x20\x20\x20//\x20...\n\n\x0b\n\x03\x04\0\
    \x01\x12\x04\x93\x01\x08\r\nQ\n\x04\x04\0\x02\0\x12\x04\x95\x01\x02\x10\
    \x1aC\x20The\x20amount\x20of\x20red\x20in\x20the\x20color\x20as\x20a\x20\
    value\x20in\x20the\x20interval\x20[0,\x201].\n\n\r\n\x05\x04\0\x02\0\x05\
    \x12\x04\x95\x01\x02\x07\n\r\n\x05\x04\0\x02\0\x01\x12\x04\x95\x01\x08\
    \x0b\n\r\n\x05\x04\0\x02\0\x03\x12\x04\x95\x01\x0e\x0f\nS\n\x04\x04\0\
    \x02\x01\x12\x04\x98\x01\x02\x12\x1aE\x20The\x20amount\x20of\x20green\
    \x20in\x20the\x20color\x20as\x20a\x20value\x20in\x20the\x20interval\x20[\
    0,\x201].\n\n\r\n\x05\x04\0\x02\x01\x05\x12\x04\x98\x01\x02\x07\n\r\n\
    \x05\x04\0\x02\x01\x01\x12\x04\x98\x01\x08\r\n\r\n\x05\x04\0\x02\x01\x03\
    \x12\x04\x98\x01\x10\x11\nR\n\x04\x04\0\x02\x02\x12\x04\x9b\x01\x02\x11\
    \x1aD\x20The\x20amount\x20of\x20blue\x20in\x20the\x20color\x20as\x20a\
    \x20value\x20in\x20the\x20interval\x20[0,\x201].\n\n\r\n\x05\x04\0\x02\
    \x02\x05\x12\x04\x9b\x01\x02\x07\n\r\n\x05\x04\0\x02\x02\x01\x12\x04\x9b\
    \x01\x08\x0c\n\r\n\x05\x04\0\x02\x02\x03\x12\x04\x9b\x01\x0f\x10\n\x81\
    \x05\n\x04\x04\0\x02\x03\x12\x04\xa8\x01\x02'\x1a\xf2\x04\x20The\x20frac\
    tion\x20of\x20this\x20color\x20that\x20should\x20be\x20applied\x20to\x20\
    the\x20pixel.\x20That\x20is,\n\x20the\x20final\x20pixel\x20color\x20is\
    \x20defined\x20by\x20the\x20equation:\n\n\x20\x20\x20pixel\x20color\x20=\
    \x20alpha\x20*\x20(this\x20color)\x20+\x20(1.0\x20-\x20alpha)\x20*\x20(b\
    ackground\x20color)\n\n\x20This\x20means\x20that\x20a\x20value\x20of\x20\
    1.0\x20corresponds\x20to\x20a\x20solid\x20color,\x20whereas\n\x20a\x20va\
    lue\x20of\x200.0\x20corresponds\x20to\x20a\x20completely\x20transparent\
    \x20color.\x20This\n\x20uses\x20a\x20wrapper\x20message\x20rather\x20tha\
    n\x20a\x20simple\x20float\x20scalar\x20so\x20that\x20it\x20is\n\x20possi\
    ble\x20to\x20distinguish\x20between\x20a\x20default\x20value\x20and\x20t\
    he\x20value\x20being\x20unset.\n\x20If\x20omitted,\x20this\x20color\x20o\
    bject\x20is\x20to\x20be\x20rendered\x20as\x20a\x20solid\x20color\n\x20(a\
    s\x20if\x20the\x20alpha\x20value\x20had\x20been\x20explicitly\x20given\
    \x20with\x20a\x20value\x20of\x201.0).\n\n\r\n\x05\x04\0\x02\x03\x06\x12\
    \x04\xa8\x01\x02\x1c\n\r\n\x05\x04\0\x02\x03\x01\x12\x04\xa8\x01\x1d\"\n\
    \r\n\x05\x04\0\x02\x03\x03\x12\x04\xa8\x01%&b\x06proto3\
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
