use jni::objects::JString;

use super::{ *};
use std::{char, iter, mem::transmute, slice};

/// Represents an env.GetStringChars + env.GetStringLength query.
/// Will automatically env.ReleaseStringChars when dropped.
pub struct StringChars<'env> {
    env: &'env jni::JNIEnv<'env>,
    string: JString<'env>,
    chars: *const i8,
    length: jsize, // in characters
}

impl<'env> StringChars<'env> {
    /// Construct a StringChars from a jni::JNIEnv + jstring.
    pub unsafe fn from_env_jstring(
        env: &'env jni::JNIEnv,
        string: JString<'env>,
    ) -> jni::errors::Result<Self> {
        debug_assert!(!string.is_null());

        let chars = env.get_string_utf_chars(string)?;
        let length = env.get_array_length(chars as jarray)?;

        Ok(Self {
            env,
            string,
            chars,
            length,
        })
    }

    /// Get an array of [jchar]s.  Generally UTF16, but not guaranteed to be valid UTF16.
    ///
    /// [jchar]:                    struct.jchar.html
    pub fn chars(&self) -> &[jchar] {
        unsafe { slice::from_raw_parts(self.chars as *const u16, self.length as usize) }
    }

    /// Get an array of [u16]s.  Generally UTF16, but not guaranteed to be valid UTF16.
    pub fn as_u16_slice(&self) -> &[u16] {
        unsafe { transmute::<&[jchar], &[u16]>(self.chars()) }
    }

    /// std::char::[decode_utf16]\(...\)s these string characters.
    ///
    /// [decode_utf16]:             https://doc.rust-lang.org/std/char/fn.decode_utf16.html
    pub fn decode(&self) -> char::DecodeUtf16<iter::Cloned<slice::Iter<u16>>> {
        char::decode_utf16(self.as_u16_slice().iter().cloned())
    }

    /// Returns a new [Ok]\([String]\), or an [Err]\([DecodeUtf16Error]\) if if it contained any invalid UTF16.
    ///
    /// [Ok]:                       https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [Err]:                      https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    /// [DecodeUtf16Error]:         https://doc.rust-lang.org/std/char/struct.DecodeUtf16Error.html
    /// [String]:                   https://doc.rust-lang.org/std/string/struct.String.html
    /// [REPLACEMENT_CHARACTER]:    https://doc.rust-lang.org/std/char/constant.REPLACEMENT_CHARACTER.html
    pub fn to_string(&self) -> Result<String, char::DecodeUtf16Error> {
        self.decode().collect()
    }

    /// Returns a new [String] with any invalid UTF16 characters replaced with [REPLACEMENT_CHARACTER]s (`'\u{FFFD}'`.)
    ///
    /// [String]:                   https://doc.rust-lang.org/std/string/struct.String.html
    /// [REPLACEMENT_CHARACTER]:    https://doc.rust-lang.org/std/char/constant.REPLACEMENT_CHARACTER.html
    pub fn to_string_lossy(&self) -> String {
        self.decode()
            .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
            .collect()
    }
}

impl<'env> Drop for StringChars<'env> {
    fn drop(&mut self) {
        self.env.release_string_utf_chars(self.string, self.chars).unwrap();
    }
}
