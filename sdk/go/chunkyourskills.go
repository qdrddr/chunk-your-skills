// Package chunkyourskills provides Go bindings for chunk-your-skills via cgo.
//
// Build the shared C library first:
//
//	./sdk/c/scripts/build-c-lib.sh
package chunkyourskills

// ClearError clears the thread-local error message.
func ClearError() {
	cgoClearError()
}

// LastError returns the thread-local error message, or empty string if none.
func LastError() string {
	return cgoGetLastError()
}

// Version returns the chunk-your-skills library version string.
func Version() (string, error) {
	return cgoGetVersion()
}
