# Bitgreen Cache Engine

The Bitgreen Caching Engine makes on-chain actions and function results available with REST API calls, and provides a JSON response (Javascript Object Notation). This simplifies the process of querying the Bitgreen chain, making it accessible to anyone with an Internet connection, and does not require a node or other blockchain related deployments. You can use the example links below to see the data returned when the cache engine is queried.


Examples: 

http://157.90.126.46:3000/vcu/projects/?originator=5G6M646srfn77x6uzwReRhCsVdBDRtYMd74nQuHkxvZX1SLC

http://157.90.126.46:3000/assets/transactions?date_start=2022-05-01&date_end=2022-05-31

http://157.90.126.46:3000/asset?asset_id=1000


If you require any assistance using these features, or this repo, please contact us at [contact], we are happy to assist.

Additional, detailed technical information regarding the use and response of the caching engine can be found below. 

### Installation
```
npm install
```

---

#### Database
**node-pg-migrate** package is being used for `postgresql` migrations.

To run migrations for this project run the command underneath. It will create all necessary tables and handle configuration.
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

### Run Fetcher
This script will process all blocks from X to Y.
Ending block number is optional.
```
npm run fetch -- --block-start=X --block-end=Y
```
To analyze blockchain for data, you can use `-a` flag.
```
npm run fetch -- --block-start=X -a
```
Run the following command for more details:
```
npm run fetch -- --help
```

---

### Run API
This script will serve API. Please refer to [this page](docs/api.md) for more info.
```
npm run api
```
