package worker

import (
	"context"
	"time"

	"github.com/alitto/pond"
	"github.com/rs/zerolog"
)

type Pool struct {
	pool        *pond.WorkerPool
	logger      zerolog.Logger
	stopTimeout time.Duration
}

type PoolOption func(*Pool)

func PoolStopTimeout(t time.Duration) PoolOption {
	return func(p *Pool) {
		p.stopTimeout = t
	}
}

func NewPool(logger zerolog.Logger, options ...PoolOption) *Pool {
	pool := pond.New(100, 1000)
	p := &Pool{
		pool:        pool,
		logger:      logger,
		stopTimeout: 5 * time.Second,
	}

	for _, opt := range options {
		opt(p)
	}

	return p
}

func (p *Pool) Shutdown() {
	p.pool.StopAndWaitFor(p.stopTimeout)
}

func (p *Pool) Submit(ctx context.Context, j Job) JobHandle {
	l := p.logger.With().Str("job", j.Name()).Logger()

	ch := make(chan error)

	p.pool.Submit(func() {
		l.Debug().Msg("starting job run...")
		err := j.Run(ctx)
		l.Debug().Err(err).Msg("job run completed")
		ch <- err
	})

	l.Debug().Msg("job scheduled successfully")
	return &defaultHandle{ch}
}
