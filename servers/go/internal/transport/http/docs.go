package httptransport

import (
	"encoding/json"
	"net/http"
)

var openAPIDoc = map[string]any{
	"openapi": "3.0.3",
	"info": map[string]any{
		"title":       "Demo Library",
		"version":     "1.0.0",
		"description": "Demo of a library system",
		"contact": map[string]any{
			"name":  "",
			"email": "christopher@craftcode.solutions",
		},
	},
	"paths": map[string]any{
		"/health": map[string]any{
			"get": operation("Performs a health check", nil),
		},
		"/books": map[string]any{
			"get":  operation("Get book catalog", nil),
			"post": operation("Add book", schemaRef("CreateCatalogTitleInput")),
		},
		"/books/{isbn}/copies": map[string]any{
			"post": operation("Add book copy", schemaRef("AddInventoryCopyInput"), pathParam("isbn")),
		},
		"/book-copies/{barcode}": map[string]any{
			"get": operation("Get book copy details", nil, pathParam("barcode")),
		},
		"/book-copies/{barcode}/lost": map[string]any{
			"put":    operation("Mark book copy lost", nil, pathParam("barcode")),
			"delete": operation("Mark book copy found", nil, pathParam("barcode")),
		},
		"/book-copies/{barcode}/maintenance": map[string]any{
			"put":    operation("Send book copy to maintenance", nil, pathParam("barcode")),
			"delete": operation("Complete book copy maintenance", nil, pathParam("barcode")),
		},
		"/book-copies/{barcode}/return": map[string]any{
			"post": operation("Return book copy", nil, pathParam("barcode")),
		},
		"/book-copies/{barcode}/report-loss": map[string]any{
			"post": operation("Report lost loaned book copy", nil, pathParam("barcode")),
		},
		"/members": map[string]any{
			"post": operation("Register member", schemaRef("RegisterMemberInput")),
		},
		"/members/{ident}": map[string]any{
			"get": operation("Get member details", nil, pathParam("ident")),
		},
		"/members/{ident}/suspension": map[string]any{
			"put":    operation("Suspend member", nil, pathParam("ident")),
			"delete": operation("Reactivate member", nil, pathParam("ident")),
		},
		"/members/{ident}/loans": map[string]any{
			"get": operation("Get member loans", nil, pathParam("ident")),
		},
		"/loans": map[string]any{
			"post": operation("Check out book copy", schemaRef("StartLoanInput")),
		},
		"/loans/overdue": map[string]any{
			"get": operation("Get overdue loans", nil),
		},
	},
	"components": map[string]any{
		"schemas": map[string]any{
			"CreateCatalogTitleInput": objectSchema(map[string]any{
				"isbn":        stringSchema(),
				"title":       stringSchema(),
				"author_name": stringSchema(),
			}, []string{"isbn", "title", "author_name"}),
			"AddInventoryCopyInput": objectSchema(map[string]any{
				"barcode": stringSchema(),
			}, []string{"barcode"}),
			"RegisterMemberInput": objectSchema(map[string]any{
				"full_name":        stringSchema(),
				"max_active_loans": map[string]any{"type": "integer", "format": "int32"},
			}, []string{"full_name", "max_active_loans"}),
			"StartLoanInput": objectSchema(map[string]any{
				"member_ident":      stringSchema(),
				"book_copy_barcode": stringSchema(),
			}, []string{"member_ident", "book_copy_barcode"}),
			"ErrorResponseBody": objectSchema(map[string]any{
				"error": stringSchema(),
			}, []string{"error"}),
		},
	},
}

func getOpenAPIDoc(w http.ResponseWriter, _ *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	_ = json.NewEncoder(w).Encode(openAPIDoc)
}

func getSwaggerUI(w http.ResponseWriter, _ *http.Request) {
	w.Header().Set("Content-Type", "text/html; charset=utf-8")
	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(swaggerHTML))
}

func operation(summary string, requestBody any, parameters ...map[string]any) map[string]any {
	op := map[string]any{
		"summary": summary,
		"responses": map[string]any{
			"200": map[string]any{"description": "OK"},
			"400": errorResponse("Bad Request"),
			"401": errorResponse("Unauthorized"),
			"404": errorResponse("Not Found"),
			"409": errorResponse("Conflict"),
			"500": errorResponse("Internal Server Error"),
		},
	}
	if len(parameters) > 0 {
		op["parameters"] = parameters
	}
	if requestBody != nil {
		op["requestBody"] = map[string]any{
			"required": true,
			"content": map[string]any{
				"application/json": map[string]any{
					"schema": requestBody,
				},
			},
		}
	}
	return op
}

func pathParam(name string) map[string]any {
	return map[string]any{
		"name":     name,
		"in":       "path",
		"required": true,
		"schema":   stringSchema(),
	}
}

func errorResponse(description string) map[string]any {
	return map[string]any{
		"description": description,
		"content": map[string]any{
			"application/json": map[string]any{
				"schema": schemaRef("ErrorResponseBody"),
			},
		},
	}
}

func objectSchema(properties map[string]any, required []string) map[string]any {
	return map[string]any{
		"type":       "object",
		"properties": properties,
		"required":   required,
	}
}

func schemaRef(name string) map[string]any {
	return map[string]any{"$ref": "#/components/schemas/" + name}
}

func stringSchema() map[string]any {
	return map[string]any{"type": "string"}
}

const swaggerHTML = `<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Demo Library API</title>
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css">
</head>
<body>
  <div id="swagger-ui"></div>
  <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
  <script>
    window.onload = function() {
      SwaggerUIBundle({ url: "/api-docs/openapi.json", dom_id: "#swagger-ui" });
    };
  </script>
</body>
</html>`
