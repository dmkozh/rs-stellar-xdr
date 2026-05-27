#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TxAdvertVector is an XDR Typedef defined as:
///
/// ```text
/// typedef Hash TxAdvertVector<TX_ADVERT_VECTOR_MAX_SIZE>;
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
pub struct TxAdvertVector(pub VecM<Hash, 1000>);

impl From<TxAdvertVector> for VecM<Hash, 1000> {
    #[must_use]
    fn from(x: TxAdvertVector) -> Self {
        x.0
    }
}

impl From<VecM<Hash, 1000>> for TxAdvertVector {
    #[must_use]
    fn from(x: VecM<Hash, 1000>) -> Self {
        TxAdvertVector(x)
    }
}

impl AsRef<VecM<Hash, 1000>> for TxAdvertVector {
    #[must_use]
    fn as_ref(&self) -> &VecM<Hash, 1000> {
        &self.0
    }
}

impl ReadXdr for TxAdvertVector {
    #[cfg(feature = "std")]
    #[inline]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        // A newtype is a transparent wrapper; the inner type performs its own
        // depth/length accounting, so no extra depth is charged here.
        let i = VecM::<Hash, 1000>::read_xdr(r)?;
        let v = TxAdvertVector(i);
        Ok(v)
    }
}

impl WriteXdr for TxAdvertVector {
    #[cfg(feature = "std")]
    #[inline]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        self.0.write_xdr(w)
    }
}

#[cfg(feature = "std")]
impl ReadXdrRc for TxAdvertVector {
    #[inline]
    fn read_xdr_with_buffer(r: &mut RcReader) -> Result<Self, Error> {
        Ok(TxAdvertVector(VecM::<Hash, 1000>::read_xdr_with_buffer(r)?))
    }
}

impl Deref for TxAdvertVector {
    type Target = VecM<Hash, 1000>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TxAdvertVector> for Vec<Hash> {
    #[must_use]
    fn from(x: TxAdvertVector) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<Hash>> for TxAdvertVector {
    type Error = Error;
    fn try_from(x: Vec<Hash>) -> Result<Self, Error> {
        Ok(TxAdvertVector(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<Hash>> for TxAdvertVector {
    type Error = Error;
    fn try_from(x: &Vec<Hash>) -> Result<Self, Error> {
        Ok(TxAdvertVector(x.try_into()?))
    }
}
impl AsRef<Vec<Hash>> for TxAdvertVector {
    #[must_use]
    fn as_ref(&self) -> &Vec<Hash> {
        &self.0 .0
    }
}

impl AsRef<[Hash]> for TxAdvertVector {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[Hash] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[Hash] {
        self.0 .0
    }
}
