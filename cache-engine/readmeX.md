# Bitgreen Cache Engine

Bitgreen is a parachain on the Polkadot Network. This repository will manage a caching engine that will (1) download and store blocks to a Postgresql DB, (2) calculate derived data from on-chain sources, which is also stored in the DB, and (3) make this data available via public API.
---

### Installation and Setup
```
chmod +x bitgreen-cache-server-install.sh
./bitgreen-cache-server-install.sh
```

### Installation
```
npm install
```

---

#### Database
**node-pg-migrate** package is being used for `postgresql` migrations.

Run migrations for this project with the following commands. It will create all necessary tables and configuration.
```
npm run migrate up
```
To revert migration:
```
npm run migrate down
```

---

### Config
Copy `.env.example` to `.env` and set environment variables.
```
cp .env.example .env
nano .env
```
Edit `.env` file, and save it.

---

### Run Crawler
This script will listen for any new blocks.
```
npm run node
```

---

### Run API
This script will serve API. Please refer to [this page](docs/api.md) for more info.
```
npm run api
```