package algorithms

import (
	"sync"
)

type WeightedRoundRobin struct {
	Weights    []int
	NewServers []string
	mutex      sync.Mutex
	Count      int
}

func (weightedRoundRobin *WeightedRoundRobin) InitializeServers(servers *ServersStruct) {
	new_servers := make([]string, 0)
	servers.mutex.RLock()
	for index, weight := range weightedRoundRobin.Weights {
		i := 0
		for i < weight {
			new_servers = append(new_servers, servers.Servers[index])
			i++
		}
	}
	servers.mutex.RUnlock()
	weightedRoundRobin.mutex.Lock()
	defer weightedRoundRobin.mutex.Unlock()
	weightedRoundRobin.NewServers = new_servers
	weightedRoundRobin.Count = 0
}

func (weightedRoundRobin *WeightedRoundRobin) GetServer() string {
	weightedRoundRobin.mutex.Lock()
	defer weightedRoundRobin.mutex.Unlock()
	toBeReturnedServerUrl := weightedRoundRobin.NewServers[weightedRoundRobin.Count]
	weightedRoundRobin.Count = (weightedRoundRobin.Count + 1) % (len(weightedRoundRobin.NewServers))
	return toBeReturnedServerUrl
}
