package instance

import (
	"fmt"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type Service struct {
	users user.Users
}

func NewService(users user.Users) Service {
	return Service{users: users}
}

func (s *Service) CompleteFirstTimeSetup(admin InitialAdmin) error {
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
