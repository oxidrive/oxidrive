package list

import (
	"encoding/base64"
	"errors"
	"fmt"
)

var (
	ErrInvalidAfter error = errors.New("invalid 'after' cursor provided")
)

type Of[T any] struct {
	Items []T
	Count int
	Total int
	Next  *Cursor
}

type Param func(p *Params)

func First(first *int) Param {
	return func(p *Params) {
		if first != nil {
			p.First = *first
		}
	}
}

func After(after *Cursor) Param {
	return func(p *Params) {
		if after != nil {
			p.After = after
		}
	}
}

type Params struct {
	First int
	After *Cursor
}

var DefaultParams Params = Params{
	First: 100,
	After: nil,
}

func NewParams(params ...Param) Params {
	pp := DefaultParams
	for _, p := range params {
		p(&pp)
	}

	return pp
}

func Empty[T any]() Of[T] {
	return Of[T]{
		Items: []T{},
		Total: 0,
		Next:  nil,
	}
}

type Cursor string

func CursorFromString(s *string) *Cursor {
	if s == nil {
		return nil
	}
	c := Cursor(*s)
	return &c
}

func (c *Cursor) ToString() *string {
	if c == nil {
		return nil
	}
	s := string(*c)
	return &s
}

func EncodeCursor(value string) Cursor {
	c := base64.StdEncoding.EncodeToString([]byte(value))
	return Cursor(c)
}

func DecodeCursor(c Cursor) string {
	value, err := base64.StdEncoding.DecodeString(string(c))
	if err != nil {
		panic(fmt.Errorf("failed to decode cursor %s using Base64: %w", c, err))
	}

	return string(value)
}
