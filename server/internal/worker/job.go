package worker

import "context"

type Job interface {
	Run(context.Context) error
}

type JobHandle interface {
	Wait() <-chan error
}

type defaultHandle struct {
	ch chan error
}

func (h *defaultHandle) Wait() <-chan error {
	return h.ch
}
