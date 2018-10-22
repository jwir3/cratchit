extern crate json;

use super::currency::*;

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AccountType {
    Asset = 1,
    Equity = 2,
    Expense = 3,
    Income = 4,
    Liability = 5,
    Other = 6,
}

impl<'a> From<&'a str> for AccountType {
    fn from(value: &'a str) -> AccountType {
        let lowercase_value = String::from(value).to_lowercase();

        match lowercase_value.as_str() {
            "asset" => AccountType::Asset,
            "equity" => AccountType::Equity,
            "expense" => AccountType::Expense,
            "income" => AccountType::Income,
            "liability" => AccountType::Liability,
            _ => AccountType::Other,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Account {
    id: String,
    name: String,
    description: String,
    account_type: AccountType,
    currency: Currency,
    placeholder: bool,
    sub_accounts: Vec<Account>,
}

impl<'a> From<&'a json::JsonValue> for Account {
    fn from(value: &'a json::JsonValue) -> Account {
        let sub_accounts_json = &value["subaccounts"];

        let name = value["name"].as_str().unwrap();
        let id = value["id"].as_str().unwrap();
        let description = value["description"].as_str().unwrap();
        let act_type = AccountType::Asset;
        let currency_str = value["currency"].as_str().unwrap();
        let placeholder: bool = value["placeholder"].as_bool().unwrap_or(false);

        let mut result_account = Account::new(
            id,
            name,
            description,
            act_type,
            Currency::from(currency_str),
            placeholder,
        );
        result_account.parse_and_add_sub_accounts(sub_accounts_json);

        result_account
    }
}

impl Account {
    pub fn new(
        id: &str,
        name: &str,
        description: &str,
        account_type: AccountType,
        currency: Currency,
        placeholder: bool,
    ) -> Account {
        let id = String::from(id);
        let name = String::from(name);
        let description = String::from(description);
        Account {
            id: id,
            name: name,
            description: description,
            account_type: account_type,
            currency: currency,
            placeholder: placeholder,
            sub_accounts: vec![],
        }
    }

    fn parse_and_add_sub_accounts(&mut self, json: &json::JsonValue) {
        for next_json_value in json.members() {
            let account = Account::from(next_json_value);
            self.add_sub_account(account);
        }
    }

    pub fn get_id(&self) -> String {
        String::clone(&self.id)
    }

    pub fn get_name(&self) -> String {
        String::clone(&self.name)
    }

    pub fn get_description(&self) -> String {
        String::clone(&self.description)
    }

    pub fn get_account_type(&self) -> AccountType {
        AccountType::clone(&self.account_type)
    }

    pub fn get_currency(&self) -> Currency {
        Currency::clone(&self.currency)
    }

    pub fn is_placeholder(&self) -> bool {
        self.placeholder
    }

    pub fn add_sub_account(&mut self, account: Account) {
        self.sub_accounts.push(account);
    }

    pub fn get_sub_accounts(&self) -> Vec<Account> {
        Vec::clone(&self.sub_accounts)
    }

    pub fn get_sub_tree(&self) -> Vec<Account> {
        let mut accounts = vec![];
        for account in self.get_sub_accounts() {
            let tree = account.get_sub_tree();
            for tree_account in tree {
                accounts.push(tree_account);
            }

            accounts.push(account);
        }

        accounts
    }
}

pub struct AccountsChart {
    top_level_accounts: Vec<Account>,
}

impl<'a> From<&'a json::JsonValue> for AccountsChart {
    fn from(value: &'a json::JsonValue) -> AccountsChart {
        let mut top_level_accounts = vec![];
        let accounts_json = &value["accounts"];
        let mut count = 0;

        for next_account_json in accounts_json.members() {
            let account_data = Account::from(next_account_json);
            top_level_accounts.push(account_data);

            count = count + 1;
        }

        AccountsChart { top_level_accounts }
    }
}
impl AccountsChart {
    pub fn new() -> AccountsChart {
        let top_level_accounts = vec![];

        AccountsChart { top_level_accounts }
    }

    pub fn add_top_level_account(&mut self, account: Account) {
        self.top_level_accounts.push(account);
    }

    pub fn get_num_accounts(&self) -> usize {
        self.get_account_map().len()
    }

    pub fn get_account_by_id(&self, id: &str) -> Option<Account> {
        let mapping = self.get_account_map();
        let result = mapping.get(&String::from(id));

        match result {
            Some(v) => Some(Account::clone(v)),
            None => None,
        }
    }

    pub fn get_account_ids(&self) -> Vec<String> {
        let mapping = self.get_account_map();
        mapping.keys().map(|x| String::clone(x)).collect()
    }

    fn get_account_map(&self) -> HashMap<String, Account> {
        let mut mapping: HashMap<String, Account> = HashMap::new();
        let accounts_to_process = Vec::clone(&self.top_level_accounts);

        for next_account in accounts_to_process {
            mapping.insert(next_account.get_id(), Account::clone(&next_account));

            let sub_tree = next_account.get_sub_tree();
            for sub_account in sub_tree {
                mapping.insert(sub_account.get_id(), Account::clone(&sub_account));
            }
        }

        mapping
    }
}
