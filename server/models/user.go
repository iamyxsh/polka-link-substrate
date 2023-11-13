package models

import (
	"fmt"

	"gorm.io/gorm"
)

type User struct {
	gorm.Model
	Address          string `gorm:"index"`
	Email            string
	Twitter          string
	EmailVerified    bool
	Twitterverified  bool
	EmailChallenge   string
	TwitterChallenge string
}

func (u *User) CreateUser(db *gorm.DB) error {
	result := db.Create(u)
	if result.Error != nil {
		return result.Error
	}
	return nil
}

func (u *User) GetUserByAddress(db *gorm.DB) error {
	result := db.First(u)
	if result.Error != nil {
		return result.Error
	}
	return nil
}

func (u *User) UpdateUser(db *gorm.DB) error {
	result := db.Save(u)
	fmt.Println(u)
	if result.Error != nil {
		return result.Error
	}
	return nil
}

func (u *User) DeleteUser(db *gorm.DB) error {
	result := db.Delete(u)
	if result.Error != nil {
		return result.Error
	}
	return nil
}
