#!/usr/bin/env bash
set -x
set -eo pipefail

if ! command -v sqlx &> /dev/null; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it."
  exit 1
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

podman-compose up -d
until podman-compose ps | grep postgres | grep -q "healthy"; do
  echo "Waiting for postgres to be healthy..."
  sleep 3
done

>&2 echo "Postgres is healthy and running on port ${DB_PORT}! - migrating now!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@127.0.0.1:${DB_PORT}/newsletter
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"

