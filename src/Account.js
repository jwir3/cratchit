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
}
