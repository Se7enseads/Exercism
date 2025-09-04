package lasagna

// TODO: define the 'OvenTime' constant
const OvenTime = 40

// RemainingOvenTime returns the remaining minutes based on the `actual` minutes already in the oven.
func RemainingOvenTime(actualMinutesInOven int) (timeRemaining int){
	timeRemaining = OvenTime - actualMinutesInOven
    return
}

// PreparationTime calculates the time needed to prepare the lasagna based on the amount of layers.
func PreparationTime(numberOfLayers int) (prepTime int) {
	prepTime = numberOfLayers * 2
    return 
}

// ElapsedTime calculates the time elapsed cooking the lasagna. This time includes the preparation time and the time the lasagna is baking in the oven.
func ElapsedTime(numberOfLayers, actualMinutesInOven int) (elapsedTime int) {
	elapsedTime = PreparationTime(numberOfLayers) + actualMinutesInOven
    return 
}
