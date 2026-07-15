package chunkyourskills_test

import (
	"testing"

	"github.com/qdrddr/chunk-your-skills/sdk/go/chunkyourskills"
)

func TestMdToTreeSmoke(t *testing.T) {
	t.Parallel()
	out, err := chunkyourskills.MdToTree("# Title\n\nBody", "skill.md", "{}")
	if err != nil {
		t.Fatalf("MdToTree: %v", err)
	}
	if out == "" {
		t.Fatal("expected non-empty tree JSON")
	}
}

func TestParseSkillChunkIDsSmoke(t *testing.T) {
	t.Parallel()
	out, err := chunkyourskills.ParseSkillChunkIDs("1,2,3")
	if err != nil {
		t.Fatalf("ParseSkillChunkIDs: %v", err)
	}
	if out == "" {
		t.Fatal("expected non-empty JSON")
	}
}

func TestCountTokensSmoke(t *testing.T) {
	t.Parallel()
	n, err := chunkyourskills.CountTokens("hello world")
	if err != nil {
		t.Fatalf("CountTokens: %v", err)
	}
	if n <= 0 {
		t.Fatalf("expected positive token count, got %d", n)
	}
}

func TestVersionSmoke(t *testing.T) {
	t.Parallel()
	v, err := chunkyourskills.Version()
	if err != nil {
		t.Fatalf("Version: %v", err)
	}
	if v == "" {
		t.Fatal("expected non-empty version")
	}
}
