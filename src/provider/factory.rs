use std::collections::HashMap;
use crate::provider::provider::Provider;
use crate::provider::he_net::HeNetProvider;

pub struct ProviderFactory {
    pub providers: HashMap<String, Box<dyn Provider>>,
}

impl ProviderFactory {
    pub fn get(&mut self, name: &str) -> &Box<dyn Provider> {
        let provider = self.providers.entry(name.to_owned()).or_insert_with(|| Box::new(HeNetProvider::new()));
        provider
    }
}