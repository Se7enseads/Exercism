package collatzconjecture
import "errors"

func CollatzConjecture(n int) (int, error) {
	var steps int
    if n <= 0 { return 0, errors.New("input must be positive")}
    for n != 1 {
        if n%2 == 0 { n /= 2 } else { n = (3 * n) + 1 }
        steps ++
	}
    return steps, nil
}
