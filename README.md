### 1. Database docker push

Postgres Database (Docker Container)

```
docker run -d \
  -p 8281:5432 \
  -v /wooyeon/wy_backend/data/:/var/lib/postgresql/data \
  -e POSTGRES_USER=wooyeon \
  -e POSTGRES_PASSWORD=wooyeon \
  -e POSTGRES_HOST_AUTH_METHOD=trust \
  -e POSTGRES_DB=wy-postgres \
  --net wooyeon \
  --name postgres.wooyeon \
  ghcr.io/wy-studio/wy-postgres:16.4
```

then

```
docker build \
--no-cache \
-t ghcr.io/wy-studio/wy-backend:latest .
```

then

```
docker push ghcr.io/wy-studio/wy-backend:latest
```

then

```
docker pull ghcr.io/wy-studio/wy-backend:latest
```

```
docker run -d --name backend.wooyeon \
      --net wooyeon \
      -p 8280:3000 \
      ghcr.io/wy-studio/wy-backend:latest
```
```
docker logs -f <컨테이너_ID_또는_이름>
```
