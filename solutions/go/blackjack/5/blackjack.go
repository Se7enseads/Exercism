package blackjack

// ParseCard returns the integer value of a card following blackjack ruleset.
func ParseCard(card string) int {
	switch card {
        case "ace": return 11
        case "two": return 2
        case "three": return 3
        case "four": return 4
        case "five": return 5
        case "six": return 6
        case "seven": return 7
        case "eight": return 8
        case "nine": return 9
        case "ten", "jack", "queen", "king": return 10
        default: return 0
    }
}

// FirstTurn returns the decision for the first turn, given two cards of the
// player and one card of the dealer.
func FirstTurn(card1, card2, dealerCard string) string {
    d := ParseCard(dealerCard)
    mySum := ParseCard(card1) + ParseCard(card2)
    var strategy string
    
    switch {
        case mySum == 22: strategy = "P"
        case mySum == 21:
        	switch {
                case d >= 10: strategy = "S"
                default: strategy = "W"
            }
        case mySum >= 17 && mySum <= 20: strategy = "S"
        case mySum >= 12 && mySum <= 16: 
        	switch {
                case d >= 7: strategy = "H"
                default: strategy = "S"
            }
        case mySum <= 11: strategy = "H" 
    }
    return strategy
 }
