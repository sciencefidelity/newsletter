# Newsletter

Newsletter API in Rust.

## Migrations

To run migrations with no local Docker running run from project root:

```shell
./scripts/init_db.sh
```

To run migrations with Docker already running set `SKIP_DOCKER` environment variable:

```shell
SKIP_DOCKER=true ./scripts/init_db.sh
```

