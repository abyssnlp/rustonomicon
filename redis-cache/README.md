# Redis Caching

- redis crate with tokio async support

```bash
docker run --name redis-rs --rm -p 6379:6379 -it redis --loglevel verbose
```

### Redis Notes

Docs: https://redis.io/docs/data-types

- Basic (get, set, exists, ttl key, setex key seconds value, )
  - keys \*
- Lists
  - lpush key value
  - lrange key start-stopindex
  - lpop key, rpop key
- Sets
  - sadd key value
  - smembers key
  - srem key
- Hashes
  - hset key field value
  - hget key field -> value
  - hgetall key -> return all (key, value)
  - hexists (membership)
- Most used (setex, get)
- Pattern for ex. (itemid -> vector cache, collectionname-itemid)
