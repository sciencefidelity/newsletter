# Newsletter

Newsletter API in Rust.

## Deploy

Deploy to Digital Ocean App Platform:

```shell
doctl apps create --spec spec.yaml
```

Migrate database on Digital Ocean

```shell
DATABASE_URL=<digital-ocean-db-connection-string> sqlx migrate run
```

Update running app on Digital Ocean

```shell
# get the app id
doctl apps list

# use the id to update the app
doctl apps update <digital-ocean-app-id> --spec spec.yaml
```

## Migrations

Run migrations with no local Docker running run from project root:

```shell
./scripts/init_db.sh
```

Run migrations with Docker already running set `SKIP_DOCKER` environment variable:

```shell
SKIP_DOCKER=true ./scripts/init_db.sh
```

