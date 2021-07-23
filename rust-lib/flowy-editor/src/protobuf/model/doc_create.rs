// This file is generated by rust-protobuf 2.22.1. Do not edit
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
//! Generated file from `doc_create.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct CreateDocRequest {
    // message fields
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub desc: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CreateDocRequest {
    fn default() -> &'a CreateDocRequest {
        <CreateDocRequest as ::protobuf::Message>::default_instance()
    }
}

impl CreateDocRequest {
    pub fn new() -> CreateDocRequest {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // string name = 2;


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

    // string desc = 3;


    pub fn get_desc(&self) -> &str {
        &self.desc
    }
    pub fn clear_desc(&mut self) {
        self.desc.clear();
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: ::std::string::String) {
        self.desc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_desc(&mut self) -> &mut ::std::string::String {
        &mut self.desc
    }

    // Take field
    pub fn take_desc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.desc, ::std::string::String::new())
    }
}

impl ::protobuf::Message for CreateDocRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.desc)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if !self.desc.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.desc);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if !self.desc.is_empty() {
            os.write_string(3, &self.desc)?;
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

    fn new() -> CreateDocRequest {
        CreateDocRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &CreateDocRequest| { &m.id },
                |m: &mut CreateDocRequest| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &CreateDocRequest| { &m.name },
                |m: &mut CreateDocRequest| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "desc",
                |m: &CreateDocRequest| { &m.desc },
                |m: &mut CreateDocRequest| { &mut m.desc },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CreateDocRequest>(
                "CreateDocRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CreateDocRequest {
        static instance: ::protobuf::rt::LazyV2<CreateDocRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CreateDocRequest::new)
    }
}

impl ::protobuf::Clear for CreateDocRequest {
    fn clear(&mut self) {
        self.id.clear();
        self.name.clear();
        self.desc.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateDocRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateDocRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DocDescription {
    // message fields
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub desc: ::std::string::String,
    pub path: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DocDescription {
    fn default() -> &'a DocDescription {
        <DocDescription as ::protobuf::Message>::default_instance()
    }
}

impl DocDescription {
    pub fn new() -> DocDescription {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // string name = 2;


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

    // string desc = 3;


    pub fn get_desc(&self) -> &str {
        &self.desc
    }
    pub fn clear_desc(&mut self) {
        self.desc.clear();
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: ::std::string::String) {
        self.desc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_desc(&mut self) -> &mut ::std::string::String {
        &mut self.desc
    }

    // Take field
    pub fn take_desc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.desc, ::std::string::String::new())
    }

    // string path = 4;


    pub fn get_path(&self) -> &str {
        &self.path
    }
    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.path, ::std::string::String::new())
    }
}

