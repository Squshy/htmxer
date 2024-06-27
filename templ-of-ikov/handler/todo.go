package handler

import (
	"ikov/model"
	"ikov/view/todo"
	"time"

	"github.com/labstack/echo/v4"
)

type TodoHandler struct{}

func (h TodoHandler) HandleTodoShow(c echo.Context) error {
	t := model.Todo{
		Id:        "hehe",
		Title:     "Fake",
		Completed: false,
		CreatedAt: time.Now(),
	}
	return render(c, todo.Show(t))
}
