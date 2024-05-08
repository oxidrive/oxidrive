package worker

import (
	"context"
	"time"

	"github.com/go-co-op/gocron/v2"
	"github.com/rs/zerolog"
)

type Scheduler struct {
	logger zerolog.Logger
	sched  gocron.Scheduler
}

func NewScheduler(logger zerolog.Logger) (*Scheduler, error) {
	sched, err := gocron.NewScheduler()
	if err != nil {
		return nil, err
	}

	return &Scheduler{
		sched:  sched,
		logger: logger,
	}, nil
}

func (s *Scheduler) Shutdown() {
	if err := s.sched.Shutdown(); err != nil {
		s.logger.Error().Err(err).Msg("failed to shutdown job scheduler")
	}
}

func (s *Scheduler) Start() {
	s.logger.Info().Int("jobs", len(s.sched.Jobs())).Msg("starting job scheduler")
	s.sched.Start()
}

func (s *Scheduler) Schedule(ctx context.Context, cronexpr string, j Job) (JobHandle, error) {
	l := s.logger.With().Str("cron", cronexpr).Logger()
	return s.schedule(ctx, l, gocron.CronJob(cronexpr, false), j)
}

func (s *Scheduler) ScheduleEvery(ctx context.Context, t time.Duration, j Job) (JobHandle, error) {
	l := s.logger.With().Dur("everyMS", t).Logger()
	return s.schedule(ctx, l, gocron.DurationJob(t), j)
}

func (s *Scheduler) schedule(ctx context.Context, l zerolog.Logger, def gocron.JobDefinition, j Job) (JobHandle, error) {
	ch := make(chan error)
	l = l.With().Str("job", j.Name()).Logger()

	_, err := s.sched.NewJob(def, gocron.NewTask(func() {
		l.Debug().Msg("starting job run...")
		err := j.Run(ctx)
		l.Debug().Err(err).Msg("job run completed")
		ch <- err
	}))

	if err != nil {
		return nil, err
	}

	l.Debug().Msg("job scheduled successfully")
	return &defaultHandle{ch}, nil
}
