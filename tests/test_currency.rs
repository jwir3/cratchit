extern crate cratchit;

use cratchit::currency::Currency;

#[test]
fn currency_translation_from_string() {
    let usd = Currency::from("USD");
    let unknown = Currency::from("EUR");

    assert_eq!(usd, Currency::USDollar);
    assert_eq!(unknown, Currency::Unknown);
}
