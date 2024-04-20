package instance

import (
	"errors"
	"fmt"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

var (
	ErrSetupAlreadyCompleted = errors.New("first time setup has been already performed")
)

type Service struct {
	users user.Users
}

func NewService(users user.Users) Service {
	return Service{users: users}
}

func (s *Service) FirstTimeSetupCompleted() (bool, error) {
	count, err := s.users.Count()
	if err != nil {
		return false, fmt.Errorf("failed to count existing users: %w", err)
	}

	return count > 0, nil
}

func (s *Service) CompleteFirstTimeSetup(admin InitialAdmin) error {
	completed, err := s.FirstTimeSetupCompleted()
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

	_, err = s.users.Save(u)
	if err != nil {
		return fmt.Errorf("failed to save initial admin: %w", err)
	}

	return nil
}

type InitialAdmin struct {
	Username string
	Password string
}
