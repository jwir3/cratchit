//! A library for manipulating accounting data.
//!
//! This crate can be compiled to both Rust native code (for use within a native
//! application directly), or to WebAssembly, for use within either a webapp or
//! an electron application.
#![feature(custom_attribute)]

extern crate cfg_if;
use cfg_if::cfg_if;

use std::collections::HashMap;

extern crate json;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        extern crate wasm_bindgen;

        use wasm_bindgen::prelude::*;
    }
}

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(target_arch = "wasm32")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;

    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

/// An enumeration for controlling the type of currency.
///
/// Currently, only US Dollars are supported.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Currency {
    /// United States Dollars
    USDollar,

    /// Default currency type. This is used when deserialization didn't resolve to a known value.
    Unknown,
}

/// Convert from a string slice to a `Currency` type.
///
/// # Examples
/// ```
///# use cratchit::Currency;
/// let currency_string = "USD";
/// let currency = Currency::from(currency_string);
/// assert_eq!(currency, Currency::USDollar);
/// ```
impl<'a> From<&'a str> for Currency {
    fn from(abbrev: &'a str) -> Currency {
        match abbrev {
            "USD" => Currency::USDollar,
            _ => Currency::Unknown,
        }
    }
}

/// An enumeration for specifying the type of an `Account`.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AccountType {
    /// An Asset account, Accounts of this type should normally have a debit balance.
    Asset = 1,

    /// An Equity account. Accounts of this type should normally have a debit balance.
    Equity = 2,

    /// An Expense account. Accounts of this type should normally have a debit balance.
    Expense = 3,

    /// An Income account. Accounts of this type should normally have a credit balance.
    Income = 4,

    /// A Liability account. Accounts of this type should normally have a credit balance.
    Liability = 5,

    /// An account for which the account type isn't one of the other specified types.
    Other = 6,
}

/// Convert from a string slice to an `AccountType` value.
///
/// # Examples
/// ```
/// use cratchit::AccountType;
///
/// let account_type = AccountType::from("liability");
/// assert_eq!(account_type, AccountType::Liability);
/// ```
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

/// A formal record that represents certain resources, claims to such resources, and transactions
/// or other events that result in changes to those resources.
///
/// The resource type for an Account is determined by the `Currency` enumeration.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
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

