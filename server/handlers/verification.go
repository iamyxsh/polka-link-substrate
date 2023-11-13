package handler

import (
	"github.com/gofiber/fiber/v2"
	"github.com/iamyxsh/PolkaLink/server/db"
	"github.com/iamyxsh/PolkaLink/server/models"
	"github.com/iamyxsh/PolkaLink/server/utils"
)

type EmailCreateChallengeReq struct {
	Email   string `validate:"required,email"`
	Address string `validate:"required,min=48,max=48"`
}

type TwitterCreateChallengeReq struct {
	Username string `validate:"required,min=2,max=20"`
	Address  string `validate:"required,min=48,max=48"`
}

type EmailVerifyChallengeReq struct {
	Code    string `validate:"required,min=12,max=12"`
	Address string `validate:"required,min=48,max=48"`
}

type TwitterVerifyChallengeReq struct {
	Link    string `validate:"required,min=10,max=50"`
	Address string `validate:"required,min=48,max=48"`
}

func CreateChallenge(c *fiber.Ctx) error {
	emailReq := EmailCreateChallengeReq{}
	twitterReq := TwitterCreateChallengeReq{}
	q := c.Queries()

	challenge, err := utils.CreateChallenge()
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
	}

	switch q["platform"] {
	case "email":
		if err := c.BodyParser(&emailReq); err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
		}

		if err := validate.Struct(emailReq); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
		}

		user := models.User{
			Address: emailReq.Address,
		}

		if err := user.GetUserByAddress(db.DB); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
		}

		if user.EmailChallenge == "" || user.EmailVerified {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": "challenge already created or verified"})
		}

		if err := utils.SendMail("Email Verifaction", challenge, emailReq.Email); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
		}

		user.EmailChallenge = challenge
		user.Email = emailReq.Email
		user.UpdateUser(db.DB)
		if err := user.UpdateUser(db.DB); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
		}

		if err := user.GetUserByAddress(db.DB); err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
		}

		return c.Status(fiber.StatusOK).JSON(fiber.Map{"msg": "challenge created"})
	case "twitter":
		if err := c.BodyParser(&twitterReq); err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
		}

		if err := validate.Struct(twitterReq); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
		}

		user := models.User{
			Address: twitterReq.Address,
		}

		if err := user.GetUserByAddress(db.DB); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
		}

		if user.TwitterChallenge != "" || user.Twitterverified {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": "challenge already created or verified"})
		}

		user.TwitterChallenge = challenge
		user.Twitter = twitterReq.Username
		user.UpdateUser(db.DB)
		if err := user.UpdateUser(db.DB); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
		}

		return c.Status(fiber.StatusOK).JSON(fiber.Map{"msg": "challenge created"})
	default:
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": "please attach a 'platform' query - 'email'/'twitter'"})
	}
}

func VerifyChallenge(c *fiber.Ctx) error {
	emailReq := EmailVerifyChallengeReq{}
	twitterReq := TwitterVerifyChallengeReq{}
	q := c.Queries()

	switch q["platform"] {
	case "email":
		if err := c.BodyParser(&emailReq); err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
		}

		if err := validate.Struct(emailReq); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
		}

		user := models.User{
			Address: emailReq.Address,
		}

		if err := user.GetUserByAddress(db.DB); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
		}

		if user.EmailChallenge != "" || user.EmailVerified {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": "challenge not created or already verified"})
		}

		if emailReq.Code != user.EmailChallenge {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": "not verified"})
		}

		user.EmailVerified = true
		user.UpdateUser(db.DB)
		if err := user.UpdateUser(db.DB); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
		}

		if err := user.GetUserByAddress(db.DB); err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
		}

		return c.Status(fiber.StatusOK).JSON(fiber.Map{"msg": "challenge created"})
	case "twitter":
		if err := c.BodyParser(&twitterReq); err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{"error": err.Error()})
		}

		if err := validate.Struct(twitterReq); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
		}

		user := models.User{
			Address: twitterReq.Address,
		}

		if err := user.GetUserByAddress(db.DB); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
		}

		if user.TwitterChallenge == "" || user.Twitterverified {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": "challenge not created or already verified"})
		}

		ok, err := utils.CheckTweet(user.Twitter, user.TwitterChallenge, twitterReq.Link)
		if err != nil || !ok {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": "not verified"})
		}
		user.Twitterverified = true
		user.UpdateUser(db.DB)
		if err := user.UpdateUser(db.DB); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": err.Error()})
		}

		return c.Status(fiber.StatusOK).JSON(fiber.Map{"msg": "challenge created"})
	default:
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{"error": "please attach a 'platform' query - 'email'/'twitter'"})
	}
}
