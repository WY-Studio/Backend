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
