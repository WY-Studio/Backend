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

```
docker run -d --name backend.wooyeon \
  --net wooyeon \
  -p 8280:3000 \
  -e APP_ENV=stage \
  -e DATABASE_URL="postgresql://wooyeon:wooyeon@postgres.wooyeon:5432/wy-postgres" \
  ghcr.io/wy-studio/wy-backend:stage
```

then

```
docker build --build-arg APP_ENV=stage -t ghcr.io/wy-studio/wy-backend:stage .
```

then

```
docker push ghcr.io/wy-studio/wy-backend:stage
```

then

```
docker pull ghcr.io/wy-studio/wy-backend:stage
```
