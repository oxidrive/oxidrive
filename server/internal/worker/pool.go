package worker

import (
	"context"
	"time"

	"github.com/alitto/pond"
)

type Pool struct {
	pool        *pond.WorkerPool
	stopTimeout time.Duration
}

type PoolOption func(*Pool)

func PoolStopTimeout(t time.Duration) PoolOption {
	return func(p *Pool) {
		p.stopTimeout = t
	}
}

func NewPool(options ...PoolOption) *Pool {
	pool := pond.New(100, 1000)
	p := &Pool{pool: pool, stopTimeout: 5 * time.Second}

	for _, opt := range options {
		opt(p)
	}

	return p
}

func (p *Pool) Shutdown() {
	p.pool.StopAndWaitFor(p.stopTimeout)
}

func (p *Pool) Submit(ctx context.Context, j Job) JobHandle {
	ch := make(chan error)

	p.pool.Submit(func() {
		ch <- j.Run(ctx)
	})

	return &defaultHandle{ch}
}