impl ::protobuf::Message for DocDescription {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.desc)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.path)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if !self.desc.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.desc);
        }
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.path);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if !self.desc.is_empty() {
            os.write_string(3, &self.desc)?;
        }
        if !self.path.is_empty() {
            os.write_string(4, &self.path)?;
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

    fn new() -> DocDescription {
        DocDescription::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &DocDescription| { &m.id },
                |m: &mut DocDescription| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &DocDescription| { &m.name },
                |m: &mut DocDescription| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "desc",
                |m: &DocDescription| { &m.desc },
                |m: &mut DocDescription| { &mut m.desc },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "path",
                |m: &DocDescription| { &m.path },
                |m: &mut DocDescription| { &mut m.path },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DocDescription>(
                "DocDescription",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DocDescription {
        static instance: ::protobuf::rt::LazyV2<DocDescription> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DocDescription::new)
    }
}

impl ::protobuf::Clear for DocDescription {
    fn clear(&mut self) {
        self.id.clear();
        self.name.clear();
        self.desc.clear();
        self.path.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DocDescription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DocDescription {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Doc {
    // message fields
    pub desc: ::protobuf::SingularPtrField<DocDescription>,
    pub content: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Doc {
    fn default() -> &'a Doc {
        <Doc as ::protobuf::Message>::default_instance()
    }
}

impl Doc {
    pub fn new() -> Doc {
        ::std::default::Default::default()
    }

    // .DocDescription desc = 1;


    pub fn get_desc(&self) -> &DocDescription {
        self.desc.as_ref().unwrap_or_else(|| <DocDescription as ::protobuf::Message>::default_instance())
    }
    pub fn clear_desc(&mut self) {
        self.desc.clear();
    }

    pub fn has_desc(&self) -> bool {
        self.desc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: DocDescription) {
        self.desc = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_desc(&mut self) -> &mut DocDescription {
        if self.desc.is_none() {
            self.desc.set_default();
        }
        self.desc.as_mut().unwrap()
    }

    // Take field
    pub fn take_desc(&mut self) -> DocDescription {
        self.desc.take().unwrap_or_else(|| DocDescription::new())
    }

    // string content = 2;


    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.content, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Doc {
    fn is_initialized(&self) -> bool {
        for v in &self.desc {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.desc)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.content)?;
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
        if let Some(ref v) = self.desc.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.content);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.desc.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.content.is_empty() {
            os.write_string(2, &self.content)?;
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

    fn new() -> Doc {
        Doc::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DocDescription>>(
                "desc",
                |m: &Doc| { &m.desc },
                |m: &mut Doc| { &mut m.desc },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "content",
                |m: &Doc| { &m.content },
                |m: &mut Doc| { &mut m.content },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Doc>(
                "Doc",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Doc {
        static instance: ::protobuf::rt::LazyV2<Doc> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Doc::new)
    }
}

impl ::protobuf::Clear for Doc {
    fn clear(&mut self) {
        self.desc.clear();
        self.content.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Doc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Doc {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10doc_create.proto\"J\n\x10CreateDocRequest\x12\x0e\n\x02id\x18\x01\
    \x20\x01(\tR\x02id\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x12\
    \n\x04desc\x18\x03\x20\x01(\tR\x04desc\"\\\n\x0eDocDescription\x12\x0e\n\
    \x02id\x18\x01\x20\x01(\tR\x02id\x12\x12\n\x04name\x18\x02\x20\x01(\tR\
    \x04name\x12\x12\n\x04desc\x18\x03\x20\x01(\tR\x04desc\x12\x12\n\x04path\
    \x18\x04\x20\x01(\tR\x04path\"D\n\x03Doc\x12#\n\x04desc\x18\x01\x20\x01(\
    \x0b2\x0f.DocDescriptionR\x04desc\x12\x18\n\x07content\x18\x02\x20\x01(\
    \tR\x07contentJ\xc9\x04\n\x06\x12\x04\0\0\x10\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x06\x01\n\n\n\x03\x04\0\x01\
    \x12\x03\x02\x08\x18\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\x12\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03\x03\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x03\x0b\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x10\x11\n\x0b\n\
    \x04\x04\0\x02\x01\x12\x03\x04\x04\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\
    \x03\x04\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\x0b\x0f\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\x04\x12\x13\n\x0b\n\x04\x04\0\x02\x02\x12\
    \x03\x05\x04\x14\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x05\x04\n\n\x0c\n\
    \x05\x04\0\x02\x02\x01\x12\x03\x05\x0b\x0f\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03\x05\x12\x13\n\n\n\x02\x04\x01\x12\x04\x07\0\x0c\x01\n\n\n\x03\
    \x04\x01\x01\x12\x03\x07\x08\x16\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x08\
    \x04\x12\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x08\x04\n\n\x0c\n\x05\x04\
    \x01\x02\0\x01\x12\x03\x08\x0b\r\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\
    \x08\x10\x11\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\t\x04\x14\n\x0c\n\x05\
    \x04\x01\x02\x01\x05\x12\x03\t\x04\n\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\
    \x03\t\x0b\x0f\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\t\x12\x13\n\x0b\n\
    \x04\x04\x01\x02\x02\x12\x03\n\x04\x14\n\x0c\n\x05\x04\x01\x02\x02\x05\
    \x12\x03\n\x04\n\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\n\x0b\x0f\n\x0c\
    \n\x05\x04\x01\x02\x02\x03\x12\x03\n\x12\x13\n\x0b\n\x04\x04\x01\x02\x03\
    \x12\x03\x0b\x04\x14\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x0b\x04\n\n\
    \x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x0b\x0b\x0f\n\x0c\n\x05\x04\x01\
    \x02\x03\x03\x12\x03\x0b\x12\x13\n\n\n\x02\x04\x02\x12\x04\r\0\x10\x01\n\
    \n\n\x03\x04\x02\x01\x12\x03\r\x08\x0b\n\x0b\n\x04\x04\x02\x02\0\x12\x03\
    \x0e\x04\x1c\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x0e\x04\x12\n\x0c\n\
    \x05\x04\x02\x02\0\x01\x12\x03\x0e\x13\x17\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x03\x0e\x1a\x1b\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x0f\x04\x17\n\
    \x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x0f\x04\n\n\x0c\n\x05\x04\x02\x02\
    \x01\x01\x12\x03\x0f\x0b\x12\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x0f\
    \x15\x16b\x06proto3\
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
