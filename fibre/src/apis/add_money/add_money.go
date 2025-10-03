package add_money

import (
	"github.com/gofiber/fiber/v2"
)

type MoneyResponse struct {
	Money  string `json:"money"`
	Status int    `json:"status"`
}

// AddMoneyBlueprint - Blueprint for add money routes
func AddMoneyBlueprint(router fiber.Router) {
	router.Get("/add", func(c *fiber.Ctx) error {
		// Get money from headers
		money := c.Get("money")
		
		// If no money header provided, use default
		if money == "" {
			money = "5645455"
		}

		return c.Status(fiber.StatusOK).JSON(MoneyResponse{
			Money:  money,
			Status: 200,
		})
	})

}
