package web

import (
	"context"
	"encoding/json"
	"net/http"

	"testing"

	"github.com/steinfletcher/apitest"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
	"github.com/oxidrive/oxidrive/server/internal/web/api"
)

func TestApi_Sessions(t *testing.T) {
	t.Run("creates a new session for a user", func(t *testing.T) {
		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir(), testutil.WithSqliteDB())
		defer done()

		app, handler := setup(ctx, t)

		username := "test"
		password := "test"

		testutil.Must(app.Users().Save(ctx, *testutil.Must(user.Create(username, password))))

		creds := api.Credentials{
			Kind: api.CredentialsKindPassword,
		}

		require.NoError(t, creds.FromPasswordCredentials(api.PasswordCredentials{
			Kind:     api.PasswordCredentialsKindPassword,
			Username: username,
			Password: password,
		}))

		req := api.SessionRequest{
			Credentials: creds,
		}

		body := testutil.Must(json.Marshal(req))

		apitest.New().
			Debug().
			Handler(handler).
			Post("/api/sessions").
			JSON(body).
			Expect(t).
			Status(http.StatusOK).
			End()
	})
}
