const Cratchit = require('../pkg/cratchit.js');
var assert = require('assert');

describe("Account", function() {
  it ("should be able to create an account object from valid parameters", function() {
    var account = Cratchit.Account.new('01-01', 'SomeAccount',
                                       'Something that we got from wasm',
                                       Cratchit.AccountType.Asset,
                                       Cratchit.Currency.USDollar, false, []);

    assert.ok(account);
    assert.equal(account.get_id(), "01-01");
    assert.equal(account.get_account_type(), Cratchit.AccountType.Asset);
    assert.equal(account.get_name(), "SomeAccount");
    assert.equal(account.get_description(), "Something that we got from wasm");
    assert.equal(account.get_currency(), Cratchit.Currency.USDollar);
    assert.ok(!account.is_placeholder());
  });
});
