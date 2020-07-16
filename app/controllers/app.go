package controllers

import (
  "github.com/revel/revel"
)

app, err := firebase.NewApp(context.Background(), nil)
if err != nil {
  log.Fatalf("error initializing app: %v\n", err)
}

type App struct {
  *revel.Controller
}

func (c App) Index() revel.Result {
  return c.Render()
}

func (c App) User() revel.Result {
  
}
