package auth

import (
	"context"
	"crypto/rand"
	"fmt"
	"math/big"
	"time"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type TokenID string

func (id TokenID) String() string {
	return string(id)
}

type Token struct {
	Value TokenID

	UserID    user.ID
	ExpiresAt time.Time
}

func (t *Token) IsExpired() bool {
	return t.ExpiresAt.Before(time.Now())
}

func NewToken(u *user.User, expiresAt time.Time) (*Token, error) {
	value, err := generate()
	if err != nil {
		return nil, err
	}

	return &Token{
		Value:     value,
		UserID:    u.ID,
		ExpiresAt: expiresAt,
	}, nil
}

type Tokens interface {
	ExpiringBefore(context.Context, time.Time) ([]Token, error)
	ByID(context.Context, TokenID) (*Token, error)
	Store(context.Context, Token) (*Token, error)
	DeleteAll(context.Context, []Token) error
}

const alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
const size = 64
const prefix = "oxitkn"

func generate() (TokenID, error) {
	t := make([]byte, size)
	for i := 0; i < size; i++ {
		n, err := rand.Int(rand.Reader, big.NewInt(int64(len(alphabet))))
		if err != nil {
			return TokenID(""), err
		}

		t[i] = alphabet[n.Int64()]
	}

	return TokenID(fmt.Sprintf("%s.%s", prefix, string(t))), nil
}
