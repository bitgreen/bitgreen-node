# Bitgreen API Documentation
Serve API via command `npm run api`, API endpoint will be available at **localhost:port**, where **port** is defined in `.env` file. Default port: **3000**

---

### Base Endpoint
```
http://localhost:3000/
```

---

### Search Transactions by Account and Date
```
http://localhost:3000/transactions
```
#### Params:
```
account: account of sender or recipient
date_start: date of transaction seen on chain (optional)
date_end: date of transaction seen on chain (optional)
```

---

### Get Transaction by Hash
```
http://localhost:3000/transaction
```
#### Params:
```
hash: transaction hash you want to retrieve
```

---

### Get All Assets
Returns all _assets_
```
http://localhost:3000/assets
```

---

### Get Assets Transactions
Returns all _asset transactions_
```
http://localhost:3000/assets/transactions
```
#### Params:
```
asset_id: asset id
account: account of sender or recipient
date_start: date of transaction seen on chain (optional)
date_end: date of transaction seen on chain (optional)
```

---

### Get Asset Transaction by Hash
```
http://localhost:3000/assets/transaction
```
#### Params:
```
hash: transaction hash you want to retrieve
```

---

### Get Impact Actions
Returns all _impact actions_
```
http://localhost:3000/impact_actions
```

---

### Get Impact Actions - Auditors
Returns all _auditors_
```
http://localhost:3000/impact_actions/auditors
```

---

### Get Impact Actions - Categories
Returns all _categories_
```
http://localhost:3000/impact_actions/categories
```

---

### Get Impact Actions - Oracles
Returns all _oracles_
```
http://localhost:3000/impact_actions/oracles
```

---

### Get Impact Actions - Proxies
Returns all _proxies_
```
http://localhost:3000/impact_actions/proxy
```

---

### Get Impact Actions - Approval Requests
Returns all _approval requests_
```
http://localhost:3000/impact_actions/approval_requests
```

---

### Get Impact Actions - Approval Request By ID
Returns _approval request_
```
http://localhost:3000/impact_actions/approval_request
```
#### Params:
```
approval_request_id: approval request id
```

---

### Get Impact Actions - Approval Requests - Auditors
Returns all _auditors_ for given _approval request_
```
http://localhost:3000/impact_actions/approval_requests/auditors
```
#### Params:
```
approval_request_id: approval request id
```

---

### Get Impact Actions - Approval Requests - Auditors - Votes
Returns all _votes_ for given _approval request_
```
http://localhost:3000/impact_actions/approval_requests/auditors/votes
```
#### Params:
```
approval_request_id: approval request id
```

## Analyze Data
Returns all possible sections/methods fetched from a chain.
```
http://localhost:3000/analyze-data/
```
#### Params:
```
section: name of section to search for (optional)
```

---