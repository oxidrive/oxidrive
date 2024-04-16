package integration

import (
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

// IntegrationTest skips a test if the INTEGRATION_TEST env variable is not set while running go tests
func IntegrationTest(t *testing.T) {
	t.Helper()

	if value, ok := os.LookupEnv("INTEGRATION_TEST"); !ok || !isEnvVarTrue(value) {
		t.Skip("skipping integration tests, call go test with INTEGRATION_TEST set")
	}
}
