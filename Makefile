# Some small shortcuts

all:
	cargo run

docs:
	cargo doc --no-deps -p spaceapi
	cargo doc --no-deps -p spaceapi_server
	cargo doc --no-deps -p coredump_status

clean:
	cargo clean
