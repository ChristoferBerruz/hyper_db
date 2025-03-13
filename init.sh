#!/bin/bash
set -e

# Start PostgreSQL in the background
docker-entrypoint.sh postgres &

# Wait for PostgreSQL to be ready
until pg_isready; do
  sleep 1
done

# Connect and create the extension
psql -U myuser -d mydb -c "CREATE EXTENSION IF NOT EXISTS optim;"

# Bring PostgreSQL to the foreground
wait