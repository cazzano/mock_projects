package dummy

import (
	"database/sql"
	"fmt"
	"log"
	"github.com/gofiber/fiber/v2"
	 _"github.com/mattn/go-sqlite3"
)

type Gangster struct {
	GangsterID   string `json:"gangster_id"`
	GangsterName string `json:"gangster_name"`
}

type Response struct {
	Success bool        `json:"success"`
	Message string      `json:"message"`
	Data    interface{} `json:"data,omitempty"`
}

var db *sql.DB

// InitDB initializes the database connection
func InitDB() error {
	var err error
	db, err = sql.Open("sqlite3", "./gangsters.db")
	if err != nil {
		return err
	}

	// Create table if not exists
	createTableSQL := `CREATE TABLE IF NOT EXISTS gangsters (
		gangster_id TEXT PRIMARY KEY,
		gangster_name TEXT NOT NULL UNIQUE
	);`

	_, err = db.Exec(createTableSQL)
	if err != nil {
		return err
	}

	log.Println("✅ Database initialized successfully")
	return nil
}

// generateGangsterID generates a unique gangster ID like G01, G02, etc.
func generateGangsterID() (string, error) {
	var count int
	err := db.QueryRow("SELECT COUNT(*) FROM gangsters").Scan(&count)
	if err != nil {
		return "", err
	}
	return fmt.Sprintf("G%02d", count+1), nil
}

// GangsterBlueprint - Blueprint for gangster routes
func GangsterBlueprint(router fiber.Router) {

	// POST - Add new gangster
	router.Post("/add", func(c *fiber.Ctx) error {
		gangsterName := c.Get("gangster")

		if gangsterName == "" {
			return c.Status(fiber.StatusBadRequest).JSON(Response{
				Success: false,
				Message: "gangster header is required",
			})
		}

		// Generate gangster ID
		gangsterID, err := generateGangsterID()
		if err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(Response{
				Success: false,
				Message: "Failed to generate gangster ID",
			})
		}

		// Insert into database
		_, err = db.Exec("INSERT INTO gangsters (gangster_id, gangster_name) VALUES (?, ?)", gangsterID, gangsterName)
		if err != nil {
			return c.Status(fiber.StatusConflict).JSON(Response{
				Success: false,
				Message: "Gangster already exists or database error",
			})
		}

		return c.Status(fiber.StatusCreated).JSON(Response{
			Success: true,
			Message: "Gangster added successfully",
			Data: Gangster{
				GangsterID:   gangsterID,
				GangsterName: gangsterName,
			},
		})
	})

	// GET - Get all gangsters
	router.Get("/gangsters", func(c *fiber.Ctx) error {
		rows, err := db.Query("SELECT gangster_id, gangster_name FROM gangsters")
		if err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(Response{
				Success: false,
				Message: "Failed to fetch gangsters",
			})
		}
		defer rows.Close()

		var gangsters []Gangster
		for rows.Next() {
			var g Gangster
			if err := rows.Scan(&g.GangsterID, &g.GangsterName); err != nil {
				continue
			}
			gangsters = append(gangsters, g)
		}

		return c.JSON(Response{
			Success: true,
			Message: "Gangsters fetched successfully",
			Data:    gangsters,
		})
	})

	// GET - Get single gangster by ID
	router.Get("/gangsters/:id", func(c *fiber.Ctx) error {
		id := c.Params("id")

		var g Gangster
		err := db.QueryRow("SELECT gangster_id, gangster_name FROM gangsters WHERE gangster_id = ?", id).Scan(&g.GangsterID, &g.GangsterName)
		if err == sql.ErrNoRows {
			return c.Status(fiber.StatusNotFound).JSON(Response{
				Success: false,
				Message: "Gangster not found",
			})
		} else if err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(Response{
				Success: false,
				Message: "Database error",
			})
		}

		return c.JSON(Response{
			Success: true,
			Message: "Gangster fetched successfully",
			Data:    g,
		})
	})

	// PUT - Update gangster
	router.Put("/gangsters/:id", func(c *fiber.Ctx) error {
		id := c.Params("id")
		newName := c.Get("gangster")

		if newName == "" {
			return c.Status(fiber.StatusBadRequest).JSON(Response{
				Success: false,
				Message: "gangster header is required",
			})
		}

		result, err := db.Exec("UPDATE gangsters SET gangster_name = ? WHERE gangster_id = ?", newName, id)
		if err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(Response{
				Success: false,
				Message: "Failed to update gangster",
			})
		}

		rowsAffected, _ := result.RowsAffected()
		if rowsAffected == 0 {
			return c.Status(fiber.StatusNotFound).JSON(Response{
				Success: false,
				Message: "Gangster not found",
			})
		}

		return c.JSON(Response{
			Success: true,
			Message: "Gangster updated successfully",
			Data: Gangster{
				GangsterID:   id,
				GangsterName: newName,
			},
		})
	})

	// DELETE - Delete gangster
	router.Delete("/gangsters/:id", func(c *fiber.Ctx) error {
		id := c.Params("id")

		result, err := db.Exec("DELETE FROM gangsters WHERE gangster_id = ?", id)
		if err != nil {
			return c.Status(fiber.StatusInternalServerError).JSON(Response{
				Success: false,
				Message: "Failed to delete gangster",
			})
		}

		rowsAffected, _ := result.RowsAffected()
		if rowsAffected == 0 {
			return c.Status(fiber.StatusNotFound).JSON(Response{
				Success: false,
				Message: "Gangster not found",
			})
		}

		return c.JSON(Response{
			Success: true,
			Message: "Gangster deleted successfully",
		})
	})
}
