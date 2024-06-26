//go:build go1.22

// Package api provides primitives to interact with the openapi HTTP API.
//
// Code generated by github.com/deepmap/oapi-codegen/v2 version v2.2.1-0.20240604070534-2f0ff757704b DO NOT EDIT.
package api

import (
	"bytes"
	"compress/gzip"
	"context"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"mime/multipart"
	"net/http"
	"net/url"
	"path"
	"strings"
	"time"

	"github.com/getkin/kin-openapi/openapi3"
	"github.com/oapi-codegen/runtime"
	strictnethttp "github.com/oapi-codegen/runtime/strictmiddleware/nethttp"
	openapi_types "github.com/oapi-codegen/runtime/types"
)

const (
	SessionScopes = "session.Scopes"
)

// Defines values for CredentialsKind.
const (
	CredentialsKindPassword CredentialsKind = "password"
)

// Defines values for FileType.
const (
	FileTypeFile   FileType = "file"
	FileTypeFolder FileType = "folder"
)

// Defines values for InstanceStatusStatusDatabase.
const (
	Postgres InstanceStatusStatusDatabase = "postgres"
	Sqlite   InstanceStatusStatusDatabase = "sqlite"
)

// Defines values for InstanceStatusStatusFileStorage.
const (
	Filesystem InstanceStatusStatusFileStorage = "filesystem"
	S3         InstanceStatusStatusFileStorage = "s3"
)

// Defines values for NotFoundErrorError.
const (
	NotFoundErrorErrorNotFound NotFoundErrorError = "not_found"
)

// Defines values for PasswordCredentialsKind.
const (
	PasswordCredentialsKindPassword PasswordCredentialsKind = "password"
)

// Credentials defines model for Credentials.
type Credentials struct {
	Kind  CredentialsKind `json:"kind"`
	union json.RawMessage
}

// CredentialsKind defines model for Credentials.Kind.
type CredentialsKind string

// Error defines model for Error.
type Error struct {
	// Error machine-readable error tag
	Error string `json:"error"`

	// Message human readable error message
	Message string `json:"message"`
}

// File defines model for File.
type File struct {
	ContentType string             `json:"contentType"`
	Id          openapi_types.UUID `json:"id"`
	Name        string             `json:"name"`
	Path        string             `json:"path"`

	// Size Size of the file in bytes
	Size int      `json:"size"`
	Type FileType `json:"type"`
}

// FileType defines model for File.Type.
type FileType string

// FileList defines model for FileList.
type FileList struct {
	// Count number of items in the current slice of the collection
	Count int    `json:"count"`
	Items []File `json:"items"`

	// Next Cursor of the next element, to be used as the `after` parameter in paginated operations
	Next *string `json:"next"`

	// Total total number of items in the collection
	Total int `json:"total"`
}

// FileUpload defines model for FileUpload.
type FileUpload struct {
	File openapi_types.File `json:"file"`
	Path string             `json:"path"`
}

// FileUploadResponse defines model for FileUploadResponse.
type FileUploadResponse struct {
	Id string `json:"id"`
	Ok bool   `json:"ok"`
}

// InstanceSetupRequest defines model for InstanceSetupRequest.
type InstanceSetupRequest struct {
	Admin struct {
		Password string `json:"password"`
		Username string `json:"username"`
	} `json:"admin"`
}

// InstanceSetupResponse defines model for InstanceSetupResponse.
type InstanceSetupResponse struct {
	Ok bool `json:"ok"`
}

// InstanceStatus defines model for InstanceStatus.
type InstanceStatus struct {
	Status struct {
		Database       InstanceStatusStatusDatabase    `json:"database"`
		FileStorage    InstanceStatusStatusFileStorage `json:"fileStorage"`
		PublicURL      string                          `json:"publicURL"`
		SetupCompleted bool                            `json:"setupCompleted"`
	} `json:"status"`
}

// InstanceStatusStatusDatabase defines model for InstanceStatus.Status.Database.
type InstanceStatusStatusDatabase string

// InstanceStatusStatusFileStorage defines model for InstanceStatus.Status.FileStorage.
type InstanceStatusStatusFileStorage string

// ListInfo defines model for ListInfo.
type ListInfo struct {
	// Count number of items in the current slice of the collection
	Count int `json:"count"`

	// Next Cursor of the next element, to be used as the `after` parameter in paginated operations
	Next *string `json:"next"`

	// Total total number of items in the collection
	Total int `json:"total"`
}

// NotFoundError defines model for NotFoundError.
type NotFoundError struct {
	Error NotFoundErrorError `json:"error"`

	// Message human readable error message
	Message string `json:"message"`
}

// NotFoundErrorError defines model for NotFoundError.Error.
type NotFoundErrorError string

// PasswordCredentials defines model for PasswordCredentials.
type PasswordCredentials struct {
	Kind     PasswordCredentialsKind `json:"kind"`
	Password string                  `json:"password"`
	Username string                  `json:"username"`
}

// PasswordCredentialsKind defines model for PasswordCredentials.Kind.
type PasswordCredentialsKind string

// Session defines model for Session.
type Session struct {
	ExpiresAt time.Time `json:"expiresAt"`
	User      User      `json:"user"`
}

// SessionRequest defines model for SessionRequest.
type SessionRequest struct {
	Credentials Credentials `json:"credentials"`
}

// User defines model for User.
type User struct {
	Id       openapi_types.UUID `json:"id"`
	Username string             `json:"username"`
}

