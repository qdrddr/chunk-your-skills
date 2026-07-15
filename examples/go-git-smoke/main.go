// Smoke test for github.com/qdrddr/chunk-your-skills/sdk/go consumed from git tag.
//
// Intended to run outside the monorepo (copy this folder anywhere).
package main

import (
	"fmt"
	"log"
	"os"

	chunkyourskills "github.com/qdrddr/chunk-your-skills/sdk/go/chunkyourskills"
)

func main() {
	libVersion, err := chunkyourskills.Version()
	if err != nil {
		log.Fatalf("Version(): %v", err)
	}

	count, err := chunkyourskills.CountTokens("hello world")
	if err != nil {
		log.Fatalf("CountTokens(): %v", err)
	}

	fmt.Println("chunk-your-skills Go git smoke OK")
	fmt.Printf("  sdk module version: %s\n", chunkyourskills.ModuleVersion)
	fmt.Printf("  native lib version: %s\n", libVersion)
	fmt.Printf("  count_tokens(hello world): %d\n", count)

	if wd, err := os.Getwd(); err == nil {
		fmt.Printf("  cwd: %s\n", wd)
	}
}
