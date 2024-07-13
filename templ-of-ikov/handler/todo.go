package handler

import (
	"ikov/model"
	"ikov/repository"
	"ikov/view/todo"
	"time"

	"github.com/google/uuid"
	"github.com/labstack/echo/v4"
)

type TodoHandler struct {
	TodoRepository *repository.TodoRepository
}

func (h TodoHandler) HandleTodoShow(c echo.Context) error {
	t := model.Todo{
		Id:        uuid.Max,
		Title:     "Fake",
		Completed: false,
		CreatedAt: time.Now(),
	}
	return render(c, todo.Show(t))
}
