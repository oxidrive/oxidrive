package user

import "github.com/google/uuid"

type ID (uuid.UUID)

func EmptyID() ID {
	return ID(uuid.UUID{})
}

func NewID() ID {
	return ID(uuid.Must(uuid.NewV7()))
}

func ParseID(s string) (ID, error) {
	id, err := uuid.Parse(s)
	if err != nil {
		return ID{}, err
	}

	return ID(id), nil
}

func (i ID) AsUUID() uuid.UUID {
	return uuid.UUID(i)
}

func (i ID) String() string {
	return i.AsUUID().String()
}