/// Create an `Account` object from a `JsonValue`.
///
/// # Examples
/// ```
///#     extern crate json;
///#     extern crate cratchit;
///#     use cratchit::{Account, AccountType};
///#     use json::JsonValue;
///     let parsed = json::parse(
///        r#"
///        {
///            "name": "Assets",
///            "description": "Assets",
///            "id": "01",
///            "type": 1,
///            "currency": "USD",
///            "placeholder": true,
///            "subaccounts": []
///        }
///      "#,
///    );
///
///    let val: JsonValue = parsed.unwrap();
///    let account = Account::from(&val);
///    assert_eq!(account.get_name(), "Assets");
///    assert_eq!(account.get_id(), "01");
///    assert_eq!(account.get_account_type(), AccountType::Asset);
/// ```
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Account {
    /// Construct a new `Account` object.
    ///
    /// * id: An identifier for this account. This should be unique throughout all `Account`
    ///   objects in an `AccountsChart`, but this is not enforced yet.
    /// * name: The name of this `Account`. This should be human-readable, as it will most likely
    ///   be used for displaying this `Account` object within a user interface.
    /// * description: A longer descriptive explanation of this `Account`.
    /// * account_type: The `AccountType` for this `Account`.
    /// * currency: The `Currency` that denotes the resource this `Account` represents transactions
    ///   for.
    /// * placeholder: A boolean value indicating whether this `Account` will be a placeholder
    ///   `Account`. Placeholder accounts serve only as a grouping of other sub-accounts, and do
    ///   not have transactions assigned to them directly.
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

    /// Retrieve the id for this `Account`, as a `String`.
    pub fn get_id(&self) -> String {
        String::clone(&self.id)
    }

    /// Retrieve the name for this `Account`, as a `String`.
    pub fn get_name(&self) -> String {
        String::clone(&self.name)
    }

    /// Retrieve the description of this `Account`, as a `String.`
    pub fn get_description(&self) -> String {
        String::clone(&self.description)
    }

    /// Retrieve the account type of this `Account`.
    pub fn get_account_type(&self) -> AccountType {
        AccountType::clone(&self.account_type)
    }

    /// Retrieve the currency type of this `Account`.
    pub fn get_currency(&self) -> Currency {
        Currency::clone(&self.currency)
    }

    /// Determine if this `Account` is a placeholder account.
    ///
    /// A placeholder account is one that does not have an individual balance, but rather is used
    /// simply to group other accounts. Its balance is derived from the balances of its
    /// sub-accounts, and no transactions can be assigned to it directly.
    pub fn is_placeholder(&self) -> bool {
        self.placeholder
    }

    /// Add a new sub-account to this `Account`'s sub-tree.
    ///
    /// This adds a new child `Account` to this `Account`. This also effectively adds the child
    /// `Account` to whatever `AccountsChart` the parent account is part of, since Charts of
    /// Accounts are organized into top-level accounts, which are directly added to the
    /// `AccountsChart` object, and any descendent accounts of these top-level `Account`s are
    /// also part of the chart of accounts.
    ///
    /// # Examples
    /// ```
    ///# use cratchit::{Currency, Account, AccountType};
    /// let mut top_level_account = Account::new("01", "Assets",
    ///                                          "Economic resources that are expected to be of benefit in the future.",
    ///                                          AccountType::Asset,
    ///                                          Currency::USDollar,
    ///                                          true);
    /// let child_account = Account::new("01-01", "Current Assets",
    ///                                  "Assets that one can reasonably expect to convert into cash, sell, or consume within a single operating cycle, or within a single year if more than one cycle is completed each year.",
    ///                                  AccountType::Asset,
    ///                                  Currency::USDollar,
    ///                                  true);
    /// top_level_account.add_sub_account(child_account);
    /// ```
    pub fn add_sub_account(&mut self, account: Account) {
        self.sub_accounts.push(account);
    }

    fn get_sub_accounts(&self) -> Vec<Account> {
        Vec::clone(&self.sub_accounts)
    }

    fn get_sub_tree(&self) -> Vec<Account> {
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

/// A data structure representing a Chart of Accounts.
///
/// A Chart of Accounts is the listing of all of the `Account`s in a ledger. The `AccountsChart`
/// is actually composed of a set of "top-level" `Account`s, each of which have sub accounts that
/// contain individual transactions, as well as a balance.

//#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct AccountsChart {
    top_level_accounts: Vec<Account>,
}

impl<'a> From<&'a json::JsonValue> for AccountsChart {
    /// Convert from a string in Javascript-Object-Notation (JSON) form to an `AccountsChart`
    /// object.
    ///
    /// The `AccountsChart` in JSON form should be a single key-value pair, where the key is simply
    /// the word "accounts", and the value is an array of `Account` objects in JSON form.
    ///
    ///  # Arguments
    ///  * `value`: A `json::JsonValue` object representing the `AccountsChart` object in JSON
    ///    form.
    ///
    ///  # Returns
    ///  - An `AccountsChart` object.
    ///
    ///  # Examples
    ///  ```
    ///#     extern crate json;
    ///#     extern crate cratchit;
    ///#     use cratchit::{Account, AccountType, AccountsChart, Currency};
    ///#     use json::JsonValue;
    ///
    ///let parsed = json::parse(
    ///    r#"
    ///
    ///    {
    ///      "accounts": [
    ///        {
    ///            "name": "Assets",
    ///            "description": "Assets",
    ///            "id": "01",
    ///            "type": 1,
    ///            "currency": "USD",
    ///            "placeholder": true,
    ///            "subaccounts": [
    ///                {
    ///                  "name": "Accounts Receivable",
    ///                  "description": "Accounts Receivable",
    ///                  "id": "01-01",
    ///                  "type": 1,
    ///                  "currency": "USD",
    ///                  "placeholder": true,
    ///                  "subaccounts": [
    ///                    {
    ///                      "name": "Lakeville North High School",
    ///                      "description": "A/R for Lakeville North High School Hockey",
    ///                      "id": "01-0101",
    ///                      "type": 1,
    ///                      "currency": "USD",
    ///                      "placeholder": false,
    ///                      "subaccounts": []
    ///                    }
    ///                  ]
    ///                }
    ///            ]
    ///        }
    ///      ]
    ///  }
    ///
    ///  "#,
    ///);
    ///
    ///let accounts_json = parsed.unwrap();
    ///let accounts_chart = AccountsChart::from(&accounts_json);
    ///assert_eq!(accounts_chart.get_num_accounts(), 3);
    ///
    ///let account = accounts_chart.get_account_by_id("01-0101").unwrap();
    ///assert_eq!(account.get_name(), "Lakeville North High School");
    ///assert_eq!(
    ///    account.get_description(),
    ///    "A/R for Lakeville North High School Hockey"
    ///);
    ///assert_eq!(account.get_account_type(), AccountType::Asset);
    ///assert_eq!(account.is_placeholder(), false);
    ///assert_eq!(account.get_currency(), Currency::USDollar);
    ///  ```
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

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl AccountsChart {
    /// Create a new `AccountsChart` object.
    pub fn new() -> AccountsChart {
        let top_level_accounts = vec![];

        AccountsChart { top_level_accounts }
    }

    /// Add a top-level `Account` to this `AccountsChart` object.
    ///
    /// # Arguments
    /// * `account`: An `Account` object to add as a new top-level account.
    ///
    /// # Examples
    /// ```
    ///# use cratchit::{Account, AccountsChart, AccountType, Currency};
    ///     let account = Account::new(
    ///        "01",
    ///        "Accounts Receivable",
    ///        "Accounts Receivable",
    ///        AccountType::Asset,
    ///        Currency::USDollar,
    ///        true,
    ///    );
    ///
    ///    let mut accounts_chart = AccountsChart::new();
    ///    accounts_chart.add_top_level_account(account);
    /// ```
    pub fn add_top_level_account(&mut self, account: Account) {
        self.top_level_accounts.push(account);
    }

    /// Retrieve the number of `Account` objects in this `AccountsChart`.
    ///
    /// # Returns
    /// * A 'usize' indicating the number of `Account` objects in the entire tree of accessible
    ///   `Account`s from this `AccountsChart`.
    ///
    /// # Examples
    /// ```
    ///# use cratchit::{Account, AccountsChart, AccountType, Currency};
    ///     let account = Account::new(
    ///        "01",
    ///        "Accounts Receivable",
    ///        "Accounts Receivable",
    ///        AccountType::Asset,
    ///        Currency::USDollar,
    ///        true,
    ///    );
    ///
    ///    let mut accounts_chart = AccountsChart::new();
    ///    accounts_chart.add_top_level_account(account);
    ///
    ///    assert_eq!(accounts_chart.get_num_accounts(), 1);
    /// ```
    pub fn get_num_accounts(&self) -> usize {
        self.get_account_map().len()
    }

    /// Retrieve a single `Account` by its unique identifier.
    ///
    /// # Arguments
    /// * `id`: An `&str` containing the unique identifier for the `Account` to retrieve,
    ///
    /// # Returns
    /// * An `Option<Account>` having the `Account` with the specified id, if it exists in the
    ///   `AccountsChart` tree, or `None`, otherwise.
    ///
    /// # Examples
    /// ```
    ///# use cratchit::{Account, AccountsChart, AccountType, Currency};
    ///     let account = Account::new(
    ///        "01",
    ///        "Accounts Receivable",
    ///        "Accounts Receivable",
    ///        AccountType::Asset,
    ///        Currency::USDollar,
    ///        true,
    ///    );
    ///
    ///    let mut accounts_chart = AccountsChart::new();
    ///    accounts_chart.add_top_level_account(account);
    ///
    ///    let account1 = accounts_chart.get_account_by_id("01").unwrap();
    ///    assert_eq!(account1.get_name(), "Accounts Receivable");
    /// ```
    pub fn get_account_by_id(&self, id: &str) -> Option<Account> {
        let mapping = self.get_account_map();
        let result = mapping.get(&String::from(id));

        match result {
            Some(v) => Some(Account::clone(v)),
            None => None,
        }
    }

    /// Retrieve a list of all unique identifiers for `Account`s in this `AccountsChart`.
    ///
    /// # Returns
    ///   * A `Vec<String>` containing all of the known identifiers for `Account`s in the
    ///     `AccountsChart`.
    ///
    /// # Examples
    /// ```
    ///# use cratchit::{Account, AccountsChart, AccountType, Currency};
    ///     let account = Account::new(
    ///        "01",
    ///        "Accounts Receivable",
    ///        "Accounts Receivable",
    ///        AccountType::Asset,
    ///        Currency::USDollar,
    ///        true,
    ///    );
    ///
    ///    let mut accounts_chart = AccountsChart::new();
    ///    accounts_chart.add_top_level_account(account);
    ///
    ///    assert_eq!(accounts_chart.get_account_ids(), vec!["01"]);
    /// ```
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
