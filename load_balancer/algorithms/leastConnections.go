package algorithms

import "sync"

type LeastConnectionAlgo struct {
	ServerUrl           []string
	NumberOfConnections []int
	mutex               sync.Mutex
}

func (leastConnectionStruct *LeastConnectionAlgo) Connect() string {
	leastConnectionStruct.mutex.Lock()
	defer leastConnectionStruct.mutex.Unlock()
	indexToReturn := 0
	count := leastConnectionStruct.NumberOfConnections[0]
	i := 0
	for i < len(leastConnectionStruct.ServerUrl) {
		if count > leastConnectionStruct.NumberOfConnections[i] {
			count = leastConnectionStruct.NumberOfConnections[i]
			indexToReturn = i
		}
		i++
	}
	leastConnectionStruct.NumberOfConnections[indexToReturn]++
	return leastConnectionStruct.ServerUrl[indexToReturn]
}

func (leastConnectionStruct *LeastConnectionAlgo) Disconnect(serverToDisconnect string) {
	leastConnectionStruct.mutex.Lock()
	defer leastConnectionStruct.mutex.Unlock()
	i := 0
	for i < len(leastConnectionStruct.ServerUrl) {
		if leastConnectionStruct.ServerUrl[i] == serverToDisconnect {
			leastConnectionStruct.NumberOfConnections[i]--
			return
		}
		i++
	}
}
