####################################################################################################
# DOCKER
####################################################################################################

export BUILDKIT_PROGRESS = plain
DOCKER_BUILD_OPT = --force-recreate --build

up:
	docker compose up -d $(DOCKER_BUILD_OPT)

down:
	docker compose down

rebuild:
	docker compose up -d auth $(DOCKER_BUILD_OPT)

####################################################################################################
# DEVELOPMENT
####################################################################################################

i-cw:
	cargo install cargo-watch
w-c:
	cargo watch -x check
w-b:
	cargo watch -x build
w-r:
	cargo watch -x 'run --bin app'


####################################################################################################
# DATABASE
####################################################################################################

database-env:
	export $$(cat .env | grep DATABASE_URL)

mi:
	sea-orm-cli init

mg:
	sea-orm-cli generate create_user_table

mge:
	sea-orm-cli generate entity -o src/module/user/entity

mup: database-env
	sea-orm-cli migrate up

mdown: database-env
	sea-orm-cli migrate down
