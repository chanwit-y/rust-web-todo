
## Test
```sh
cargo watch -q -c -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture'
```

## DB
```sh
# Start the database
docker compose u -d

# Optional psql (other terminal)
docker exec -it -u postgres pg psql
```