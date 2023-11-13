package db

import (
	"log"

	"github.com/iamyxsh/PolkaLink/server/models"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

var DB *gorm.DB

func init() {
	dsn := "host=localhost user=root password=admin dbname=postgres port=5432"
	db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})
	if err != nil {
		log.Fatal("failed to connect database")
	}
	db.AutoMigrate(&models.User{})

	DB = db
}
