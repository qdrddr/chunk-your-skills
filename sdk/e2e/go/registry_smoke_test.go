package e2esupport_test

import (
	"testing"

	chunkyourskills "github.com/qdrddr/chunk-your-skills/sdk/go"
)

func TestVersionFromReleaseModule(t *testing.T) {
	version, err := chunkyourskills.Version()
	if err != nil {
		t.Fatalf("Version: %v", err)
	}
	if version == "" {
		t.Fatal("expected non-empty version")
	}
}
