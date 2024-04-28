# example-api-rust

Example API with docker setup.

## Local

1. Start the application

    `docker compose up -d`

2. Test the application

    `curl -X POST localhost:8000/users -d '{"username":"test"}' -H "Content-Type:application/json"`
