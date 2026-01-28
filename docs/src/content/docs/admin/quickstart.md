---
title: Quickstart
---
This guide quickly runs through how to get set up with Drop in about five minutes, depending on your experience.

## Setting up the instance

The easiest way to get Drop running is using our pre-built Docker container.

```yaml compose.yaml
services:
  postgres:
    image: postgres:14-alpine
    healthcheck:
      test: pg_isready -d drop -U drop
      interval: 30s
      timeout: 60s
      retries: 5
      start_period: 10s
    volumes:
      - ./db:/var/lib/postgresql/data
    environment:
      - POSTGRES_PASSWORD=drop
      - POSTGRES_USER=drop
      - POSTGRES_DB=drop
  drop:
    image: ghcr.io/drop-oss/drop:latest
    depends_on:
      postgres:
        condition: service_healthy
    ports:
      - 3000:3000
    volumes:
      - ./library:/library
      - ./data:/data
    environment:
      - DATABASE_URL=postgres://drop:drop@postgres:5432/drop
      - EXTERNAL_URL=http://localhost:3000 # default, customise if accessing from another computer or behind a reverse proxy
```

**The main things in this `compose.yaml` is the volumes attached to the `drop` service:**

1. `./library` is where you will put your games to be imported into Drop. See '[Structuring your library](../library.md#structuring-your-library)' once you're set up.
2. `./data` is where Drop will store anything that's using the default file-system backed storage system. Typically, these are objects.

**In addition, configure your chosen metadata provider:**

- [GiantBomb](../metadata/giantbomb.md)
- [IGDB](../metadata/igdb.md)
- [PCGamingWiki](../metadata/pcgamingwiki.md)
- Manual - you can import games without metadata and enter it yourself. 

:::tip
If you want to, you can generate a more secure PostgreSQL username & password.
:::
