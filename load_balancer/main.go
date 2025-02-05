package main

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net"
	"net/http"
	"os"
	"sync"

	"github.com/Alfazal007/load-balancer/algorithms"
	"github.com/Alfazal007/load-balancer/internal/database"
	"github.com/joho/godotenv"
	_ "github.com/lib/pq"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
	"github.com/redis/go-redis/v9"
)

type Message struct {
	Type string `json:"type"`
	Url  string `json:"url"`
}

var ctx = context.Background()

var promGauge = prometheus.NewGaugeVec(
	prometheus.GaugeOpts{
		Name: "active_tcp_connections",
		Help: "Shows the current number of active tcp connections",
	},
	[]string{},
)

func main() {
	// register to prometheus
	prometheus.MustRegister(promGauge)

	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	postgresUrl := os.Getenv("DATABASE_URL")
	redisUrl := os.Getenv("REDIS_URL")
	port := os.Getenv("PORT")

	if postgresUrl == "" || redisUrl == "" || port == "" {
		log.Fatal("Invalid env file data")
	}

	conn, err := sql.Open("postgres", postgresUrl)
	if err != nil {
		log.Fatal("Error opening database connection", err)
	}

	rdb := redis.NewClient(&redis.Options{
		Addr: redisUrl,
	})

	apiCfg := algorithms.ApiCfg{DB: database.New(conn), Rdb: rdb}
	fmt.Println("Connected to the database")

	serverUpdateChannel := apiCfg.Rdb.Subscribe(ctx, "server-update")
	serversFromDB, err := apiCfg.DB.GetServers(ctx)
	if err != nil {
		fmt.Println(err)
		log.Fatal("Issue connecting to the database server")
	}

	servers := make([]string, 0)

	for _, server := range serversFromDB {
		servers = append(servers, server.ServerUrl)
	}

	serverStruct := algorithms.ServersStruct{
		Servers: servers,
	}

	roundrobinStruct := algorithms.RoundRobin{
		Count: 0,
	}
	var wg sync.WaitGroup

	wg.Add(2)

	go func() {
		defer serverUpdateChannel.Close()
		defer wg.Done()
		for {
			redisMsg, err := serverUpdateChannel.ReceiveMessage(ctx)
			if err != nil {
				log.Fatal("Error in pubsub receive message ", err)
			} else if redisMsg.Channel == "server-update" {
				var msg Message
				err := json.Unmarshal([]byte(redisMsg.Payload), &msg)
				if err != nil || msg.Type == "" || msg.Url == "" {
					log.Fatal("Invalid message type", err)
				}
				switch msg.Type {
				case "add":
					serverStruct.AddItem(msg.Url)
					roundrobinStruct.ResetCount()
				case "remove":
					serverStruct.RemoveItem(msg.Url)
					roundrobinStruct.ResetCount()
				default:
					fmt.Println("Invalid messsage type")
				}
			}
		}
	}()

	go func() {
		defer wg.Done()
		conn, err := net.Listen("tcp", fmt.Sprint("127.0.0.1:", port))
		if err != nil {
			log.Fatal("Issue starting the load balancer")
		}
		defer conn.Close()
		for {
			client, err := conn.Accept()
			if err != nil {
				log.Fatal("Issue starting the load balancer")
				continue
			}
			// got a client so increase
			promGauge.WithLabelValues().Inc()

			go func(client net.Conn) {
				serverCount := serverStruct.CountServers()
				serverUrl := roundrobinStruct.GetServerUrl(&serverStruct, serverCount)
				fmt.Println(serverUrl)
				server, err := net.Dial("tcp", serverUrl)
				if err != nil {
					client.Close()
					// decrement as server could not be connected
					promGauge.WithLabelValues().Dec()
					return
				}
				var wgPerConn sync.WaitGroup
				wgPerConn.Add(2)

				go func() {
					defer client.Close()
					defer server.Close()
					defer wgPerConn.Done()
					io.Copy(server, client)
				}()

				go func() {
					defer client.Close()
					defer server.Close()
					defer wgPerConn.Done()
					io.Copy(client, server)
				}()

				wgPerConn.Wait()
				promGauge.WithLabelValues().Dec()
			}(client)
		}
	}()

	http.Handle("/metrics", promhttp.Handler())
	go func() {
		fmt.Println("Starting HTTP server for Prometheus metrics on port 8080...")
		if err := http.ListenAndServe(":8080", nil); err != nil {
			log.Fatal("Error starting HTTP server for Prometheus metrics:", err)
		}
	}()
	wg.Wait()
}

/*
The usage of weighted round robin would look something like this

	func main() {
		servers_list := make([]string, 0)
		servers_list = append(servers_list, "1")
		servers_list = append(servers_list, "2")
		servers_list = append(servers_list, "3")
		servers_list = append(servers_list, "4")
		servers := algorithms.ServersStruct{
			Servers: servers_list,
		}

		weightedRR := algorithms.WeightedRoundRobin{
			Weights: []int{1, 2, 3, 4},
		}
		weightedRR.InitializeServers(&servers)
		i := 0
		for i < 15 {
			current_server := weightedRR.GetServer()
			fmt.Println(current_server)
			i++
		}
	}
*/

/*
func main() {
	servers_list := make([]string, 0)
	servers_list = append(servers_list, "1")
	servers_list = append(servers_list, "2")
	number_of_conn := make([]int, 0)
	i := 0
	for i < len(servers_list) {
		number_of_conn = append(number_of_conn, 0)
		i++
	}

	leastConnectionAlgoStruct := algorithms.LeastConnectionAlgo{
		ServerUrl:           servers_list,
		NumberOfConnections: number_of_conn,
	}

	connectionUrls := make([]string, 0)
	i = 0
	for i < 20 {
		// call this when establishing a connection with the server
		connectionUrls = append(connectionUrls, leastConnectionAlgoStruct.Connect())
		i++
	}
	i = 0
	for i < 10 {
		// call this in defer after establishing connection with server
		leastConnectionAlgoStruct.Disconnect("1")
		i++
	}
	for i < 20 {
		connectionUrls = append(connectionUrls, leastConnectionAlgoStruct.Connect())
		i++
	}
	fmt.Println(connectionUrls)
}
*/
