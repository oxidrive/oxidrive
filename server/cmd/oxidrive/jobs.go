package main

import (
	"github.com/oxidrive/oxidrive/server/internal/app"
	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/worker"
)

func cron(deps app.ApplicationDependencies) []worker.CronJob {
	return []worker.CronJob{
		// every 5 minutes
		{Cron: "*/5 * * * *", Job: auth.NewTokenCleanupJob(deps.Tokens)},
	}
}
