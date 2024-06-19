//! Contains RPC handler implementations specific to blocks.

use reth_provider::{BlockReaderIdExt, HeaderProvider};

use crate::{
    servers::{EthBlocks, LoadBlock, LoadPendingBlock, SpawnBlocking},
    EthApi, EthStateCache,
};

impl<Provider, Pool, Network, EvmConfig> EthBlocks for EthApi<Provider, Pool, Network, EvmConfig>
where
    Self: LoadBlock,
    Provider: HeaderProvider,
{
    #[inline]
    fn provider(&self) -> impl reth_provider::HeaderProvider {
        self.inner.provider()
    }
}

impl<Provider, Pool, Network, EvmConfig> LoadBlock for EthApi<Provider, Pool, Network, EvmConfig>
where
    Self: LoadPendingBlock + SpawnBlocking,
    Provider: BlockReaderIdExt,
{
    #[inline]
    fn provider(&self) -> impl BlockReaderIdExt {
        self.inner.provider()
    }

    #[inline]
    fn cache(&self) -> &EthStateCache {
        self.inner.cache()
    }
}
