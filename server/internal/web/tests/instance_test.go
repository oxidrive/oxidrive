package web

import (
	"context"
	"encoding/json"
	"net/http"

	"testing"

	"github.com/steinfletcher/apitest"

	"github.com/oxidrive/oxidrive/server/internal/testutil"
	"github.com/oxidrive/oxidrive/server/internal/web/api"
)

func TestApi_Instance(t *testing.T) {
	t.Run("completes the setup flow", func(t *testing.T) {
		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir(), testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		_, handler := setup(ctx, t)

		apitest.New().
			Debug().
			Handler(handler).
			Get("/api/instance").
			Expect(t).
			Status(http.StatusOK).
			Body(`{"status":{"database":"sqlite","fileStorage":"filesystem","publicURL":"https://example.com","setupCompleted":false}}`).
			End()

		req := api.InstanceSetupRequest{
			Admin: struct {
				Password string "json:\"password\""
				Username string "json:\"username\""
			}{
				Password: "test",
				Username: "test",
			},
		}

		body := testutil.Must(json.Marshal(req))

		apitest.New().
			Debug().
			Handler(handler).
			Post("/api/instance/setup").
			JSON(body).
			Expect(t).
			Status(http.StatusOK).
			Body(`{"ok": true}`).
			End()

		apitest.New().
			Debug().
			Handler(handler).
			Get("/api/instance").
			Expect(t).
			Status(http.StatusOK).
			Body(`{"status":{"database":"sqlite","fileStorage":"filesystem","publicURL":"https://example.com","setupCompleted":true}}`).
			End()
	})
}
