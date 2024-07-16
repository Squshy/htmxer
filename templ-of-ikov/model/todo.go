package model

import (
	"github.com/google/uuid"
	"time"
)

type Todo struct {
	Id        uuid.UUID `db:"id"`
	Title     string    `db:"title"`
	Completed bool      `db:"completed"`
	CreatedAt time.Time `db:"created_at"`
}
