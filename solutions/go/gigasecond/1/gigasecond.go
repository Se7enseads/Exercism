// Package Gigasecond provides function to add a giasecond to times.
package gigasecond

// import path for the time package from the standard library.
import "time"

// AddGigasecond adds 1 billion to specified time.
func AddGigasecond(t time.Time) time.Time { return t.Add(1e9 * time.Second) }
