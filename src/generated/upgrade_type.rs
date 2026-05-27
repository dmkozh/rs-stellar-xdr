#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// UpgradeType is an XDR Typedef defined as:
///
/// ```text
/// typedef opaque UpgradeType<128>;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[derive(Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug)]
pub struct UpgradeType(pub BytesM<128>);

impl From<UpgradeType> for BytesM<128> {
    #[must_use]
    fn from(x: UpgradeType) -> Self {
        x.0
    }
}

impl From<BytesM<128>> for UpgradeType {
    #[must_use]
    fn from(x: BytesM<128>) -> Self {
        UpgradeType(x)
    }
}

impl AsRef<BytesM<128>> for UpgradeType {
    #[must_use]
    fn as_ref(&self) -> &BytesM<128> {
        &self.0
    }
}

impl ReadXdr for UpgradeType {
    #[cfg(feature = "std")]
    #[inline]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        // A newtype is a transparent wrapper; the inner type performs its own
        // depth/length accounting, so no extra depth is charged here.
        let i = BytesM::<128>::read_xdr(r)?;
        let v = UpgradeType(i);
        Ok(v)
    }
}

impl WriteXdr for UpgradeType {
    #[cfg(feature = "std")]
    #[inline]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        self.0.write_xdr(w)
    }
}

#[cfg(feature = "std")]
impl ReadXdrRc for UpgradeType {
    #[inline]
    fn read_xdr_with_buffer(r: &mut RcReader) -> Result<Self, Error> {
        Ok(UpgradeType(BytesM::<128>::read_xdr_with_buffer(r)?))
    }
}

impl Deref for UpgradeType {
    type Target = BytesM<128>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<UpgradeType> for Vec<u8> {
    #[must_use]
    fn from(x: UpgradeType) -> Self {
        x.0.into()
    }
}

impl TryFrom<Vec<u8>> for UpgradeType {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(UpgradeType(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for UpgradeType {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(UpgradeType(x.try_into()?))
    }
}

impl AsRef<[u8]> for UpgradeType {
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
