use failure;
use toml::Value;

use std::collections::BTreeMap;

use config::VoteServiceConfig;
use exonum::helpers::fabric::{keys, Argument, CommandExtension, Context};
use exonum::node::NodeConfig;

pub struct GenerateCommonConfig;

impl CommandExtension for GenerateCommonConfig {
    fn args(&self) -> Vec<Argument> {
        vec![Argument::new_named(
            "AUTHOR_PUBLIC_KEY",
            false,
            "Public key of vote author",
            None,
            "author-public-key",
            false,
        )]
    }

    fn execute(&self, mut context: Context) -> Result<Context, failure::Error> {
        let author_public_key = context
            .arg::<String>("AUTHOR_PUBLIC_KEY")
            .expect("AUTHOR_PUBLIC_KEY not found");

        let mut values: BTreeMap<String, Value> = context
            .get(keys::SERVICES_CONFIG)
            .expect("Expected services_config in context");

        values.extend(
            vec![(
                "author_public_key".to_owned(),
                Value::try_from(author_public_key).unwrap(),
            )].into_iter(),
        );

        context.set(keys::SERVICES_CONFIG, values);
        Ok(context)
    }
}

pub struct Finalize;

impl CommandExtension for Finalize {
    fn args(&self) -> Vec<Argument> {
        vec![]
    }

    fn execute(&self, mut context: Context) -> Result<Context, failure::Error> {
        let mut node_config: NodeConfig = context.get(keys::NODE_CONFIG).unwrap();
        let common_config = context.get(keys::COMMON_CONFIG).unwrap();

        let author_public_key = if let Some(author_public_key) =
            common_config.services_config.get("author_public_key")
        {
            Value::try_into(author_public_key.clone()).unwrap_or_default()
        } else {
            Default::default()
        };

        node_config.services_configs.insert(
            "voteservice_service".to_owned(),
            Value::try_from(VoteServiceConfig { author_public_key })
                .expect("Failed to serialize voteservice config"),
        );
        context.set(keys::NODE_CONFIG, node_config);
        Ok(context)
    }
}
