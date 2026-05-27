#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// String64 is an XDR Typedef defined as:
///
/// ```text
/// typedef string string64<64>;
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
pub struct String64(pub StringM<64>);

impl From<String64> for StringM<64> {
    #[must_use]
    fn from(x: String64) -> Self {
        x.0
    }
}

impl From<StringM<64>> for String64 {
    #[must_use]
    fn from(x: StringM<64>) -> Self {
        String64(x)
    }
}

impl AsRef<StringM<64>> for String64 {
    #[must_use]
    fn as_ref(&self) -> &StringM<64> {
        &self.0
    }
}

impl ReadXdr for String64 {
    #[cfg(feature = "std")]
    #[inline]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        // A newtype is a transparent wrapper; the inner type performs its own
        // depth/length accounting, so no extra depth is charged here.
        let i = StringM::<64>::read_xdr(r)?;
        let v = String64(i);
        Ok(v)
    }
}

impl WriteXdr for String64 {
    #[cfg(feature = "std")]
    #[inline]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        self.0.write_xdr(w)
    }
}

#[cfg(feature = "std")]
impl ReadXdrRc for String64 {
    #[inline]
    fn read_xdr_with_buffer(r: &mut RcReader) -> Result<Self, Error> {
        Ok(String64(StringM::<64>::read_xdr_with_buffer(r)?))
    }
}

impl Deref for String64 {
    type Target = StringM<64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String64> for Vec<u8> {
    #[must_use]
    fn from(x: String64) -> Self {
        x.0.into()
    }
}

impl TryFrom<Vec<u8>> for String64 {
    type Error = Error;
    fn try_from(x: Vec<u8>) -> Result<Self, Error> {
        Ok(String64(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for String64 {
    type Error = Error;
    fn try_from(x: &Vec<u8>) -> Result<Self, Error> {
        Ok(String64(x.try_into()?))
    }
}

impl AsRef<[u8]> for String64 {
    #[must_use]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
