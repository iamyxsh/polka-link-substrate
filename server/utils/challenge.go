package utils

import (
	"crypto/rand"
	"encoding/base64"
)

func CreateChallenge() (string, error) {
	length := 12
	randomBytes := make([]byte, length)
	_, err := rand.Read(randomBytes)
	if err != nil {
		return "", err
	}
	randomString := base64.URLEncoding.EncodeToString(randomBytes)
	return randomString[:length], nil
}
