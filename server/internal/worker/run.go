package worker

import (
	"context"

	"github.com/rs/zerolog"
)

type Config struct {
	Logger  zerolog.Logger
	Crontab []CronJob
}

func StartScheduled(cfg Config) error {
	ctx := context.Background()

	s, err := NewScheduler(cfg.Logger)
	if err != nil {
		return err
	}

	for _, j := range cfg.Crontab {
		_, err := s.Schedule(ctx, j.Cron, j.Job)
		if err != nil {
			return err
		}
	}

	s.Start()

	return nil
}
