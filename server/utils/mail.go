package utils

import (
	"fmt"
	"net/smtp"
)

func SendMail(subject, text, recipient string) error {
	from := "polkalink@proton.me" // Replace with your ProtonMail email address
	password := "U(#bJ7X2%A.7_@R" // Replace with your ProtonMail password
	to := recipient               // Replace with the recipient's email address

	smtpHost := "mail.protonmail.ch"
	smtpPort := 587

	auth := smtp.PlainAuth("", from, password, smtpHost)

	message := []byte("To: " + to + "\r\n" +
		"Subject: Test Email\r\n" +
		"\r\n" +
		"This is a test email sent using ProtonMail SMTP in Go.")

	err := smtp.SendMail(smtpHost+":"+fmt.Sprint(smtpPort), auth, from, []string{to}, message)
	if err != nil {
		fmt.Println("Error sending email:", err)
		return nil
	}
	fmt.Println("Email sent successfully!")
	return nil
}
