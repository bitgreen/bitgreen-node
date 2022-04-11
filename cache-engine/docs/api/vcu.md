# VCU Pallet API Documentation

### Search Accounts by Account and/or Date
```
http://localhost:3000/vcu/authorized_accounts
```
#### Search Destroyed Accounts by Account and/or Date
```
http://localhost:3000/vcu/authorized_accounts/destroyed
```
#### Params:
```
account: account
date_start: date of transaction seen on chain (optional)
date_end: date of transaction seen on chain (optional)
```

---

### Search Assets Generation by Account and/or Date
```
http://localhost:3000/vcu/assets_generating
```
#### Search Destroyed Assets Generation by Account and/or Date
```
http://localhost:3000/vcu/assets_generating/destroyed
```
#### Params:
```
account: account
date_start: date of transaction seen on chain (optional)
date_end: date of transaction seen on chain (optional)
```

---

### Search Assets Generation Schedule by Account and/or Date
```
http://localhost:3000/vcu/assets_generating_schedule
```
#### Search Destroyed Assets Generation Schedule by Account and/or Date
```
http://localhost:3000/vcu/assets_generating_schedule/destroyed
```
#### Params:
```
account: account
date_start: date of transaction seen on chain (optional)
date_end: date of transaction seen on chain (optional)
```

---

### Search Oracle Account Minting by Account and/or Date
```
http://localhost:3000/vcu/oracle_account_minting
```
##### Search Destroyed Oracle Account Minting by Account and/or Date
```
http://localhost:3000/vcu/oracle_account_minting/destroyed
```
#### Params:
```
account: account
date_start: date of transaction seen on chain (optional)
date_end: date of transaction seen on chain (optional)
```

---

### Search Proxy Settings by Date
```
http://localhost:3000/vcu/proxy_settings
```
#### Params:
```
date_start: date of transaction seen on chain (optional)
date_end: date of transaction seen on chain (optional)
```

---