extern crate exonum;
extern crate exonum_configuration;
extern crate vote_service;

use exonum::helpers::fabric::NodeBuilder;

fn main() {
    exonum::helpers::init_logger().unwrap();

    NodeBuilder::new()
        .with_service(Box::new(exonum_configuration::ServiceFactory))
        .with_service(Box::new(vote_service::ServiceFactory))
        .run();
}
