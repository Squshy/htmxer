package repository

import (
	"ikov/model"
	"time"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
)

type TodoRepository struct {
	DB *sqlx.DB
}

type TodoUpdateFields struct {
	Completed bool
	Title     string
}

type TodoCreateFields struct {
	Completed bool
	Title     string
}

func (tr *TodoRepository) Create(tcf *TodoCreateFields) (model.Todo, error) {
	id, err := uuid.NewRandom()

	if err != nil {
		return model.Todo{}, err
	}

	query := `INSERT INTO todos (id, title, completed, created_at)
    VALUES ($1, $2, $3, $4)
    RETURNING *`
	var todo model.Todo
	err = tr.DB.Get(&todo, query, id, tcf.Title, tcf.Completed, time.Now())

	// Probably need better error handling for these bad errors
	if err != nil {
		return todo, err
	}

	return todo, nil
}

func (tr *TodoRepository) Get(id uuid.UUID) (model.Todo, error) {
	todo := model.Todo{}
	err := tr.DB.Get(&todo, "SELECT * FROM todos WHERE id = $1", id)

	if err != nil {
		return todo, err
	}

	return todo, nil
}

func (tr *TodoRepository) List() ([]model.Todo, error) {
	var todos []model.Todo
	// No max query, get them all :^)
	err := tr.DB.Select(&todos, "SELECT * FROM todos ORDER BY created_at DESC")

	if err != nil {
		return nil, err
	}

	return todos, nil
}

func (tr *TodoRepository) Update(id uuid.UUID, tuf *TodoUpdateFields) (err error) {
	// Currently means the caller needs to pass title and completed. If they don't
	// they will get killed : (
	_, err = tr.DB.Exec("UPDATE todos SET title = $1, completed = $2 WHERE id = $3", tuf.Title, tuf.Completed, id)
	return
}

func (tr *TodoRepository) Delete(id uuid.UUID) (err error) {
	_, err = tr.DB.Exec("DELETE FROM todos WHERE id = $1", id)
	return
}
