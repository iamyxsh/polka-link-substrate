package handler

import (
	"github.com/go-playground/validator/v10"
	"github.com/gofiber/fiber/v2"
	"github.com/iamyxsh/PolkaLink/server/db"
	"github.com/iamyxsh/PolkaLink/server/models"
)

var validate = validator.New()

type CreateUserReq struct {
	Address string `validate:"required,min=48,max=48"`
}

func RegisterUser(c *fiber.Ctx) error {
	req := CreateUserReq{}
	if err := c.BodyParser(&req); err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
	}

	if err := validate.Struct(req); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
	}

	user := models.User{
		Address: req.Address,
	}

	err := user.GetUserByAddress(db.DB)
	if err != nil {
		if err.Error() == "record not found" {
			err = user.CreateUser(db.DB)
			if err != nil {
				return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
			}

			return c.JSON(fiber.Map{"msg": "user created"})
		} else {
			return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
		}
	} else {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": "user already exists"})
	}
}
