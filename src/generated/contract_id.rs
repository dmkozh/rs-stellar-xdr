#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ContractId is an XDR Typedef defined as:
///
/// ```text
/// typedef Hash ContractID;
/// ```
///
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[derive(Debug)]
pub struct ContractId(pub Hash);

impl From<ContractId> for Hash {
    #[must_use]
    fn from(x: ContractId) -> Self {
        x.0
    }
}

impl From<Hash> for ContractId {
    #[must_use]
    fn from(x: Hash) -> Self {
        ContractId(x)
    }
}

impl AsRef<Hash> for ContractId {
    #[must_use]
    fn as_ref(&self) -> &Hash {
        &self.0
    }
}

impl ReadXdr for ContractId {
    #[cfg(feature = "std")]
    #[inline]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        // A newtype is a transparent wrapper; the inner type performs its own
        // depth/length accounting, so no extra depth is charged here.
        let i = Hash::read_xdr(r)?;
        let v = ContractId(i);
        Ok(v)
    }
}

impl WriteXdr for ContractId {
    #[cfg(feature = "std")]
    #[inline]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        self.0.write_xdr(w)
    }
}

#[cfg(feature = "std")]
impl ReadXdrRc for ContractId {
    #[inline]
    fn read_xdr_with_buffer(r: &mut RcReader) -> Result<Self, Error> {
        Ok(ContractId(Hash::read_xdr_with_buffer(r)?))
    }
}
