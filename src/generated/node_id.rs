#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// NodeId is an XDR Typedef defined as:
///
/// ```text
/// typedef PublicKey NodeID;
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
pub struct NodeId(pub PublicKey);

impl From<NodeId> for PublicKey {
    #[must_use]
    fn from(x: NodeId) -> Self {
        x.0
    }
}

impl From<PublicKey> for NodeId {
    #[must_use]
    fn from(x: PublicKey) -> Self {
        NodeId(x)
    }
}

impl AsRef<PublicKey> for NodeId {
    #[must_use]
    fn as_ref(&self) -> &PublicKey {
        &self.0
    }
}

impl ReadXdr for NodeId {
    #[cfg(feature = "std")]
    #[inline]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        // A newtype is a transparent wrapper; the inner type performs its own
        // depth/length accounting, so no extra depth is charged here.
        let i = PublicKey::read_xdr(r)?;
        let v = NodeId(i);
        Ok(v)
    }
}

impl WriteXdr for NodeId {
    #[cfg(feature = "std")]
    #[inline]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        self.0.write_xdr(w)
    }
}

#[cfg(feature = "std")]
impl ReadXdrRc for NodeId {
    #[inline]
    fn read_xdr_with_buffer(r: &mut RcReader) -> Result<Self, Error> {
        Ok(NodeId(PublicKey::read_xdr_with_buffer(r)?))
    }
}
