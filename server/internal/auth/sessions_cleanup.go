package auth

import (
	"context"
	"time"

	"github.com/oxidrive/oxidrive/server/internal/worker"
)

var _ worker.Job = (*SessionsCleanupJob)(nil)

type SessionsCleanupJob struct {
	sessions Sessions
}

func NewSessionsCleanupJob(sessions Sessions) *SessionsCleanupJob {
	return &SessionsCleanupJob{sessions: sessions}
}

func (*SessionsCleanupJob) Name() string {
	return "auth sessions cleanup"
}

func (j *SessionsCleanupJob) Run(ctx context.Context) error {
	tt, err := j.sessions.ExpiringBefore(ctx, time.Now())
	if err != nil {
		return err
	}

	if len(tt) == 0 {
		return nil
	}

	if err := j.sessions.DeleteAll(ctx, tt); err != nil {
		return err
	}

	return nil
}
