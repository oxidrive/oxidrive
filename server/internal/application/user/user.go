package user

import (
	"errors"
	"time"
)

var (
	ErrNameNotValid = errors.New("pippo is not valid")
)

type User struct {
	Username  string
	Password  string
	Name      string
	Surname   string
	Email     string
	Role      string
	CreatedAt time.Time
	UpdatedAt time.Time
}

type UserCreateDTO struct {
	ID        string
	Username  string
	Password  string
	Name      string
	Surname   string
	Email     string
	Role      string
	CreatedAt time.Time
	UpdatedAt time.Time
}

type UserUpdateDTO struct {
	ID       string
	Username *string
	Password *string
	Name     *string
	Surname  *string
	Email    *string
	Role     *string
}

type UserRetrieveDTO struct {
	ID        string
	Username  string
	Password  string
	Name      string
	Surname   string
	Email     string
	Role      string
	CreatedAt time.Time
	UpdatedAt time.Time
}

type UserRepository interface {
	Register(u UserCreateDTO) error
	Update(u UserUpdateDTO) error
	FindByUsername(username string) (UserRetrieveDTO, error)
	FindByEmail(email string) (UserRetrieveDTO, error)
}
