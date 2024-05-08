package auth

import (
	"context"
	"crypto/rand"
	"errors"
	"fmt"
	"math/big"
	"strings"
	"time"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

const oneMonth = 730 * time.Hour

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

func (t *Token) String() string {
	return string(t.Value)
}

func TokenFor(u *user.User) (*Token, error) {
	value, err := generate()
	if err != nil {
		return nil, err
	}

	expiresAt := time.Now().Add(oneMonth)

	return &Token{
		Value:     value,
		UserID:    u.ID,
		ExpiresAt: expiresAt,
	}, nil
}

var ErrInvalidToken = errors.New("invalid token")

type TokenVerifier struct {
	tokens Tokens
}

func NewTokenVerifier(tokens Tokens) TokenVerifier {
	return TokenVerifier{tokens: tokens}
}

func (v *TokenVerifier) VerifyToken(ctx context.Context, token TokenID) error {
	if token == "" || !strings.HasPrefix(string(token), prefix) {
		return ErrInvalidToken
	}

	t, err := v.tokens.ByID(ctx, token)
	if err != nil {
		return err
	}

	if t.IsExpired() {
		return ErrInvalidToken
	}

	return nil
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
