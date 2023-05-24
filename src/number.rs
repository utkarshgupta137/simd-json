use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;

use value_trait::StaticNode;

use crate::{BorrowedValue, Deserializer, OwnedValue, Result};

/// Borrowed arbitrary precision number.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BorrowedNumber<'num> {
    inner: Cow<'num, [u8]>,
}

impl<'num> BorrowedNumber<'num> {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    pub(crate) fn parse(&self) -> Result<StaticNode> {
        Deserializer::parse_number(0, &self.inner, self.inner[0] == b'-')
    }
}

impl<'num> From<&'num [u8]> for BorrowedNumber<'num> {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn from(number: &'num [u8]) -> Self {
        Self {
            inner: Cow::Borrowed(number),
        }
    }
}

impl<'num> From<Vec<u8>> for BorrowedNumber<'num> {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn from(number: Vec<u8>) -> Self {
        Self {
            inner: Cow::Owned(number),
        }
    }
}

impl<'num> Deref for BorrowedNumber<'num> {
    type Target = [u8];

    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'num> fmt::Debug for BorrowedNumber<'num> {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<'num> fmt::Display for BorrowedNumber<'num> {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(number) = self.parse() {
            number.fmt(f)
        } else {
            Err(fmt::Error)
        }
    }
}

/// Owned arbitrary precision number.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct OwnedNumber {
    inner: Vec<u8>,
}

impl OwnedNumber {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    pub(crate) fn parse(&self) -> Result<StaticNode> {
        Deserializer::parse_number(0, &self.inner, self.inner[0] == b'-')
    }
}

impl From<&[u8]> for OwnedNumber {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn from(number: &[u8]) -> Self {
        Self {
            inner: number.to_vec(),
        }
    }
}

impl From<Vec<u8>> for OwnedNumber {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn from(number: Vec<u8>) -> Self {
        Self { inner: number }
    }
}

impl Deref for OwnedNumber {
    type Target = [u8];

    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl fmt::Debug for OwnedNumber {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl fmt::Display for OwnedNumber {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(number) = self.parse() {
            number.fmt(f)
        } else {
            Err(fmt::Error)
        }
    }
}

/********** impls **********/
impl<'value> From<BorrowedNumber<'value>> for OwnedValue {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn from(number: BorrowedNumber<'value>) -> Self {
        Self::Number(OwnedNumber {
            inner: number.inner.into_owned(),
        })
    }
}

impl<'value> From<OwnedNumber> for BorrowedValue<'value> {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn from(number: OwnedNumber) -> Self {
        Self::Number(BorrowedNumber {
            inner: Cow::Owned(number.inner),
        })
    }
}

impl<'value> PartialEq<BorrowedNumber<'value>> for OwnedNumber {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn eq(&self, other: &BorrowedNumber<'value>) -> bool {
        self.inner == *other.inner
    }
}

impl<'value> PartialEq<OwnedNumber> for BorrowedNumber<'value> {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn eq(&self, other: &OwnedNumber) -> bool {
        self.inner == other.inner
    }
}

impl<'value> PartialEq<StaticNode> for BorrowedNumber<'value> {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn eq(&self, other: &StaticNode) -> bool {
        if let Ok(number) = self.parse() {
            &number == other
        } else {
            false
        }
    }
}

impl<'value> PartialEq<BorrowedNumber<'value>> for StaticNode {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn eq(&self, other: &BorrowedNumber<'value>) -> bool {
        if let Ok(number) = other.parse() {
            self == &number
        } else {
            false
        }
    }
}

impl<'value> PartialEq<OwnedNumber> for StaticNode {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn eq(&self, other: &OwnedNumber) -> bool {
        if let Ok(number) = other.parse() {
            self == &number
        } else {
            false
        }
    }
}

impl<'value> PartialEq<StaticNode> for OwnedNumber {
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    fn eq(&self, other: &StaticNode) -> bool {
        if let Ok(number) = self.parse() {
            &number == other
        } else {
            false
        }
    }
}
