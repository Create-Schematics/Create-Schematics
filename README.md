<p align="center">
<img style="width: 15em;" src="display/schematic_cannon.png" alt="fern"/>
<h1 align="center"> Create Schematics </h1>
</p>

A site to find and share schematics for the [Create](https://github.com/Creators-of-Create/Create) mod.

## Developing

If you want to contribute to Create Schematics, first of all thank you for you're help, but you're going to need to get some things set up first.

First obtain the source 

```bash
git clone https://github.com/Rabbitminers/Create-Schematics

cd Create-Schematics
```

### Frontend

Our frontend is built with svelte kit, you'll need to install [pnpm](https://pnpm.io/installation) and then to start developing run

```bash
cd frontend

pnpm install

pnpm run dev
```

This will start the frontend on `localhost:5173` and will automatically reload when you make changes.

### Backend

Our backend is built with rust and a postgres database. You'll want to install both [rust](https://www.rust-lang.org/tools/install) and docker, or docker compose.

Once you have installed these run

```bash
docker-compose up
```

This will start a development postges database on `localhost:5432`, by default the database name, username and password will all be `postgres`. And the default url is specified in `.env.sample` which youll need to rename to `.env` 

From there you can install the sqlx cli in order to set up the database

```bash
cargo install sqlx-cli

cd backend

cargo sqlx database setup
```

Now you've got everything set up you're ready to start developing create schematics, to run the server itself run

```
cargo run server
````

This will deploy the api itself on `localhost:3000`

## Talk To Us

Have any questions, want to request a feature or founda bug? You can either submit it here on GitHub, or on our Discord Server


[![Discord][1]][2]

[1]: https://discordapp.com/api/guilds/1069326955742244884/widget.png?style=banner2
[2]: https://discord.gg/GJsQadv9Mc


## Tech Stack

- Front End: Sveltekit, Svelte, Tailwind
- Back End: Rust, Axum, Sqlx
- Database: Postgres

## Licenses

Create Schematics is open source, and is licensed under the MIT License, see the [License](./LICENSE) for more information.