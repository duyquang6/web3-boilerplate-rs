# Detect container runtime
ifeq ($(shell command -v docker 2>/dev/null),)
    ifeq ($(shell command -v podman 2>/dev/null),)
        $(error "Neither Docker nor Podman is installed. Please install one of them.")
    else
        CONTAINER_CMD := podman
        COMPOSE_CMD := podman-compose
    endif
else
    CONTAINER_CMD := docker
    COMPOSE_CMD := docker-compose
endif

.PHONY: all build run test clean integration-test

all: build

build:
	$(COMPOSE_CMD) build

run:
	$(COMPOSE_CMD) up

test:
	cargo test

integration-test:
	$(COMPOSE_CMD) up -d postgres redis
	sleep 5
	cargo test --test '*'
	$(COMPOSE_CMD) down -v

clean:
	$(COMPOSE_CMD) down -v
	cargo clean
