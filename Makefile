DATABASE_URL=postgres://develop:effimatebackend@localhost:5433/postgres

export DATABASE_URL

i-cw:
	cargo install cargo-watch
w-c:
	cargo watch -x check
w-b:
	cargo watch -x build
w-r:
	cargo watch -x 'run --bin main'

mi:
	sea-orm-cli init

mg:
	sea-orm-cli generate create_user_table

mge:
	sea-orm-cli generate entity -o src/module/user/entity

mup:
	sea-orm-cli migrate up

mdown:
	sea-orm-cli migrate down
