package handler

import (
	"github.com/a-h/templ"
	"github.com/labstack/echo/v4"
	"ikov/view/layout"
)

// TODO: Look into creating renders which render with a layout instead of passing
// the layout into each view
func render(c echo.Context, component templ.Component) error {
	return renderWithTitle(c, component, "Templ of Ikov")
}

func renderWithTitle(c echo.Context, component templ.Component, title string) error {
	if c.Request().Header.Get("HTTP_HX_REQUEST") == "" {
		return layout.BaseWithComponent(component, title).Render(c.Request().Context(), c.Response())
	}
	return component.Render(c.Request().Context(), c.Response())
}
