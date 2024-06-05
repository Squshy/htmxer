package main

import (
	"ikov/handler"

	"github.com/labstack/echo/v4"
)

func main() {
	app := echo.New()

	todoHandler := handler.TodoHandler{}
	app.GET("/todo", todoHandler.HandleTodoShow)
	app.Static("/css", "css")

	// TODO: read from env
	app.Start(":3000")
}
