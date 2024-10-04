//! Traits for configuring a node.

use std::marker::PhantomData;

use reth_evm::execute::BlockExecutorProvider;
use reth_network_api::FullNetwork;
use reth_node_types::{NodeTypesWithDB, NodeTypesWithEngine};
use reth_payload_builder::PayloadBuilderHandle;
use reth_primitives::Header;
use reth_provider::FullProvider;
use reth_rpc_eth_api::EthApiTypes;
use reth_tasks::TaskExecutor;
use reth_transaction_pool::TransactionPool;

use crate::ConfigureEvm;

/// A helper trait that is downstream of the [`NodeTypesWithEngine`] trait and adds stateful
/// components to the node.
///
/// Its types are configured by node internally and are not intended to be user configurable.
pub trait FullNodeTypes: Send + Sync + Unpin + 'static {
    /// Node's types with the database.
    type Types: NodeTypesWithDB + NodeTypesWithEngine;
    /// The provider type used to interact with the node.
    type Provider: FullProvider<Self::Types>;
}

/// An adapter type that adds the builtin provider type to the user configured node types.
#[derive(Debug)]
pub struct FullNodeTypesAdapter<Types, Provider> {
    /// An instance of the user configured node types.
    pub types: PhantomData<Types>,
    /// The provider type used by the node.
    pub provider: PhantomData<Provider>,
}

impl<Types, Provider> FullNodeTypes for FullNodeTypesAdapter<Types, Provider>
where
    Types: NodeTypesWithDB + NodeTypesWithEngine,
    Provider: FullProvider<Types>,
{
    type Types = Types;
    type Provider = Provider;
}

/// Encapsulates all types and components of the node.
pub trait FullNodeComponents: FullNodeTypes + Clone + 'static {
    /// The transaction pool of the node.
    type Pool: TransactionPool + Unpin;

    /// The node's EVM configuration, defining settings for the Ethereum Virtual Machine.
    type Evm: ConfigureEvm<Header = Header>;

    /// The type that knows how to execute blocks.
    type Executor: BlockExecutorProvider;

    /// Network API.
    type Network: FullNetwork;

    /// Returns the transaction pool of the node.
    fn pool(&self) -> &Self::Pool;

    /// Returns the node's evm config.
    fn evm_config(&self) -> &Self::Evm;

    /// Returns the node's executor type.
    fn block_executor(&self) -> &Self::Executor;

    /// Returns the provider of the node.
    fn provider(&self) -> &Self::Provider;

    /// Returns the handle to the network
    fn network(&self) -> &Self::Network;

    /// Returns the handle to the payload builder service.
    fn payload_builder(
        &self,
    ) -> &PayloadBuilderHandle<<Self::Types as NodeTypesWithEngine>::Engine>;

    /// Returns handle to runtime.
    fn task_executor(&self) -> &TaskExecutor;
}

/// Customizable node add-on types.
pub trait NodeAddOns<N: FullNodeComponents>: Send + Sync + Unpin + Clone + 'static {
    /// The core `eth` namespace API type to install on the RPC server (see
    /// `reth_rpc_eth_api::EthApiServer`).
    type EthApi: EthApiTypes + Send + Clone;
}

impl<N: FullNodeComponents> NodeAddOns<N> for () {
    type EthApi = ();
}

/// Returns the builder for type.
pub trait BuilderProvider<N>: Send {
    /// Context required to build type.
    type Ctx<'a>;

    /// Returns builder for type.
    #[allow(clippy::type_complexity)]
    fn builder() -> Box<dyn for<'a> Fn(Self::Ctx<'a>) -> Self + Send>;
}

impl<N> BuilderProvider<N> for () {
    type Ctx<'a> = ();

    fn builder() -> Box<dyn for<'a> Fn(Self::Ctx<'a>) -> Self + Send> {
        Box::new(noop_builder)
    }
}

const fn noop_builder(_: ()) {}

/// Helper trait to relax trait bounds on [`NodeTypes`], when defining types.
pub trait NodeTy {
    /// The node's primitive types, defining basic operations and structures.
    type Primitives;
    /// The node's engine types, defining the interaction with the consensus engine.
    type Engine: Send + Sync + Clone + Unpin;
    /// The type used for configuration of the EVM.
    type ChainSpec: Send + Sync;
}

impl<T> NodeTy for T
where
    T: NodeTypesWithEngine + Clone,
{
    type Primitives = <T as NodeTypes>::Primitives;
    type Engine = <T as NodeTypesWithEngine>::Engine;
    type ChainSpec = <T as NodeTypes>::ChainSpec;
}

/// Helper trait to relax trait bounds on [`FullNodeComponents`] and [`FullNodeTypes`], when
/// defining types.
pub trait NodeCore: NodeTy + Clone {
    /// Underlying database type used by the node to store and retrieve data.
    type DB: Send + Sync + Clone + Unpin;
    /// The provider type used to interact with the node.
    type Provider: Send + Sync + Clone + Unpin;
    /// The transaction pool of the node.
    type Pool: Send + Sync + Clone + Unpin;
    /// The node's EVM configuration, defining settings for the Ethereum Virtual Machine.
    type Evm: Send + Sync + Clone + Unpin;
    /// The type that knows how to execute blocks.
    type Executor: Send + Sync + Clone + Unpin;
    /// Network API.
    type Network: Send + Sync + Clone;
}

impl<T> NodeCore for T
where
    T: FullNodeComponents,
{
    type DB = <T as FullNodeTypes>::DB;
    type Provider = <T as FullNodeTypes>::Provider;
    type Pool = <T as FullNodeComponents>::Pool;
    type Network = <T as FullNodeComponents>::Network;
    type Evm = <T as FullNodeComponents>::Evm;
    type Executor = <T as FullNodeComponents>::Executor;
}
