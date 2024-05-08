package worker

import (
	"context"
	"testing"
	"time"

	"github.com/rs/zerolog"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

type testCron struct {
	done  chan struct{}
	sleep time.Duration
}

var _ Job = (*testCron)(nil)

func (j *testCron) Name() string {
	return "test job"
}

func (j *testCron) Run(_ context.Context) error {
	time.Sleep(j.sleep)
	if j.done != nil {
		j.done <- struct{}{}
	}
	return nil
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

		_, err = p.ScheduleEvery(ctx, 1*time.Second, &testCron{done: ch})
		require.NoError(t, err)

		p.Start()

		select {
		case <-ch:
		case <-ctx.Done():
			t.Fatal("context timeout expired")
		}
	})
}
