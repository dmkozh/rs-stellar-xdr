//! Shared, immutable byte storage for large variable-length XDR fields.
//!
//! `RcBytes` is a cheaply-clonable, immutable view over a reference-counted
//! byte buffer. It exists to make two operations cheap for large opaque/string
//! fields (e.g. contract Wasm, which can be 10s–100s of KB):
//!
//! - **Clone** is O(1): an `Rc` reference-count bump plus copying two `usize`s,
//!   never a copy of the bytes. Cloning a decoded structure that carries a
//!   large `RcBytes` therefore does not copy the payload.
//! - **Decode** can be zero-copy: [`RcBytes::read_opaque`] reads a
//!   length-prefixed XDR `opaque<>` directly out of a shared input buffer and
//!   returns a view into that same allocation — no allocation, no copy.
//!
//! `Rc` (not `Arc`) is used deliberately: XDR wrappers are not shared across
//! threads, so a non-atomic reference count is sufficient and cheaper. The
//! consequence is that types embedding `RcBytes` are `!Send + !Sync`; this is
//! why the storage is feature-gated rather than the default.
//!
//! Access is a plain pointer dereference via [`Deref`] to `[u8]` — there is no
//! re-parsing on access, so embedding `RcBytes` as a leaf in an otherwise
//! eagerly-parsed tree keeps nested-field access (e.g. `ScVal`) unchanged.

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::ops::Deref;

#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::rc::Rc;

use crate::Error;

/// An immutable, cheaply-clonable view over a reference-counted byte buffer.
#[derive(Clone)]
pub struct RcBytes {
    buf: Rc<[u8]>,
    start: usize,
    len: usize,
}

impl RcBytes {
    /// Creates an `RcBytes` by copying `s` into a freshly allocated buffer.
    ///
    /// This is the fallback path for callers that only have a borrowed slice
    /// (one copy). Subsequent clones are free.
    #[must_use]
    pub fn from_slice(s: &[u8]) -> Self {
        Self {
            buf: Rc::from(s),
            start: 0,
            len: s.len(),
        }
    }

    /// Wraps an entire shared buffer with no copy.
    #[must_use]
    pub fn from_rc(buf: Rc<[u8]>) -> Self {
        let len = buf.len();
        Self {
            buf,
            start: 0,
            len,
        }
    }

    /// Creates a zero-copy view of `buf[start..start + len]`, sharing `buf`.
    ///
    /// # Panics
    ///
    /// Panics if `start + len > buf.len()`.
    #[must_use]
    pub fn subslice(buf: &Rc<[u8]>, start: usize, len: usize) -> Self {
        assert!(
            start <= buf.len() && len <= buf.len() - start,
            "RcBytes::subslice out of bounds"
        );
        Self {
            buf: Rc::clone(buf),
            start,
            len,
        }
    }

    /// Returns the bytes this view refers to.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[self.start..self.start + self.len]
    }

    /// Returns the strong reference count of the shared buffer. Intended for
    /// tests that assert sharing rather than copying.
    #[must_use]
    pub fn strong_count(&self) -> usize {
        Rc::strong_count(&self.buf)
    }

    /// Reads a length-prefixed XDR `opaque<MAX>` out of the shared buffer `buf`
    /// starting at `*pos`, returning a zero-copy view that shares `buf`, and
    /// advances `*pos` past the value (including XDR 4-byte padding).
    ///
    /// # Errors
    ///
    /// - [`Error::LengthExceedsMax`] if the encoded length exceeds `max`.
    /// - [`Error::Invalid`] if the buffer is too short for the declared length
    ///   plus padding.
    /// - [`Error::NonZeroPadding`] if the trailing padding bytes are not zero.
    pub fn read_opaque(buf: &Rc<[u8]>, pos: &mut usize, max: u32) -> Result<Self, Error> {
        let p = *pos;
        // Length prefix (4 bytes, big-endian).
        let len_end = p.checked_add(4).ok_or(Error::Invalid)?;
        let len_slice = buf.get(p..len_end).ok_or(Error::Invalid)?;
        let len = u32::from_be_bytes(len_slice.try_into().map_err(|_| Error::Invalid)?);
        if len > max {
            return Err(Error::LengthExceedsMax);
        }
        let len = len as usize;
        // Data, then XDR padding to the next 4-byte boundary.
        let data_end = len_end.checked_add(len).ok_or(Error::Invalid)?;
        let padding = (4 - (len % 4)) % 4;
        let pad_end = data_end.checked_add(padding).ok_or(Error::Invalid)?;
        // Bounds-check both the data and the padding against the buffer.
        let _ = buf.get(len_end..data_end).ok_or(Error::Invalid)?;
        let pad = buf.get(data_end..pad_end).ok_or(Error::Invalid)?;
        if pad.iter().any(|b| *b != 0) {
            return Err(Error::NonZeroPadding);
        }
        *pos = pad_end;
        Ok(Self {
            buf: Rc::clone(buf),
            start: len_end,
            len,
        })
    }
}

