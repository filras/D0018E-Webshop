# D0018E Webshop

## To deploy:

The easiest way to deploy is to use `docker compose`:

```bash
docker compose up
```

This will download all required components (including the DB) and run the web server (frontend and API) on port 80. To try the webshop out, simply visit [http://localhost](http://localhost) in your favorite browser.

### Note on admin users

As the website admin, create a user with the "email" field set to `admin`. This will make your user into an admin user, which can be used to give other users admin privileges.

## To setup for development:

The project can also be deployed locally without having to run the full `docker compose` build procedure. You can run the backend, the frontend, or both as completely separate modules during the development process.


### Setup and run the frontend

To setup and run the frontend from scratch, run the following commands:

```bash
cd frontend
npm ci
npm run dev
```

This will download all dependencies and run a Vite dev server with HMR for a fast development cycle. Requires `npm` and `node` installed on your system.

If you want to build the frontend, so that it can be deployed on the backend, `cd` into `/frontend` and run the following:
```bash
npm run build
```

### Setup and run the backend

If the backend is run in dev mode, it will accept requests from the frontend dev server, otherwise you need to `build` the frontend (see above) so that the backend can serve it itself. This is due to the dev server being on a different port and requests from there are thus treated as cross-origin, which is blocked by default.

To run the backend outside the docker environment requires a locally installed MySQL/MariaDB database (we recommend the latter), and requires `cargo` to be installed on your system.

First, create a `.env` file containing MYSQL parameters to your local database:
```properties
# .env
DATABASE_URL=mysql://[mysql_user]:[mysql_password]@127.0.0.1/webshop
```

Next, install the Diesel CLI: [(You can find more information about this here)](https://diesel.rs/guides/getting-started#installing-diesel-cli)
```bash
# Linux/MacOS
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh

# Windows (powershell)
Set-ExecutionPolicy RemoteSigned -scope CurrentUser
irm https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.ps1 | iex
```

Finally, before we can run the backend, we need to setup/create the tables in our DB. This is done with the following command:
```bash
diesel migration run
```

We can now run the backend in dev mode: (allows debug and CORS requests from the dev server)
```bash
cargo run
```

Or in release/production mode: (longer compile but better performance, doesn't allow CORS requests)
```bash
cargo run --release
```

The backend will try to serve the frontend from the `/frontend/dist` directory, so remember to `build` the frontend first if you want it to be served properly. This is of course not necessary when only testing out the API, however.
