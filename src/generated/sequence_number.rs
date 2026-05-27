#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// SequenceNumber is an XDR Typedef defined as:
///
/// ```text
/// typedef int64 SequenceNumber;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct SequenceNumber(
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub i64,
);

impl From<SequenceNumber> for i64 {
    #[must_use]
    fn from(x: SequenceNumber) -> Self {
        x.0
    }
}

impl From<i64> for SequenceNumber {
    #[must_use]
    fn from(x: i64) -> Self {
        SequenceNumber(x)
    }
}

impl AsRef<i64> for SequenceNumber {
    #[must_use]
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl ReadXdr for SequenceNumber {
    #[cfg(feature = "std")]
    #[inline]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        // A newtype is a transparent wrapper; the inner type performs its own
        // depth/length accounting, so no extra depth is charged here.
        let i = i64::read_xdr(r)?;
        let v = SequenceNumber(i);
        Ok(v)
    }
}

impl WriteXdr for SequenceNumber {
    #[cfg(feature = "std")]
    #[inline]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        self.0.write_xdr(w)
    }
}

#[cfg(feature = "std")]
impl ReadXdrRc for SequenceNumber {
    #[inline]
    fn read_xdr_with_buffer(r: &mut RcReader) -> Result<Self, Error> {
        Ok(SequenceNumber(i64::read_xdr_with_buffer(r)?))
    }
}