// After defines model for After.
type After = string

// FilePrefix defines model for FilePrefix.
type FilePrefix = string

// First defines model for First.
type First = int

// InternalError defines model for InternalError.
type InternalError = Error

// NotFound defines model for NotFound.
type NotFound = NotFoundError

// FilesListParams defines parameters for FilesList.
type FilesListParams struct {
	// First Limit the number of items to return to only the first N
	First *First `form:"first,omitempty" json:"first,omitempty"`

	// After Cursor to fetch the next slice of the collection
	After *After `form:"after,omitempty" json:"after,omitempty"`

	// Prefix Prefix to filter files for. This is matched against the directory the files resides in, not as a generic prefix.
	// E.g. a prefix `hello` will match `hello/world.txt` but not `hello/dear/world.txt`.
	Prefix *FilePrefix `form:"prefix,omitempty" json:"prefix,omitempty"`
}

// FilesUploadMultipartRequestBody defines body for FilesUpload for multipart/form-data ContentType.
type FilesUploadMultipartRequestBody = FileUpload

// InstanceSetupJSONRequestBody defines body for InstanceSetup for application/json ContentType.
type InstanceSetupJSONRequestBody = InstanceSetupRequest

// AuthCreateSessionJSONRequestBody defines body for AuthCreateSession for application/json ContentType.
type AuthCreateSessionJSONRequestBody = SessionRequest

// AsPasswordCredentials returns the union data inside the Credentials as a PasswordCredentials
func (t Credentials) AsPasswordCredentials() (PasswordCredentials, error) {
	var body PasswordCredentials
	err := json.Unmarshal(t.union, &body)
	return body, err
}

// FromPasswordCredentials overwrites any union data inside the Credentials as the provided PasswordCredentials
func (t *Credentials) FromPasswordCredentials(v PasswordCredentials) error {
	b, err := json.Marshal(v)
	t.union = b
	return err
}

// MergePasswordCredentials performs a merge with any union data inside the Credentials, using the provided PasswordCredentials
func (t *Credentials) MergePasswordCredentials(v PasswordCredentials) error {
	b, err := json.Marshal(v)
	if err != nil {
		return err
	}

	merged, err := runtime.JSONMerge(t.union, b)
	t.union = merged
	return err
}

func (t Credentials) MarshalJSON() ([]byte, error) {
	b, err := t.union.MarshalJSON()
	if err != nil {
		return nil, err
	}
	object := make(map[string]json.RawMessage)
	if t.union != nil {
		err = json.Unmarshal(b, &object)
		if err != nil {
			return nil, err
		}
	}

	object["kind"], err = json.Marshal(t.Kind)
	if err != nil {
		return nil, fmt.Errorf("error marshaling 'kind': %w", err)
	}

	b, err = json.Marshal(object)
	return b, err
}

func (t *Credentials) UnmarshalJSON(b []byte) error {
	err := t.union.UnmarshalJSON(b)
	if err != nil {
		return err
	}
	object := make(map[string]json.RawMessage)
	err = json.Unmarshal(b, &object)
	if err != nil {
		return err
	}

	if raw, found := object["kind"]; found {
		err = json.Unmarshal(raw, &t.Kind)
		if err != nil {
			return fmt.Errorf("error reading 'kind': %w", err)
		}
	}

	return err
}

// ServerInterface represents all server handlers.
type ServerInterface interface {
	// List all available files
	// (GET /api/files)
	FilesList(w http.ResponseWriter, r *http.Request, params FilesListParams)
	// Upload a file to Oxidrive
	// (POST /api/files)
	FilesUpload(w http.ResponseWriter, r *http.Request)

	// (DELETE /api/files/{id})
	FileDelete(w http.ResponseWriter, r *http.Request, id openapi_types.UUID)
	// Get the instance status
	// (GET /api/instance)
	InstanceStatus(w http.ResponseWriter, r *http.Request)
	// Setup the instance and create the initial admin user
	// (POST /api/instance/setup)
	InstanceSetup(w http.ResponseWriter, r *http.Request)
	// Return the user information related to the current session
	// (GET /api/session)
	AuthGetSession(w http.ResponseWriter, r *http.Request)
	// Create a new session and generate the corresponding token
	// (POST /api/sessions)
	AuthCreateSession(w http.ResponseWriter, r *http.Request)
}

// ServerInterfaceWrapper converts contexts to parameters.
type ServerInterfaceWrapper struct {
	Handler            ServerInterface
	HandlerMiddlewares []MiddlewareFunc
	ErrorHandlerFunc   func(w http.ResponseWriter, r *http.Request, err error)
}

type MiddlewareFunc func(http.Handler) http.Handler

// FilesList operation middleware
func (siw *ServerInterfaceWrapper) FilesList(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	var err error

	ctx = context.WithValue(ctx, SessionScopes, []string{})

	// Parameter object where we will unmarshal all parameters from the context
	var params FilesListParams

	// ------------- Optional query parameter "first" -------------

	err = runtime.BindQueryParameter("form", true, false, "first", r.URL.Query(), &params.First)
	if err != nil {
		siw.ErrorHandlerFunc(w, r, &InvalidParamFormatError{ParamName: "first", Err: err})
		return
	}

	// ------------- Optional query parameter "after" -------------

	err = runtime.BindQueryParameter("form", true, false, "after", r.URL.Query(), &params.After)
	if err != nil {
		siw.ErrorHandlerFunc(w, r, &InvalidParamFormatError{ParamName: "after", Err: err})
		return
	}

	// ------------- Optional query parameter "prefix" -------------

	err = runtime.BindQueryParameter("form", true, false, "prefix", r.URL.Query(), &params.Prefix)
	if err != nil {
		siw.ErrorHandlerFunc(w, r, &InvalidParamFormatError{ParamName: "prefix", Err: err})
		return
	}

	handler := http.Handler(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		siw.Handler.FilesList(w, r, params)
	}))

	for _, middleware := range siw.HandlerMiddlewares {
		handler = middleware(handler)
	}

	handler.ServeHTTP(w, r.WithContext(ctx))
}

