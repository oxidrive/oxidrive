package testutil

import (
	"context"
	"os"
	"strings"
	"testing"
)

func isEnvVarTrue(value string) bool {
	switch strings.ToLower(value) {
	case "y", "yes", "t", "true", "1":
		return true
	default:
		return false
	}
}

type IntegrationDependency func(context.Context, *testing.T) (context.Context, func())

// IntegrationTest skips a test if the INTEGRATION_TEST env variable is not set while running go tests
func IntegrationTest(ctx context.Context, t *testing.T, integrationDeps ...IntegrationDependency) (context.Context, func()) {
	t.Helper()

	if value, ok := os.LookupEnv("INTEGRATION_TEST"); !ok || !isEnvVarTrue(value) {
		t.Skip("skipping integration tests, call go test with INTEGRATION_TEST set")
	}

	closeFunctions := make([]func(), len(integrationDeps))
	for i, dependency := range integrationDeps {
		c, close := dependency(ctx, t)
		ctx = c
		closeFunctions[i] = close
	}

	return ctx, func() {
		for _, closeFunc := range closeFunctions {
			closeFunc()
		}
	}
}
