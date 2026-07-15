package e2esupport_test

import (
	"testing"

	chunkyourskills "github.com/qdrddr/chunk-your-skills/sdk/go/chunkyourskills"
)

func TestCountTokensFromReleaseModule(t *testing.T) {
	count, err := chunkyourskills.CountTokens("hello world")
	if err != nil {
		t.Fatalf("CountTokens: %v", err)
	}
	if count < 1 {
		t.Fatalf("expected count >= 1, got %d", count)
	}
}

func TestVersionFromReleaseModule(t *testing.T) {
	version, err := chunkyourskills.Version()
	if err != nil {
		t.Fatalf("Version: %v", err)
	}
	if version == "" {
		t.Fatal("expected non-empty version")
	}
}
