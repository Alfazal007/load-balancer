package algorithms

import (
	"github.com/Alfazal007/load-balancer/internal/database"
	"github.com/redis/go-redis/v9"
)

type ApiCfg struct {
	DB  *database.Queries
	Rdb *redis.Client
}
