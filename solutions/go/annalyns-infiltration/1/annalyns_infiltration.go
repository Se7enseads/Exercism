package annalyn

func CanFastAttack(knight bool) bool {if knight {return false } else {return true}}

func CanSpy(knight, archer, prisoner bool) bool {
    if knight || archer || prisoner {return true} else {return false}
}

func CanSignalPrisoner(archer, prisoner bool) bool {
	if !archer && prisoner {return true} else {return false}
}

func CanFreePrisoner(knight, archer, prisoner, dog bool) bool {
    if (prisoner && !(archer || knight)) || (dog && !archer) {return true} else {return false}
}
