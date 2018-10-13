export var AccountType = {
  "Asset": 1,
  "Equity": 2,
  "Expense": 3,
  "Income": 4,
  "Liability": 5,
};

export class Account {
  constructor (id, name, description, type, currency, placeholder,
               subaccountIds) {
    this.mId = id;
    this.mName = name;
    this.mDescription = description;

    this.mType = type;
    this.mCurrency = currency;
    this.mPlaceholder = placeholder;
    this.mSubaccountIds = subaccountIds;
  }

  get id() {
    return this.mId;
  }

  get name() {
    return this.mName;
  }

  get type() {
    return this.mType;
  }

  get description() {
    return this.mDescription;
  }

  get currency() {
    return this.mCurrency;
  }

  isPlaceholder() {
    return this.mPlaceholder;
  }
}
