package purchase

// NeedsLicense determines whether a license is needed to drive a type of vehicle. Only "car" and "truck" require a license.
func NeedsLicense(kind string) bool { return kind == "car" || kind == "truck" }

// ChooseVehicle recommends a vehicle for selection. It always recommends the vehicle that comes first in lexicographical order.
func ChooseVehicle(o1, o2 string) string { if o1 < o2 {return o1 + " is clearly the better choice."} else {return o2 + " is clearly the better choice." } }

// CalculateResellPrice calculates how much a vehicle can resell for at a certain age.
func CalculateResellPrice(price, age float64) float64 { switch { case int(age) < 3: return price * 0.8; case int(age) >= 10: return price * 0.5; default: return price * 0.7 } }
