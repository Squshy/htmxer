package handler

import (
	"github.com/a-h/templ"
	"github.com/labstack/echo/v4"
)

// TODO: Look into creating renders which render with a layout instead of passing
// the layout into each view
func render(c echo.Context, componet templ.Component) error {
	return componet.Render(c.Request().Context(), c.Response())
}
