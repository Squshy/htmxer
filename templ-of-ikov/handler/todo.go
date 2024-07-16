package handler

import (
	"ikov/repository"
	"ikov/view/todo"
	"log"

	"github.com/google/uuid"
	"github.com/labstack/echo/v4"
)

type TodoHandler struct {
	TodoRepository *repository.TodoRepository
}

func (h TodoHandler) handleList(c echo.Context) error {
	todos, err := h.TodoRepository.List()
	if err != nil {
		log.Print(err)
		return err
	}

	return render(c, todo.List(todos))
}

func (h TodoHandler) handleCreate(c echo.Context) error {
	title := c.FormValue("title")
	completed := c.FormValue("completed") == "on"

	newTodo, err := h.TodoRepository.Create(&repository.TodoCreateFields{
		Title:     title,
		Completed: completed,
	})

	if err != nil {
		log.Print(err)
		return err
	}

	return render(c, todo.Show(newTodo))
}

func (h TodoHandler) handleUpdate(c echo.Context) error {
	title := c.FormValue("title")
	completed := c.FormValue("completed") == "on"
    id := c.Param("id")
    uuid, err := uuid.Parse(id)

    if err != nil {
        return err
    }

	err = h.TodoRepository.Update(uuid, &repository.TodoUpdateFields{
		Title:     title,
		Completed: completed,
	})

	if err != nil {
		log.Print(err)
		return err
	}

    out, err := h.TodoRepository.Get(uuid)

	if err != nil {
		log.Print(err)
		return err
	}


	return render(c, todo.Show(out))
}

func (h TodoHandler) handleDelete(c echo.Context) error {
    id := c.Param("id")
    uuid, err := uuid.Parse(id)

    if err != nil {
        return err
    }
    
    return h.TodoRepository.Delete(uuid)
}

func (h TodoHandler) Register(app *echo.Router) {
	app.Add("GET", "/todos", h.handleList)
	app.Add("POST", "/todos/create", h.handleCreate)
    app.Add("POST", "/todos/update/:id", h.handleUpdate)
    app.Add("DELETE", "/todos/delete/:id", h.handleDelete)
}
