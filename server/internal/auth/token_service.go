package auth

import (
	"context"
	"errors"
	"strings"
	"time"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

var ErrInvalidToken = errors.New("invalid token")

type TokenService struct {
	tokens   Tokens
	tokenTTL time.Duration
}

func NewTokenService(tokens Tokens, tokenTTL time.Duration) TokenService {
	return TokenService{
		tokens:   tokens,
		tokenTTL: tokenTTL,
	}
}

func (s *TokenService) Generate(ctx context.Context, u *user.User) (*Token, error) {
	value, err := generate()
	if err != nil {
		return nil, err
	}

	expiresAt := time.Now().Add(s.tokenTTL)

	return &Token{
		Value:     value,
		UserID:    u.ID,
		ExpiresAt: expiresAt,
	}, nil
}

func (s *TokenService) Verify(ctx context.Context, token TokenID) error {
	if token == "" || !strings.HasPrefix(string(token), prefix) {
		return ErrInvalidToken
	}

	t, err := s.tokens.ByID(ctx, token)
	if err != nil {
		return err
	}

	if t == nil || t.IsExpired() {
		return ErrInvalidToken
	}

	return nil
}

func (s *TokenService) ByID(ctx context.Context, id TokenID) (*Token, error) {
	return s.tokens.ByID(ctx, id)
}

func (s *TokenService) Store(ctx context.Context, token Token) (*Token, error) {
	return s.tokens.Store(ctx, token)
}
