package worker

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

type testJob struct {
	sleep time.Duration
}

var _ Job = (*testJob)(nil)

func (j *testJob) Run(_ context.Context) error {
	time.Sleep(j.sleep)
	return nil
}

func TestWorkerPool(t *testing.T) {
	t.Run("runs a background job", func(t *testing.T) {
		ctx, done := context.WithTimeout(context.Background(), 200*time.Millisecond)
		defer done()

		p := NewPool(PoolStopTimeout(1 * time.Millisecond))
		defer p.Shutdown()

		j := p.Submit(ctx, &testJob{})

		select {
		case err := <-j.Wait():
			assert.NoError(t, err)
		case <-ctx.Done():
			t.Fail()
		}
	})

	t.Run("runs a background job for too long", func(t *testing.T) {
		ctx, done := context.WithTimeout(context.Background(), 200*time.Millisecond)
		defer done()

		p := NewPool(PoolStopTimeout(1 * time.Millisecond))
		defer p.Shutdown()

		j := p.Submit(ctx, &testJob{sleep: 1 * time.Second})

		select {
		case <-j.Wait():
			t.FailNow()
		case <-ctx.Done():
			// Pass
		}
	})
}
