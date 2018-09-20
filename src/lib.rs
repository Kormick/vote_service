#[macro_use]
extern crate exonum;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate byteorder;
extern crate ring;
extern crate serde_json;
extern crate toml;
#[macro_use]
extern crate lazy_static;
extern crate untrusted;

pub mod agreement;
pub mod api;
pub mod cipher;
pub mod cmd;
pub mod config;
pub mod contracts;
pub mod errors;
pub mod schema;
pub mod transactions;

use api::VoteServiceApi;
use config::VoteServiceConfig;
use exonum::encoding::serialize::json::reexport::Value;
use exonum::{
    api::ServiceApiBuilder,
    blockchain::{Service, Transaction, TransactionSet},
    crypto::Hash,
    encoding,
    messages::RawTransaction,
    storage::{Fork, Snapshot},
};
use serde_json::to_value;
use transactions::VoteTransactions;

pub const SERVICE_ID: u16 = 42;

#[derive(Debug, Default)]
pub struct VoteService {
    config: VoteServiceConfig,
}

impl Service for VoteService {
    fn service_name(&self) -> &'static str {
        "voteservice"
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<dyn Transaction>, encoding::Error> {
        let tx = VoteTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    fn state_hash(&self, _: &dyn Snapshot) -> Vec<Hash> {
        vec![]
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        VoteServiceApi::wire(builder);
    }

    fn initialize(&self, _fork: &mut Fork) -> Value {
        to_value(self.config.clone()).unwrap()
    }
}

use cmd::{Finalize, GenerateCommonConfig};
use exonum::blockchain;
use exonum::helpers::fabric::{self, keys, Command, CommandExtension, CommandName};

#[derive(Debug, Clone, Copy)]
pub struct ServiceFactory;

impl fabric::ServiceFactory for ServiceFactory {
    fn service_name(&self) -> &str {
        "voteservice"
    }

    fn command(&mut self, command: CommandName) -> Option<Box<dyn CommandExtension>> {
        use exonum::helpers::fabric;

        Some(match command {
            v if v == fabric::GenerateCommonConfig.name() => Box::new(GenerateCommonConfig),
            v if v == fabric::Finalize.name() => Box::new(Finalize),
            _ => return None,
        })
    }

    fn make_service(&mut self, context: &fabric::Context) -> Box<dyn blockchain::Service> {
        let service_config: VoteServiceConfig =
            context.get(keys::NODE_CONFIG).unwrap().services_configs["voteservice_service"]
                .clone()
                .try_into()
                .unwrap();

        let author_key = service_config.author_public_key.unwrap();
        let author_key = author_key.as_ref();
        agreement::init_ephemeral(author_key);

        Box::new(VoteService {
            config: service_config,
        })
    }
}

#[cfg(test)]
mod tests {
    use exonum::blockchain::Service;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use service;
    #[test]
    fn service_name() {
        let service = service::VoteService;
        let name = service.service_name();
        assert_eq!("voteservice", name);
    }
}
