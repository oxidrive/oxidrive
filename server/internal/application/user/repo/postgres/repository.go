package postgres

import (
	"errors"
	"fmt"
	"strings"

	"github.com/jmoiron/sqlx"
	"github.com/oxidrive/oxidrive/server/internal/application/user"
)

type PostgreSQLUserRepository struct {
	db *sqlx.DB
}

func NewPostgreSQLUserRepository(db *sqlx.DB) *PostgreSQLUserRepository {
	return &PostgreSQLUserRepository{
		db: db,
	}
}

func (r *PostgreSQLUserRepository) Register(u user.UserCreateDTO) error {
	_, err := r.db.Exec(`
        INSERT INTO users (id, username, password, name, surname, email, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`,
		u.ID, u.Username, u.Password, u.Name, u.Surname, u.Email, u.Role, u.CreatedAt, u.UpdatedAt)
	if err != nil {
		return err
	}
	return nil
}

func (r *PostgreSQLUserRepository) Update(u user.UserUpdateDTO) error {
	var fieldsToUpdate []string
	var params []interface{}

	if u.Username != nil {
		fieldsToUpdate = append(fieldsToUpdate, "username = $1")
		params = append(params, *u.Username)
	}
	if u.Password != nil {
		fieldsToUpdate = append(fieldsToUpdate, "password = $2")
		params = append(params, *u.Password)
	}
	if u.Name != nil {
		fieldsToUpdate = append(fieldsToUpdate, "name = $3")
		params = append(params, *u.Name)
	}
	if u.Surname != nil {
		fieldsToUpdate = append(fieldsToUpdate, "surname = $4")
		params = append(params, *u.Surname)
	}
	if u.Email != nil {
		fieldsToUpdate = append(fieldsToUpdate, "email = $5")
		params = append(params, *u.Email)
	}
	if u.Role != nil {
		fieldsToUpdate = append(fieldsToUpdate, "role = $6")
		params = append(params, *u.Role)
	}

	if len(fieldsToUpdate) == 0 {
		return errors.New("no fields to update")
	}

	query := fmt.Sprintf(`
		UPDATE users
		SET %s
		WHERE id = $%d`,
		strings.Join(fieldsToUpdate, ", "),
		len(fieldsToUpdate)+1)

	params = append(params, u.ID)

	result, err := r.db.Exec(query, params...)
	if err != nil {
		return err
	}

	rowsAffected, err := result.RowsAffected()
	if err != nil {
		return err
	}
	if rowsAffected == 0 {
		return errors.New("no user found with the given ID")
	}

	return nil
}

func (r *PostgreSQLUserRepository) FindByUsername(username string) (user.UserRetrieveDTO, error) {
	var u user.UserRetrieveDTO
	err := r.db.QueryRow(`
        SELECT id, username, password, name, surname, email, role, created_at, updated_at
        FROM users WHERE username = $1`, username).
		Scan(&u.ID, &u.Username, &u.Password, &u.Name, &u.Surname, &u.Email, &u.Role, &u.CreatedAt, &u.UpdatedAt)
	if err != nil {
		return user.UserRetrieveDTO{}, err
	}
	return u, nil
}

func (r *PostgreSQLUserRepository) FindByEmail(email string) (user.UserRetrieveDTO, error) {
	var u user.UserRetrieveDTO
	err := r.db.QueryRow(`
        SELECT id, username, password, name, surname, email, role, created_at, updated_at
        FROM users WHERE email = $1`, email).
		Scan(&u.ID, &u.Username, &u.Password, &u.Name, &u.Surname, &u.Email, &u.Role, &u.CreatedAt, &u.UpdatedAt)
	if err != nil {
		return user.UserRetrieveDTO{}, err
	}
	return u, nil
}
