// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use core::fmt;
use std::borrow::Borrow;
use std::ops::Deref;

use super::TD0Error;

const CHUNK_TAG_LEN: usize = 4;

pub fn is_chunk_name(val: &str) -> bool {
    val.chars().count() == CHUNK_TAG_LEN
        && val
            .get(0..3)
            .unwrap()
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
        && val.chars().nth(3).unwrap().is_ascii_lowercase()
}

pub fn is_ascii_printable(val: &str) -> bool {
    val.chars().all(|ch| ch.is_ascii_graphic() || ch == ' ')
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AsciiPrintableStr(str);

impl AsciiPrintableStr {
    const fn from_str_unchecked(s: &str) -> &Self {
        unsafe { &*(std::ptr::from_ref::<str>(s) as *const Self) }
    }

    #[allow(dead_code)]
    fn from_mut_str_unchecked(s: &mut str) -> &mut Self {
        unsafe { &mut *(std::ptr::from_mut::<str>(s) as *mut Self) }
    }
}

impl<'a> TryFrom<&'a str> for &'a AsciiPrintableStr {
    type Error = TD0Error;

    fn try_from(val: &'a str) -> Result<Self, Self::Error> {
        if is_ascii_printable(val) {
            Ok(AsciiPrintableStr::from_str_unchecked(val))
        } else {
            Err(TD0Error::NotAsciiPrintable)
        }
    }
}

impl AsRef<str> for AsciiPrintableStr {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for AsciiPrintableStr {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Deref for AsciiPrintableStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for AsciiPrintableStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl ToOwned for AsciiPrintableStr {
    type Owned = AsciiPrintableString;

