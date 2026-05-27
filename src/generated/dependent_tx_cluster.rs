#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// DependentTxCluster is an XDR Typedef defined as:
///
/// ```text
/// typedef TransactionEnvelope DependentTxCluster<>;
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
pub struct DependentTxCluster(pub VecM<TransactionEnvelope>);

impl From<DependentTxCluster> for VecM<TransactionEnvelope> {
    #[must_use]
    fn from(x: DependentTxCluster) -> Self {
        x.0
    }
}

impl From<VecM<TransactionEnvelope>> for DependentTxCluster {
    #[must_use]
    fn from(x: VecM<TransactionEnvelope>) -> Self {
        DependentTxCluster(x)
    }
}

impl AsRef<VecM<TransactionEnvelope>> for DependentTxCluster {
    #[must_use]
    fn as_ref(&self) -> &VecM<TransactionEnvelope> {
        &self.0
    }
}

impl ReadXdr for DependentTxCluster {
    #[cfg(feature = "std")]
    #[inline]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        // A newtype is a transparent wrapper; the inner type performs its own
        // depth/length accounting, so no extra depth is charged here.
        let i = VecM::<TransactionEnvelope>::read_xdr(r)?;
        let v = DependentTxCluster(i);
        Ok(v)
    }
}

impl WriteXdr for DependentTxCluster {
    #[cfg(feature = "std")]
    #[inline]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        self.0.write_xdr(w)
    }
}

#[cfg(feature = "std")]
impl ReadXdrRc for DependentTxCluster {
    #[inline]
    fn read_xdr_with_buffer(r: &mut RcReader) -> Result<Self, Error> {
        Ok(DependentTxCluster(
            VecM::<TransactionEnvelope>::read_xdr_with_buffer(r)?,
        ))
    }
}

impl Deref for DependentTxCluster {
    type Target = VecM<TransactionEnvelope>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DependentTxCluster> for Vec<TransactionEnvelope> {
    #[must_use]
    fn from(x: DependentTxCluster) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<TransactionEnvelope>> for DependentTxCluster {
    type Error = Error;
    fn try_from(x: Vec<TransactionEnvelope>) -> Result<Self, Error> {
        Ok(DependentTxCluster(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<TransactionEnvelope>> for DependentTxCluster {
    type Error = Error;
    fn try_from(x: &Vec<TransactionEnvelope>) -> Result<Self, Error> {
        Ok(DependentTxCluster(x.try_into()?))
    }
}
impl AsRef<Vec<TransactionEnvelope>> for DependentTxCluster {
    #[must_use]
    fn as_ref(&self) -> &Vec<TransactionEnvelope> {
        &self.0 .0
    }
}

impl AsRef<[TransactionEnvelope]> for DependentTxCluster {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[TransactionEnvelope] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[TransactionEnvelope] {
        self.0 .0
    }
}