// FilesUpload operation middleware
func (siw *ServerInterfaceWrapper) FilesUpload(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	ctx = context.WithValue(ctx, SessionScopes, []string{})

	handler := http.Handler(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		siw.Handler.FilesUpload(w, r)
	}))

	for _, middleware := range siw.HandlerMiddlewares {
		handler = middleware(handler)
	}

	handler.ServeHTTP(w, r.WithContext(ctx))
}

// FileDelete operation middleware
func (siw *ServerInterfaceWrapper) FileDelete(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	var err error

	// ------------- Path parameter "id" -------------
	var id openapi_types.UUID

	err = runtime.BindStyledParameterWithOptions("simple", "id", r.PathValue("id"), &id, runtime.BindStyledParameterOptions{ParamLocation: runtime.ParamLocationPath, Explode: false, Required: true})
	if err != nil {
		siw.ErrorHandlerFunc(w, r, &InvalidParamFormatError{ParamName: "id", Err: err})
		return
	}

	handler := http.Handler(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		siw.Handler.FileDelete(w, r, id)
	}))

	for _, middleware := range siw.HandlerMiddlewares {
		handler = middleware(handler)
	}

	handler.ServeHTTP(w, r.WithContext(ctx))
}

// InstanceStatus operation middleware
func (siw *ServerInterfaceWrapper) InstanceStatus(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	handler := http.Handler(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		siw.Handler.InstanceStatus(w, r)
	}))

	for _, middleware := range siw.HandlerMiddlewares {
		handler = middleware(handler)
	}

	handler.ServeHTTP(w, r.WithContext(ctx))
}

// InstanceSetup operation middleware
func (siw *ServerInterfaceWrapper) InstanceSetup(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	handler := http.Handler(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		siw.Handler.InstanceSetup(w, r)
	}))

	for _, middleware := range siw.HandlerMiddlewares {
		handler = middleware(handler)
	}

	handler.ServeHTTP(w, r.WithContext(ctx))
}

// AuthGetSession operation middleware
func (siw *ServerInterfaceWrapper) AuthGetSession(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	ctx = context.WithValue(ctx, SessionScopes, []string{})

	handler := http.Handler(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		siw.Handler.AuthGetSession(w, r)
	}))

	for _, middleware := range siw.HandlerMiddlewares {
		handler = middleware(handler)
	}

	handler.ServeHTTP(w, r.WithContext(ctx))
}

// AuthCreateSession operation middleware
func (siw *ServerInterfaceWrapper) AuthCreateSession(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	handler := http.Handler(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		siw.Handler.AuthCreateSession(w, r)
	}))

	for _, middleware := range siw.HandlerMiddlewares {
		handler = middleware(handler)
	}

	handler.ServeHTTP(w, r.WithContext(ctx))
}

type UnescapedCookieParamError struct {
	ParamName string
	Err       error
}

func (e *UnescapedCookieParamError) Error() string {
	return fmt.Sprintf("error unescaping cookie parameter '%s'", e.ParamName)
}

func (e *UnescapedCookieParamError) Unwrap() error {
	return e.Err
}

type UnmarshalingParamError struct {
	ParamName string
	Err       error
}

func (e *UnmarshalingParamError) Error() string {
	return fmt.Sprintf("Error unmarshaling parameter %s as JSON: %s", e.ParamName, e.Err.Error())
}

func (e *UnmarshalingParamError) Unwrap() error {
	return e.Err
}

type RequiredParamError struct {
	ParamName string
}

func (e *RequiredParamError) Error() string {
	return fmt.Sprintf("Query argument %s is required, but not found", e.ParamName)
}

type RequiredHeaderError struct {
	ParamName string
	Err       error
}

func (e *RequiredHeaderError) Error() string {
	return fmt.Sprintf("Header parameter %s is required, but not found", e.ParamName)
}

func (e *RequiredHeaderError) Unwrap() error {
	return e.Err
}

type InvalidParamFormatError struct {
	ParamName string
	Err       error
}

func (e *InvalidParamFormatError) Error() string {
	return fmt.Sprintf("Invalid format for parameter %s: %s", e.ParamName, e.Err.Error())
}

func (e *InvalidParamFormatError) Unwrap() error {
	return e.Err
}

type TooManyValuesForParamError struct {
	ParamName string
	Count     int
}

func (e *TooManyValuesForParamError) Error() string {
	return fmt.Sprintf("Expected one value for %s, got %d", e.ParamName, e.Count)
}

// Handler creates http.Handler with routing matching OpenAPI spec.
func Handler(si ServerInterface) http.Handler {
	return HandlerWithOptions(si, StdHTTPServerOptions{})
}

type StdHTTPServerOptions struct {
	BaseURL          string
	BaseRouter       *http.ServeMux
	Middlewares      []MiddlewareFunc
	ErrorHandlerFunc func(w http.ResponseWriter, r *http.Request, err error)
}

