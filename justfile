dev-up:
    docker compose -f docker/docker-compose-dev.yml up -d --pull never --build

dev-down:
    docker compose -f docker/docker-compose-dev.yml down

prod-up:
    docker compose -f docker/docker-compose.yml up -d

prod-down:
    docker compose -f docker/docker-compose.yml down
