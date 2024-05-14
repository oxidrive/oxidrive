package list

import (
	"errors"
)

var (
	ErrInvalidAfter error = errors.New("invalid 'after' cursor provided")
)

type Of[T any] struct {
	Items []T
	Count int
	Total int
	Next  *string
}

type Param func(p *Params)

func First(first *int) Param {
	return func(p *Params) {
		if first != nil {
			p.First = *first
		}
	}
}

func After(after *string) Param {
	return func(p *Params) {
		if after != nil {
			p.After = after
		}
	}
}

type Params struct {
	First int
	After *string
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