// HandlerFromMux creates http.Handler with routing matching OpenAPI spec based on the provided mux.
func HandlerFromMux(si ServerInterface, m *http.ServeMux) http.Handler {
	return HandlerWithOptions(si, StdHTTPServerOptions{
		BaseRouter: m,
	})
}

func HandlerFromMuxWithBaseURL(si ServerInterface, m *http.ServeMux, baseURL string) http.Handler {
	return HandlerWithOptions(si, StdHTTPServerOptions{
		BaseURL:    baseURL,
		BaseRouter: m,
	})
}

// HandlerWithOptions creates http.Handler with additional options
func HandlerWithOptions(si ServerInterface, options StdHTTPServerOptions) http.Handler {
	m := options.BaseRouter

	if m == nil {
		m = http.NewServeMux()
	}
	if options.ErrorHandlerFunc == nil {
		options.ErrorHandlerFunc = func(w http.ResponseWriter, r *http.Request, err error) {
			http.Error(w, err.Error(), http.StatusBadRequest)
		}
	}

	wrapper := ServerInterfaceWrapper{
		Handler:            si,
		HandlerMiddlewares: options.Middlewares,
		ErrorHandlerFunc:   options.ErrorHandlerFunc,
	}

	m.HandleFunc("GET "+options.BaseURL+"/api/files", wrapper.FilesList)
	m.HandleFunc("POST "+options.BaseURL+"/api/files", wrapper.FilesUpload)
	m.HandleFunc("DELETE "+options.BaseURL+"/api/files/{id}", wrapper.FileDelete)
	m.HandleFunc("GET "+options.BaseURL+"/api/instance", wrapper.InstanceStatus)
	m.HandleFunc("POST "+options.BaseURL+"/api/instance/setup", wrapper.InstanceSetup)
	m.HandleFunc("GET "+options.BaseURL+"/api/session", wrapper.AuthGetSession)
	m.HandleFunc("POST "+options.BaseURL+"/api/sessions", wrapper.AuthCreateSession)

	return m
}

type ErrorJSONResponse Error

type InternalErrorJSONResponse Error

type NotFoundJSONResponse NotFoundError

type FilesListRequestObject struct {
	Params FilesListParams
}

type FilesListResponseObject interface {
	VisitFilesListResponse(w http.ResponseWriter) error
}

type FilesList200JSONResponse FileList

func (response FilesList200JSONResponse) VisitFilesListResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(200)

	return json.NewEncoder(w).Encode(response)
}

type FilesListdefaultJSONResponse struct {
	Body       Error
	StatusCode int
}

func (response FilesListdefaultJSONResponse) VisitFilesListResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(response.StatusCode)

	return json.NewEncoder(w).Encode(response.Body)
}

type FilesUploadRequestObject struct {
	Body *multipart.Reader
}

type FilesUploadResponseObject interface {
	VisitFilesUploadResponse(w http.ResponseWriter) error
}

type FilesUpload200JSONResponse FileUploadResponse

func (response FilesUpload200JSONResponse) VisitFilesUploadResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(200)

	return json.NewEncoder(w).Encode(response)
}

type FilesUpload400JSONResponse struct{ ErrorJSONResponse }

func (response FilesUpload400JSONResponse) VisitFilesUploadResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(400)

	return json.NewEncoder(w).Encode(response)
}

type FilesUploaddefaultJSONResponse struct {
	Body       Error
	StatusCode int
}

func (response FilesUploaddefaultJSONResponse) VisitFilesUploadResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(response.StatusCode)

	return json.NewEncoder(w).Encode(response.Body)
}

type FileDeleteRequestObject struct {
	Id openapi_types.UUID `json:"id"`
}

type FileDeleteResponseObject interface {
	VisitFileDeleteResponse(w http.ResponseWriter) error
}

type FileDelete200JSONResponse File

func (response FileDelete200JSONResponse) VisitFileDeleteResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(200)

	return json.NewEncoder(w).Encode(response)
}

type FileDelete404JSONResponse struct{ NotFoundJSONResponse }

func (response FileDelete404JSONResponse) VisitFileDeleteResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(404)

	return json.NewEncoder(w).Encode(response)
}

type FileDeletedefaultJSONResponse struct {
	Body       Error
	StatusCode int
}

func (response FileDeletedefaultJSONResponse) VisitFileDeleteResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(response.StatusCode)

	return json.NewEncoder(w).Encode(response.Body)
}

type InstanceStatusRequestObject struct {
}

type InstanceStatusResponseObject interface {
	VisitInstanceStatusResponse(w http.ResponseWriter) error
}

type InstanceStatus200JSONResponse InstanceStatus

func (response InstanceStatus200JSONResponse) VisitInstanceStatusResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(200)

	return json.NewEncoder(w).Encode(response)
}

type InstanceStatusdefaultJSONResponse struct {
	Body       Error
	StatusCode int
}

func (response InstanceStatusdefaultJSONResponse) VisitInstanceStatusResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(response.StatusCode)

	return json.NewEncoder(w).Encode(response.Body)
}

type InstanceSetupRequestObject struct {
	Body *InstanceSetupJSONRequestBody
}

type InstanceSetupResponseObject interface {
	VisitInstanceSetupResponse(w http.ResponseWriter) error
}

type InstanceSetup200JSONResponse InstanceSetupResponse

func (response InstanceSetup200JSONResponse) VisitInstanceSetupResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(200)

	return json.NewEncoder(w).Encode(response)
}

