#[macro_use]
extern crate exonum;
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod api;
pub mod contracts;
pub mod schema;
pub mod transactions;

pub mod service {
    use exonum::{
        api::ServiceApiBuilder,
        blockchain::{Service, Transaction, TransactionSet},
        crypto::Hash,
        encoding,
        messages::RawTransaction,
        storage::Snapshot,
    };

    use api::VoteServiceApi;
    use transactions::VoteTransactions;

    pub const SERVICE_ID: u16 = 42;

    #[derive(Debug)]
    pub struct VoteService;

    impl Service for VoteService {
        fn service_name(&self) -> &'static str {
            println!("VoteService::service_name");
            "voteservice"
        }

        fn service_id(&self) -> u16 {
            SERVICE_ID
        }

        fn tx_from_raw(
            &self,
            raw: RawTransaction,
        ) -> Result<Box<dyn Transaction>, encoding::Error> {
            println!("VoteService::tx_from_raw");
            let tx = VoteTransactions::tx_from_raw(raw)?;
            Ok(tx.into())
        }

        fn state_hash(&self, _: &dyn Snapshot) -> Vec<Hash> {
            vec![]
        }

        fn wire_api(&self, builder: &mut ServiceApiBuilder) {
            println!("VoteService::wire_api");
            VoteServiceApi::wire(builder);
        }
    }
}

pub mod factory {
    use exonum::blockchain;
    use exonum::helpers::fabric;
    use service::VoteService;

    #[derive(Debug, Clone, Copy)]
    pub struct ServiceFactory;

    impl fabric::ServiceFactory for ServiceFactory {
        fn service_name(&self) -> &str {
            "voteservice"
        }

        fn make_service(&mut self, _: &fabric::Context) -> Box<dyn blockchain::Service> {
            Box::new(VoteService)
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
