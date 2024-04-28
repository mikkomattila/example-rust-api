# example-api-rust

1. Create docker image

    `docker build --tag example-api-rust .`

2. Run docker container from image

    `docker run -d -p 8000:3000 example-api-rust`

3. Test the application

    `curl -X POST localhost:8000/users -d '{"username":"jebaited"}' -H "Content-Type:application/json"`
