#!/usr/bin/env bash

export GOOSE_DBSTRING=postgres://postgres:password@localhost:5432/ikov
export GOOSE_DRIVER=postgres
export GOOSE_MIGRATION_DIR=./migrations

goose up 
