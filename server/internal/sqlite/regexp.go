package sqlite

import (
	"database/sql/driver"
	"errors"
	"regexp"
	"strings"

	"modernc.org/sqlite"
)

var alreadyRegistered string = `a function named "regexp" is already registered`

func Init() {
	if err := sqlite.RegisterDeterministicScalarFunction("regexp", 2, func(ctx *sqlite.FunctionContext, args []driver.Value) (driver.Value, error) {
		if len(args) != 2 {
			return nil, errors.New("regexp accepts only two arguments: the regex and the string to match")
		}

		expr := args[0].(string)
		s := args[1].(string)
		return regexp.MatchString(expr, s)
	}); err != nil {
		if !strings.Contains(err.Error(), alreadyRegistered) {
			panic(err)
		}
	}
}
