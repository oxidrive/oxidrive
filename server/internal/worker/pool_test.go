package worker

import (
	"context"
	"testing"
	"time"

	"github.com/oxidrive/oxidrive/server/internal/testutil"
	"github.com/rs/zerolog"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type testJob struct {
	done  chan struct{}
	sleep time.Duration
}

var _ Job = (*testJob)(nil)

func (j *testJob) Name() string {
	return "test job"
}

func (j *testJob) Run(_ context.Context) error {
	time.Sleep(j.sleep)
	if j.done != nil {
		j.done <- struct{}{}
	}
	return nil
}

func TestWorkerPool(t *testing.T) {
	t.Run("runs a background job", func(t *testing.T) {
		t.Parallel()

		ctx, done := context.WithTimeout(context.Background(), 200*time.Millisecond)
		defer done()

		p := NewPool(zerolog.New(zerolog.NewTestWriter(t)), PoolStopTimeout(1*time.Millisecond))
		defer p.Shutdown()

		j := p.Submit(ctx, &testJob{})

		select {
		case err := <-j.Wait():
			assert.NoError(t, err)
		case <-ctx.Done():
			t.Fatal("context timeout expired")
		}
	})

	t.Run("runs a background job for too long", func(t *testing.T) {
		t.Parallel()

		ctx, done := context.WithTimeout(context.Background(), 200*time.Millisecond)
		defer done()

		p := NewPool(zerolog.New(zerolog.NewTestWriter(t)), PoolStopTimeout(1*time.Millisecond))
		defer p.Shutdown()

		j := p.Submit(ctx, &testJob{sleep: 1 * time.Second})

		select {
		case <-j.Wait():
			t.Fatal("received result instead of timeout expiring")
		case <-ctx.Done():
			// Pass
		}
	})
}

func TestWorkerScheduler(t *testing.T) {
	t.Run("schedules a background job", func(t *testing.T) {
		t.Parallel()

		ctx, done := context.WithTimeout(context.Background(), 3*time.Second)
		defer done()

		testutil.IntegrationTest(ctx, t)

		p, err := NewScheduler(zerolog.New(zerolog.NewTestWriter(t)))
		require.NoError(t, err)
		defer p.Shutdown()

		ch := make(chan struct{})

		_, err = p.ScheduleEvery(ctx, 1*time.Second, &testJob{done: ch})
		require.NoError(t, err)

		p.Start()

		select {
		case <-ch:
		case <-ctx.Done():
			t.Fatal("context timeout expired")
		}
	})
}
