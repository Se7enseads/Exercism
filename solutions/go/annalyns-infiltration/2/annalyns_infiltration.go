package annalyn

func CanFastAttack(knight bool) bool {return !knight}

func CanSpy(knight, archer, prisoner bool) bool {return knight || archer || prisoner}

func CanSignalPrisoner(archer, prisoner bool) bool {return !archer && prisoner}

func CanFreePrisoner(knight, archer, prisoner, dog bool) bool {
    return (prisoner && !archer && !knight) || (dog && !archer)
}
