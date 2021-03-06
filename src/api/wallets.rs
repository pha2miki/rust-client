use http::client::Client;
use std::borrow::Borrow;

use api::models::{Transaction, Wallet};
use api::Result;

pub struct Wallets {
    client: Client,
}

impl Wallets {
    pub fn new(client: Client) -> Wallets {
        Wallets { client }
    }

    pub fn all(&self) -> Result<Vec<Wallet>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&self, parameters: I) -> Result<Vec<Wallet>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("wallets", parameters)
    }

    pub fn top(&self) -> Result<Vec<Wallet>> {
        self.top_params(Vec::<(String, String)>::new())
    }

    pub fn top_params<I, K, V>(&self, parameters: I) -> Result<Vec<Wallet>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("wallets/top", parameters)
    }

    pub fn show(&self, id: &str) -> Result<Wallet> {
        let endpoint = format!("wallets/{}", id);
        self.client.get(&endpoint)
    }

    pub fn transactions(&self, id: &str) -> Result<Vec<Transaction>> {
        self.transactions_params(id, Vec::<(String, String)>::new())
    }

    pub fn transactions_params<I, K, V>(&self, id: &str, parameters: I) -> Result<Vec<Transaction>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("wallets/{}/transactions", id);
        self.client.get_with_params(&endpoint, parameters)
    }

    pub fn sent_transactions(&self, id: &str) -> Result<Vec<Transaction>> {
        self.sent_transactions_params(id, Vec::<(String, String)>::new())
    }

    pub fn sent_transactions_params<I, K, V>(
        &self,
        id: &str,
        parameters: I,
    ) -> Result<Vec<Transaction>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("wallets/{}/transactions/sent", id);
        self.client.get_with_params(&endpoint, parameters)
    }

    pub fn received_transactions(&self, id: &str) -> Result<Vec<Transaction>> {
        self.received_transactions_params(id, Vec::<(String, String)>::new())
    }

    pub fn received_transactions_params<I, K, V>(
        &self,
        id: &str,
        parameters: I,
    ) -> Result<Vec<Transaction>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("wallets/{}/transactions/received", id);
        self.client.get_with_params(&endpoint, parameters)
    }

    pub fn votes(&self, id: &str) -> Result<Vec<Transaction>> {
        let endpoint = format!("wallets/{}/votes", id);
        self.client.get(&endpoint)
    }

    pub fn search<I, K, V>(&self, parameters: I) -> Result<Vec<Wallet>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("wallets/search", parameters)
    }
}
