package handler

import (
	"context"
	"errors"
	"fmt"
	"net/http"

	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/app"
	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/web/api"
)

const SessionCookieName string = "oxidrive-session"

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

		t, u, err := a.App.Auth().AuthenticateWithPassword(ctx, pwd.Username, pwd.Password)
		if errors.Is(err, auth.ErrAuthenticationFailed) {
			return api.AuthCreateSession401JSONResponse{ErrorJSONResponse: api.ErrorJSONResponse(api.Error{
				Error:   "authentication_failed",
				Message: "Authentication failed",
			})}, nil
		}

		if err != nil {
			return nil, err
		}

		session := http.Cookie{
			Name:     SessionCookieName,
			Value:    t.Value.String(),
			Expires:  t.ExpiresAt,
			HttpOnly: true,
			SameSite: http.SameSiteStrictMode,
			Path:     "/",
		}

		return api.AuthCreateSession200JSONResponse{
			Body: api.Session{
				ExpiresAt: t.ExpiresAt,
				User: api.User{
					Id:       u.ID.AsUUID(),
					Username: u.Username,
				},
			},
			Headers: api.AuthCreateSession200ResponseHeaders{
				SetCookie: session.String(),
			},
		}, nil
	default:
		a.Logger.Error().Str("kind", string(request.Body.Credentials.Kind)).Msg("invalid credentials kind. This should have been caught by the validation middleware!")
		return nil, fmt.Errorf("invalid credentials kind '%s", request.Body.Credentials.Kind)
	}
}

func (a *Sessions) GetSession(ctx context.Context, _ api.AuthGetSessionRequestObject) (api.AuthGetSessionResponseObject, error) {
	u := auth.GetCurrentUser(ctx)
	s := auth.GetCurrentSession(ctx)
	return api.AuthGetSession200JSONResponse{
		ExpiresAt: s.ExpiresAt,
		User: api.User{
			Id:       u.ID.AsUUID(),
			Username: u.Username,
		},
	}, nil
}
