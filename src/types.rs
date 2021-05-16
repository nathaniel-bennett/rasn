//! # Types
//! The `types` modules is a collection of Rust types and data structures that
//! are defined to represent various ASN.1 data types, and renamed to use
//! ASN.1's terminology.

mod instance;
mod oid;
mod open;
mod prefix;

pub use rasn_derive::AsnType;

pub use alloc::string::String as Utf8String;
pub use bytes::Bytes as OctetString;
pub use num_bigint::BigInt as Integer;

pub use super::tag::{Class, Tag};
pub use instance::InstanceOf;
pub use oid::{ConstOid, ObjectIdentifier, Oid};
pub use open::Open;
pub use prefix::{Explicit, Implicit};

///  Alias for `bitvec::BitVec` mapped to ASN.1'a `BIT STRING`.
pub type BitString = bitvec::vec::BitVec<bitvec::order::Msb0, u8>;
///  `IA5String` string alias that matches BER's encoding rules.
pub type IA5String = Implicit<{ Tag::IA5_STRING }, Utf8String>;
///  `PrintableString` string alias that matches BER's encoding rules.
pub type PrintableString = Implicit<{ Tag::PRINTABLE_STRING }, Utf8String>;
///  `VisibleString` string alias that matches BER's encoding rules.
pub type VisibleString = Implicit<{ Tag::VISIBLE_STRING }, Utf8String>;
///  `String` alias that matches `BmpString` BER's encoding rules.
pub type BmpString = Implicit<{ Tag::BMP_STRING }, Utf8String>;
///  Alias to `Vec<T>`.
pub type SetOf<T> = alloc::collections::BTreeSet<T>;
///  `UniversalString` string alias that matches BER's encoding rules.
pub type UniversalString = Implicit<{ Tag::UNIVERSAL_STRING }, Utf8String>;
///  Alias for `chrono::DateTime<Utc>`.
pub type UtcTime = chrono::DateTime<chrono::Utc>;
///  Alias for `chrono::DateTime<FixedOffset>`.
pub type GeneralizedTime = chrono::DateTime<chrono::FixedOffset>;

/// A trait representing any type that can represented in ASN.1.
pub trait AsnType {
    /// The associated tag for the type.
    ///
    /// **Note** When implementing `CHOICE` types, this should be set to
    /// `Tag::EOC` to represent that's invalid for use.
    const TAG: Tag;
}

pub trait SequenceOf {}

impl<A: AsnType> SequenceOf for alloc::vec::Vec<A> {}
impl<A: AsnType, const N: usize> SequenceOf for [A; N] {}
impl<A: AsnType> SequenceOf for [A] {}

macro_rules! asn_type {
    ($($name:ty: $value:ident),+) => {
        $(
            impl AsnType for $name {
                const TAG: Tag = Tag::$value;
            }
        )+
    }
}

asn_type! {
    bool: BOOL,
    i8: INTEGER,
    i16: INTEGER,
    i32: INTEGER,
    i64: INTEGER,
    i128: INTEGER,
    isize: INTEGER,
    u8: INTEGER,
    u16: INTEGER,
    u32: INTEGER,
    u64: INTEGER,
    u128: INTEGER,
    usize: INTEGER,
    Integer: INTEGER,
    OctetString: OCTET_STRING,
    ObjectIdentifier: OBJECT_IDENTIFIER,
    Oid: OBJECT_IDENTIFIER,
    ConstOid: OBJECT_IDENTIFIER,
    BitString: BIT_STRING,
    Utf8String: UTF8_STRING,
    UtcTime: UTC_TIME,
    GeneralizedTime: GENERALIZED_TIME,
    (): NULL,
    &'_ str: UTF8_STRING

}

impl<T: AsnType> AsnType for alloc::vec::Vec<T> {
    const TAG: Tag = Tag::SEQUENCE;
}

impl<T: AsnType> AsnType for Option<T> {
    const TAG: Tag = T::TAG;
}

impl<T> AsnType for SetOf<T> {
    const TAG: Tag = Tag::SET;
}

impl<T: AsnType, const N: usize> AsnType for [T; N] {
    const TAG: Tag = Tag::SEQUENCE;
}

impl<T> AsnType for &'_ [T] {
    const TAG: Tag = Tag::SEQUENCE;
}

impl<const TAG: Tag, V> AsnType for Implicit<TAG, V> {
    const TAG: Tag = TAG;
}

impl<const TAG: Tag, V> AsnType for Explicit<TAG, V> {
    const TAG: Tag = TAG;
}

impl<K, V> AsnType for alloc::collections::BTreeMap<K, V> {
    const TAG: Tag = Tag::SEQUENCE;
}
