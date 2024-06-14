package main

import (
	"github.com/labstack/echo/v4"
	"ikov/handler"
)

func main() {
	app := echo.New()

	todoHandler := handler.TodoHandler{}
	homeHandler := handler.HomeHandler{}
	app.GET("/", homeHandler.HandleHome)
	app.GET("/todo", todoHandler.HandleTodoShow)
	// Serve CSS/JS
	app.Static("/css", "assets/css")

	// TODO: read from env
	app.Start(":3000")
}
