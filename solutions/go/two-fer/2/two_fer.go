package twofer

import "fmt"

func ShareWith(name string) string {
    if name == "" {
        return "One for you, one for me."
    } else {
        return fmt.Sprintf("One for %s, one for me.", name)
    }
}