import { memory } from "cratchit/cratchit_bg";
import { Currency, AccountType, Account } from "cratchit";

const main = () => {
  let account = Account.new('01-01', 'SomeAccount',
                            'Something that we got from wasm',
                            AccountType.Asset, Currency.USDollar, false, []);
  console.log(account.get_id());
};

main();
