package model

import "time"

type Todo struct {
	Id        string // TODO: uuid
	Title     string
	Completed bool
	CreatedAt time.Time
}
