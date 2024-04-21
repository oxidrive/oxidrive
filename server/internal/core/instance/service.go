package instance

import (
	"context"
	"errors"
	"fmt"
	"net/url"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

var (
	ErrSetupAlreadyCompleted = errors.New("first time setup has been already performed")
)

type Info struct {
	PublicURL   *url.URL
	Database    StatusDB
	FileStorage StatusFileStorage
}

type Service struct {
	info  Info
	users user.Users
}

func NewService(info Info, users user.Users) Service {
	return Service{
		info:  info,
		users: users,
	}
}

func (s *Service) FirstTimeSetupCompleted(ctx context.Context) (bool, error) {
	count, err := s.users.Count(ctx)
	if err != nil {
		return false, fmt.Errorf("failed to count existing users: %w", err)
	}

	return count > 0, nil
}

func (s *Service) CompleteFirstTimeSetup(ctx context.Context, admin InitialAdmin) error {
	completed, err := s.FirstTimeSetupCompleted(ctx)
	if err != nil {
		return err
	}

	if completed {
		return ErrSetupAlreadyCompleted
	}

	u, err := user.Create(admin.Username, admin.Password)
	if err != nil {
		return fmt.Errorf("failed to create initial admin: %w", err)
	}

	_, err = s.users.Save(ctx, u)
	if err != nil {
		return fmt.Errorf("failed to save initial admin: %w", err)
	}

	return nil
}

type InitialAdmin struct {
	Username string
	Password string
}

func (s *Service) Status(ctx context.Context) (Status, error) {
	completed, err := s.FirstTimeSetupCompleted(ctx)
	if err != nil {
		return Status{}, err
	}

	return Status{
		Info:                    s.info,
		FirstTimeSetupCompleted: completed,
	}, nil
}

type StatusDB string
type StatusFileStorage string

const (
	StatusDBPostgres StatusDB = "postgres"
	StatusDBSqlite   StatusDB = "sqlite"
)

const (
	StatusFileStorageFS StatusFileStorage = "filesystem"
	StatusFileStorageS3 StatusFileStorage = "s3"
)

type Status struct {
	Info
	FirstTimeSetupCompleted bool
}
