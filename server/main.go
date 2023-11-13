package main

import (
	"log"

	"github.com/gofiber/fiber/v2"
	handler "github.com/iamyxsh/PolkaLink/server/handlers"
	"github.com/joho/godotenv"
)

func main() {
	err := godotenv.Load(".env")
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	app := fiber.New()

	app.Get("/ping", func(c *fiber.Ctx) error {
		return c.SendString("pong")
	})

	userRoutes := app.Group("/user")
	userRoutes.Post("/", handler.RegisterUser)

	verifyRoutes := app.Group("/verification")
	verifyRoutes.Post("/create-challenge", handler.CreateChallenge)
	verifyRoutes.Post("/verify-challenge", handler.VerifyChallenge)

	log.Fatal(app.Listen(":3000"))
}
