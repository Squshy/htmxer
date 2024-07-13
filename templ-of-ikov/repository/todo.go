package repository

import (
	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"ikov/model"
)

type TodoRepository struct {
	DB *sqlx.DB
}

func Get(tr *TodoRepository, id uuid.UUID) (*model.Todo, error) {
	todo := &model.Todo{}
	err := tr.DB.Get(&todo, "SELECT * FROM todos WHERE id = $1", id)

	if err != nil {
		return nil, err
	}

	return todo, nil
}