type InstanceSetup400JSONResponse struct{ ErrorJSONResponse }

func (response InstanceSetup400JSONResponse) VisitInstanceSetupResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(400)

	return json.NewEncoder(w).Encode(response)
}

type InstanceSetup409JSONResponse Error

func (response InstanceSetup409JSONResponse) VisitInstanceSetupResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(409)

	return json.NewEncoder(w).Encode(response)
}

type InstanceSetupdefaultJSONResponse struct {
	Body       Error
	StatusCode int
}

func (response InstanceSetupdefaultJSONResponse) VisitInstanceSetupResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(response.StatusCode)

	return json.NewEncoder(w).Encode(response.Body)
}

type AuthGetSessionRequestObject struct {
}

type AuthGetSessionResponseObject interface {
	VisitAuthGetSessionResponse(w http.ResponseWriter) error
}

type AuthGetSession200JSONResponse User

func (response AuthGetSession200JSONResponse) VisitAuthGetSessionResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(200)

	return json.NewEncoder(w).Encode(response)
}

type AuthCreateSessionRequestObject struct {
	Body *AuthCreateSessionJSONRequestBody
}

type AuthCreateSessionResponseObject interface {
	VisitAuthCreateSessionResponse(w http.ResponseWriter) error
}

type AuthCreateSession200ResponseHeaders struct {
	SetCookie string
}

type AuthCreateSession200JSONResponse struct {
	Body    Session
	Headers AuthCreateSession200ResponseHeaders
}

func (response AuthCreateSession200JSONResponse) VisitAuthCreateSessionResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.Header().Set("Set-Cookie", fmt.Sprint(response.Headers.SetCookie))
	w.WriteHeader(200)

	return json.NewEncoder(w).Encode(response.Body)
}

type AuthCreateSession401JSONResponse struct{ ErrorJSONResponse }

func (response AuthCreateSession401JSONResponse) VisitAuthCreateSessionResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(401)

	return json.NewEncoder(w).Encode(response)
}

type AuthCreateSessiondefaultJSONResponse struct {
	Body       Error
	StatusCode int
}

func (response AuthCreateSessiondefaultJSONResponse) VisitAuthCreateSessionResponse(w http.ResponseWriter) error {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(response.StatusCode)

	return json.NewEncoder(w).Encode(response.Body)
}

// StrictServerInterface represents all server handlers.
type StrictServerInterface interface {
	// List all available files
	// (GET /api/files)
	FilesList(ctx context.Context, request FilesListRequestObject) (FilesListResponseObject, error)
	// Upload a file to Oxidrive
	// (POST /api/files)
	FilesUpload(ctx context.Context, request FilesUploadRequestObject) (FilesUploadResponseObject, error)

	// (DELETE /api/files/{id})
	FileDelete(ctx context.Context, request FileDeleteRequestObject) (FileDeleteResponseObject, error)
	// Get the instance status
	// (GET /api/instance)
	InstanceStatus(ctx context.Context, request InstanceStatusRequestObject) (InstanceStatusResponseObject, error)
	// Setup the instance and create the initial admin user
	// (POST /api/instance/setup)
	InstanceSetup(ctx context.Context, request InstanceSetupRequestObject) (InstanceSetupResponseObject, error)
	// Return the user information related to the current session
	// (GET /api/session)
	AuthGetSession(ctx context.Context, request AuthGetSessionRequestObject) (AuthGetSessionResponseObject, error)
	// Create a new session and generate the corresponding token
	// (POST /api/sessions)
	AuthCreateSession(ctx context.Context, request AuthCreateSessionRequestObject) (AuthCreateSessionResponseObject, error)
}

type StrictHandlerFunc = strictnethttp.StrictHTTPHandlerFunc
type StrictMiddlewareFunc = strictnethttp.StrictHTTPMiddlewareFunc

type StrictHTTPServerOptions struct {
	RequestErrorHandlerFunc  func(w http.ResponseWriter, r *http.Request, err error)
	ResponseErrorHandlerFunc func(w http.ResponseWriter, r *http.Request, err error)
}

func NewStrictHandler(ssi StrictServerInterface, middlewares []StrictMiddlewareFunc) ServerInterface {
	return &strictHandler{ssi: ssi, middlewares: middlewares, options: StrictHTTPServerOptions{
		RequestErrorHandlerFunc: func(w http.ResponseWriter, r *http.Request, err error) {
			http.Error(w, err.Error(), http.StatusBadRequest)
		},
		ResponseErrorHandlerFunc: func(w http.ResponseWriter, r *http.Request, err error) {
			http.Error(w, err.Error(), http.StatusInternalServerError)
		},
	}}
}

func NewStrictHandlerWithOptions(ssi StrictServerInterface, middlewares []StrictMiddlewareFunc, options StrictHTTPServerOptions) ServerInterface {
	return &strictHandler{ssi: ssi, middlewares: middlewares, options: options}
}

type strictHandler struct {
	ssi         StrictServerInterface
	middlewares []StrictMiddlewareFunc
	options     StrictHTTPServerOptions
}

