package chunkyourskills_test

// Blank imports pin minimum versions of transitive dev-tool dependencies so
// go mod tidy retains security fixes in go.mod/go.sum (Snyk/govulncheck).
import (
	_ "github.com/aws/aws-sdk-go-v2/aws"
	_ "github.com/aws/aws-sdk-go-v2/aws/protocol/eventstream"
	_ "github.com/golang-jwt/jwt/v5"
	_ "github.com/modelcontextprotocol/go-sdk/mcp"
	_ "github.com/yuin/goldmark"
)
