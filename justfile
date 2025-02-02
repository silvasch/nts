dev-up:
    docker compose -f docker/docker-compose-dev.yml up -d --pull never --build

dev-down:
    docker compose -f docker/docker-compose-dev.yml down

dev-logs:
    docker compose -f docker/docker-compose-dev.yml logs -f
