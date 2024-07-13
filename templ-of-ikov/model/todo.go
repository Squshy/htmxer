package model

import (
	"github.com/google/uuid"
	"time"
)

type Todo struct {
	Id        uuid.UUID
	Title     string
	Completed bool
	CreatedAt time.Time
}
