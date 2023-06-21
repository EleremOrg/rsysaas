dokup:
	docker build -t my-redis-image .
	docker run -p 6379:6379 --rm -it my-redis-image

