package file

import (
	"fmt"
	"path"
	"strings"
)

type Path string

func ParsePath(p string) (Path, error) {
	if p == "" {
		return Path("/"), nil
	}

	cleaned := path.Clean(p)

	if strings.HasPrefix(cleaned, "../") {
		return Path(""), ErrInvalidPath
	}

	if !path.IsAbs(cleaned) {
		cleaned = fmt.Sprintf("/%s", cleaned)
	}

	return Path(cleaned), nil
}

func (p Path) Parent() Path {
	return Path(path.Dir(string(p)))
}

func (p Path) IsRoot() bool {
	return p == "/"
}

func (p Path) Name() Name {
	return Name(path.Base(string(p)))
}

func (p Path) String() string {
	return string(p)
}
