package main

import (
	"fmt"
	"log"
	"net/http"
	"text/template"
	//"encoding/json"
	"github.com/gorilla/websocket"
)

var UpgraderServer = websocket.Upgrader{}
var UpgraderClient = websocket.Upgrader{}
var base string = ""

type PP struct{
	Titulo string
	Autor string	
}

func Pagina_principal(w http.ResponseWriter, r *http.Request){
	tmpl,err := template.ParseFiles("index.html")
	if err != nil{
		fmt.Fprintf(w,"Ha ocurrido un error al cargar la página!")
		return
	}
	tmpl.Execute(w,PP{
		Titulo:"Prometeo",
		Autor:"Eduardo Gracidas Reyes",
	})
}

func WSClientSide(w http.ResponseWriter, r *http.Request){
	UpgraderClient.CheckOrigin = func(r *http.Request) bool{return true}

	ws,err := UpgraderClient.Upgrade(w,r,nil)
	if err != nil{
		log.Println(err)
	}
	ClientReader(ws)
}
func ClientReader(conn *websocket.Conn){
	for{
		conn.WriteMessage(websocket.TextMessage,[]byte("uwu"))
		_,msg,err := conn.ReadMessage()
		if err != nil{
			fmt.Println(err)
			return
		}
		if string(msg) != "actualizar"{
			base = string(msg)
		}else{
			conn.WriteMessage(websocket.TextMessage,[]byte(base))
		}
	}
}
func WSServerSide(w http.ResponseWriter, r *http.Request){
	UpgraderServer.CheckOrigin = func(r *http.Request) bool{return true}

	ws,err := UpgraderServer.Upgrade(w,r,nil)
	if err != nil{
		log.Println(err)
	}
	ServerReader(ws)
}
func ServerReader(conn *websocket.Conn){
	
	var Paquete struct{
		Titulo string `json:"titulo"`
		Cuerpo []float64 `json:"cuerpo"`
	}

	for{
		conn.ReadJSON(&Paquete)

		log.Println(Paquete)
	}
}
func EstablecerRutas(){
	http.HandleFunc("/",Pagina_principal)
	http.HandleFunc("/ws",WSClientSide)
	http.HandleFunc("/wss",WSServerSide)
}

func main(){
	EstablecerRutas()
	log.Fatal(http.ListenAndServe(":3000",nil))
	fmt.Println("Se ha iniciado correctamente el servidor!")
}