build:
	docker build -t my-redis-image .

run:
	docker run -p 6379:6379 --rm -it my-redis-image

up:
	make build
	make run	
