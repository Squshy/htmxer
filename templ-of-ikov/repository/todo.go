package repository

import (
	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"ikov/model"
)

type TodoRepository struct {
	DB *sqlx.DB
}

type TodoUpdateFields struct {
	Completed bool
	Title     string
}

func Get(tr *TodoRepository, id uuid.UUID) (model.Todo, error) {
	todo := model.Todo{}
	err := tr.DB.Get(&todo, "SELECT * FROM todos WHERE id = $1", id)

	if err != nil {
		return model.Todo{}, err
	}

	return todo, nil
}

func List(tr *TodoRepository) ([]model.Todo, error) {
	var todos []model.Todo
	// No max query, get them all :^)
	err := tr.DB.Get(&todos, "SELECT * FROM todos")

	if err != nil {
		return nil, err
	}

	return todos, nil
}

func Update(tr *TodoRepository, id uuid.UUID, tuf *TodoUpdateFields) (err error) {
	// Currently means the caller needs to pass title and completed. If they don't
	// they will get killed : (
	_, err = tr.DB.Exec("UPDATE todos SET title = $1, completed = $2 WHERE id = $3", tuf.Title, tuf.Completed, id)
	return
}

func Delete(tr *TodoRepository, id uuid.UUID) (err error) {
	_, err = tr.DB.Exec("DELETE FROM todos WHERE id = $1", id)
	return
}
