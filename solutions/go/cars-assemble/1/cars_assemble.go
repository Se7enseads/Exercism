package cars

// CalculateWorkingCarsPerHour calculates how many working cars are
// produced by the assembly line every hour.
func CalculateWorkingCarsPerHour(prodR int, successR float64) float64 {
    return float64(prodR) * (successR/100)
}

// CalculateWorkingCarsPerMinute calculates how many working cars are
// produced by the assembly line every minute.
func CalculateWorkingCarsPerMinute(prodR int, successR float64) int {
    return int(CalculateWorkingCarsPerHour(prodR,successR)) / 60
}

// CalculateCost works out the cost of producing the given number of cars.
func CalculateCost(cars int) uint {
    return uint(((cars % 10) * 10000) + ((cars/10) * 95000))
}
