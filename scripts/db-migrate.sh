#!/bin/bash
# export environment variables
set -a
source .env
set +a
# exit on error
set -x

# check if sqlx is installed
if ! sqlx --version; then
    echo "Error: sqlx not installed."
    echo "Use: cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
    exit
fi

# run migrations when no argument is passed
if [ $# = 0 ]; then
    echo "Running migrations..."
    sqlx migrate run
    exit
fi

# run migration generation only when an argument (migration name) is passed
if [ $# = 1 ]; then
    echo "Creating migration file with migrations/[timestamp_here]_$1"
    sqlx migrate add $1
    exit
fi