    fn to_owned(&self) -> Self::Owned {
        AsciiPrintableString(self.0.to_string())
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AsciiPrintableString(String);

impl AsciiPrintableString {
    #[allow(dead_code)]
    pub fn from_string_unchecked(val: impl Into<String>) -> Self {
        Self(val.into())
    }
}

impl AsRef<AsciiPrintableStr> for AsciiPrintableString {
    fn as_ref(&self) -> &AsciiPrintableStr {
        AsciiPrintableStr::from_str_unchecked(&self.0)
    }
}

impl Borrow<AsciiPrintableStr> for AsciiPrintableString {
    fn borrow(&self) -> &AsciiPrintableStr {
        self.as_ref()
    }
}

impl Deref for AsciiPrintableString {
    type Target = AsciiPrintableStr;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl TryFrom<String> for AsciiPrintableString {
    type Error = TD0Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        <&AsciiPrintableStr>::try_from(s.as_str())?;
        Ok(Self(s))
    }
}

impl TryFrom<&str> for AsciiPrintableString {
    type Error = TD0Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        <&AsciiPrintableStr>::try_from(s)?;
        Ok(Self(s.to_string()))
    }
}

impl AsRef<str> for AsciiPrintableString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AsciiPrintableString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl PartialEq<str> for AsciiPrintableStr {
    fn eq(&self, other: &str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<AsciiPrintableStr> for str {
    fn eq(&self, other: &AsciiPrintableStr) -> bool {
        self == &other.0
    }
}

impl PartialEq<str> for AsciiPrintableString {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<AsciiPrintableStr> for AsciiPrintableString {
    fn eq(&self, other: &AsciiPrintableStr) -> bool {
        self.0 == other.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChunkTagStr(str);

impl ChunkTagStr {
    pub const fn from_str_unchecked(s: &str) -> &Self {
        unsafe { &*(std::ptr::from_ref::<str>(s) as *const Self) }
    }

    #[allow(dead_code)]
    fn from_mut_str_unchecked(s: &mut str) -> &mut Self {
        unsafe { &mut *(std::ptr::from_mut::<str>(s) as *mut Self) }
    }
}

impl<'a> TryFrom<&'a str> for &'a ChunkTagStr {
    type Error = TD0Error;

    fn try_from(val: &'a str) -> Result<Self, Self::Error> {
        if is_chunk_name(val) {
            Ok(ChunkTagStr::from_str_unchecked(val))
        } else {
            Err(TD0Error::InvalidChunkName)
        }
    }
}

impl From<&ChunkTagStr> for String {
    fn from(value: &ChunkTagStr) -> Self {
        value.0.to_string()
    }
}

impl AsRef<str> for ChunkTagStr {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for ChunkTagStr {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Deref for ChunkTagStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ChunkTagStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl ToOwned for ChunkTagStr {
    type Owned = ChunkTagString;

    fn to_owned(&self) -> Self::Owned {
        ChunkTagString(self.0.to_string())
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChunkTagString(String);

impl ChunkTagString {
    #[allow(dead_code)]
    pub fn from_string_unchecked(val: impl Into<String>) -> Self {
        Self(val.into())
    }
}

impl AsRef<ChunkTagStr> for ChunkTagString {
    fn as_ref(&self) -> &ChunkTagStr {
        ChunkTagStr::from_str_unchecked(&self.0)
    }
}

impl Borrow<ChunkTagStr> for ChunkTagString {
    fn borrow(&self) -> &ChunkTagStr {
        self.as_ref()
    }
}

impl Deref for ChunkTagString {
    type Target = ChunkTagStr;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl From<&ChunkTagStr> for ChunkTagString {
    fn from(value: &ChunkTagStr) -> Self {
        Self(value.to_string())
    }
}

impl TryFrom<String> for ChunkTagString {
    type Error = TD0Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        <&ChunkTagStr>::try_from(s.as_str())?;
        Ok(Self(s))
    }
}

impl TryFrom<&String> for ChunkTagString {
    type Error = TD0Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        <&ChunkTagStr>::try_from(value.as_str())?;
        Ok(Self(value.clone()))
    }
}

impl TryFrom<&str> for ChunkTagString {
    type Error = TD0Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        <&ChunkTagStr>::try_from(s)?;
        Ok(Self(s.to_string()))
    }
}

impl AsRef<str> for ChunkTagString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ChunkTagString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl PartialEq<str> for ChunkTagStr {
    fn eq(&self, other: &str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<ChunkTagStr> for str {
    fn eq(&self, other: &ChunkTagStr) -> bool {
        self == &other.0
    }
}

impl PartialEq<str> for ChunkTagString {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<ChunkTagStr> for ChunkTagString {
    fn eq(&self, other: &ChunkTagStr) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    pub fn test_ascii_printable_str_success() {
        let valid_bytes: Vec<u8> = Vec::from_iter(32u8..126u8);
        let valid_str = str::from_utf8(&valid_bytes).unwrap();
        let result = <&AsciiPrintableStr>::try_from(valid_str);
        result.unwrap();
    }

    #[test]
    pub fn test_ascii_printable_str_fails_non_printable() {
        let invalid_bytes: Vec<u8> = Vec::from_iter(31u8..35u8);
        let invalid_str = str::from_utf8(&invalid_bytes).unwrap();
        let result = <&AsciiPrintableStr>::try_from(invalid_str);
        assert_eq!(result, Err(TD0Error::NotAsciiPrintable));

        let result = <&AsciiPrintableStr>::try_from("Test\x7F");
        assert_eq!(result, Err(TD0Error::NotAsciiPrintable));
    }

    #[test]
    pub fn test_ascii_printable_str_fails_non_ascii_utf8() {
        let result = <&AsciiPrintableStr>::try_from("This ĭs a test");
        assert_eq!(result, Err(TD0Error::NotAsciiPrintable));
    }

    #[test]
    pub fn test_chunk_tag_string_success() {
        let result = ChunkTagString::try_from("TD0a");
        result.unwrap();

        let result = ChunkTagString::try_from("TD0b");
        result.unwrap();

        let result = ChunkTagString::try_from("TD0z");
        result.unwrap();
    }

    #[test]
    pub fn test_chunk_tag_string_failure() {
        let result = ChunkTagString::try_from("TD01");
        assert_eq!(
            result,
            Err(TD0Error::InvalidChunkName),
            "fails due to digit as last char."
        );

        let result = ChunkTagString::try_from("tD0a");
        assert_eq!(
            result,
            Err(TD0Error::InvalidChunkName),
            "fails due to lowercase first char."
        );

        let result = ChunkTagString::try_from("TD0A");
        assert_eq!(
            result,
            Err(TD0Error::InvalidChunkName),
            "fails due to uppercase last char."
        );
    }
}
