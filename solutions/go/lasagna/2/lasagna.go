package lasagna

const OvenTime = 40

func RemainingOvenTime(inOven int) int {
    return OvenTime - inOven
}

func PreparationTime(layers int) int { 
	return layers * 2
}

func ElapsedTime(layers, inOven int) int {
    return PreparationTime(layers) + inOven
}
