package algorithms

import (
	"fmt"
	"sync"
)

type ServersStruct struct {
	Servers []string
	mutex   sync.RWMutex
}

func (serverStruct *ServersStruct) RemoveItem(item string) {
	serverStruct.mutex.Lock()
	defer serverStruct.mutex.Unlock()
	newSlice := make([]string, 0)
	for _, v := range serverStruct.Servers {
		if v != item {
			newSlice = append(newSlice, v)
		}
	}
	serverStruct.Servers = newSlice
}

func (serverStruct *ServersStruct) AddItem(item string) {
	serverStruct.mutex.Lock()
	defer serverStruct.mutex.Unlock()
	for _, v := range serverStruct.Servers {
		if v == item {
			return
		}
	}
	serverStruct.Servers = append(serverStruct.Servers, item)
}

func (serverStruct *ServersStruct) CountServers() int {
	serverStruct.mutex.RLock()
	defer serverStruct.mutex.RUnlock()
	return len(serverStruct.Servers)
}

func (serverStruct *ServersStruct) PrintServers() {
	serverStruct.mutex.RLock()
	defer serverStruct.mutex.RUnlock()
	fmt.Println(serverStruct.Servers)
}
