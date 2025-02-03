## To setup:
```bash
cd frontend
npm i
```

Create a `.env` file containing MYSQL parameters:
```properties
MYSQL_USER = [MYSQL USERNAME HERE]
MYSQL_PASSWORD = [MYSQL PASSWORD HERE]
MYSQL_HOST = [MYSQL SERVER HOSTNAME/IP HERE (e.g. localhost)]
MYSQL_PORT = [MYSQL TCP PORT HERE (usually 3306)]
MYSQL_DB_NAME = [SHOP DB NAME HERE (e.g. shop)]
```

Setup/update the database/tables
```bash
cargo run --bin update_db
```

## To run frontend in dev mode:
```bash
cd frontend
npm run dev
```

## To run for prod:
```bash
cd frontend
npm run build
cd ..
cargo run --release
```
