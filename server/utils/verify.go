package utils

import (
	"encoding/json"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"

	"gopkg.in/resty.v1"
)

type Status struct {
	ID   int    `json:"id"`
	Text string `json:"text"`
}

type TwitterResponse struct {
	ID         int    `json:"id"`
	ScreenName string `json:"screen_name"`
	Status     Status `json:"status"`
}

func CheckTweet(username, challenge, link string) (bool, error) {
	id := getTweetId(link)
	url := fmt.Sprintf("https://api.twitter.com/1.1/users/show.json?screen_name=%s", username)
	token := os.Getenv("TWITTER_TOKEN")
	client := resty.New()
	resp, err := client.R().
		SetHeader("Authorization", "Bearer "+token).
		Get(url)
	if err != nil {
		return false, err
	}

	var twitterResp TwitterResponse
	err = json.Unmarshal(resp.Body(), &twitterResp)
	if err != nil {
		return false, err
	}

	if twitterResp.Status.ID == id && strings.Contains(twitterResp.Status.Text, challenge) {
		return true, nil
	}

	return false, nil
}

func getTweetId(link string) int {
	re := regexp.MustCompile(`(?i)\/status\/(\d+)`)
	matches := re.FindStringSubmatch(link)

	id, _ := strconv.Atoi(matches[1])
	return id
}