// FilesList operation middleware
func (sh *strictHandler) FilesList(w http.ResponseWriter, r *http.Request, params FilesListParams) {
	var request FilesListRequestObject

	request.Params = params

	handler := func(ctx context.Context, w http.ResponseWriter, r *http.Request, request interface{}) (interface{}, error) {
		return sh.ssi.FilesList(ctx, request.(FilesListRequestObject))
	}
	for _, middleware := range sh.middlewares {
		handler = middleware(handler, "FilesList")
	}

	response, err := handler(r.Context(), w, r, request)

	if err != nil {
		sh.options.ResponseErrorHandlerFunc(w, r, err)
	} else if validResponse, ok := response.(FilesListResponseObject); ok {
		if err := validResponse.VisitFilesListResponse(w); err != nil {
			sh.options.ResponseErrorHandlerFunc(w, r, err)
		}
	} else if response != nil {
		sh.options.ResponseErrorHandlerFunc(w, r, fmt.Errorf("unexpected response type: %T", response))
	}
}

// FilesUpload operation middleware
func (sh *strictHandler) FilesUpload(w http.ResponseWriter, r *http.Request) {
	var request FilesUploadRequestObject

	if reader, err := r.MultipartReader(); err != nil {
		sh.options.RequestErrorHandlerFunc(w, r, fmt.Errorf("can't decode multipart body: %w", err))
		return
	} else {
		request.Body = reader
	}

	handler := func(ctx context.Context, w http.ResponseWriter, r *http.Request, request interface{}) (interface{}, error) {
		return sh.ssi.FilesUpload(ctx, request.(FilesUploadRequestObject))
	}
	for _, middleware := range sh.middlewares {
		handler = middleware(handler, "FilesUpload")
	}

	response, err := handler(r.Context(), w, r, request)

	if err != nil {
		sh.options.ResponseErrorHandlerFunc(w, r, err)
	} else if validResponse, ok := response.(FilesUploadResponseObject); ok {
		if err := validResponse.VisitFilesUploadResponse(w); err != nil {
			sh.options.ResponseErrorHandlerFunc(w, r, err)
		}
	} else if response != nil {
		sh.options.ResponseErrorHandlerFunc(w, r, fmt.Errorf("unexpected response type: %T", response))
	}
}

// FileDelete operation middleware
func (sh *strictHandler) FileDelete(w http.ResponseWriter, r *http.Request, id openapi_types.UUID) {
	var request FileDeleteRequestObject

	request.Id = id

	handler := func(ctx context.Context, w http.ResponseWriter, r *http.Request, request interface{}) (interface{}, error) {
		return sh.ssi.FileDelete(ctx, request.(FileDeleteRequestObject))
	}
	for _, middleware := range sh.middlewares {
		handler = middleware(handler, "FileDelete")
	}

	response, err := handler(r.Context(), w, r, request)

	if err != nil {
		sh.options.ResponseErrorHandlerFunc(w, r, err)
	} else if validResponse, ok := response.(FileDeleteResponseObject); ok {
		if err := validResponse.VisitFileDeleteResponse(w); err != nil {
			sh.options.ResponseErrorHandlerFunc(w, r, err)
		}
	} else if response != nil {
		sh.options.ResponseErrorHandlerFunc(w, r, fmt.Errorf("unexpected response type: %T", response))
	}
}

// InstanceStatus operation middleware
func (sh *strictHandler) InstanceStatus(w http.ResponseWriter, r *http.Request) {
	var request InstanceStatusRequestObject

	handler := func(ctx context.Context, w http.ResponseWriter, r *http.Request, request interface{}) (interface{}, error) {
		return sh.ssi.InstanceStatus(ctx, request.(InstanceStatusRequestObject))
	}
	for _, middleware := range sh.middlewares {
		handler = middleware(handler, "InstanceStatus")
	}

	response, err := handler(r.Context(), w, r, request)

	if err != nil {
		sh.options.ResponseErrorHandlerFunc(w, r, err)
	} else if validResponse, ok := response.(InstanceStatusResponseObject); ok {
		if err := validResponse.VisitInstanceStatusResponse(w); err != nil {
			sh.options.ResponseErrorHandlerFunc(w, r, err)
		}
	} else if response != nil {
		sh.options.ResponseErrorHandlerFunc(w, r, fmt.Errorf("unexpected response type: %T", response))
	}
}

// InstanceSetup operation middleware
func (sh *strictHandler) InstanceSetup(w http.ResponseWriter, r *http.Request) {
	var request InstanceSetupRequestObject

	var body InstanceSetupJSONRequestBody
	if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
		sh.options.RequestErrorHandlerFunc(w, r, fmt.Errorf("can't decode JSON body: %w", err))
		return
	}
	request.Body = &body

	handler := func(ctx context.Context, w http.ResponseWriter, r *http.Request, request interface{}) (interface{}, error) {
		return sh.ssi.InstanceSetup(ctx, request.(InstanceSetupRequestObject))
	}
	for _, middleware := range sh.middlewares {
		handler = middleware(handler, "InstanceSetup")
	}

	response, err := handler(r.Context(), w, r, request)

	if err != nil {
		sh.options.ResponseErrorHandlerFunc(w, r, err)
	} else if validResponse, ok := response.(InstanceSetupResponseObject); ok {
		if err := validResponse.VisitInstanceSetupResponse(w); err != nil {
			sh.options.ResponseErrorHandlerFunc(w, r, err)
		}
	} else if response != nil {
		sh.options.ResponseErrorHandlerFunc(w, r, fmt.Errorf("unexpected response type: %T", response))
	}
}

