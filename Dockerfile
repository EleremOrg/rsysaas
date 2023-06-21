FROM redis:latest

COPY ./populate-redis.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/populate-redis.sh
RUN ["/bin/bash", "/usr/local/bin/populate-redis.sh"]
