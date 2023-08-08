build:
	docker build -t my-redis-image .

rrun:
	docker run -p 6379:6379 --rm -it my-redis-image

up:
	make build
	make rrun	

run:
	cargo watch -q -c -w src/ -x run

clean-logs:
	rm -rf logs
	mkdir logs

clean:
	cargo fix --bin "webservice" --allow-dirty

build:
	cargo build --release --timings --target-dir ./dist