// AuthGetSession operation middleware
func (sh *strictHandler) AuthGetSession(w http.ResponseWriter, r *http.Request) {
	var request AuthGetSessionRequestObject

	handler := func(ctx context.Context, w http.ResponseWriter, r *http.Request, request interface{}) (interface{}, error) {
		return sh.ssi.AuthGetSession(ctx, request.(AuthGetSessionRequestObject))
	}
	for _, middleware := range sh.middlewares {
		handler = middleware(handler, "AuthGetSession")
	}

	response, err := handler(r.Context(), w, r, request)

	if err != nil {
		sh.options.ResponseErrorHandlerFunc(w, r, err)
	} else if validResponse, ok := response.(AuthGetSessionResponseObject); ok {
		if err := validResponse.VisitAuthGetSessionResponse(w); err != nil {
			sh.options.ResponseErrorHandlerFunc(w, r, err)
		}
	} else if response != nil {
		sh.options.ResponseErrorHandlerFunc(w, r, fmt.Errorf("unexpected response type: %T", response))
	}
}

// AuthCreateSession operation middleware
func (sh *strictHandler) AuthCreateSession(w http.ResponseWriter, r *http.Request) {
	var request AuthCreateSessionRequestObject

	var body AuthCreateSessionJSONRequestBody
	if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
		sh.options.RequestErrorHandlerFunc(w, r, fmt.Errorf("can't decode JSON body: %w", err))
		return
	}
	request.Body = &body

	handler := func(ctx context.Context, w http.ResponseWriter, r *http.Request, request interface{}) (interface{}, error) {
		return sh.ssi.AuthCreateSession(ctx, request.(AuthCreateSessionRequestObject))
	}
	for _, middleware := range sh.middlewares {
		handler = middleware(handler, "AuthCreateSession")
	}

	response, err := handler(r.Context(), w, r, request)

	if err != nil {
		sh.options.ResponseErrorHandlerFunc(w, r, err)
	} else if validResponse, ok := response.(AuthCreateSessionResponseObject); ok {
		if err := validResponse.VisitAuthCreateSessionResponse(w); err != nil {
			sh.options.ResponseErrorHandlerFunc(w, r, err)
		}
	} else if response != nil {
		sh.options.ResponseErrorHandlerFunc(w, r, fmt.Errorf("unexpected response type: %T", response))
	}
}

// Base64 encoded, gzipped, json marshaled Swagger object
var swaggerSpec = []string{

	"H4sIAAAAAAAC/8xZ23LbONJ+FRT+/5I62PFmKpqai6x3JuvaVJKKx1eJawyRLQljEmCAZiKNi+++hQZ4",
	"EiHJu7FTe2UTp258ffoaeuCpLkqtQKHliwdeCiMKQDD09XqFYNw/GdjUyBKlVnzBLytjtWGo2Qow3TDc",
	"AFOwRWZzmQLTKxpJdZ5DSlsSLt2+LxWYHU+4EgXwBRd0esJtuoFCODG4K92ERSPVmtd1wn+TOXwwsJLb",
	"sRp+nNSQOYJxf8CylTZT9vtGWiYtKwSmG8iYWAupLJJimTSQojY7+vKbDFiZgWVSJUxpZMIywdagwMiU",
	"lSRo+ln9Ol1PmQjf7G4Dea7v2DeZ515QGJp90ybPprjFO7askA4MMxkI05uefj4EjpcxQGelTSHQzQnc",
	"8CSKlrE4BuqtLKS/uqqKJRhnIYlQWIedAayMcv9plTeQGIvs3QHNaHagWAYrUeXIF2fzecILsZVFVfDF",
	"OX1J5b/OWoWlQliD4bVT2YAttbJA/varMZr8LdUKQdFVRFnmMhXuKrM/rbvPQ0/0/xtY8QX/v1nnxjM/",
	"a2f+tJrEXCkEo0T+TBKSPcRvFGxLSBEyFtYk/J3G33SlsieT3hzY3bNurEJoXhrIQKEUOX2KPH+/4otP",
	"D7w0ugSD0oN+L71OoJyZPvFSWPtNm4zfxlzMwJdKGsjcStrZrdLLPyFFXicPXCsIso5d4EOQ1Fe0vq1v",
	"66TzhKGu0AwP4S5EupEKJgZEJpY5MFrHUKzHYZLwAqwVaxgfs6kKodjeIc3qU2B41brTI7hQNhtfKjjD",
	"77T6gcNWFGVOgVKINcxKFb2FzAY5oapkFlvmg7Z/qkt4LvnEVlNmGax2IzPUs96uE3ko4Vb+FUH3Wv7V",
	"1gZ3GpOKLXcIliedvIvzXsqYj1NGM9K5qzuKlMozMKd9tkMpGQCfNLchwMIdDtnwrfRptgupY27uVl+p",
	"labIGJqesvDgn2MHkfu0EHBhjNiNL0gnjTW/DbrflLkW2dgLV8E3W+supRKU9w96yXGkA5507iEgvTIf",
	"Qw0YK+WdfKSAvu8NL7XOQaiRfH3PKUpisq+URaFSuAasyo/wpQJv0KF0kRVSjYfbBBlTrbKuzviYO45P",
	"uzKJpdxG171NXqVHXOkQoo+F7qgIFFjZ8dn2wHgmUCyFHYRtqS2uDQW//ZJLhEjoeue5Rm1Cuu7HvN1Z",
	"hMJtfxHdWlbLXKY3H98O89kGsbSL2cz6U6dhZprqop/aKiOjmc2Be6ndDoTsEUh2WiQdDMN7jU496QQB",
	"55iJ2lwTqTKVinDDfUYolSfvlTGgjjH6FtKzUxnbtQYHm4hwNLUPkEMBChNHRpfAKuuIu6X5O2oW7ljb",
	"nzhFS7GWSjiS5S5KFGpQTPjL1cslZK+Wkxfnc5hcvDr7aSJezcUEzl/CxU9n6YX427lL+VWeu4rPF2gq",
	"iJgdNYp8fAUaZocQjIJ1qr7tWdobrVEgQBmz+5ALHuZNTQQpjX+siI3e/k8xpBglXHwHY02eKluTzOSR",
	"STvh12Ct1JHiAdtSGrCvcVBrM4EwQVlALOk4maeowY2NeA9tTHoij2h6sAimQ0scU2LA4/c9uTcXU+Mm",
	"XDJW/0+S3MebUQ6MGM20FtLKSNxdu1uFwtZZkzriVOt7CV1LrLcyM/IrTJqFHUcr5b9g51tEGfIySqTs",
	"9D7sYq8/XPGEfwXjhfCz6Xw6J55TghKl5Av+goY89SKVZqKURMrpaw1kuTYNXmV8QQTLElVNBu86B+hq",
	"t2Tm3xEcXT2x0D8QPWJh7x2nvt3r+s/n8yfriVt2XoeuPLxMxHe1asyG7wN9NyC8Wgf4dOvUt1VROG68",
	"oHrLRJ4z8VVIKiL+Qcl5gFjblqxwR78d5zlgpsDKvbOCxb/rbLcHSlHlKEthcObiYeLYxH+GS5BRD2PC",
	"lbz6mS2yR/O9bS68kON2aV9Pnt2SXkcmfG+KmjXhGTFlnfTib/Ygs9qXSsff4gb+h58bBeKwvF5lgwYZ",
	"NcuafZR3+i3qwueyoSGjz4XxrPnsYdia+eK00dq3se+z9MBOf2jcgOlZS4b25WDC3OtvnhGfPUnfn6xa",
	"N34D/qW3uSyzzWUacFoYxsjMqBOhMhxNVYMe82iyegJo+q35D85Z8V76v0lbF/NXPyjJtR5ASg99QKiM",
	"pQYEQhiXjogxekxggSUecY8e/YnGzesKN28Ar1vy82yGCTS3fnxa/xh+3thQP+naRp8XpXZ9TE7dI+ph",
	"z9uRuICJqHAzxsMeDhSHyCUB3gfl6YNlj7z/4DBp7kZiNyCy8LPhNeDk0lPkwVldY75PmH/5XM3nL9Lw",
	"NZEZfcPP7IPAzS+zn9k/Ecv3Ko88R4agPPvRYebNywRT8K1xGQo0+tWwCbVUG39cJtWaob6HmF91peuh",
	"+5XNs8jtJJO2zMXuXX/cMe7+wlDrxsv9eG95G97jte1UbzlpOF5Kwy7QthMU6zdGV+VA+7a1IXIbGpwh",
	"i0r2qnQy3tyk4b39vQwV2RSccm9PQPq2/ncAAAD///Jzl/XxHgAA",
}

