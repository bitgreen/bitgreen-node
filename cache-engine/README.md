# Bitgreen Cache Engine

### Installation
```
npm install
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

### Database
**prisma** package is being used for `postgresql` database.

To run migrations for this project run the command underneath. It will create all necessary tables and handle configuration.
```
npx prisma generate
npx prisma db push
```

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
