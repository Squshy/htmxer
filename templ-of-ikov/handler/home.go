package handler

import (
	"github.com/labstack/echo/v4"
	"ikov/view/home"
)

type HomeHandler struct{}

func (h HomeHandler) HandleHome(c echo.Context) error {
	return render(c, home.Show())
}
