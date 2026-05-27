#![cfg(feature = "std")]

//! Verifies the zero-copy buffer decode path (`ReadXdrRc` / `from_xdr_with_buffer`):
//! decoding from a shared `Rc<[u8]>` produces a correct value whose large
//! opaque field borrows the input buffer instead of copying it.

use std::rc::Rc;

use stellar_xdr::{ContractCodeEntry, Limits, ReadXdrRc, WriteXdr};

#[test]
fn rc_decode_roundtrip_and_zero_copy() {
    let mut cce = ContractCodeEntry::default();
    cce.code = vec![0xABu8; 10_000].try_into().unwrap();
    let bytes = cce.to_xdr(Limits::none()).unwrap();

    let buf: Rc<[u8]> = Rc::from(bytes.as_slice());
    let decoded =
        ContractCodeEntry::from_xdr_with_buffer(Rc::clone(&buf), Limits::none()).unwrap();

    // Correctness: the buffer-decoded value equals the original.
    assert_eq!(decoded, cce);

    // Zero copy: the decoded `code` points into the shared input buffer.
    let code_ptr = decoded.code.as_ref().as_ptr();
    let buf_start = buf.as_ptr();
    let buf_end = buf_start.wrapping_add(buf.len());
    assert!(
        code_ptr >= buf_start && code_ptr < buf_end,
        "decoded `code` must borrow the input buffer, not copy it"
    );

    // Sharing: exactly the original handle plus the one inside `decoded.code`.
    assert_eq!(Rc::strong_count(&buf), 2);
}

#[test]
fn rc_decode_rejects_trailing_bytes() {
    let cce = ContractCodeEntry::default();
    let mut bytes = cce.to_xdr(Limits::none()).unwrap();
    bytes.push(0); // one extra byte -> not fully consumed
    let buf: Rc<[u8]> = Rc::from(bytes.as_slice());
    assert!(ContractCodeEntry::from_xdr_with_buffer(buf, Limits::none()).is_err());
}
