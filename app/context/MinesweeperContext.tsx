'use client'

import { createContext, useContext, useState } from "react";
import { GameState } from "../types/game";

type ContextType = {
    gameState: GameState | undefined,
    setGameState: (gameState: GameState) => void;
    email: string | undefined,
    setEmail: (email: string | undefined) => void;
    flagging: boolean,
    setFlagging: (flagging: boolean) => void;
    difficulty: number | undefined,
    setDifficulty: (difficulty: number | undefined) => void;
}

const MinesweeperContext = createContext<ContextType>({
    gameState: undefined,
    setGameState() { },
    email: undefined,
    setEmail() { },
    flagging: false,
    setFlagging() { },
    difficulty: undefined,
    setDifficulty() { }
});

type Props = React.PropsWithChildren;

export const MinesweeperProvider = (props: Props) => {
    const { children } = props;

    const [gameState, setGameState] = useState<GameState | undefined>(undefined);
    const [email, setEmail] = useState<string | undefined>(undefined);
    const [flagging, setFlagging] = useState<boolean>(false);
    const [difficulty, setDifficulty] = useState<number | undefined>(undefined);

    return <MinesweeperContext.Provider value={{
        gameState: gameState,
        setGameState: setGameState,
        email: email,
        setEmail: setEmail,
        flagging: flagging,
        setFlagging: setFlagging,
        difficulty: difficulty,
        setDifficulty: setDifficulty
    }}>{children}</MinesweeperContext.Provider>
}

export const useMinesweeper = () => {
    const context = useContext(MinesweeperContext);

    if (context === undefined) {
        throw new Error('useMinesweeper must be used within a MinesweeperProvider');
    }

    return context;
}
