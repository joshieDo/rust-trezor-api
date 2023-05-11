// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `messages-bootloader.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

/// *
///  Request: Ask device to erase its firmware (so it can be replaced via FirmwareUpload)
///  @start
///  @next FirmwareRequest
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:hw.trezor.messages.bootloader.FirmwareErase)
pub struct FirmwareErase {
    // message fields
    // @@protoc_insertion_point(field:hw.trezor.messages.bootloader.FirmwareErase.length)
    pub length: ::std::option::Option<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:hw.trezor.messages.bootloader.FirmwareErase.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FirmwareErase {
    fn default() -> &'a FirmwareErase {
        <FirmwareErase as ::protobuf::Message>::default_instance()
    }
}

impl FirmwareErase {
    pub fn new() -> FirmwareErase {
        ::std::default::Default::default()
    }

    // optional uint32 length = 1;

    pub fn length(&self) -> u32 {
        self.length.unwrap_or(0)
    }

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u32) {
        self.length = ::std::option::Option::Some(v);
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "length",
            |m: &FirmwareErase| { &m.length },
            |m: &mut FirmwareErase| { &mut m.length },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FirmwareErase>(
            "FirmwareErase",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FirmwareErase {
    const NAME: &'static str = "FirmwareErase";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.length = ::std::option::Option::Some(is.read_uint32()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::uint32_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.length {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> FirmwareErase {
        FirmwareErase::new()
    }

    fn clear(&mut self) {
        self.length = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FirmwareErase {
        static instance: FirmwareErase = FirmwareErase {
            length: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FirmwareErase {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FirmwareErase").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FirmwareErase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FirmwareErase {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// *
///  Response: Ask for firmware chunk
///  @next FirmwareUpload
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:hw.trezor.messages.bootloader.FirmwareRequest)
pub struct FirmwareRequest {
    // message fields
    // @@protoc_insertion_point(field:hw.trezor.messages.bootloader.FirmwareRequest.offset)
    pub offset: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:hw.trezor.messages.bootloader.FirmwareRequest.length)
    pub length: ::std::option::Option<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:hw.trezor.messages.bootloader.FirmwareRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FirmwareRequest {
    fn default() -> &'a FirmwareRequest {
        <FirmwareRequest as ::protobuf::Message>::default_instance()
    }
}

impl FirmwareRequest {
    pub fn new() -> FirmwareRequest {
        ::std::default::Default::default()
    }

    // optional uint32 offset = 1;

    pub fn offset(&self) -> u32 {
        self.offset.unwrap_or(0)
    }

    pub fn clear_offset(&mut self) {
        self.offset = ::std::option::Option::None;
    }

    pub fn has_offset(&self) -> bool {
        self.offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: u32) {
        self.offset = ::std::option::Option::Some(v);
    }

    // optional uint32 length = 2;

    pub fn length(&self) -> u32 {
        self.length.unwrap_or(0)
    }

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u32) {
        self.length = ::std::option::Option::Some(v);
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "offset",
            |m: &FirmwareRequest| { &m.offset },
            |m: &mut FirmwareRequest| { &mut m.offset },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "length",
            |m: &FirmwareRequest| { &m.length },
            |m: &mut FirmwareRequest| { &mut m.length },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FirmwareRequest>(
            "FirmwareRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FirmwareRequest {
    const NAME: &'static str = "FirmwareRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.offset = ::std::option::Option::Some(is.read_uint32()?);
                },
                16 => {
                    self.length = ::std::option::Option::Some(is.read_uint32()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.offset {
            my_size += ::protobuf::rt::uint32_size(1, v);
        }
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::uint32_size(2, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.offset {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.length {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> FirmwareRequest {
        FirmwareRequest::new()
    }

    fn clear(&mut self) {
        self.offset = ::std::option::Option::None;
        self.length = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FirmwareRequest {
        static instance: FirmwareRequest = FirmwareRequest {
            offset: ::std::option::Option::None,
            length: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FirmwareRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FirmwareRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FirmwareRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FirmwareRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// *
///  Request: Send firmware in binary form to the device
///  @next FirmwareRequest
///  @next Success
///  @next Failure
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:hw.trezor.messages.bootloader.FirmwareUpload)
pub struct FirmwareUpload {
    // message fields
    // @@protoc_insertion_point(field:hw.trezor.messages.bootloader.FirmwareUpload.payload)
    pub payload: ::std::option::Option<::std::vec::Vec<u8>>,
    // @@protoc_insertion_point(field:hw.trezor.messages.bootloader.FirmwareUpload.hash)
    pub hash: ::std::option::Option<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:hw.trezor.messages.bootloader.FirmwareUpload.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FirmwareUpload {
    fn default() -> &'a FirmwareUpload {
        <FirmwareUpload as ::protobuf::Message>::default_instance()
    }
}

impl FirmwareUpload {
    pub fn new() -> FirmwareUpload {
        ::std::default::Default::default()
    }

    // required bytes payload = 1;

    pub fn payload(&self) -> &[u8] {
        match self.payload.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_payload(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional bytes hash = 2;

    pub fn hash(&self) -> &[u8] {
        match self.hash.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_hash(&mut self) {
        self.hash = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.hash.is_none() {
            self.hash = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        self.hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "payload",
            |m: &FirmwareUpload| { &m.payload },
            |m: &mut FirmwareUpload| { &mut m.payload },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "hash",
            |m: &FirmwareUpload| { &m.hash },
            |m: &mut FirmwareUpload| { &mut m.hash },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FirmwareUpload>(
            "FirmwareUpload",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FirmwareUpload {
    const NAME: &'static str = "FirmwareUpload";

    fn is_initialized(&self) -> bool {
        if self.payload.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.payload = ::std::option::Option::Some(is.read_bytes()?);
                },
                18 => {
                    self.hash = ::std::option::Option::Some(is.read_bytes()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.payload.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.hash.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.payload.as_ref() {
            os.write_bytes(1, v)?;
        }
        if let Some(v) = self.hash.as_ref() {
            os.write_bytes(2, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> FirmwareUpload {
        FirmwareUpload::new()
    }

    fn clear(&mut self) {
        self.payload = ::std::option::Option::None;
        self.hash = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FirmwareUpload {
        static instance: FirmwareUpload = FirmwareUpload {
            payload: ::std::option::Option::None,
            hash: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FirmwareUpload {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FirmwareUpload").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FirmwareUpload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FirmwareUpload {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// *
///  Request: Perform a device self-test
///  @next Success
///  @next Failure
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:hw.trezor.messages.bootloader.SelfTest)
pub struct SelfTest {
    // message fields
    // @@protoc_insertion_point(field:hw.trezor.messages.bootloader.SelfTest.payload)
    pub payload: ::std::option::Option<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:hw.trezor.messages.bootloader.SelfTest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SelfTest {
    fn default() -> &'a SelfTest {
        <SelfTest as ::protobuf::Message>::default_instance()
    }
}

impl SelfTest {
    pub fn new() -> SelfTest {
        ::std::default::Default::default()
    }

    // optional bytes payload = 1;

    pub fn payload(&self) -> &[u8] {
        match self.payload.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_payload(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "payload",
            |m: &SelfTest| { &m.payload },
            |m: &mut SelfTest| { &mut m.payload },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SelfTest>(
            "SelfTest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SelfTest {
    const NAME: &'static str = "SelfTest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.payload = ::std::option::Option::Some(is.read_bytes()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.payload.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.payload.as_ref() {
            os.write_bytes(1, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SelfTest {
        SelfTest::new()
    }

    fn clear(&mut self) {
        self.payload = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SelfTest {
        static instance: SelfTest = SelfTest {
            payload: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SelfTest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SelfTest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SelfTest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SelfTest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19messages-bootloader.proto\x12\x1dhw.trezor.messages.bootloader\"'\
    \n\rFirmwareErase\x12\x16\n\x06length\x18\x01\x20\x01(\rR\x06length\"A\n\
    \x0fFirmwareRequest\x12\x16\n\x06offset\x18\x01\x20\x01(\rR\x06offset\
    \x12\x16\n\x06length\x18\x02\x20\x01(\rR\x06length\">\n\x0eFirmwareUploa\
    d\x12\x18\n\x07payload\x18\x01\x20\x02(\x0cR\x07payload\x12\x12\n\x04has\
    h\x18\x02\x20\x01(\x0cR\x04hash\"$\n\x08SelfTest\x12\x18\n\x07payload\
    \x18\x01\x20\x01(\x0cR\x07payloadB>\n#com.satoshilabs.trezor.lib.protobu\
    fB\x17TrezorMessageBootloaderJ\x99\t\n\x06\x12\x04\0\0+\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\0&\n\x08\n\x01\x08\x12\
    \x03\x04\0<\n.\n\x02\x08\x01\x12\x03\x04\0<\x1a#\x20Sugar\x20for\x20easi\
    er\x20handling\x20in\x20Java\n\n\x08\n\x01\x08\x12\x03\x05\08\n\t\n\x02\
    \x08\x08\x12\x03\x05\08\n\x83\x01\n\x02\x04\0\x12\x04\x0c\0\x0e\x01\x1aw\
    *\n\x20Request:\x20Ask\x20device\x20to\x20erase\x20its\x20firmware\x20(s\
    o\x20it\x20can\x20be\x20replaced\x20via\x20FirmwareUpload)\n\x20@start\n\
    \x20@next\x20FirmwareRequest\n\n\n\n\x03\x04\0\x01\x12\x03\x0c\x08\x15\n\
    %\n\x04\x04\0\x02\0\x12\x03\r\x04\x1f\"\x18\x20length\x20of\x20new\x20fi\
    rmware\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\r\x04\x0c\n\x0c\n\x05\x04\0\
    \x02\0\x05\x12\x03\r\r\x13\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\r\x14\x1a\
    \n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\r\x1d\x1e\nF\n\x02\x04\x01\x12\x04\
    \x14\0\x17\x01\x1a:*\n\x20Response:\x20Ask\x20for\x20firmware\x20chunk\n\
    \x20@next\x20FirmwareUpload\n\n\n\n\x03\x04\x01\x01\x12\x03\x14\x08\x17\
    \n1\n\x04\x04\x01\x02\0\x12\x03\x15\x04\x1f\"$\x20offset\x20of\x20reques\
    ted\x20firmware\x20chunk\n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x15\x04\
    \x0c\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x15\r\x13\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03\x15\x14\x1a\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x15\
    \x1d\x1e\n1\n\x04\x04\x01\x02\x01\x12\x03\x16\x04\x1f\"$\x20length\x20of\
    \x20requested\x20firmware\x20chunk\n\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\
    \x03\x16\x04\x0c\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x16\r\x13\n\x0c\
    \n\x05\x04\x01\x02\x01\x01\x12\x03\x16\x14\x1a\n\x0c\n\x05\x04\x01\x02\
    \x01\x03\x12\x03\x16\x1d\x1e\nx\n\x02\x04\x02\x12\x04\x1f\0\"\x01\x1al*\
    \n\x20Request:\x20Send\x20firmware\x20in\x20binary\x20form\x20to\x20the\
    \x20device\n\x20@next\x20FirmwareRequest\n\x20@next\x20Success\n\x20@nex\
    t\x20Failure\n\n\n\n\x03\x04\x02\x01\x12\x03\x1f\x08\x16\n0\n\x04\x04\
    \x02\x02\0\x12\x03\x20\x04\x1f\"#\x20firmware\x20to\x20be\x20loaded\x20i\
    nto\x20device\n\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x20\x04\x0c\n\x0c\
    \n\x05\x04\x02\x02\0\x05\x12\x03\x20\r\x12\n\x0c\n\x05\x04\x02\x02\0\x01\
    \x12\x03\x20\x13\x1a\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x20\x1d\x1e\n\
    \"\n\x04\x04\x02\x02\x01\x12\x03!\x04\x1c\"\x15\x20hash\x20of\x20the\x20\
    payload\n\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03!\x04\x0c\n\x0c\n\x05\
    \x04\x02\x02\x01\x05\x12\x03!\r\x12\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\
    \x03!\x13\x17\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03!\x1a\x1b\nQ\n\x02\
    \x04\x03\x12\x04)\0+\x01\x1aE*\n\x20Request:\x20Perform\x20a\x20device\
    \x20self-test\n\x20@next\x20Success\n\x20@next\x20Failure\n\n\n\n\x03\
    \x04\x03\x01\x12\x03)\x08\x10\n.\n\x04\x04\x03\x02\0\x12\x03*\x04\x1f\"!\
    \x20payload\x20to\x20be\x20used\x20in\x20self-test\n\n\x0c\n\x05\x04\x03\
    \x02\0\x04\x12\x03*\x04\x0c\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03*\r\x12\
    \n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03*\x13\x1a\n\x0c\n\x05\x04\x03\x02\
    \0\x03\x12\x03*\x1d\x1e\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(FirmwareErase::generated_message_descriptor_data());
            messages.push(FirmwareRequest::generated_message_descriptor_data());
            messages.push(FirmwareUpload::generated_message_descriptor_data());
            messages.push(SelfTest::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
