package handler

type ErrorResponse struct {
	Error   string                 `json:"error"`
	Message string                 `json:"message"`
	Details map[string]interface{} `json:"details,omitempty"`
}

func ErrUnknown(err error) ErrorResponse {
	return ErrorResponse{
		Error:   "unknown",
		Message: err.Error(),
	}
}
