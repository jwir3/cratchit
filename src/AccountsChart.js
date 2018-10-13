import { Account } from './Account';

export class AccountsChart {
  constructor(jsonDescription) {
    this.accounts = [];

    for(let idx in jsonDescription.accounts) {
      let account = jsonDescription.accounts[idx];
      this._parseAccounts(account);
    }
  }

  getNumAccounts() {
    return this.accounts.length;
  }

  getAccountById(id) {
    for (let idx in this.accounts) {
      if (this.accounts[idx].id == id) {
        return this.accounts[idx];
      }
    }

    return null;
  }

  getAccountIds() {
    return this.allAccountNumbers;
  }

  isAccountIdUsed(accountId) {
    for (let idx in this.allAccountNumbers) {
      if (this.allAccountNumbers[idx] == accountId) {
        return true;
      }
    }

    return false;
  }

  _parseAccounts(account) {
    this.allAccountNumbers = [];

    let subActIds = [];

    // Parse all of the sub accounts first
    for (let subIdx in account.subaccounts) {
      this._parseAccounts(account.subaccounts[subIdx]);
      subActIds.push(account.subaccounts[subIdx].id);
      this.allAccountNumbers.push(account.subaccounts[subIdx].id);
    }

    // Then construct an Account object having the appropriate member
    // variables.
    this.allAccountNumbers.push(account.id);
    let act = new Account(account.id, account.name, account.description,
                          account.type, account.currency, account.placeholder,
                          subActIds);
    this.accounts.push(act);
  }
}
