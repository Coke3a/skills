// Template: replace ExampleEntityHandler, CreateExampleEntity*, and route
// names with project-specific names. Target location:
// internal/handler/router/{surface}/{entity}.go
// Replace "app" in imports with the project's module path.
// Handlers stay thin: bind, map request to input, call usecase, map output to
// response. The central error handler owns status codes — just return errors.
package publicapi

import (
	"time"

	"github.com/gofiber/fiber/v3"
	"github.com/google/uuid"

	"app/internal/handler/auth"
	"app/internal/usecase/examplefeature"
)

type CreateExampleEntityRequest struct {
	Name string `json:"name"`
	URL  string `json:"url"`
}

type CreateExampleEntityResponse struct {
	ID        uuid.UUID `json:"id"`
	OwnerID   uuid.UUID `json:"owner_id"`
	Name      string    `json:"name"`
	URL       string    `json:"url"`
	Status    string    `json:"status"`
	CreatedAt time.Time `json:"created_at"`
}

// ExampleEntityHandler groups the example-entity routes. Dependencies are
// injected once at startup in cmd/api/main.go — no globals, no init().
type ExampleEntityHandler struct {
	createUsecase *examplefeature.CreateExampleEntityUsecase
}

func NewExampleEntityHandler(
	createUsecase *examplefeature.CreateExampleEntityUsecase,
) *ExampleEntityHandler {
	return &ExampleEntityHandler{createUsecase: createUsecase}
}

func (h *ExampleEntityHandler) Register(r fiber.Router) {
	r.Post("/example-entities", h.Create)
}

func (h *ExampleEntityHandler) Create(c fiber.Ctx) error {
	user, err := auth.UserFromContext(c)
	if err != nil {
		return err
	}

	var req CreateExampleEntityRequest
	if err := c.Bind().Body(&req); err != nil {
		return fiber.NewError(fiber.StatusBadRequest, "invalid request body")
	}

	// Binding into the DTO copied the values, so nothing here aliases
	// fasthttp's reused buffers. Values from c.Params()/c.Query() would need
	// utils.CopyString before outliving this handler.
	//
	// Fiber v3 fiber.Ctx satisfies context.Context, so cancellation reaches
	// pgx. On Fiber v2, pass c.UserContext() instead of c.
	output, err := h.createUsecase.Execute(c, examplefeature.CreateExampleEntityInput{
		OwnerID: user.ID,
		Name:    req.Name,
		URL:     req.URL,
	})
	if err != nil {
		return err
	}

	return c.Status(fiber.StatusCreated).JSON(CreateExampleEntityResponse{
		ID:        output.ID,
		OwnerID:   output.OwnerID,
		Name:      output.Name,
		URL:       output.URL,
		Status:    output.Status,
		CreatedAt: output.CreatedAt,
	})
}
