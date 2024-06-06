//! Error type

/// Optimism specific payload building errors.
#[derive(Debug, thiserror::Error)]
pub enum OptimismPayloadBuilderError {
    /// Thrown when a transaction fails to convert to a
    /// [`reth_primitives::TransactionSignedEcRecovered`].
    #[error("failed to convert deposit transaction to TransactionSignedEcRecovered")]
    TransactionEcRecoverFailed,
    /// Thrown when the L1 block info could not be parsed from the calldata of the
    /// first transaction supplied in the payload attributes.
    #[error("failed to parse L1 block info from L1 info tx calldata")]
    L1BlockInfoParseFailed,
    /// Thrown when a database account could not be loaded.
    #[error("failed to load account {0}")]
    AccountLoadFailed(reth_primitives::Address),
    /// Thrown when force deploy of create2deployer code fails.
    #[error("failed to force create2deployer account code")]
    ForceCreate2DeployerFail,
    /// Thrown when a blob transaction is included in a sequencer's block.
    #[error("blob transaction included in sequencer block")]
    BlobTransactionRejected,
    /// OP requires system tx to be included in the block.
    ///
    /// Without at least the system tx in the block, no valid block can be produced, because the
    /// first tx in the block contains the L1 block info.
    #[error("empty payload building is unsupported")]
    EmptyPayloadUnsupported,
}
