package handler

import (
	"context"
	"errors"
	"fmt"

	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/app"
	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/web/api"
)

type Sessions struct {
	Logger zerolog.Logger
	App    *app.Application
}

func (a *Sessions) CreateSession(ctx context.Context, request api.AuthCreateSessionRequestObject) (api.AuthCreateSessionResponseObject, error) {
	creds := request.Body.Credentials
	switch creds.Kind {
	case api.CredentialsKindPassword:
		pwd, err := creds.AsPasswordCredentials()
		if err != nil {
			return nil, err
		}

		t, _, err := a.App.Auth().AuthenticateWithPassword(ctx, pwd.Username, pwd.Password)
		if errors.Is(err, auth.ErrAuthenticationFailed) {
			return api.AuthCreateSession401JSONResponse{ErrorJSONResponse: api.ErrorJSONResponse(api.Error{
				Error:   "authentication_failed",
				Message: "Authentication failed",
			})}, nil
		}

		if err != nil {
			return nil, err
		}

		return api.AuthCreateSession200JSONResponse(api.Session{
			ExpiresAt: t.ExpiresAt,
			Token:     t.Value.String(),
		}), nil
	default:
		a.Logger.Error().Str("kind", string(request.Body.Credentials.Kind)).Msg("invalid credentials kind. This should have been caught by the validation middleware!")
		return nil, fmt.Errorf("invalid credentials kind '%s", request.Body.Credentials.Kind)
	}
}
