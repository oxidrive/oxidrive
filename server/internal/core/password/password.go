package password

import (
	"errors"
	"fmt"

	"github.com/matthewhartstonge/argon2"
)

var argon = argon2.DefaultConfig()

var (
	ErrPasswordEmpty = errors.New("password cannot be empty")
	ErrHashingFailed = "failed to hash password: %w"
)

type Hash ([]byte)

func ValidateAndHash(password string) (Hash, error) {
	if password == "" {
		return nil, ErrPasswordEmpty
	}

	hash, err := argon.HashEncoded([]byte(password))
	if err != nil {
		return nil, fmt.Errorf(ErrHashingFailed, err)
	}

	return Hash(hash), nil
}

func Must(hash []byte) Hash {
	return Hash(hash)
}

func (h Hash) Verify(password string) (bool, error) {
	return argon2.VerifyEncoded([]byte(password), h)
}

func (h Hash) Expose() string {
	return string(h)
}
