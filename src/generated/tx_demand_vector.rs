#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// TxDemandVector is an XDR Typedef defined as:
///
/// ```text
/// typedef Hash TxDemandVector<TX_DEMAND_VECTOR_MAX_SIZE>;
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
pub struct TxDemandVector(pub VecM<Hash, 1000>);

impl From<TxDemandVector> for VecM<Hash, 1000> {
    #[must_use]
    fn from(x: TxDemandVector) -> Self {
        x.0
    }
}

impl From<VecM<Hash, 1000>> for TxDemandVector {
    #[must_use]
    fn from(x: VecM<Hash, 1000>) -> Self {
        TxDemandVector(x)
    }
}

impl AsRef<VecM<Hash, 1000>> for TxDemandVector {
    #[must_use]
    fn as_ref(&self) -> &VecM<Hash, 1000> {
        &self.0
    }
}

impl ReadXdr for TxDemandVector {
    #[cfg(feature = "std")]
    #[inline]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        // A newtype is a transparent wrapper; the inner type performs its own
        // depth/length accounting, so no extra depth is charged here.
        let i = VecM::<Hash, 1000>::read_xdr(r)?;
        let v = TxDemandVector(i);
        Ok(v)
    }
}

impl WriteXdr for TxDemandVector {
    #[cfg(feature = "std")]
    #[inline]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        self.0.write_xdr(w)
    }
}

#[cfg(feature = "std")]
impl ReadXdrRc for TxDemandVector {
    #[inline]
    fn read_xdr_with_buffer(r: &mut RcReader) -> Result<Self, Error> {
        Ok(TxDemandVector(VecM::<Hash, 1000>::read_xdr_with_buffer(r)?))
    }
}

impl Deref for TxDemandVector {
    type Target = VecM<Hash, 1000>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TxDemandVector> for Vec<Hash> {
    #[must_use]
    fn from(x: TxDemandVector) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<Hash>> for TxDemandVector {
    type Error = Error;
    fn try_from(x: Vec<Hash>) -> Result<Self, Error> {
        Ok(TxDemandVector(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<Hash>> for TxDemandVector {
    type Error = Error;
    fn try_from(x: &Vec<Hash>) -> Result<Self, Error> {
        Ok(TxDemandVector(x.try_into()?))
    }
}
impl AsRef<Vec<Hash>> for TxDemandVector {
    #[must_use]
    fn as_ref(&self) -> &Vec<Hash> {
        &self.0 .0
    }
}

impl AsRef<[Hash]> for TxDemandVector {
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
