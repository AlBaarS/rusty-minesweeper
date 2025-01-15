export type TwoDVector = {
    matrix: [[any]],
    size: number,
}

export type Board = {
    mines: TwoDVector,
    vicinity: TwoDVector,
    flagged: TwoDVector,
    revealed: TwoDVector,
}

export enum Progress {
    Win,
    Lose,
    InProgress,
}

export type GameState = {
    game: Board,
    progress: Progress,
}

export function isGameState(gameState: unknown): gameState is GameState {
    return (gameState as GameState).game !== undefined;
}
