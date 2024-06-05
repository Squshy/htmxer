package handler

import (
	"github.com/labstack/echo/v4"
	"ikov/model"
	"ikov/view/todo"
)

type TodoHandler struct{}

func (h TodoHandler) HandleTodoShow(c echo.Context) error {
	t := model.Todo{
		Id: "hehe",
	}
	return render(c, todo.Show(t))
}
