package db

import (
	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
)

func NewDB(dataSourceName string) (*sqlx.DB, error) {
	db, err := sqlx.Connect("postgres", "user=postgres password=password dbname=ikov sslmode=disable")
	if err != nil {
		return nil, err
	}
	return db, nil
}