// GetSwagger returns the content of the embedded swagger specification file
// or error if failed to decode
func decodeSpec() ([]byte, error) {
	zipped, err := base64.StdEncoding.DecodeString(strings.Join(swaggerSpec, ""))
	if err != nil {
		return nil, fmt.Errorf("error base64 decoding spec: %w", err)
	}
	zr, err := gzip.NewReader(bytes.NewReader(zipped))
	if err != nil {
		return nil, fmt.Errorf("error decompressing spec: %w", err)
	}
	var buf bytes.Buffer
	_, err = buf.ReadFrom(zr)
	if err != nil {
		return nil, fmt.Errorf("error decompressing spec: %w", err)
	}

	return buf.Bytes(), nil
}

var rawSpec = decodeSpecCached()

// a naive cached of a decoded swagger spec
func decodeSpecCached() func() ([]byte, error) {
	data, err := decodeSpec()
	return func() ([]byte, error) {
		return data, err
	}
}

// Constructs a synthetic filesystem for resolving external references when loading openapi specifications.
func PathToRawSpec(pathToFile string) map[string]func() ([]byte, error) {
	res := make(map[string]func() ([]byte, error))
	if len(pathToFile) > 0 {
		res[pathToFile] = rawSpec
	}

	return res
}

// GetSwagger returns the Swagger specification corresponding to the generated code
// in this file. The external references of Swagger specification are resolved.
// The logic of resolving external references is tightly connected to "import-mapping" feature.
// Externally referenced files must be embedded in the corresponding golang packages.
// Urls can be supported but this task was out of the scope.
func GetSwagger() (swagger *openapi3.T, err error) {
	resolvePath := PathToRawSpec("")

	loader := openapi3.NewLoader()
	loader.IsExternalRefsAllowed = true
	loader.ReadFromURIFunc = func(loader *openapi3.Loader, url *url.URL) ([]byte, error) {
		pathToFile := url.String()
		pathToFile = path.Clean(pathToFile)
		getSpec, ok := resolvePath[pathToFile]
		if !ok {
			err1 := fmt.Errorf("path not found: %s", pathToFile)
			return nil, err1
		}
		return getSpec()
	}
	var specData []byte
	specData, err = rawSpec()
	if err != nil {
		return
	}
	swagger, err = loader.LoadFromData(specData)
	if err != nil {
		return
	}
	return
}
