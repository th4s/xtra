mod header {
    use ethereum_types::{H160, H256, H64, U256, U64};
    use serde::Deserialize;
    pub struct Header {
        pub parent_hash: H256,
        pub sha3_uncles: H256,
        pub miner: H160,
        pub state_root: H256,
        pub transactions_root: H256,
        pub receipts_root: H256,
        pub logs_bloom: H256,
        pub difficulty: U256,
        pub number: U256,
        pub gas_limit: U256,
        pub gas_used: U256,
        pub time_stamp: U64,
        pub extra_data: Vec<u8>,
        pub mix_hash: H256,
        pub nonce: H64,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Header {
        #[inline]
        fn clone(&self) -> Header {
            match *self {
                Header {
                    parent_hash: ref __self_0_0,
                    sha3_uncles: ref __self_0_1,
                    miner: ref __self_0_2,
                    state_root: ref __self_0_3,
                    transactions_root: ref __self_0_4,
                    receipts_root: ref __self_0_5,
                    logs_bloom: ref __self_0_6,
                    difficulty: ref __self_0_7,
                    number: ref __self_0_8,
                    gas_limit: ref __self_0_9,
                    gas_used: ref __self_0_10,
                    time_stamp: ref __self_0_11,
                    extra_data: ref __self_0_12,
                    mix_hash: ref __self_0_13,
                    nonce: ref __self_0_14,
                } => Header {
                    parent_hash: ::core::clone::Clone::clone(&(*__self_0_0)),
                    sha3_uncles: ::core::clone::Clone::clone(&(*__self_0_1)),
                    miner: ::core::clone::Clone::clone(&(*__self_0_2)),
                    state_root: ::core::clone::Clone::clone(&(*__self_0_3)),
                    transactions_root: ::core::clone::Clone::clone(&(*__self_0_4)),
                    receipts_root: ::core::clone::Clone::clone(&(*__self_0_5)),
                    logs_bloom: ::core::clone::Clone::clone(&(*__self_0_6)),
                    difficulty: ::core::clone::Clone::clone(&(*__self_0_7)),
                    number: ::core::clone::Clone::clone(&(*__self_0_8)),
                    gas_limit: ::core::clone::Clone::clone(&(*__self_0_9)),
                    gas_used: ::core::clone::Clone::clone(&(*__self_0_10)),
                    time_stamp: ::core::clone::Clone::clone(&(*__self_0_11)),
                    extra_data: ::core::clone::Clone::clone(&(*__self_0_12)),
                    mix_hash: ::core::clone::Clone::clone(&(*__self_0_13)),
                    nonce: ::core::clone::Clone::clone(&(*__self_0_14)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Header {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Header {
                    parent_hash: ref __self_0_0,
                    sha3_uncles: ref __self_0_1,
                    miner: ref __self_0_2,
                    state_root: ref __self_0_3,
                    transactions_root: ref __self_0_4,
                    receipts_root: ref __self_0_5,
                    logs_bloom: ref __self_0_6,
                    difficulty: ref __self_0_7,
                    number: ref __self_0_8,
                    gas_limit: ref __self_0_9,
                    gas_used: ref __self_0_10,
                    time_stamp: ref __self_0_11,
                    extra_data: ref __self_0_12,
                    mix_hash: ref __self_0_13,
                    nonce: ref __self_0_14,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Header");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "parent_hash",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "sha3_uncles",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "miner",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "state_root",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "transactions_root",
                        &&(*__self_0_4),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "receipts_root",
                        &&(*__self_0_5),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "logs_bloom",
                        &&(*__self_0_6),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "difficulty",
                        &&(*__self_0_7),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "number",
                        &&(*__self_0_8),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "gas_limit",
                        &&(*__self_0_9),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "gas_used",
                        &&(*__self_0_10),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "time_stamp",
                        &&(*__self_0_11),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "extra_data",
                        &&(*__self_0_12),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "mix_hash",
                        &&(*__self_0_13),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "nonce",
                        &&(*__self_0_14),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Header {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Header {
        #[inline]
        fn eq(&self, other: &Header) -> bool {
            match *other {
                Header {
                    parent_hash: ref __self_1_0,
                    sha3_uncles: ref __self_1_1,
                    miner: ref __self_1_2,
                    state_root: ref __self_1_3,
                    transactions_root: ref __self_1_4,
                    receipts_root: ref __self_1_5,
                    logs_bloom: ref __self_1_6,
                    difficulty: ref __self_1_7,
                    number: ref __self_1_8,
                    gas_limit: ref __self_1_9,
                    gas_used: ref __self_1_10,
                    time_stamp: ref __self_1_11,
                    extra_data: ref __self_1_12,
                    mix_hash: ref __self_1_13,
                    nonce: ref __self_1_14,
                } => match *self {
                    Header {
                        parent_hash: ref __self_0_0,
                        sha3_uncles: ref __self_0_1,
                        miner: ref __self_0_2,
                        state_root: ref __self_0_3,
                        transactions_root: ref __self_0_4,
                        receipts_root: ref __self_0_5,
                        logs_bloom: ref __self_0_6,
                        difficulty: ref __self_0_7,
                        number: ref __self_0_8,
                        gas_limit: ref __self_0_9,
                        gas_used: ref __self_0_10,
                        time_stamp: ref __self_0_11,
                        extra_data: ref __self_0_12,
                        mix_hash: ref __self_0_13,
                        nonce: ref __self_0_14,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                            && (*__self_0_5) == (*__self_1_5)
                            && (*__self_0_6) == (*__self_1_6)
                            && (*__self_0_7) == (*__self_1_7)
                            && (*__self_0_8) == (*__self_1_8)
                            && (*__self_0_9) == (*__self_1_9)
                            && (*__self_0_10) == (*__self_1_10)
                            && (*__self_0_11) == (*__self_1_11)
                            && (*__self_0_12) == (*__self_1_12)
                            && (*__self_0_13) == (*__self_1_13)
                            && (*__self_0_14) == (*__self_1_14)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Header) -> bool {
            match *other {
                Header {
                    parent_hash: ref __self_1_0,
                    sha3_uncles: ref __self_1_1,
                    miner: ref __self_1_2,
                    state_root: ref __self_1_3,
                    transactions_root: ref __self_1_4,
                    receipts_root: ref __self_1_5,
                    logs_bloom: ref __self_1_6,
                    difficulty: ref __self_1_7,
                    number: ref __self_1_8,
                    gas_limit: ref __self_1_9,
                    gas_used: ref __self_1_10,
                    time_stamp: ref __self_1_11,
                    extra_data: ref __self_1_12,
                    mix_hash: ref __self_1_13,
                    nonce: ref __self_1_14,
                } => match *self {
                    Header {
                        parent_hash: ref __self_0_0,
                        sha3_uncles: ref __self_0_1,
                        miner: ref __self_0_2,
                        state_root: ref __self_0_3,
                        transactions_root: ref __self_0_4,
                        receipts_root: ref __self_0_5,
                        logs_bloom: ref __self_0_6,
                        difficulty: ref __self_0_7,
                        number: ref __self_0_8,
                        gas_limit: ref __self_0_9,
                        gas_used: ref __self_0_10,
                        time_stamp: ref __self_0_11,
                        extra_data: ref __self_0_12,
                        mix_hash: ref __self_0_13,
                        nonce: ref __self_0_14,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                            || (*__self_0_5) != (*__self_1_5)
                            || (*__self_0_6) != (*__self_1_6)
                            || (*__self_0_7) != (*__self_1_7)
                            || (*__self_0_8) != (*__self_1_8)
                            || (*__self_0_9) != (*__self_1_9)
                            || (*__self_0_10) != (*__self_1_10)
                            || (*__self_0_11) != (*__self_1_11)
                            || (*__self_0_12) != (*__self_1_12)
                            || (*__self_0_13) != (*__self_1_13)
                            || (*__self_0_14) != (*__self_1_14)
                    }
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Header {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __field12,
                    __field13,
                    __field14,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            9u64 => _serde::__private::Ok(__Field::__field9),
                            10u64 => _serde::__private::Ok(__Field::__field10),
                            11u64 => _serde::__private::Ok(__Field::__field11),
                            12u64 => _serde::__private::Ok(__Field::__field12),
                            13u64 => _serde::__private::Ok(__Field::__field13),
                            14u64 => _serde::__private::Ok(__Field::__field14),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "parent_hash" => _serde::__private::Ok(__Field::__field0),
                            "sha3_uncles" => _serde::__private::Ok(__Field::__field1),
                            "miner" => _serde::__private::Ok(__Field::__field2),
                            "state_root" => _serde::__private::Ok(__Field::__field3),
                            "transactions_root" => _serde::__private::Ok(__Field::__field4),
                            "receipts_root" => _serde::__private::Ok(__Field::__field5),
                            "logs_bloom" => _serde::__private::Ok(__Field::__field6),
                            "difficulty" => _serde::__private::Ok(__Field::__field7),
                            "number" => _serde::__private::Ok(__Field::__field8),
                            "gas_limit" => _serde::__private::Ok(__Field::__field9),
                            "gas_used" => _serde::__private::Ok(__Field::__field10),
                            "time_stamp" => _serde::__private::Ok(__Field::__field11),
                            "extra_data" => _serde::__private::Ok(__Field::__field12),
                            "mix_hash" => _serde::__private::Ok(__Field::__field13),
                            "nonce" => _serde::__private::Ok(__Field::__field14),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"parent_hash" => _serde::__private::Ok(__Field::__field0),
                            b"sha3_uncles" => _serde::__private::Ok(__Field::__field1),
                            b"miner" => _serde::__private::Ok(__Field::__field2),
                            b"state_root" => _serde::__private::Ok(__Field::__field3),
                            b"transactions_root" => _serde::__private::Ok(__Field::__field4),
                            b"receipts_root" => _serde::__private::Ok(__Field::__field5),
                            b"logs_bloom" => _serde::__private::Ok(__Field::__field6),
                            b"difficulty" => _serde::__private::Ok(__Field::__field7),
                            b"number" => _serde::__private::Ok(__Field::__field8),
                            b"gas_limit" => _serde::__private::Ok(__Field::__field9),
                            b"gas_used" => _serde::__private::Ok(__Field::__field10),
                            b"time_stamp" => _serde::__private::Ok(__Field::__field11),
                            b"extra_data" => _serde::__private::Ok(__Field::__field12),
                            b"mix_hash" => _serde::__private::Ok(__Field::__field13),
                            b"nonce" => _serde::__private::Ok(__Field::__field14),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Header>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Header;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Header")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<H256>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Header with 15 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<H256>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Header with 15 elements",
                                        ),
                                    );
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<H160>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Header with 15 elements",
                                        ),
                                    );
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<H256>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Header with 15 elements",
                                        ),
                                    );
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<H256>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Header with 15 elements",
                                        ),
                                    );
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<H256>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Header with 15 elements",
                                        ),
                                    );
                                }
                            };
                        let __field6 =
                            match match _serde::de::SeqAccess::next_element::<H256>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Header with 15 elements",
                                        ),
                                    );
                                }
                            };
                        let __field7 =
                            match match _serde::de::SeqAccess::next_element::<U256>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Header with 15 elements",
                                        ),
                                    );
                                }
                            };
                        let __field8 =
                            match match _serde::de::SeqAccess::next_element::<U256>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct Header with 15 elements",
                                        ),
                                    );
                                }
                            };
                        let __field9 =
                            match match _serde::de::SeqAccess::next_element::<U256>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct Header with 15 elements",
                                        ),
                                    );
                                }
                            };
                        let __field10 =
                            match match _serde::de::SeqAccess::next_element::<U256>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            10usize,
                                            &"struct Header with 15 elements",
                                        ),
                                    );
                                }
                            };
                        let __field11 =
                            match match _serde::de::SeqAccess::next_element::<U64>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            11usize,
                                            &"struct Header with 15 elements",
                                        ),
                                    );
                                }
                            };
                        let __field12 = match match _serde::de::SeqAccess::next_element::<Vec<u8>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    12usize,
                                    &"struct Header with 15 elements",
                                ));
                            }
                        };
                        let __field13 =
                            match match _serde::de::SeqAccess::next_element::<H256>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            13usize,
                                            &"struct Header with 15 elements",
                                        ),
                                    );
                                }
                            };
                        let __field14 =
                            match match _serde::de::SeqAccess::next_element::<H64>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            14usize,
                                            &"struct Header with 15 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(Header {
                            parent_hash: __field0,
                            sha3_uncles: __field1,
                            miner: __field2,
                            state_root: __field3,
                            transactions_root: __field4,
                            receipts_root: __field5,
                            logs_bloom: __field6,
                            difficulty: __field7,
                            number: __field8,
                            gas_limit: __field9,
                            gas_used: __field10,
                            time_stamp: __field11,
                            extra_data: __field12,
                            mix_hash: __field13,
                            nonce: __field14,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<H256> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<H256> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<H160> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<H256> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<H256> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<H256> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<H256> = _serde::__private::None;
                        let mut __field7: _serde::__private::Option<U256> = _serde::__private::None;
                        let mut __field8: _serde::__private::Option<U256> = _serde::__private::None;
                        let mut __field9: _serde::__private::Option<U256> = _serde::__private::None;
                        let mut __field10: _serde::__private::Option<U256> =
                            _serde::__private::None;
                        let mut __field11: _serde::__private::Option<U64> = _serde::__private::None;
                        let mut __field12: _serde::__private::Option<Vec<u8>> =
                            _serde::__private::None;
                        let mut __field13: _serde::__private::Option<H256> =
                            _serde::__private::None;
                        let mut __field14: _serde::__private::Option<H64> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "parent_hash",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<H256>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "sha3_uncles",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<H256>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "miner",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<H160>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "state_root",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<H256>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "transactions_root",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<H256>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "receipts_root",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<H256>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "logs_bloom",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<H256>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "difficulty",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<U256>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::__private::Option::is_some(&__field8) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "number",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<U256>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field9 => {
                                    if _serde::__private::Option::is_some(&__field9) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "gas_limit",
                                            ),
                                        );
                                    }
                                    __field9 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<U256>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field10 => {
                                    if _serde::__private::Option::is_some(&__field10) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "gas_used",
                                            ),
                                        );
                                    }
                                    __field10 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<U256>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field11 => {
                                    if _serde::__private::Option::is_some(&__field11) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "time_stamp",
                                            ),
                                        );
                                    }
                                    __field11 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<U64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field12 => {
                                    if _serde::__private::Option::is_some(&__field12) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "extra_data",
                                            ),
                                        );
                                    }
                                    __field12 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<u8>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field13 => {
                                    if _serde::__private::Option::is_some(&__field13) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "mix_hash",
                                            ),
                                        );
                                    }
                                    __field13 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<H256>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field14 => {
                                    if _serde::__private::Option::is_some(&__field14) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "nonce",
                                            ),
                                        );
                                    }
                                    __field14 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<H64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("parent_hash") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("sha3_uncles") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("miner") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("state_root") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("transactions_root") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("receipts_root") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("logs_bloom") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("difficulty") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::__private::Some(__field8) => __field8,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("number") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field9 = match __field9 {
                            _serde::__private::Some(__field9) => __field9,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("gas_limit") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field10 = match __field10 {
                            _serde::__private::Some(__field10) => __field10,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("gas_used") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field11 = match __field11 {
                            _serde::__private::Some(__field11) => __field11,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("time_stamp") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field12 = match __field12 {
                            _serde::__private::Some(__field12) => __field12,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("extra_data") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field13 = match __field13 {
                            _serde::__private::Some(__field13) => __field13,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("mix_hash") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field14 = match __field14 {
                            _serde::__private::Some(__field14) => __field14,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("nonce") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Header {
                            parent_hash: __field0,
                            sha3_uncles: __field1,
                            miner: __field2,
                            state_root: __field3,
                            transactions_root: __field4,
                            receipts_root: __field5,
                            logs_bloom: __field6,
                            difficulty: __field7,
                            number: __field8,
                            gas_limit: __field9,
                            gas_used: __field10,
                            time_stamp: __field11,
                            extra_data: __field12,
                            mix_hash: __field13,
                            nonce: __field14,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "parent_hash",
                    "sha3_uncles",
                    "miner",
                    "state_root",
                    "transactions_root",
                    "receipts_root",
                    "logs_bloom",
                    "difficulty",
                    "number",
                    "gas_limit",
                    "gas_used",
                    "time_stamp",
                    "extra_data",
                    "mix_hash",
                    "nonce",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Header",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Header>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Header {
        #[inline]
        fn default() -> Header {
            Header {
                parent_hash: ::core::default::Default::default(),
                sha3_uncles: ::core::default::Default::default(),
                miner: ::core::default::Default::default(),
                state_root: ::core::default::Default::default(),
                transactions_root: ::core::default::Default::default(),
                receipts_root: ::core::default::Default::default(),
                logs_bloom: ::core::default::Default::default(),
                difficulty: ::core::default::Default::default(),
                number: ::core::default::Default::default(),
                gas_limit: ::core::default::Default::default(),
                gas_used: ::core::default::Default::default(),
                time_stamp: ::core::default::Default::default(),
                extra_data: ::core::default::Default::default(),
                mix_hash: ::core::default::Default::default(),
                nonce: ::core::default::Default::default(),
            }
        }
    }
}
