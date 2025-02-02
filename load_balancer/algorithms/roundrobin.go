package algorithms

import "sync"

type RoundRobin struct {
	Count int
	mutex sync.Mutex
}

func (roundRobinStruct *RoundRobin) GetServerUrl(serverStruct *ServersStruct, countServers int) string {
	roundRobinStruct.mutex.Lock()
	count := roundRobinStruct.Count
	roundRobinStruct.Count = (roundRobinStruct.Count + 1) % countServers
	roundRobinStruct.mutex.Unlock()
	serverStruct.mutex.RLock()
	defer serverStruct.mutex.RUnlock()
	return serverStruct.Servers[count]
}

func (roundRobinStruct *RoundRobin) ResetCount() {
	roundRobinStruct.mutex.Lock()
	roundRobinStruct.Count = 0
	roundRobinStruct.mutex.Unlock()
}
