// Template: replace ExampleEntity*, publicapi, and examplefeature with
// project-specific names. Target location:
// internal/handler/router/{surface}/{entity}_test.go
// Replace "app" in imports with the project's module path.
//
// Wires the REAL usecase with a fake repository, not a fake usecase. The
// handler holds a concrete *Usecase, so there is nothing to substitute without
// adding an interface production code does not need — and the real usecase
// means one test proves the whole chain: repository sentinel -> usecase
// sentinel -> HTTP status.
//
// Assert the contract here, not the business rules. The rule that an empty name
// is invalid belongs to the domain test; this level asserts that a
// usecase.ErrValidation surfaces as 400 with the expected body.
package publicapi_test

import (
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v3"
	"github.com/google/uuid"

	"app/internal/domain/repository"
	"app/internal/domain/repository/repotest"
	"app/internal/handler/apierror"
	"app/internal/handler/auth"
	"app/internal/handler/router/publicapi"
	"app/internal/usecase/examplefeature"
)

func TestCreateExampleEntity_Created(t *testing.T) {
	t.Parallel()

	repo := &repotest.FakeExampleRepository{}
	app, ownerID := newTestApp(t, repo)

	resp := do(t, app, http.MethodPost, "/v1/example-entities",
		`{"name":"quarterly report","url":"https://example.test/a"}`)
	defer resp.Body.Close()

	if resp.StatusCode != fiber.StatusCreated {
		t.Fatalf("status = %d, want %d (body: %s)", resp.StatusCode, fiber.StatusCreated, readBody(t, resp))
	}

	var got publicapi.CreateExampleEntityResponse
	if err := json.NewDecoder(resp.Body).Decode(&got); err != nil {
		t.Fatalf("decode response: %v", err)
	}
	if got.Name != "quarterly report" {
		t.Errorf("name = %q, want %q", got.Name, "quarterly report")
	}
	if got.ID == uuid.Nil {
		t.Error("id = zero UUID, want a generated id")
	}

	// The owner comes from the authenticated context, never from the body —
	// this assertion is the reason to test at the handler level at all.
	if got.OwnerID != ownerID {
		t.Errorf("owner_id = %v, want %v", got.OwnerID, ownerID)
	}
}

// TestCreateExampleEntity_StatusMapping is the payoff of wiring the real
// usecase: each row drives a repository sentinel in at one end and asserts the
// HTTP status that comes out the other, through usecase.ConvertError and
// apierror.Handler.
func TestCreateExampleEntity_StatusMapping(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name        string
		giveBody    string
		giveRepoErr error
		wantStatus  int
		wantCode    string
	}{
		{
			name:       "invalid body is rejected before the usecase",
			giveBody:   `{"name":`,
			wantStatus: fiber.StatusBadRequest,
			wantCode:   "REQUEST_ERROR",
		},
		{
			name:       "domain validation surfaces as 400",
			giveBody:   `{"name":"","url":"https://example.test/a"}`,
			wantStatus: fiber.StatusBadRequest,
			wantCode:   "VALIDATION_ERROR",
		},
		{
			name:        "unique violation surfaces as 409",
			giveBody:    `{"name":"quarterly report","url":"https://example.test/a"}`,
			giveRepoErr: repotest.Error("create", repository.ErrUniqueViolation),
			wantStatus:  fiber.StatusConflict,
			wantCode:    "CONFLICT",
		},
		{
			name:        "unrecognised failure surfaces as 500 without leaking the chain",
			giveBody:    `{"name":"quarterly report","url":"https://example.test/a"}`,
			giveRepoErr: repotest.Error("create", io.ErrUnexpectedEOF),
			wantStatus:  fiber.StatusInternalServerError,
			wantCode:    "INTERNAL_ERROR",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()

			repo := &repotest.FakeExampleRepository{CreateErr: tt.giveRepoErr}
			app, _ := newTestApp(t, repo)

			resp := do(t, app, http.MethodPost, "/v1/example-entities", tt.giveBody)
			defer resp.Body.Close()

			if resp.StatusCode != tt.wantStatus {
				t.Fatalf("status = %d, want %d (body: %s)", resp.StatusCode, tt.wantStatus, readBody(t, resp))
			}

			var body struct {
				Code    string `json:"code"`
				Message string `json:"message"`
			}
			if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
				t.Fatalf("decode error response: %v", err)
			}
			if body.Code != tt.wantCode {
				t.Errorf("code = %q, want %q", body.Code, tt.wantCode)
			}
			if tt.wantStatus == fiber.StatusInternalServerError && body.Message != "internal error" {
				t.Errorf("message = %q, want the generic message — internals must not reach the client", body.Message)
			}
		})
	}
}

// --- Fixtures ---------------------------------------------------------------

// newTestApp builds the app the way cmd/api/main.go does, with the project's
// real ErrorHandler. A test app using Fiber's default handler proves nothing
// about status mapping, which is the main reason to write this test at all.
func newTestApp(t *testing.T, repo *repotest.FakeExampleRepository) (*fiber.App, uuid.UUID) {
	t.Helper()

	ownerID := uuid.New()

	app := fiber.New(fiber.Config{ErrorHandler: apierror.Handler})

	// auth.WithUser is the single writer of the request-scoped user and the
	// real middleware uses it too, so this is not a test-only export. Without
	// it an external test package cannot reach the unexported context key.
	app.Use(func(c fiber.Ctx) error {
		auth.WithUser(c, auth.User{ID: ownerID})
		return c.Next()
	})

	api := app.Group("/v1")
	publicapi.NewExampleEntityHandler(
		examplefeature.NewCreateExampleEntityUsecase(repo),
	).Register(api)

	return app, ownerID
}

func do(t *testing.T, app *fiber.App, method, path, body string) *http.Response {
	t.Helper()

	req := httptest.NewRequest(method, path, strings.NewReader(body))
	req.Header.Set("Content-Type", "application/json")

	// The default app.Test timeout is one second. Disable it: nothing at this
	// level does real IO, so a hang is a bug worth failing on immediately.
	resp, err := app.Test(req, fiber.TestConfig{Timeout: 0})
	if err != nil {
		t.Fatalf("app.Test(%s %s) error = %v", method, path, err)
	}

	return resp
}

func readBody(t *testing.T, resp *http.Response) string {
	t.Helper()

	b, err := io.ReadAll(resp.Body)
	if err != nil {
		t.Fatalf("read body: %v", err)
	}

	return string(b)
}
