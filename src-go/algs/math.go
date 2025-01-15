package algs

func HummingDistance(X int, Y int) int {
	var diff = X ^ Y
	var count = 0

	for diff > 0 {
		count += diff & 1
		diff = diff >> 1
	}

	return count
}
