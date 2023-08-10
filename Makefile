build-docker:
	docker build -t my-redis-image .

rrun:
	docker run -p 6379:6379 --rm -it my-redis-image

up:
	make build
	make rrun	

run:
	cargo watch -q -c -w src/ -x run

clean-info:
	rm -rf logs
	mkdir logs
	rm database.sqlite
	rm database.sqlite-shm
	rm database.sqlite-wal

clean:
	cargo fix --bin "webservice" --allow-dirty

build:
	cargo build --release --timings --target-dir ./dist

minify:
	./minify.sh

js-apply-changes:
	sed 's|http://localhost:8080|https://api.elerem.com|g' assets/test-embed-widget.js > assets/og-embed-widget.js

deploy:
	make build
	make minify
	rsync -chavzP --stats --progress "dist/release/webservice" "hetzner:~/recsys"
	rsync -chavzP --stats --progress "migrations/sqlite" "hetzner:~/recsys/migrations"
	rsync -chavzP --stats --progress "assets/embed-widget.js" "hetzner:~/recsys/assets"