impl Deref for RcBytes {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl PartialEq for RcBytes {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for RcBytes {}

impl Ord for RcBytes {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

impl PartialOrd for RcBytes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for RcBytes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state);
    }
}

impl core::fmt::Debug for RcBytes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RcBytes").field(&self.as_bytes()).finish()
    }
}

// The `arbitrary` feature implies `std`, so `Vec` is in the prelude here.
#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for RcBytes {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let v = Vec::<u8>::arbitrary(u)?;
        Ok(RcBytes::from_slice(&v))
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    fn rc(bytes: &[u8]) -> Rc<[u8]> {
        Rc::from(bytes.to_vec())
    }

    #[test]
    fn from_slice_copies_and_derefs() {
        let b = RcBytes::from_slice(b"hello");
        assert_eq!(&*b, b"hello");
        assert_eq!(b.len(), 5);
    }

    #[test]
    fn subslice_is_zero_copy() {
        let buf = rc(b"0123456789");
        let view = RcBytes::subslice(&buf, 2, 3);
        assert_eq!(&*view, b"234");
        // The view points into the original allocation, not a copy.
        assert_eq!(view.as_bytes().as_ptr(), buf.as_ptr().wrapping_add(2));
    }

    #[test]
    fn clone_shares_buffer_without_copying() {
        let buf = rc(b"shared payload");
        let view = RcBytes::subslice(&buf, 0, buf.len());
        let before = view.strong_count();
        let cloned = view.clone();
        assert_eq!(cloned.strong_count(), before + 1);
        // Same underlying bytes, same address: no copy was made.
        assert_eq!(cloned.as_bytes().as_ptr(), view.as_bytes().as_ptr());
    }

    #[test]
    fn eq_ord_hash_by_content_not_identity() {
        use std::collections::hash_map::DefaultHasher;
        let a = RcBytes::from_slice(b"abc");
        let b = RcBytes::subslice(&rc(b"abc"), 0, 3); // different allocation
        assert_eq!(a, b);
        assert_eq!(a.cmp(&b), Ordering::Equal);
        let h = |x: &RcBytes| {
            let mut s = DefaultHasher::new();
            x.hash(&mut s);
            std::hash::Hasher::finish(&s)
        };
        assert_eq!(h(&a), h(&b));
        let c = RcBytes::from_slice(b"abd");
        assert!(a < c);
    }

    #[test]
    fn read_opaque_zero_copy_and_advances() {
        // XDR opaque<>: u32 length (3), then 3 data bytes, then 1 pad byte.
        let mut wire = Vec::new();
        wire.extend_from_slice(&3u32.to_be_bytes());
        wire.extend_from_slice(b"abc");
        wire.push(0); // padding to a 4-byte boundary
        let buf: Rc<[u8]> = Rc::from(wire);

        let mut pos = 0usize;
        let v = RcBytes::read_opaque(&buf, &mut pos, u32::MAX).unwrap();
        assert_eq!(&*v, b"abc");
        assert_eq!(pos, 8); // 4 (len) + 3 (data) + 1 (pad)
        // Zero copy: the decoded value references the input buffer.
        assert_eq!(v.as_bytes().as_ptr(), buf.as_ptr().wrapping_add(4));
    }

    #[test]
    fn read_opaque_rejects_len_over_max() {
        let mut wire = Vec::new();
        wire.extend_from_slice(&3u32.to_be_bytes());
        wire.extend_from_slice(b"abc\0");
        let buf: Rc<[u8]> = Rc::from(wire);
        let mut pos = 0usize;
        assert!(matches!(
            RcBytes::read_opaque(&buf, &mut pos, 2),
            Err(Error::LengthExceedsMax)
        ));
    }

    #[test]
    fn read_opaque_rejects_nonzero_padding() {
        let mut wire = Vec::new();
        wire.extend_from_slice(&3u32.to_be_bytes());
        wire.extend_from_slice(b"abc");
        wire.push(1); // non-zero padding
        let buf: Rc<[u8]> = Rc::from(wire);
        let mut pos = 0usize;
        assert!(matches!(
            RcBytes::read_opaque(&buf, &mut pos, u32::MAX),
            Err(Error::NonZeroPadding)
        ));
    }

    #[test]
    fn read_opaque_rejects_truncated_input() {
        let mut wire = Vec::new();
        wire.extend_from_slice(&10u32.to_be_bytes()); // claims 10 bytes
        wire.extend_from_slice(b"abc"); // but only 3 present
        let buf: Rc<[u8]> = Rc::from(wire);
        let mut pos = 0usize;
        assert!(matches!(
            RcBytes::read_opaque(&buf, &mut pos, u32::MAX),
            Err(Error::Invalid)
        ));
    }
}
