package main

import (
	"ikov/db"
	"ikov/handler"
	"ikov/repository"
	"log"

	"github.com/labstack/echo/v4"
)

func main() {
	app := echo.New()

	db, err := db.NewDB("postgres://postgres:password@localhost:5432/ikov")

	if err != nil {
		log.Fatalf("Db died", err)
	}

	// Repositories
	todoRepository := repository.TodoRepository{DB: db}

	// Handlers
	todoHandler := handler.TodoHandler{
		TodoRepository: &todoRepository,
	}
	homeHandler := handler.HomeHandler{}

	// Routes
	app.GET("/", homeHandler.HandleHome)
	app.GET("/todo", todoHandler.HandleTodoShow)
	// Serve CSS/JS
	app.Static("/css", "assets/css")

	// TODO: read from env
	app.Start(":3000")
}
