# pokedex

## Requirements

- [Docker and docker-compose](https://docs.docker.com/get-docker/)
- [Rust](https://www.rust-lang.org/tools/install)

## Testing

Run tests with cargo using `cargo test`. Tests will not call out to external APIs it uses mocks.

## Running in docker

To run in docker with docker compose use `docker-compose up`. The app will start on port 5000.


## Running with cargo

To run with cargo use `cargo run`. This will start the app on port 5000.

## Querying the API

Once running you can then run `curl -X GET http://localhost:5000/pokemon/ditto` to query for pokemon details or `curl -X GET http://localhost:5000/pokemon/translated/ditto` to get translated descriptions.

## Improvements

- Better handling of errors. Currently on any error a 500 status is returned with a message containing some error details. Improvements will be needed both on the information that is returned so that the error can be diagnosed but also in terms of passing forward not found errors and too many requests errors.
- Caching responses from pokeapi and fun translations. Some form of caching should be implemented to prevent the same request being made frequently to the APIs.
- Rate limiting requests. Both pokeapi and fun translations implement rate limiting. Care should be taken to ensure that the limits are not exceeded.
