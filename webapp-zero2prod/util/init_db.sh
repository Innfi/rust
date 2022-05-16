DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

docker run \
-e POSTGRES_USER=${DB_USER} \
-e POSTGRES_PASSWORD=${DB_PASSWORD} \
-e POSTGRES_DB=${DB_NAME} \
-p "${DB_PORT}":5432 \
-d postgres \
postgres -N 10

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
>&2 echo "Postgres is still unavailable - sleeping"
sleep 1
done
>&2 echo "Postgres is up and running on port ${DB_PORT}!"
