package main

import (
	"log"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
	"github.com/gofiber/fiber/v2/middleware/logger"
	"fmt"
	"backend/apis/add_money"
)

type Response struct {
	Message string `json:"message"`
	Status  int    `json:"status"`
}

// FuckYouBlueprint - Blueprint for fuck_you routes
func FuckYouBlueprint(router fiber.Router) {
	// Main endpoint
	router.Get("/fuck_you", func(c *fiber.Ctx) error {
		return c.Status(fiber.StatusOK).JSON(Response{
			Message: "Fuck You !!!",
			Status:  200,
		})
	})

}

// HealthBlueprint - Blueprint for health check routes
func HealthBlueprint(router fiber.Router) {
	router.Get("/health", func(c *fiber.Ctx) error {
		return c.JSON(fiber.Map{
			"status": "ok",
			"service": "fuck-you-api",
		})
	})
}

func main() {


	fmt.Printf("\nMy Fucking App\n\n")
	app := fiber.New(fiber.Config{
		AppName: "Fiber REST API with Blueprints v1.0",
	})

	// Middleware
	app.Use(logger.New())
	app.Use(cors.New())

	// API Group (like Flask blueprint)
	api := app.Group("/api")

	// Register blueprints
	FuckYouBlueprint(api)
	HealthBlueprint(api)
	add_money.AddMoneyBlueprint(api)

	// Root endpoint
	app.Get("/", func(c *fiber.Ctx) error {
		return c.JSON(fiber.Map{
			"message": "Welcome to Fuck You API",
			"endpoints": []string{
				"GET /api/fuck_you",
				"GET /api/health",
			},
		})
	})

	// Start server
	log.Println("Server starting on :3000")
	log.Fatal(app.Listen(":3000"))
}
