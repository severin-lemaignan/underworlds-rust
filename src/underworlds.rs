// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Empty {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Empty {}

impl Empty {
    pub fn new() -> Empty {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Empty {
        static mut instance: ::protobuf::lazy::Lazy<Empty> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Empty,
        };
        unsafe {
            instance.get(Empty::new)
        }
    }
}

impl ::protobuf::Message for Empty {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Empty {
    fn new() -> Empty {
        Empty::new()
    }

    fn descriptor_static(_: ::std::option::Option<Empty>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Empty>(
                    "Empty",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Empty {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Empty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Empty {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Bool {
    // message fields
    pub value: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Bool {}

impl Bool {
    pub fn new() -> Bool {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Bool {
        static mut instance: ::protobuf::lazy::Lazy<Bool> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Bool,
        };
        unsafe {
            instance.get(Bool::new)
        }
    }

    // bool value = 1;

    pub fn clear_value(&mut self) {
        self.value = false;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: bool) {
        self.value = v;
    }

    pub fn get_value(&self) -> bool {
        self.value
    }

    fn get_value_for_reflect(&self) -> &bool {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut bool {
        &mut self.value
    }
}

impl ::protobuf::Message for Bool {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.value = tmp;
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
        if self.value != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.value != false {
            os.write_bool(1, self.value)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Bool {
    fn new() -> Bool {
        Bool::new()
    }

    fn descriptor_static(_: ::std::option::Option<Bool>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "value",
                    Bool::get_value_for_reflect,
                    Bool::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Bool>(
                    "Bool",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Bool {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Bool {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Bool {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Time {
    // message fields
    pub time: f64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Time {}

impl Time {
    pub fn new() -> Time {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Time {
        static mut instance: ::protobuf::lazy::Lazy<Time> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Time,
        };
        unsafe {
            instance.get(Time::new)
        }
    }

    // double time = 1;

    pub fn clear_time(&mut self) {
        self.time = 0.;
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: f64) {
        self.time = v;
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    fn get_time_for_reflect(&self) -> &f64 {
        &self.time
    }

    fn mut_time_for_reflect(&mut self) -> &mut f64 {
        &mut self.time
    }
}

impl ::protobuf::Message for Time {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.time = tmp;
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
        if self.time != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.time != 0. {
            os.write_double(1, self.time)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Time {
    fn new() -> Time {
        Time::new()
    }

    fn descriptor_static(_: ::std::option::Option<Time>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "time",
                    Time::get_time_for_reflect,
                    Time::mut_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Time>(
                    "Time",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Time {
    fn clear(&mut self) {
        self.clear_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Time {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Time {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Name {
    // message fields
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Name {}

impl Name {
    pub fn new() -> Name {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Name {
        static mut instance: ::protobuf::lazy::Lazy<Name> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Name,
        };
        unsafe {
            instance.get(Name::new)
        }
    }

    // string name = 1;

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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for Name {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Name {
    fn new() -> Name {
        Name::new()
    }

    fn descriptor_static(_: ::std::option::Option<Name>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Name::get_name_for_reflect,
                    Name::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Name>(
                    "Name",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Name {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Name {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Name {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Size {
    // message fields
    pub size: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Size {}

impl Size {
    pub fn new() -> Size {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Size {
        static mut instance: ::protobuf::lazy::Lazy<Size> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Size,
        };
        unsafe {
            instance.get(Size::new)
        }
    }

    // int32 size = 1;

    pub fn clear_size(&mut self) {
        self.size = 0;
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: i32) {
        self.size = v;
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    fn get_size_for_reflect(&self) -> &i32 {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut i32 {
        &mut self.size
    }
}

impl ::protobuf::Message for Size {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.size = tmp;
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
        if self.size != 0 {
            my_size += ::protobuf::rt::value_size(1, self.size, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.size != 0 {
            os.write_int32(1, self.size)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Size {
    fn new() -> Size {
        Size::new()
    }

    fn descriptor_static(_: ::std::option::Option<Size>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "size",
                    Size::get_size_for_reflect,
                    Size::mut_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Size>(
                    "Size",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Size {
    fn clear(&mut self) {
        self.clear_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Size {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Size {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Pointf {
    // message fields
    pub x: f32,
    pub y: f32,
    pub z: f32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Pointf {}

impl Pointf {
    pub fn new() -> Pointf {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Pointf {
        static mut instance: ::protobuf::lazy::Lazy<Pointf> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Pointf,
        };
        unsafe {
            instance.get(Pointf::new)
        }
    }

    // float x = 1;

    pub fn clear_x(&mut self) {
        self.x = 0.;
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = v;
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    fn get_x_for_reflect(&self) -> &f32 {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut f32 {
        &mut self.x
    }

    // float y = 2;

    pub fn clear_y(&mut self) {
        self.y = 0.;
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = v;
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    fn get_y_for_reflect(&self) -> &f32 {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut f32 {
        &mut self.y
    }

    // float z = 3;

    pub fn clear_z(&mut self) {
        self.z = 0.;
    }

    // Param is passed by value, moved
    pub fn set_z(&mut self, v: f32) {
        self.z = v;
    }

    pub fn get_z(&self) -> f32 {
        self.z
    }

    fn get_z_for_reflect(&self) -> &f32 {
        &self.z
    }

    fn mut_z_for_reflect(&mut self) -> &mut f32 {
        &mut self.z
    }
}

impl ::protobuf::Message for Pointf {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.x = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.y = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.z = tmp;
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
        if self.x != 0. {
            my_size += 5;
        }
        if self.y != 0. {
            my_size += 5;
        }
        if self.z != 0. {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.x != 0. {
            os.write_float(1, self.x)?;
        }
        if self.y != 0. {
            os.write_float(2, self.y)?;
        }
        if self.z != 0. {
            os.write_float(3, self.z)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Pointf {
    fn new() -> Pointf {
        Pointf::new()
    }

    fn descriptor_static(_: ::std::option::Option<Pointf>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "x",
                    Pointf::get_x_for_reflect,
                    Pointf::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "y",
                    Pointf::get_y_for_reflect,
                    Pointf::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "z",
                    Pointf::get_z_for_reflect,
                    Pointf::mut_z_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Pointf>(
                    "Pointf",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Pointf {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_z();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Pointf {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Pointf {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Point {
    // message fields
    pub x: i32,
    pub y: i32,
    pub z: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Point {}

impl Point {
    pub fn new() -> Point {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Point {
        static mut instance: ::protobuf::lazy::Lazy<Point> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Point,
        };
        unsafe {
            instance.get(Point::new)
        }
    }

    // sint32 x = 1;

    pub fn clear_x(&mut self) {
        self.x = 0;
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: i32) {
        self.x = v;
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    fn get_x_for_reflect(&self) -> &i32 {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut i32 {
        &mut self.x
    }

    // sint32 y = 2;

    pub fn clear_y(&mut self) {
        self.y = 0;
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: i32) {
        self.y = v;
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    fn get_y_for_reflect(&self) -> &i32 {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut i32 {
        &mut self.y
    }

    // sint32 z = 3;

    pub fn clear_z(&mut self) {
        self.z = 0;
    }

    // Param is passed by value, moved
    pub fn set_z(&mut self, v: i32) {
        self.z = v;
    }

    pub fn get_z(&self) -> i32 {
        self.z
    }

    fn get_z_for_reflect(&self) -> &i32 {
        &self.z
    }

    fn mut_z_for_reflect(&mut self) -> &mut i32 {
        &mut self.z
    }
}

impl ::protobuf::Message for Point {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.x = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.y = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.z = tmp;
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
        if self.x != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, self.x);
        }
        if self.y != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, self.y);
        }
        if self.z != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, self.z);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.x != 0 {
            os.write_sint32(1, self.x)?;
        }
        if self.y != 0 {
            os.write_sint32(2, self.y)?;
        }
        if self.z != 0 {
            os.write_sint32(3, self.z)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Point {
    fn new() -> Point {
        Point::new()
    }

    fn descriptor_static(_: ::std::option::Option<Point>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "x",
                    Point::get_x_for_reflect,
                    Point::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "y",
                    Point::get_y_for_reflect,
                    Point::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "z",
                    Point::get_z_for_reflect,
                    Point::mut_z_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Point>(
                    "Point",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Point {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_z();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Point {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Point {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Color {
    // message fields
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Color {}

impl Color {
    pub fn new() -> Color {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Color {
        static mut instance: ::protobuf::lazy::Lazy<Color> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Color,
        };
        unsafe {
            instance.get(Color::new)
        }
    }

    // float r = 1;

    pub fn clear_r(&mut self) {
        self.r = 0.;
    }

    // Param is passed by value, moved
    pub fn set_r(&mut self, v: f32) {
        self.r = v;
    }

    pub fn get_r(&self) -> f32 {
        self.r
    }

    fn get_r_for_reflect(&self) -> &f32 {
        &self.r
    }

    fn mut_r_for_reflect(&mut self) -> &mut f32 {
        &mut self.r
    }

    // float g = 2;

    pub fn clear_g(&mut self) {
        self.g = 0.;
    }

    // Param is passed by value, moved
    pub fn set_g(&mut self, v: f32) {
        self.g = v;
    }

    pub fn get_g(&self) -> f32 {
        self.g
    }

    fn get_g_for_reflect(&self) -> &f32 {
        &self.g
    }

    fn mut_g_for_reflect(&mut self) -> &mut f32 {
        &mut self.g
    }

    // float b = 3;

    pub fn clear_b(&mut self) {
        self.b = 0.;
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: f32) {
        self.b = v;
    }

    pub fn get_b(&self) -> f32 {
        self.b
    }

    fn get_b_for_reflect(&self) -> &f32 {
        &self.b
    }

    fn mut_b_for_reflect(&mut self) -> &mut f32 {
        &mut self.b
    }

    // float a = 4;

    pub fn clear_a(&mut self) {
        self.a = 0.;
    }

    // Param is passed by value, moved
    pub fn set_a(&mut self, v: f32) {
        self.a = v;
    }

    pub fn get_a(&self) -> f32 {
        self.a
    }

    fn get_a_for_reflect(&self) -> &f32 {
        &self.a
    }

    fn mut_a_for_reflect(&mut self) -> &mut f32 {
        &mut self.a
    }
}

impl ::protobuf::Message for Color {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.r = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.g = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.b = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.a = tmp;
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
        if self.r != 0. {
            my_size += 5;
        }
        if self.g != 0. {
            my_size += 5;
        }
        if self.b != 0. {
            my_size += 5;
        }
        if self.a != 0. {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.r != 0. {
            os.write_float(1, self.r)?;
        }
        if self.g != 0. {
            os.write_float(2, self.g)?;
        }
        if self.b != 0. {
            os.write_float(3, self.b)?;
        }
        if self.a != 0. {
            os.write_float(4, self.a)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Color {
    fn new() -> Color {
        Color::new()
    }

    fn descriptor_static(_: ::std::option::Option<Color>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "r",
                    Color::get_r_for_reflect,
                    Color::mut_r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "g",
                    Color::get_g_for_reflect,
                    Color::mut_g_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "b",
                    Color::get_b_for_reflect,
                    Color::mut_b_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "a",
                    Color::get_a_for_reflect,
                    Color::mut_a_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Color>(
                    "Color",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Color {
    fn clear(&mut self) {
        self.clear_r();
        self.clear_g();
        self.clear_b();
        self.clear_a();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Color {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Color {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Client {
    // message fields
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub links: ::protobuf::RepeatedField<ClientInteraction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Client {}

impl Client {
    pub fn new() -> Client {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Client {
        static mut instance: ::protobuf::lazy::Lazy<Client> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Client,
        };
        unsafe {
            instance.get(Client::new)
        }
    }

    // string id = 1;

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

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // string name = 2;

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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // repeated .underworlds.ClientInteraction links = 3;

    pub fn clear_links(&mut self) {
        self.links.clear();
    }

    // Param is passed by value, moved
    pub fn set_links(&mut self, v: ::protobuf::RepeatedField<ClientInteraction>) {
        self.links = v;
    }

    // Mutable pointer to the field.
    pub fn mut_links(&mut self) -> &mut ::protobuf::RepeatedField<ClientInteraction> {
        &mut self.links
    }

    // Take field
    pub fn take_links(&mut self) -> ::protobuf::RepeatedField<ClientInteraction> {
        ::std::mem::replace(&mut self.links, ::protobuf::RepeatedField::new())
    }

    pub fn get_links(&self) -> &[ClientInteraction] {
        &self.links
    }

    fn get_links_for_reflect(&self) -> &::protobuf::RepeatedField<ClientInteraction> {
        &self.links
    }

    fn mut_links_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ClientInteraction> {
        &mut self.links
    }
}

impl ::protobuf::Message for Client {
    fn is_initialized(&self) -> bool {
        for v in &self.links {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.links)?;
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
        for value in &self.links {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        for v in &self.links {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Client {
    fn new() -> Client {
        Client::new()
    }

    fn descriptor_static(_: ::std::option::Option<Client>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    Client::get_id_for_reflect,
                    Client::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Client::get_name_for_reflect,
                    Client::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClientInteraction>>(
                    "links",
                    Client::get_links_for_reflect,
                    Client::mut_links_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Client>(
                    "Client",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Client {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_name();
        self.clear_links();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Client {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Client {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientInteraction {
    // message fields
    pub world: ::std::string::String,
    pub field_type: ClientInteraction_InteractionType,
    pub last_activity: ::protobuf::SingularPtrField<Time>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientInteraction {}

impl ClientInteraction {
    pub fn new() -> ClientInteraction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientInteraction {
        static mut instance: ::protobuf::lazy::Lazy<ClientInteraction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientInteraction,
        };
        unsafe {
            instance.get(ClientInteraction::new)
        }
    }

    // string world = 1;

    pub fn clear_world(&mut self) {
        self.world.clear();
    }

    // Param is passed by value, moved
    pub fn set_world(&mut self, v: ::std::string::String) {
        self.world = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_world(&mut self) -> &mut ::std::string::String {
        &mut self.world
    }

    // Take field
    pub fn take_world(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.world, ::std::string::String::new())
    }

    pub fn get_world(&self) -> &str {
        &self.world
    }

    fn get_world_for_reflect(&self) -> &::std::string::String {
        &self.world
    }

    fn mut_world_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.world
    }

    // .underworlds.ClientInteraction.InteractionType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ClientInteraction_InteractionType::READER;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ClientInteraction_InteractionType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> ClientInteraction_InteractionType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &ClientInteraction_InteractionType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ClientInteraction_InteractionType {
        &mut self.field_type
    }

    // .underworlds.Time last_activity = 3;

    pub fn clear_last_activity(&mut self) {
        self.last_activity.clear();
    }

    pub fn has_last_activity(&self) -> bool {
        self.last_activity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_activity(&mut self, v: Time) {
        self.last_activity = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_activity(&mut self) -> &mut Time {
        if self.last_activity.is_none() {
            self.last_activity.set_default();
        }
        self.last_activity.as_mut().unwrap()
    }

    // Take field
    pub fn take_last_activity(&mut self) -> Time {
        self.last_activity.take().unwrap_or_else(|| Time::new())
    }

    pub fn get_last_activity(&self) -> &Time {
        self.last_activity.as_ref().unwrap_or_else(|| Time::default_instance())
    }

    fn get_last_activity_for_reflect(&self) -> &::protobuf::SingularPtrField<Time> {
        &self.last_activity
    }

    fn mut_last_activity_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Time> {
        &mut self.last_activity
    }
}

impl ::protobuf::Message for ClientInteraction {
    fn is_initialized(&self) -> bool {
        for v in &self.last_activity {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.world)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.last_activity)?;
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
        if !self.world.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.world);
        }
        if self.field_type != ClientInteraction_InteractionType::READER {
            my_size += ::protobuf::rt::enum_size(2, self.field_type);
        }
        if let Some(ref v) = self.last_activity.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.world.is_empty() {
            os.write_string(1, &self.world)?;
        }
        if self.field_type != ClientInteraction_InteractionType::READER {
            os.write_enum(2, self.field_type.value())?;
        }
        if let Some(ref v) = self.last_activity.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClientInteraction {
    fn new() -> ClientInteraction {
        ClientInteraction::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientInteraction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "world",
                    ClientInteraction::get_world_for_reflect,
                    ClientInteraction::mut_world_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ClientInteraction_InteractionType>>(
                    "type",
                    ClientInteraction::get_field_type_for_reflect,
                    ClientInteraction::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Time>>(
                    "last_activity",
                    ClientInteraction::get_last_activity_for_reflect,
                    ClientInteraction::mut_last_activity_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientInteraction>(
                    "ClientInteraction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientInteraction {
    fn clear(&mut self) {
        self.clear_world();
        self.clear_field_type();
        self.clear_last_activity();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientInteraction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientInteraction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ClientInteraction_InteractionType {
    READER = 0,
    PROVIDER = 1,
    MONITOR = 2,
    FILTER = 3,
}

impl ::protobuf::ProtobufEnum for ClientInteraction_InteractionType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ClientInteraction_InteractionType> {
        match value {
            0 => ::std::option::Option::Some(ClientInteraction_InteractionType::READER),
            1 => ::std::option::Option::Some(ClientInteraction_InteractionType::PROVIDER),
            2 => ::std::option::Option::Some(ClientInteraction_InteractionType::MONITOR),
            3 => ::std::option::Option::Some(ClientInteraction_InteractionType::FILTER),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ClientInteraction_InteractionType] = &[
            ClientInteraction_InteractionType::READER,
            ClientInteraction_InteractionType::PROVIDER,
            ClientInteraction_InteractionType::MONITOR,
            ClientInteraction_InteractionType::FILTER,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ClientInteraction_InteractionType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ClientInteraction_InteractionType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ClientInteraction_InteractionType {
}

impl ::std::default::Default for ClientInteraction_InteractionType {
    fn default() -> Self {
        ClientInteraction_InteractionType::READER
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientInteraction_InteractionType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Context {
    // message fields
    pub client: ::std::string::String,
    pub world: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Context {}

impl Context {
    pub fn new() -> Context {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Context {
        static mut instance: ::protobuf::lazy::Lazy<Context> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Context,
        };
        unsafe {
            instance.get(Context::new)
        }
    }

    // string client = 1;

    pub fn clear_client(&mut self) {
        self.client.clear();
    }

    // Param is passed by value, moved
    pub fn set_client(&mut self, v: ::std::string::String) {
        self.client = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client(&mut self) -> &mut ::std::string::String {
        &mut self.client
    }

    // Take field
    pub fn take_client(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.client, ::std::string::String::new())
    }

    pub fn get_client(&self) -> &str {
        &self.client
    }

    fn get_client_for_reflect(&self) -> &::std::string::String {
        &self.client
    }

    fn mut_client_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.client
    }

    // string world = 2;

    pub fn clear_world(&mut self) {
        self.world.clear();
    }

    // Param is passed by value, moved
    pub fn set_world(&mut self, v: ::std::string::String) {
        self.world = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_world(&mut self) -> &mut ::std::string::String {
        &mut self.world
    }

    // Take field
    pub fn take_world(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.world, ::std::string::String::new())
    }

    pub fn get_world(&self) -> &str {
        &self.world
    }

    fn get_world_for_reflect(&self) -> &::std::string::String {
        &self.world
    }

    fn mut_world_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.world
    }
}

impl ::protobuf::Message for Context {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.client)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.world)?;
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
        if !self.client.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.client);
        }
        if !self.world.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.world);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.client.is_empty() {
            os.write_string(1, &self.client)?;
        }
        if !self.world.is_empty() {
            os.write_string(2, &self.world)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Context {
    fn new() -> Context {
        Context::new()
    }

    fn descriptor_static(_: ::std::option::Option<Context>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "client",
                    Context::get_client_for_reflect,
                    Context::mut_client_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "world",
                    Context::get_world_for_reflect,
                    Context::mut_world_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Context>(
                    "Context",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Context {
    fn clear(&mut self) {
        self.clear_client();
        self.clear_world();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Context {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Context {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Topology {
    // message fields
    pub worlds: ::protobuf::RepeatedField<::std::string::String>,
    pub clients: ::protobuf::RepeatedField<Client>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Topology {}

impl Topology {
    pub fn new() -> Topology {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Topology {
        static mut instance: ::protobuf::lazy::Lazy<Topology> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Topology,
        };
        unsafe {
            instance.get(Topology::new)
        }
    }

    // repeated string worlds = 1;

    pub fn clear_worlds(&mut self) {
        self.worlds.clear();
    }

    // Param is passed by value, moved
    pub fn set_worlds(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.worlds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_worlds(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.worlds
    }

    // Take field
    pub fn take_worlds(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.worlds, ::protobuf::RepeatedField::new())
    }

    pub fn get_worlds(&self) -> &[::std::string::String] {
        &self.worlds
    }

    fn get_worlds_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.worlds
    }

    fn mut_worlds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.worlds
    }

    // repeated .underworlds.Client clients = 2;

    pub fn clear_clients(&mut self) {
        self.clients.clear();
    }

    // Param is passed by value, moved
    pub fn set_clients(&mut self, v: ::protobuf::RepeatedField<Client>) {
        self.clients = v;
    }

    // Mutable pointer to the field.
    pub fn mut_clients(&mut self) -> &mut ::protobuf::RepeatedField<Client> {
        &mut self.clients
    }

    // Take field
    pub fn take_clients(&mut self) -> ::protobuf::RepeatedField<Client> {
        ::std::mem::replace(&mut self.clients, ::protobuf::RepeatedField::new())
    }

    pub fn get_clients(&self) -> &[Client] {
        &self.clients
    }

    fn get_clients_for_reflect(&self) -> &::protobuf::RepeatedField<Client> {
        &self.clients
    }

    fn mut_clients_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Client> {
        &mut self.clients
    }
}

impl ::protobuf::Message for Topology {
    fn is_initialized(&self) -> bool {
        for v in &self.clients {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.worlds)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.clients)?;
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
        for value in &self.worlds {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.clients {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.worlds {
            os.write_string(1, &v)?;
        };
        for v in &self.clients {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Topology {
    fn new() -> Topology {
        Topology::new()
    }

    fn descriptor_static(_: ::std::option::Option<Topology>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "worlds",
                    Topology::get_worlds_for_reflect,
                    Topology::mut_worlds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Client>>(
                    "clients",
                    Topology::get_clients_for_reflect,
                    Topology::mut_clients_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Topology>(
                    "Topology",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Topology {
    fn clear(&mut self) {
        self.clear_worlds();
        self.clear_clients();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Topology {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Topology {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Node {
    // message fields
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub field_type: Node_NodeType,
    pub parent: ::std::string::String,
    pub children: ::protobuf::RepeatedField<::std::string::String>,
    pub transformation: ::std::vec::Vec<f32>,
    pub last_update: f64,
    pub lowres: ::protobuf::RepeatedField<::std::string::String>,
    pub hires: ::protobuf::RepeatedField<::std::string::String>,
    pub cad: ::protobuf::RepeatedField<::std::string::String>,
    pub aabb: ::protobuf::RepeatedField<Pointf>,
    pub physics: bool,
    pub clipplanenear: f32,
    pub clipplanefar: f32,
    pub aspect: f32,
    pub horizontalfov: f32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Node {}

impl Node {
    pub fn new() -> Node {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Node {
        static mut instance: ::protobuf::lazy::Lazy<Node> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Node,
        };
        unsafe {
            instance.get(Node::new)
        }
    }

    // string id = 1;

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

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // string name = 2;

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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .underworlds.Node.NodeType type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type = Node_NodeType::UNDEFINED;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Node_NodeType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> Node_NodeType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &Node_NodeType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut Node_NodeType {
        &mut self.field_type
    }

    // string parent = 4;

    pub fn clear_parent(&mut self) {
        self.parent.clear();
    }

    // Param is passed by value, moved
    pub fn set_parent(&mut self, v: ::std::string::String) {
        self.parent = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parent(&mut self) -> &mut ::std::string::String {
        &mut self.parent
    }

    // Take field
    pub fn take_parent(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.parent, ::std::string::String::new())
    }

    pub fn get_parent(&self) -> &str {
        &self.parent
    }

    fn get_parent_for_reflect(&self) -> &::std::string::String {
        &self.parent
    }

    fn mut_parent_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.parent
    }

    // repeated string children = 5;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.children, ::protobuf::RepeatedField::new())
    }

    pub fn get_children(&self) -> &[::std::string::String] {
        &self.children
    }

    fn get_children_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.children
    }

    fn mut_children_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.children
    }

    // repeated float transformation = 6;

    pub fn clear_transformation(&mut self) {
        self.transformation.clear();
    }

    // Param is passed by value, moved
    pub fn set_transformation(&mut self, v: ::std::vec::Vec<f32>) {
        self.transformation = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transformation(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.transformation
    }

    // Take field
    pub fn take_transformation(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.transformation, ::std::vec::Vec::new())
    }

    pub fn get_transformation(&self) -> &[f32] {
        &self.transformation
    }

    fn get_transformation_for_reflect(&self) -> &::std::vec::Vec<f32> {
        &self.transformation
    }

    fn mut_transformation_for_reflect(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.transformation
    }

    // double last_update = 8;

    pub fn clear_last_update(&mut self) {
        self.last_update = 0.;
    }

    // Param is passed by value, moved
    pub fn set_last_update(&mut self, v: f64) {
        self.last_update = v;
    }

    pub fn get_last_update(&self) -> f64 {
        self.last_update
    }

    fn get_last_update_for_reflect(&self) -> &f64 {
        &self.last_update
    }

    fn mut_last_update_for_reflect(&mut self) -> &mut f64 {
        &mut self.last_update
    }

    // repeated string lowres = 16;

    pub fn clear_lowres(&mut self) {
        self.lowres.clear();
    }

    // Param is passed by value, moved
    pub fn set_lowres(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.lowres = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lowres(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.lowres
    }

    // Take field
    pub fn take_lowres(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.lowres, ::protobuf::RepeatedField::new())
    }

    pub fn get_lowres(&self) -> &[::std::string::String] {
        &self.lowres
    }

    fn get_lowres_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.lowres
    }

    fn mut_lowres_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.lowres
    }

    // repeated string hires = 17;

    pub fn clear_hires(&mut self) {
        self.hires.clear();
    }

    // Param is passed by value, moved
    pub fn set_hires(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.hires = v;
    }

    // Mutable pointer to the field.
    pub fn mut_hires(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.hires
    }

    // Take field
    pub fn take_hires(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.hires, ::protobuf::RepeatedField::new())
    }

    pub fn get_hires(&self) -> &[::std::string::String] {
        &self.hires
    }

    fn get_hires_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.hires
    }

    fn mut_hires_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.hires
    }

    // repeated string cad = 18;

    pub fn clear_cad(&mut self) {
        self.cad.clear();
    }

    // Param is passed by value, moved
    pub fn set_cad(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.cad = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cad(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.cad
    }

    // Take field
    pub fn take_cad(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.cad, ::protobuf::RepeatedField::new())
    }

    pub fn get_cad(&self) -> &[::std::string::String] {
        &self.cad
    }

    fn get_cad_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.cad
    }

    fn mut_cad_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.cad
    }

    // repeated .underworlds.Pointf aabb = 19;

    pub fn clear_aabb(&mut self) {
        self.aabb.clear();
    }

    // Param is passed by value, moved
    pub fn set_aabb(&mut self, v: ::protobuf::RepeatedField<Pointf>) {
        self.aabb = v;
    }

    // Mutable pointer to the field.
    pub fn mut_aabb(&mut self) -> &mut ::protobuf::RepeatedField<Pointf> {
        &mut self.aabb
    }

    // Take field
    pub fn take_aabb(&mut self) -> ::protobuf::RepeatedField<Pointf> {
        ::std::mem::replace(&mut self.aabb, ::protobuf::RepeatedField::new())
    }

    pub fn get_aabb(&self) -> &[Pointf] {
        &self.aabb
    }

    fn get_aabb_for_reflect(&self) -> &::protobuf::RepeatedField<Pointf> {
        &self.aabb
    }

    fn mut_aabb_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Pointf> {
        &mut self.aabb
    }

    // bool physics = 30;

    pub fn clear_physics(&mut self) {
        self.physics = false;
    }

    // Param is passed by value, moved
    pub fn set_physics(&mut self, v: bool) {
        self.physics = v;
    }

    pub fn get_physics(&self) -> bool {
        self.physics
    }

    fn get_physics_for_reflect(&self) -> &bool {
        &self.physics
    }

    fn mut_physics_for_reflect(&mut self) -> &mut bool {
        &mut self.physics
    }

    // float clipplanenear = 40;

    pub fn clear_clipplanenear(&mut self) {
        self.clipplanenear = 0.;
    }

    // Param is passed by value, moved
    pub fn set_clipplanenear(&mut self, v: f32) {
        self.clipplanenear = v;
    }

    pub fn get_clipplanenear(&self) -> f32 {
        self.clipplanenear
    }

    fn get_clipplanenear_for_reflect(&self) -> &f32 {
        &self.clipplanenear
    }

    fn mut_clipplanenear_for_reflect(&mut self) -> &mut f32 {
        &mut self.clipplanenear
    }

    // float clipplanefar = 41;

    pub fn clear_clipplanefar(&mut self) {
        self.clipplanefar = 0.;
    }

    // Param is passed by value, moved
    pub fn set_clipplanefar(&mut self, v: f32) {
        self.clipplanefar = v;
    }

    pub fn get_clipplanefar(&self) -> f32 {
        self.clipplanefar
    }

    fn get_clipplanefar_for_reflect(&self) -> &f32 {
        &self.clipplanefar
    }

    fn mut_clipplanefar_for_reflect(&mut self) -> &mut f32 {
        &mut self.clipplanefar
    }

    // float aspect = 42;

    pub fn clear_aspect(&mut self) {
        self.aspect = 0.;
    }

    // Param is passed by value, moved
    pub fn set_aspect(&mut self, v: f32) {
        self.aspect = v;
    }

    pub fn get_aspect(&self) -> f32 {
        self.aspect
    }

    fn get_aspect_for_reflect(&self) -> &f32 {
        &self.aspect
    }

    fn mut_aspect_for_reflect(&mut self) -> &mut f32 {
        &mut self.aspect
    }

    // float horizontalfov = 43;

    pub fn clear_horizontalfov(&mut self) {
        self.horizontalfov = 0.;
    }

    // Param is passed by value, moved
    pub fn set_horizontalfov(&mut self, v: f32) {
        self.horizontalfov = v;
    }

    pub fn get_horizontalfov(&self) -> f32 {
        self.horizontalfov
    }

    fn get_horizontalfov_for_reflect(&self) -> &f32 {
        &self.horizontalfov
    }

    fn mut_horizontalfov_for_reflect(&mut self) -> &mut f32 {
        &mut self.horizontalfov
    }
}

impl ::protobuf::Message for Node {
    fn is_initialized(&self) -> bool {
        for v in &self.aabb {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.parent)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.children)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.transformation)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.last_update = tmp;
                },
                16 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.lowres)?;
                },
                17 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.hires)?;
                },
                18 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.cad)?;
                },
                19 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.aabb)?;
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.physics = tmp;
                },
                40 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.clipplanenear = tmp;
                },
                41 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.clipplanefar = tmp;
                },
                42 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.aspect = tmp;
                },
                43 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.horizontalfov = tmp;
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
        if self.field_type != Node_NodeType::UNDEFINED {
            my_size += ::protobuf::rt::enum_size(3, self.field_type);
        }
        if !self.parent.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.parent);
        }
        for value in &self.children {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += 5 * self.transformation.len() as u32;
        if self.last_update != 0. {
            my_size += 9;
        }
        for value in &self.lowres {
            my_size += ::protobuf::rt::string_size(16, &value);
        };
        for value in &self.hires {
            my_size += ::protobuf::rt::string_size(17, &value);
        };
        for value in &self.cad {
            my_size += ::protobuf::rt::string_size(18, &value);
        };
        for value in &self.aabb {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.physics != false {
            my_size += 3;
        }
        if self.clipplanenear != 0. {
            my_size += 6;
        }
        if self.clipplanefar != 0. {
            my_size += 6;
        }
        if self.aspect != 0. {
            my_size += 6;
        }
        if self.horizontalfov != 0. {
            my_size += 6;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if self.field_type != Node_NodeType::UNDEFINED {
            os.write_enum(3, self.field_type.value())?;
        }
        if !self.parent.is_empty() {
            os.write_string(4, &self.parent)?;
        }
        for v in &self.children {
            os.write_string(5, &v)?;
        };
        for v in &self.transformation {
            os.write_float(6, *v)?;
        };
        if self.last_update != 0. {
            os.write_double(8, self.last_update)?;
        }
        for v in &self.lowres {
            os.write_string(16, &v)?;
        };
        for v in &self.hires {
            os.write_string(17, &v)?;
        };
        for v in &self.cad {
            os.write_string(18, &v)?;
        };
        for v in &self.aabb {
            os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.physics != false {
            os.write_bool(30, self.physics)?;
        }
        if self.clipplanenear != 0. {
            os.write_float(40, self.clipplanenear)?;
        }
        if self.clipplanefar != 0. {
            os.write_float(41, self.clipplanefar)?;
        }
        if self.aspect != 0. {
            os.write_float(42, self.aspect)?;
        }
        if self.horizontalfov != 0. {
            os.write_float(43, self.horizontalfov)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Node {
    fn new() -> Node {
        Node::new()
    }

    fn descriptor_static(_: ::std::option::Option<Node>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    Node::get_id_for_reflect,
                    Node::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Node::get_name_for_reflect,
                    Node::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Node_NodeType>>(
                    "type",
                    Node::get_field_type_for_reflect,
                    Node::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "parent",
                    Node::get_parent_for_reflect,
                    Node::mut_parent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "children",
                    Node::get_children_for_reflect,
                    Node::mut_children_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "transformation",
                    Node::get_transformation_for_reflect,
                    Node::mut_transformation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "last_update",
                    Node::get_last_update_for_reflect,
                    Node::mut_last_update_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "lowres",
                    Node::get_lowres_for_reflect,
                    Node::mut_lowres_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hires",
                    Node::get_hires_for_reflect,
                    Node::mut_hires_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cad",
                    Node::get_cad_for_reflect,
                    Node::mut_cad_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Pointf>>(
                    "aabb",
                    Node::get_aabb_for_reflect,
                    Node::mut_aabb_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "physics",
                    Node::get_physics_for_reflect,
                    Node::mut_physics_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "clipplanenear",
                    Node::get_clipplanenear_for_reflect,
                    Node::mut_clipplanenear_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "clipplanefar",
                    Node::get_clipplanefar_for_reflect,
                    Node::mut_clipplanefar_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "aspect",
                    Node::get_aspect_for_reflect,
                    Node::mut_aspect_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "horizontalfov",
                    Node::get_horizontalfov_for_reflect,
                    Node::mut_horizontalfov_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Node>(
                    "Node",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Node {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_name();
        self.clear_field_type();
        self.clear_parent();
        self.clear_children();
        self.clear_transformation();
        self.clear_last_update();
        self.clear_lowres();
        self.clear_hires();
        self.clear_cad();
        self.clear_aabb();
        self.clear_physics();
        self.clear_clipplanenear();
        self.clear_clipplanefar();
        self.clear_aspect();
        self.clear_horizontalfov();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Node {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Node_NodeType {
    UNDEFINED = 0,
    ENTITY = 1,
    MESH = 2,
    CAMERA = 3,
}

impl ::protobuf::ProtobufEnum for Node_NodeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Node_NodeType> {
        match value {
            0 => ::std::option::Option::Some(Node_NodeType::UNDEFINED),
            1 => ::std::option::Option::Some(Node_NodeType::ENTITY),
            2 => ::std::option::Option::Some(Node_NodeType::MESH),
            3 => ::std::option::Option::Some(Node_NodeType::CAMERA),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Node_NodeType] = &[
            Node_NodeType::UNDEFINED,
            Node_NodeType::ENTITY,
            Node_NodeType::MESH,
            Node_NodeType::CAMERA,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Node_NodeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Node_NodeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Node_NodeType {
}

impl ::std::default::Default for Node_NodeType {
    fn default() -> Self {
        Node_NodeType::UNDEFINED
    }
}

impl ::protobuf::reflect::ProtobufValue for Node_NodeType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Nodes {
    // message fields
    pub ids: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Nodes {}

impl Nodes {
    pub fn new() -> Nodes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Nodes {
        static mut instance: ::protobuf::lazy::Lazy<Nodes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Nodes,
        };
        unsafe {
            instance.get(Nodes::new)
        }
    }

    // repeated string ids = 1;

    pub fn clear_ids(&mut self) {
        self.ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_ids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ids(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.ids
    }

    // Take field
    pub fn take_ids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.ids, ::protobuf::RepeatedField::new())
    }

    pub fn get_ids(&self) -> &[::std::string::String] {
        &self.ids
    }

    fn get_ids_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.ids
    }

    fn mut_ids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.ids
    }
}

impl ::protobuf::Message for Nodes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.ids)?;
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
        for value in &self.ids {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.ids {
            os.write_string(1, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Nodes {
    fn new() -> Nodes {
        Nodes::new()
    }

    fn descriptor_static(_: ::std::option::Option<Nodes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ids",
                    Nodes::get_ids_for_reflect,
                    Nodes::mut_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Nodes>(
                    "Nodes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Nodes {
    fn clear(&mut self) {
        self.clear_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Nodes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Nodes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NodeInContext {
    // message fields
    pub context: ::protobuf::SingularPtrField<Context>,
    pub node: ::protobuf::SingularPtrField<Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NodeInContext {}

impl NodeInContext {
    pub fn new() -> NodeInContext {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodeInContext {
        static mut instance: ::protobuf::lazy::Lazy<NodeInContext> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeInContext,
        };
        unsafe {
            instance.get(NodeInContext::new)
        }
    }

    // .underworlds.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // .underworlds.Node node = 2;

    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    pub fn has_node(&self) -> bool {
        self.node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: Node) {
        self.node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node(&mut self) -> &mut Node {
        if self.node.is_none() {
            self.node.set_default();
        }
        self.node.as_mut().unwrap()
    }

    // Take field
    pub fn take_node(&mut self) -> Node {
        self.node.take().unwrap_or_else(|| Node::new())
    }

    pub fn get_node(&self) -> &Node {
        self.node.as_ref().unwrap_or_else(|| Node::default_instance())
    }

    fn get_node_for_reflect(&self) -> &::protobuf::SingularPtrField<Node> {
        &self.node
    }

    fn mut_node_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Node> {
        &mut self.node
    }
}

impl ::protobuf::Message for NodeInContext {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.node {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.node)?;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.node.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NodeInContext {
    fn new() -> NodeInContext {
        NodeInContext::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodeInContext>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    NodeInContext::get_context_for_reflect,
                    NodeInContext::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Node>>(
                    "node",
                    NodeInContext::get_node_for_reflect,
                    NodeInContext::mut_node_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeInContext>(
                    "NodeInContext",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodeInContext {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_node();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NodeInContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeInContext {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NodeInvalidation {
    // message fields
    pub field_type: NodeInvalidation_NodeInvalidationType,
    pub id: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NodeInvalidation {}

impl NodeInvalidation {
    pub fn new() -> NodeInvalidation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodeInvalidation {
        static mut instance: ::protobuf::lazy::Lazy<NodeInvalidation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeInvalidation,
        };
        unsafe {
            instance.get(NodeInvalidation::new)
        }
    }

    // .underworlds.NodeInvalidation.NodeInvalidationType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = NodeInvalidation_NodeInvalidationType::NEW;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: NodeInvalidation_NodeInvalidationType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> NodeInvalidation_NodeInvalidationType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &NodeInvalidation_NodeInvalidationType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut NodeInvalidation_NodeInvalidationType {
        &mut self.field_type
    }

    // string id = 2;

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

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }
}

impl ::protobuf::Message for NodeInvalidation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
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
        if self.field_type != NodeInvalidation_NodeInvalidationType::NEW {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != NodeInvalidation_NodeInvalidationType::NEW {
            os.write_enum(1, self.field_type.value())?;
        }
        if !self.id.is_empty() {
            os.write_string(2, &self.id)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NodeInvalidation {
    fn new() -> NodeInvalidation {
        NodeInvalidation::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodeInvalidation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<NodeInvalidation_NodeInvalidationType>>(
                    "type",
                    NodeInvalidation::get_field_type_for_reflect,
                    NodeInvalidation::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    NodeInvalidation::get_id_for_reflect,
                    NodeInvalidation::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeInvalidation>(
                    "NodeInvalidation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodeInvalidation {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NodeInvalidation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeInvalidation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NodeInvalidation_NodeInvalidationType {
    NEW = 0,
    UPDATE = 1,
    DELETE = 2,
}

impl ::protobuf::ProtobufEnum for NodeInvalidation_NodeInvalidationType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NodeInvalidation_NodeInvalidationType> {
        match value {
            0 => ::std::option::Option::Some(NodeInvalidation_NodeInvalidationType::NEW),
            1 => ::std::option::Option::Some(NodeInvalidation_NodeInvalidationType::UPDATE),
            2 => ::std::option::Option::Some(NodeInvalidation_NodeInvalidationType::DELETE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NodeInvalidation_NodeInvalidationType] = &[
            NodeInvalidation_NodeInvalidationType::NEW,
            NodeInvalidation_NodeInvalidationType::UPDATE,
            NodeInvalidation_NodeInvalidationType::DELETE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<NodeInvalidation_NodeInvalidationType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NodeInvalidation_NodeInvalidationType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NodeInvalidation_NodeInvalidationType {
}

impl ::std::default::Default for NodeInvalidation_NodeInvalidationType {
    fn default() -> Self {
        NodeInvalidation_NodeInvalidationType::NEW
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeInvalidation_NodeInvalidationType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Situation {
    // message fields
    pub id: ::std::string::String,
    pub field_type: Situation_SituationType,
    pub description: ::std::string::String,
    pub start: ::protobuf::SingularPtrField<Time>,
    pub end: ::protobuf::SingularPtrField<Time>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Situation {}

impl Situation {
    pub fn new() -> Situation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Situation {
        static mut instance: ::protobuf::lazy::Lazy<Situation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Situation,
        };
        unsafe {
            instance.get(Situation::new)
        }
    }

    // string id = 1;

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

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // .underworlds.Situation.SituationType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = Situation_SituationType::GENERIC;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Situation_SituationType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> Situation_SituationType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &Situation_SituationType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut Situation_SituationType {
        &mut self.field_type
    }

    // string description = 3;

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

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // .underworlds.Time start = 4;

    pub fn clear_start(&mut self) {
        self.start.clear();
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: Time) {
        self.start = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start(&mut self) -> &mut Time {
        if self.start.is_none() {
            self.start.set_default();
        }
        self.start.as_mut().unwrap()
    }

    // Take field
    pub fn take_start(&mut self) -> Time {
        self.start.take().unwrap_or_else(|| Time::new())
    }

    pub fn get_start(&self) -> &Time {
        self.start.as_ref().unwrap_or_else(|| Time::default_instance())
    }

    fn get_start_for_reflect(&self) -> &::protobuf::SingularPtrField<Time> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Time> {
        &mut self.start
    }

    // .underworlds.Time end = 5;

    pub fn clear_end(&mut self) {
        self.end.clear();
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: Time) {
        self.end = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end(&mut self) -> &mut Time {
        if self.end.is_none() {
            self.end.set_default();
        }
        self.end.as_mut().unwrap()
    }

    // Take field
    pub fn take_end(&mut self) -> Time {
        self.end.take().unwrap_or_else(|| Time::new())
    }

    pub fn get_end(&self) -> &Time {
        self.end.as_ref().unwrap_or_else(|| Time::default_instance())
    }

    fn get_end_for_reflect(&self) -> &::protobuf::SingularPtrField<Time> {
        &self.end
    }

    fn mut_end_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Time> {
        &mut self.end
    }
}

impl ::protobuf::Message for Situation {
    fn is_initialized(&self) -> bool {
        for v in &self.start {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.end {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.end)?;
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
        if self.field_type != Situation_SituationType::GENERIC {
            my_size += ::protobuf::rt::enum_size(2, self.field_type);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.description);
        }
        if let Some(ref v) = self.start.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.end.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if self.field_type != Situation_SituationType::GENERIC {
            os.write_enum(2, self.field_type.value())?;
        }
        if !self.description.is_empty() {
            os.write_string(3, &self.description)?;
        }
        if let Some(ref v) = self.start.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.end.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Situation {
    fn new() -> Situation {
        Situation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Situation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    Situation::get_id_for_reflect,
                    Situation::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Situation_SituationType>>(
                    "type",
                    Situation::get_field_type_for_reflect,
                    Situation::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    Situation::get_description_for_reflect,
                    Situation::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Time>>(
                    "start",
                    Situation::get_start_for_reflect,
                    Situation::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Time>>(
                    "end",
                    Situation::get_end_for_reflect,
                    Situation::mut_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Situation>(
                    "Situation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Situation {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_field_type();
        self.clear_description();
        self.clear_start();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Situation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Situation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Situation_SituationType {
    GENERIC = 0,
    MOTION = 1,
    EVT_MODELLOAD = 2,
}

impl ::protobuf::ProtobufEnum for Situation_SituationType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Situation_SituationType> {
        match value {
            0 => ::std::option::Option::Some(Situation_SituationType::GENERIC),
            1 => ::std::option::Option::Some(Situation_SituationType::MOTION),
            2 => ::std::option::Option::Some(Situation_SituationType::EVT_MODELLOAD),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Situation_SituationType] = &[
            Situation_SituationType::GENERIC,
            Situation_SituationType::MOTION,
            Situation_SituationType::EVT_MODELLOAD,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Situation_SituationType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Situation_SituationType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Situation_SituationType {
}

impl ::std::default::Default for Situation_SituationType {
    fn default() -> Self {
        Situation_SituationType::GENERIC
    }
}

impl ::protobuf::reflect::ProtobufValue for Situation_SituationType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SituationInContext {
    // message fields
    pub context: ::protobuf::SingularPtrField<Context>,
    pub situation: ::protobuf::SingularPtrField<Situation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SituationInContext {}

impl SituationInContext {
    pub fn new() -> SituationInContext {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SituationInContext {
        static mut instance: ::protobuf::lazy::Lazy<SituationInContext> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SituationInContext,
        };
        unsafe {
            instance.get(SituationInContext::new)
        }
    }

    // .underworlds.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // .underworlds.Situation situation = 2;

    pub fn clear_situation(&mut self) {
        self.situation.clear();
    }

    pub fn has_situation(&self) -> bool {
        self.situation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_situation(&mut self, v: Situation) {
        self.situation = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_situation(&mut self) -> &mut Situation {
        if self.situation.is_none() {
            self.situation.set_default();
        }
        self.situation.as_mut().unwrap()
    }

    // Take field
    pub fn take_situation(&mut self) -> Situation {
        self.situation.take().unwrap_or_else(|| Situation::new())
    }

    pub fn get_situation(&self) -> &Situation {
        self.situation.as_ref().unwrap_or_else(|| Situation::default_instance())
    }

    fn get_situation_for_reflect(&self) -> &::protobuf::SingularPtrField<Situation> {
        &self.situation
    }

    fn mut_situation_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Situation> {
        &mut self.situation
    }
}

impl ::protobuf::Message for SituationInContext {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.situation {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.situation)?;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.situation.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.situation.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SituationInContext {
    fn new() -> SituationInContext {
        SituationInContext::new()
    }

    fn descriptor_static(_: ::std::option::Option<SituationInContext>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    SituationInContext::get_context_for_reflect,
                    SituationInContext::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Situation>>(
                    "situation",
                    SituationInContext::get_situation_for_reflect,
                    SituationInContext::mut_situation_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SituationInContext>(
                    "SituationInContext",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SituationInContext {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_situation();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SituationInContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SituationInContext {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TimelineInvalidation {
    // message fields
    pub field_type: TimelineInvalidation_TimelineInvalidationType,
    pub id: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TimelineInvalidation {}

impl TimelineInvalidation {
    pub fn new() -> TimelineInvalidation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TimelineInvalidation {
        static mut instance: ::protobuf::lazy::Lazy<TimelineInvalidation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TimelineInvalidation,
        };
        unsafe {
            instance.get(TimelineInvalidation::new)
        }
    }

    // .underworlds.TimelineInvalidation.TimelineInvalidationType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = TimelineInvalidation_TimelineInvalidationType::EVENT;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: TimelineInvalidation_TimelineInvalidationType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> TimelineInvalidation_TimelineInvalidationType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &TimelineInvalidation_TimelineInvalidationType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut TimelineInvalidation_TimelineInvalidationType {
        &mut self.field_type
    }

    // string id = 2;

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

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }
}

impl ::protobuf::Message for TimelineInvalidation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
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
        if self.field_type != TimelineInvalidation_TimelineInvalidationType::EVENT {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != TimelineInvalidation_TimelineInvalidationType::EVENT {
            os.write_enum(1, self.field_type.value())?;
        }
        if !self.id.is_empty() {
            os.write_string(2, &self.id)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TimelineInvalidation {
    fn new() -> TimelineInvalidation {
        TimelineInvalidation::new()
    }

    fn descriptor_static(_: ::std::option::Option<TimelineInvalidation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<TimelineInvalidation_TimelineInvalidationType>>(
                    "type",
                    TimelineInvalidation::get_field_type_for_reflect,
                    TimelineInvalidation::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    TimelineInvalidation::get_id_for_reflect,
                    TimelineInvalidation::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TimelineInvalidation>(
                    "TimelineInvalidation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TimelineInvalidation {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TimelineInvalidation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TimelineInvalidation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum TimelineInvalidation_TimelineInvalidationType {
    EVENT = 0,
    START = 1,
    END = 2,
}

impl ::protobuf::ProtobufEnum for TimelineInvalidation_TimelineInvalidationType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<TimelineInvalidation_TimelineInvalidationType> {
        match value {
            0 => ::std::option::Option::Some(TimelineInvalidation_TimelineInvalidationType::EVENT),
            1 => ::std::option::Option::Some(TimelineInvalidation_TimelineInvalidationType::START),
            2 => ::std::option::Option::Some(TimelineInvalidation_TimelineInvalidationType::END),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [TimelineInvalidation_TimelineInvalidationType] = &[
            TimelineInvalidation_TimelineInvalidationType::EVENT,
            TimelineInvalidation_TimelineInvalidationType::START,
            TimelineInvalidation_TimelineInvalidationType::END,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<TimelineInvalidation_TimelineInvalidationType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("TimelineInvalidation_TimelineInvalidationType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for TimelineInvalidation_TimelineInvalidationType {
}

impl ::std::default::Default for TimelineInvalidation_TimelineInvalidationType {
    fn default() -> Self {
        TimelineInvalidation_TimelineInvalidationType::EVENT
    }
}

impl ::protobuf::reflect::ProtobufValue for TimelineInvalidation_TimelineInvalidationType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Mesh {
    // message fields
    pub id: ::std::string::String,
    pub vertices: ::protobuf::RepeatedField<Pointf>,
    pub faces: ::protobuf::RepeatedField<Point>,
    pub normals: ::protobuf::RepeatedField<Pointf>,
    pub colors: ::std::vec::Vec<u32>,
    pub diffuse: ::protobuf::SingularPtrField<Color>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Mesh {}

impl Mesh {
    pub fn new() -> Mesh {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Mesh {
        static mut instance: ::protobuf::lazy::Lazy<Mesh> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Mesh,
        };
        unsafe {
            instance.get(Mesh::new)
        }
    }

    // string id = 1;

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

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // repeated .underworlds.Pointf vertices = 2;

    pub fn clear_vertices(&mut self) {
        self.vertices.clear();
    }

    // Param is passed by value, moved
    pub fn set_vertices(&mut self, v: ::protobuf::RepeatedField<Pointf>) {
        self.vertices = v;
    }

    // Mutable pointer to the field.
    pub fn mut_vertices(&mut self) -> &mut ::protobuf::RepeatedField<Pointf> {
        &mut self.vertices
    }

    // Take field
    pub fn take_vertices(&mut self) -> ::protobuf::RepeatedField<Pointf> {
        ::std::mem::replace(&mut self.vertices, ::protobuf::RepeatedField::new())
    }

    pub fn get_vertices(&self) -> &[Pointf] {
        &self.vertices
    }

    fn get_vertices_for_reflect(&self) -> &::protobuf::RepeatedField<Pointf> {
        &self.vertices
    }

    fn mut_vertices_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Pointf> {
        &mut self.vertices
    }

    // repeated .underworlds.Point faces = 3;

    pub fn clear_faces(&mut self) {
        self.faces.clear();
    }

    // Param is passed by value, moved
    pub fn set_faces(&mut self, v: ::protobuf::RepeatedField<Point>) {
        self.faces = v;
    }

    // Mutable pointer to the field.
    pub fn mut_faces(&mut self) -> &mut ::protobuf::RepeatedField<Point> {
        &mut self.faces
    }

    // Take field
    pub fn take_faces(&mut self) -> ::protobuf::RepeatedField<Point> {
        ::std::mem::replace(&mut self.faces, ::protobuf::RepeatedField::new())
    }

    pub fn get_faces(&self) -> &[Point] {
        &self.faces
    }

    fn get_faces_for_reflect(&self) -> &::protobuf::RepeatedField<Point> {
        &self.faces
    }

    fn mut_faces_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Point> {
        &mut self.faces
    }

    // repeated .underworlds.Pointf normals = 4;

    pub fn clear_normals(&mut self) {
        self.normals.clear();
    }

    // Param is passed by value, moved
    pub fn set_normals(&mut self, v: ::protobuf::RepeatedField<Pointf>) {
        self.normals = v;
    }

    // Mutable pointer to the field.
    pub fn mut_normals(&mut self) -> &mut ::protobuf::RepeatedField<Pointf> {
        &mut self.normals
    }

    // Take field
    pub fn take_normals(&mut self) -> ::protobuf::RepeatedField<Pointf> {
        ::std::mem::replace(&mut self.normals, ::protobuf::RepeatedField::new())
    }

    pub fn get_normals(&self) -> &[Pointf] {
        &self.normals
    }

    fn get_normals_for_reflect(&self) -> &::protobuf::RepeatedField<Pointf> {
        &self.normals
    }

    fn mut_normals_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Pointf> {
        &mut self.normals
    }

    // repeated uint32 colors = 5;

    pub fn clear_colors(&mut self) {
        self.colors.clear();
    }

    // Param is passed by value, moved
    pub fn set_colors(&mut self, v: ::std::vec::Vec<u32>) {
        self.colors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_colors(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.colors
    }

    // Take field
    pub fn take_colors(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.colors, ::std::vec::Vec::new())
    }

    pub fn get_colors(&self) -> &[u32] {
        &self.colors
    }

    fn get_colors_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.colors
    }

    fn mut_colors_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.colors
    }

    // .underworlds.Color diffuse = 6;

    pub fn clear_diffuse(&mut self) {
        self.diffuse.clear();
    }

    pub fn has_diffuse(&self) -> bool {
        self.diffuse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_diffuse(&mut self, v: Color) {
        self.diffuse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_diffuse(&mut self) -> &mut Color {
        if self.diffuse.is_none() {
            self.diffuse.set_default();
        }
        self.diffuse.as_mut().unwrap()
    }

    // Take field
    pub fn take_diffuse(&mut self) -> Color {
        self.diffuse.take().unwrap_or_else(|| Color::new())
    }

    pub fn get_diffuse(&self) -> &Color {
        self.diffuse.as_ref().unwrap_or_else(|| Color::default_instance())
    }

    fn get_diffuse_for_reflect(&self) -> &::protobuf::SingularPtrField<Color> {
        &self.diffuse
    }

    fn mut_diffuse_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Color> {
        &mut self.diffuse
    }
}

impl ::protobuf::Message for Mesh {
    fn is_initialized(&self) -> bool {
        for v in &self.vertices {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.faces {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.normals {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.diffuse {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.vertices)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.faces)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.normals)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.colors)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.diffuse)?;
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
        for value in &self.vertices {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.faces {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.normals {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.colors {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(ref v) = self.diffuse.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        for v in &self.vertices {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.faces {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.normals {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.colors {
            os.write_uint32(5, *v)?;
        };
        if let Some(ref v) = self.diffuse.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Mesh {
    fn new() -> Mesh {
        Mesh::new()
    }

    fn descriptor_static(_: ::std::option::Option<Mesh>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    Mesh::get_id_for_reflect,
                    Mesh::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Pointf>>(
                    "vertices",
                    Mesh::get_vertices_for_reflect,
                    Mesh::mut_vertices_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Point>>(
                    "faces",
                    Mesh::get_faces_for_reflect,
                    Mesh::mut_faces_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Pointf>>(
                    "normals",
                    Mesh::get_normals_for_reflect,
                    Mesh::mut_normals_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "colors",
                    Mesh::get_colors_for_reflect,
                    Mesh::mut_colors_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Color>>(
                    "diffuse",
                    Mesh::get_diffuse_for_reflect,
                    Mesh::mut_diffuse_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Mesh>(
                    "Mesh",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Mesh {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_vertices();
        self.clear_faces();
        self.clear_normals();
        self.clear_colors();
        self.clear_diffuse();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Mesh {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Mesh {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MeshInContext {
    // message fields
    pub client: ::protobuf::SingularPtrField<Client>,
    pub mesh: ::protobuf::SingularPtrField<Mesh>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MeshInContext {}

impl MeshInContext {
    pub fn new() -> MeshInContext {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MeshInContext {
        static mut instance: ::protobuf::lazy::Lazy<MeshInContext> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MeshInContext,
        };
        unsafe {
            instance.get(MeshInContext::new)
        }
    }

    // .underworlds.Client client = 1;

    pub fn clear_client(&mut self) {
        self.client.clear();
    }

    pub fn has_client(&self) -> bool {
        self.client.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client(&mut self, v: Client) {
        self.client = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client(&mut self) -> &mut Client {
        if self.client.is_none() {
            self.client.set_default();
        }
        self.client.as_mut().unwrap()
    }

    // Take field
    pub fn take_client(&mut self) -> Client {
        self.client.take().unwrap_or_else(|| Client::new())
    }

    pub fn get_client(&self) -> &Client {
        self.client.as_ref().unwrap_or_else(|| Client::default_instance())
    }

    fn get_client_for_reflect(&self) -> &::protobuf::SingularPtrField<Client> {
        &self.client
    }

    fn mut_client_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Client> {
        &mut self.client
    }

    // .underworlds.Mesh mesh = 2;

    pub fn clear_mesh(&mut self) {
        self.mesh.clear();
    }

    pub fn has_mesh(&self) -> bool {
        self.mesh.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mesh(&mut self, v: Mesh) {
        self.mesh = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mesh(&mut self) -> &mut Mesh {
        if self.mesh.is_none() {
            self.mesh.set_default();
        }
        self.mesh.as_mut().unwrap()
    }

    // Take field
    pub fn take_mesh(&mut self) -> Mesh {
        self.mesh.take().unwrap_or_else(|| Mesh::new())
    }

    pub fn get_mesh(&self) -> &Mesh {
        self.mesh.as_ref().unwrap_or_else(|| Mesh::default_instance())
    }

    fn get_mesh_for_reflect(&self) -> &::protobuf::SingularPtrField<Mesh> {
        &self.mesh
    }

    fn mut_mesh_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Mesh> {
        &mut self.mesh
    }
}

impl ::protobuf::Message for MeshInContext {
    fn is_initialized(&self) -> bool {
        for v in &self.client {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.mesh {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.client)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.mesh)?;
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
        if let Some(ref v) = self.client.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.mesh.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.client.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.mesh.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MeshInContext {
    fn new() -> MeshInContext {
        MeshInContext::new()
    }

    fn descriptor_static(_: ::std::option::Option<MeshInContext>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Client>>(
                    "client",
                    MeshInContext::get_client_for_reflect,
                    MeshInContext::mut_client_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Mesh>>(
                    "mesh",
                    MeshInContext::get_mesh_for_reflect,
                    MeshInContext::mut_mesh_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MeshInContext>(
                    "MeshInContext",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MeshInContext {
    fn clear(&mut self) {
        self.clear_client();
        self.clear_mesh();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MeshInContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MeshInContext {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11underworlds.proto\x12\x0bunderworlds\"\x07\n\x05Empty\"\x1c\n\x04B\
    ool\x12\x14\n\x05value\x18\x01\x20\x01(\x08R\x05value\"\x1a\n\x04Time\
    \x12\x12\n\x04time\x18\x01\x20\x01(\x01R\x04time\"\x1a\n\x04Name\x12\x12\
    \n\x04name\x18\x01\x20\x01(\tR\x04name\"\x1a\n\x04Size\x12\x12\n\x04size\
    \x18\x01\x20\x01(\x05R\x04size\"2\n\x06Pointf\x12\x0c\n\x01x\x18\x01\x20\
    \x01(\x02R\x01x\x12\x0c\n\x01y\x18\x02\x20\x01(\x02R\x01y\x12\x0c\n\x01z\
    \x18\x03\x20\x01(\x02R\x01z\"1\n\x05Point\x12\x0c\n\x01x\x18\x01\x20\x01\
    (\x11R\x01x\x12\x0c\n\x01y\x18\x02\x20\x01(\x11R\x01y\x12\x0c\n\x01z\x18\
    \x03\x20\x01(\x11R\x01z\"?\n\x05Color\x12\x0c\n\x01r\x18\x01\x20\x01(\
    \x02R\x01r\x12\x0c\n\x01g\x18\x02\x20\x01(\x02R\x01g\x12\x0c\n\x01b\x18\
    \x03\x20\x01(\x02R\x01b\x12\x0c\n\x01a\x18\x04\x20\x01(\x02R\x01a\"b\n\
    \x06Client\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12\x12\n\x04name\
    \x18\x02\x20\x01(\tR\x04name\x124\n\x05links\x18\x03\x20\x03(\x0b2\x1e.u\
    nderworlds.ClientInteractionR\x05links\"\xeb\x01\n\x11ClientInteraction\
    \x12\x14\n\x05world\x18\x01\x20\x01(\tR\x05world\x12B\n\x04type\x18\x02\
    \x20\x01(\x0e2..underworlds.ClientInteraction.InteractionTypeR\x04type\
    \x126\n\rlast_activity\x18\x03\x20\x01(\x0b2\x11.underworlds.TimeR\x0cla\
    stActivity\"D\n\x0fInteractionType\x12\n\n\x06READER\x10\0\x12\x0c\n\x08\
    PROVIDER\x10\x01\x12\x0b\n\x07MONITOR\x10\x02\x12\n\n\x06FILTER\x10\x03\
    \"7\n\x07Context\x12\x16\n\x06client\x18\x01\x20\x01(\tR\x06client\x12\
    \x14\n\x05world\x18\x02\x20\x01(\tR\x05world\"Q\n\x08Topology\x12\x16\n\
    \x06worlds\x18\x01\x20\x03(\tR\x06worlds\x12-\n\x07clients\x18\x02\x20\
    \x03(\x0b2\x13.underworlds.ClientR\x07clients\"\x9f\x04\n\x04Node\x12\
    \x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12\x12\n\x04name\x18\x02\x20\x01\
    (\tR\x04name\x12.\n\x04type\x18\x03\x20\x01(\x0e2\x1a.underworlds.Node.N\
    odeTypeR\x04type\x12\x16\n\x06parent\x18\x04\x20\x01(\tR\x06parent\x12\
    \x1a\n\x08children\x18\x05\x20\x03(\tR\x08children\x12&\n\x0etransformat\
    ion\x18\x06\x20\x03(\x02R\x0etransformation\x12\x1f\n\x0blast_update\x18\
    \x08\x20\x01(\x01R\nlastUpdate\x12\x16\n\x06lowres\x18\x10\x20\x03(\tR\
    \x06lowres\x12\x14\n\x05hires\x18\x11\x20\x03(\tR\x05hires\x12\x10\n\x03\
    cad\x18\x12\x20\x03(\tR\x03cad\x12'\n\x04aabb\x18\x13\x20\x03(\x0b2\x13.\
    underworlds.PointfR\x04aabb\x12\x18\n\x07physics\x18\x1e\x20\x01(\x08R\
    \x07physics\x12$\n\rclipplanenear\x18(\x20\x01(\x02R\rclipplanenear\x12\
    \"\n\x0cclipplanefar\x18)\x20\x01(\x02R\x0cclipplanefar\x12\x16\n\x06asp\
    ect\x18*\x20\x01(\x02R\x06aspect\x12$\n\rhorizontalfov\x18+\x20\x01(\x02\
    R\rhorizontalfov\";\n\x08NodeType\x12\r\n\tUNDEFINED\x10\0\x12\n\n\x06EN\
    TITY\x10\x01\x12\x08\n\x04MESH\x10\x02\x12\n\n\x06CAMERA\x10\x03\"\x19\n\
    \x05Nodes\x12\x10\n\x03ids\x18\x01\x20\x03(\tR\x03ids\"f\n\rNodeInContex\
    t\x12.\n\x07context\x18\x01\x20\x01(\x0b2\x14.underworlds.ContextR\x07co\
    ntext\x12%\n\x04node\x18\x02\x20\x01(\x0b2\x11.underworlds.NodeR\x04node\
    \"\xa3\x01\n\x10NodeInvalidation\x12F\n\x04type\x18\x01\x20\x01(\x0e22.u\
    nderworlds.NodeInvalidation.NodeInvalidationTypeR\x04type\x12\x0e\n\x02i\
    d\x18\x02\x20\x01(\tR\x02id\"7\n\x14NodeInvalidationType\x12\x07\n\x03NE\
    W\x10\0\x12\n\n\x06UPDATE\x10\x01\x12\n\n\x06DELETE\x10\x02\"\x82\x02\n\
    \tSituation\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x128\n\x04type\x18\
    \x02\x20\x01(\x0e2$.underworlds.Situation.SituationTypeR\x04type\x12\x20\
    \n\x0bdescription\x18\x03\x20\x01(\tR\x0bdescription\x12'\n\x05start\x18\
    \x04\x20\x01(\x0b2\x11.underworlds.TimeR\x05start\x12#\n\x03end\x18\x05\
    \x20\x01(\x0b2\x11.underworlds.TimeR\x03end\";\n\rSituationType\x12\x0b\
    \n\x07GENERIC\x10\0\x12\n\n\x06MOTION\x10\x01\x12\x11\n\rEVT_MODELLOAD\
    \x10\x02\"z\n\x12SituationInContext\x12.\n\x07context\x18\x01\x20\x01(\
    \x0b2\x14.underworlds.ContextR\x07context\x124\n\tsituation\x18\x02\x20\
    \x01(\x0b2\x16.underworlds.SituationR\tsituation\"\xb1\x01\n\x14Timeline\
    Invalidation\x12N\n\x04type\x18\x01\x20\x01(\x0e2:.underworlds.TimelineI\
    nvalidation.TimelineInvalidationTypeR\x04type\x12\x0e\n\x02id\x18\x02\
    \x20\x01(\tR\x02id\"9\n\x18TimelineInvalidationType\x12\t\n\x05EVENT\x10\
    \0\x12\t\n\x05START\x10\x01\x12\x07\n\x03END\x10\x02\"\xe6\x01\n\x04Mesh\
    \x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12/\n\x08vertices\x18\x02\
    \x20\x03(\x0b2\x13.underworlds.PointfR\x08vertices\x12(\n\x05faces\x18\
    \x03\x20\x03(\x0b2\x12.underworlds.PointR\x05faces\x12-\n\x07normals\x18\
    \x04\x20\x03(\x0b2\x13.underworlds.PointfR\x07normals\x12\x16\n\x06color\
    s\x18\x05\x20\x03(\rR\x06colors\x12,\n\x07diffuse\x18\x06\x20\x01(\x0b2\
    \x12.underworlds.ColorR\x07diffuse\"c\n\rMeshInContext\x12+\n\x06client\
    \x18\x01\x20\x01(\x0b2\x13.underworlds.ClientR\x06client\x12%\n\x04mesh\
    \x18\x02\x20\x01(\x0b2\x11.underworlds.MeshR\x04mesh2\xb9\t\n\x0bUnderwo\
    rlds\x120\n\x04helo\x12\x11.underworlds.Name\x1a\x13.underworlds.Client\
    \"\0\x122\n\x06uptime\x12\x13.underworlds.Client\x1a\x11.underworlds.Tim\
    e\"\0\x128\n\x08topology\x12\x13.underworlds.Client\x1a\x15.underworlds.\
    Topology\"\0\x122\n\x05reset\x12\x13.underworlds.Client\x1a\x12.underwor\
    lds.Empty\"\0\x128\n\x0bgetNodesLen\x12\x14.underworlds.Context\x1a\x11.\
    underworlds.Size\"\0\x129\n\x0bgetNodesIds\x12\x14.underworlds.Context\
    \x1a\x12.underworlds.Nodes\"\0\x128\n\x0bgetRootNode\x12\x14.underworlds\
    .Context\x1a\x11.underworlds.Node\"\0\x12:\n\x07getNode\x12\x1a.underwor\
    lds.NodeInContext\x1a\x11.underworlds.Node\"\0\x12>\n\nupdateNode\x12\
    \x1a.underworlds.NodeInContext\x1a\x12.underworlds.Empty\"\0\x12>\n\ndel\
    eteNode\x12\x1a.underworlds.NodeInContext\x1a\x12.underworlds.Empty\"\0\
    \x12O\n\x14getNodeInvalidations\x12\x14.underworlds.Context\x1a\x1d.unde\
    rworlds.NodeInvalidation\"\00\x01\x12;\n\x0etimelineOrigin\x12\x14.under\
    worlds.Context\x1a\x11.underworlds.Time\"\0\x12>\n\x05event\x12\x1f.unde\
    rworlds.SituationInContext\x1a\x12.underworlds.Empty\"\0\x12G\n\x0estart\
    Situation\x12\x1f.underworlds.SituationInContext\x1a\x12.underworlds.Emp\
    ty\"\0\x12E\n\x0cendSituation\x12\x1f.underworlds.SituationInContext\x1a\
    \x12.underworlds.Empty\"\0\x12W\n\x18getTimelineInvalidations\x12\x14.un\
    derworlds.Context\x1a!.underworlds.TimelineInvalidation\"\00\x01\x12:\n\
    \x07hasMesh\x12\x1a.underworlds.MeshInContext\x1a\x11.underworlds.Bool\"\
    \0\x12:\n\x07getMesh\x12\x1a.underworlds.MeshInContext\x1a\x11.underworl\
    ds.Mesh\"\0\x12<\n\x08pushMesh\x12\x1a.underworlds.MeshInContext\x1a\x12\
    .underworlds.Empty\"\0b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
