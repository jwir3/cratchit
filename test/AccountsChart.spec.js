import { expect, assert } from 'chai';
import jetpack from 'fs-jetpack';

import { AccountType } from '../src/Account.js';
import { AccountsChart } from '../src/AccountsChart';

var chartOfAccounts = jetpack.read('test/fixtures/ChartOfAccounts.json', 'json');

describe("Chart of Accounts Functionality", function() {
  it ("should load a chart of accounts successfully", function() {
    var chart = new AccountsChart(chartOfAccounts);

    expect(chart).to.be.ok;
    expect(chart.getNumAccounts()).to.eq(2);
    expect(chart.getAccountById('01').name).to.eq("Accounts Receivable");
    // console.log(chart.getAccountById('01').type);
    expect(chart.getAccountById('01').type).to.eq(AccountType.Asset);
  });
});
