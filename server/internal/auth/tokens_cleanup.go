package auth

import (
	"context"
	"time"

	"github.com/oxidrive/oxidrive/server/internal/worker"
)

var _ worker.Job = (*TokenCleanupJob)(nil)

type TokenCleanupJob struct {
	tokens Tokens
}

func NewTokenCleanupJob(tokens Tokens) *TokenCleanupJob {
	return &TokenCleanupJob{tokens: tokens}
}

func (*TokenCleanupJob) Name() string {
	return "auth tokens cleanup"
}

func (j *TokenCleanupJob) Run(ctx context.Context) error {
	tt, err := j.tokens.ExpiringBefore(ctx, time.Now())
	if err != nil {
		return err
	}

	if len(tt) == 0 {
		return nil
	}

	if err := j.tokens.DeleteAll(ctx, tt); err != nil {
		return err
	}

	return nil
}
