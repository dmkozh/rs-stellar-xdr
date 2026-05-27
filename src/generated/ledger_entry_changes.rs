#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// LedgerEntryChanges is an XDR Typedef defined as:
///
/// ```text
/// typedef LedgerEntryChange LedgerEntryChanges<>;
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
pub struct LedgerEntryChanges(pub VecM<LedgerEntryChange>);

impl From<LedgerEntryChanges> for VecM<LedgerEntryChange> {
    #[must_use]
    fn from(x: LedgerEntryChanges) -> Self {
        x.0
    }
}

impl From<VecM<LedgerEntryChange>> for LedgerEntryChanges {
    #[must_use]
    fn from(x: VecM<LedgerEntryChange>) -> Self {
        LedgerEntryChanges(x)
    }
}

impl AsRef<VecM<LedgerEntryChange>> for LedgerEntryChanges {
    #[must_use]
    fn as_ref(&self) -> &VecM<LedgerEntryChange> {
        &self.0
    }
}

impl ReadXdr for LedgerEntryChanges {
    #[cfg(feature = "std")]
    #[inline]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        // A newtype is a transparent wrapper; the inner type performs its own
        // depth/length accounting, so no extra depth is charged here.
        let i = VecM::<LedgerEntryChange>::read_xdr(r)?;
        let v = LedgerEntryChanges(i);
        Ok(v)
    }
}

impl WriteXdr for LedgerEntryChanges {
    #[cfg(feature = "std")]
    #[inline]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        self.0.write_xdr(w)
    }
}

#[cfg(feature = "std")]
impl ReadXdrRc for LedgerEntryChanges {
    #[inline]
    fn read_xdr_with_buffer(r: &mut RcReader) -> Result<Self, Error> {
        Ok(LedgerEntryChanges(
            VecM::<LedgerEntryChange>::read_xdr_with_buffer(r)?,
        ))
    }
}

impl Deref for LedgerEntryChanges {
    type Target = VecM<LedgerEntryChange>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<LedgerEntryChanges> for Vec<LedgerEntryChange> {
    #[must_use]
    fn from(x: LedgerEntryChanges) -> Self {
        x.0 .0
    }
}

impl TryFrom<Vec<LedgerEntryChange>> for LedgerEntryChanges {
    type Error = Error;
    fn try_from(x: Vec<LedgerEntryChange>) -> Result<Self, Error> {
        Ok(LedgerEntryChanges(x.try_into()?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<LedgerEntryChange>> for LedgerEntryChanges {
    type Error = Error;
    fn try_from(x: &Vec<LedgerEntryChange>) -> Result<Self, Error> {
        Ok(LedgerEntryChanges(x.try_into()?))
    }
}
impl AsRef<Vec<LedgerEntryChange>> for LedgerEntryChanges {
    #[must_use]
    fn as_ref(&self) -> &Vec<LedgerEntryChange> {
        &self.0 .0
    }
}

impl AsRef<[LedgerEntryChange]> for LedgerEntryChanges {
    #[cfg(feature = "alloc")]
    #[must_use]
    fn as_ref(&self) -> &[LedgerEntryChange] {
        &self.0 .0
    }
    #[cfg(not(feature = "alloc"))]
    #[must_use]
    fn as_ref(&self) -> &[LedgerEntryChange] {
        self.0 .0
    }
}
