import { expect, assert } from 'chai';
import jetpack from 'fs-jetpack';

import { AccountType } from '../src/Account.js';
import { AccountsChart } from '../src/AccountsChart';

var chartOfAccounts = jetpack.read('test/fixtures/ChartOfAccounts.json', 'json');

var accountIdRegex = /([0-9][0-9])(\-)([0-9][0-9])*/;

describe("Chart of Accounts Functionality", function() {
  it ("should load a chart of accounts successfully", function() {
    var chart = new AccountsChart(chartOfAccounts);

    expect(chart).to.be.ok;
    expect(chart.getNumAccounts()).to.eq(2);
    expect(chart.getAccountById('01').name).to.eq("Accounts Receivable");
    expect(chart.getAccountById('01').type).to.eq(AccountType.Asset);
    expect(chart.getAccountById('01').isPlaceholder()).to.be.true;
  });

  it ("should be able to retrieve a sub account from an existing chart of accounts", function () {
    var chart = new AccountsChart(chartOfAccounts);

    expect(chart).to.be.ok;

    var account = chart.getAccountById('01-01');
    expect(account.name).to.eq("Lakeville North High School");
    expect(account.description).to.eq("A/R for Lakeville North High School Hockey");
    expect(account.currency).to.eq("USD");
  });

  it ('should show there are two account ids in the chart of accounts', function() {
    var chart = new AccountsChart(chartOfAccounts);
    var chartIds = chart.getAccountIds();

    expect(chartIds).to.include('01');
    expect(chartIds).to.include('01-01');
  });

  it ('should identify account ids that are not unique', function() {
    var chart = new AccountsChart(chartOfAccounts);
    expect(chart.isAccountIdUsed('01-01')).to.be.true;
    expect(chart.isAccountIdUsed('01-02')).to.be.false;
  });
});
