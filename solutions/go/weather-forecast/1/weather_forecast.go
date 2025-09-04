// Package weather shows weather.
package weather

// CurrentCondition describes current condition.
var CurrentCondition string

// CurrentLocation shows current location.
var CurrentLocation string

// Forecast shows both current condition and location.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
