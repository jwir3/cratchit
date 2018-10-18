extern crate cratchit;
extern crate json;

use cratchit::accounts::{Account, AccountType, AccountsChart};
use cratchit::currency::Currency;

fn get_chart_of_accounts() -> json::JsonValue {
    let parsed = json::parse(
        r#"

        {
          "accounts": [
            {
                "name": "Assets",
                "description": "Assets",
                "id": "01",
                "type": 1,
                "currency": "USD",
                "placeholder": true,
                "subaccounts": [
                    {
                      "name": "Accounts Receivable",
                      "description": "Accounts Receivable",
                      "id": "01-01",
                      "type": 1,
                      "currency": "USD",
                      "placeholder": true,
                      "subaccounts": [
                        {
                          "name": "Lakeville North High School",
                          "description": "A/R for Lakeville North High School Hockey",
                          "id": "01-0101",
                          "type": 1,
                          "currency": "USD",
                          "placeholder": false,
                          "subaccounts": []
                        }
                      ]
                    }
                ]
            }
          ]
      }

      "#,
    );

    parsed.unwrap()
}

#[test]
fn account_type_from_string() {
    let lc_string = "asset";
    let lc_string_type = AccountType::from(lc_string);

    let mixed_case_string = "LiaBilIty";
    let mixed_case_string_type = AccountType::from(mixed_case_string);

    assert_eq!(lc_string_type, AccountType::Asset);
    assert_eq!(mixed_case_string_type, AccountType::Liability);
}

#[test]
fn account_type_from_integer() {
    assert_eq!(AccountType::Liability as u32, 5);
    assert_eq!(AccountType::Asset as u32, 1);
}

#[test]
fn account_creation() {
    let account = Account::new(
        "01",
        "Accounts Receivable",
        "Accounts Receivable",
        AccountType::Asset,
        Currency::USDollar,
        true,
    );

    assert_eq!(account.get_id(), "01");
    assert_eq!(account.get_name(), "Accounts Receivable");
    assert_eq!(account.get_description(), "Accounts Receivable");
    assert_eq!(account.get_account_type(), AccountType::Asset);
    assert_eq!(account.get_currency(), Currency::USDollar);
    assert_eq!(account.is_placeholder(), true);
}

#[test]
fn adding_top_level_accounts_to_accounts_chart() {
    let account = Account::new(
        "01",
        "Accounts Receivable",
        "Accounts Receivable",
        AccountType::Asset,
        Currency::USDollar,
        true,
    );

    let mut accounts_chart = AccountsChart::new();
    accounts_chart.add_top_level_account(account);
}

#[test]
fn creating_accounts_chart_from_json() {
    let accounts_json = get_chart_of_accounts();

    let accounts_chart = AccountsChart::from(&accounts_json);
    assert_eq!(accounts_chart.get_num_accounts(), 3);

    let account = accounts_chart.get_account_by_id("01-0101").unwrap();
    assert_eq!(account.get_name(), "Lakeville North High School");
    assert_eq!(
        account.get_description(),
        "A/R for Lakeville North High School Hockey"
    );
    assert_eq!(account.get_account_type(), AccountType::Asset);
    assert_eq!(account.is_placeholder(), false);
    assert_eq!(account.get_currency(), Currency::USDollar);
}

#[test]
fn getting_all_account_ids_in_a_chart() {
    let accounts_json = get_chart_of_accounts();

    let accounts_chart = AccountsChart::from(&accounts_json);
    let account_ids = accounts_chart.get_account_ids();

    assert_eq!(account_ids.len(), 3);
    assert!(account_ids.contains(&String::from("01")));
    assert!(account_ids.contains(&String::from("01-01")));
    assert!(account_ids.contains(&String::from("01-0101")));
}

// #[macro_use]
// extern crate galvanic_test;
// extern crate cratchit;
// extern crate json;
//
// use galvanic_test::*;
//
// // test suites are only built when a test is executed, e.g., with `cargo test`
// test_suite! {
//     // for anonymous test suites remove the name directive
//     name chart_of_accounts;
//
//     use json::JsonValue;
//
//     // suites act as modules and may contain any item
//     // fn calc(a: i32, b: i32) -> i32 { a*b }
//
//     fixture accounts_json() -> JsonValue {
//         use self::*;
//
//         setup(&mut self) {
//             use json;
//
//
//           parsed
//         }
//     }
//
//     test account_creation() {
//         let account = Account::new("01");
//
//         assert_eq!(account.id, "01");
//     }
//     // test accounts(accounts_json) {
//     //     println!("Json Value: {:?}", accounts_json.val);
//     //     assert_eq!(2, 2);
//     // }
//
//     // attributes can still be applied as for functions
//     // #[should_panic]
//     // test another_test() {
//     //     assert_eq!(calc(3,2), 7);
//     // }
// }
