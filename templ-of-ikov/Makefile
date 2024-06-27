run:
	pnpm css
	@templ generate
	@go run cmd/main.go

air:
	pnpm css
	@templ generate
	@go build -o ./tmp/main ./cmd
