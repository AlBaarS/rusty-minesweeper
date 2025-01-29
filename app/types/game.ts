export type GameState = {
    game: Board,
    progress: Progress,
}

export enum Progress {
    Win = "Win",
    Lost = "Lost",
    InProgress = "InProgress",
}

export type Board = {
    mines: TwoDVector,
    vicinity: TwoDVector,
    flagged: TwoDVector,
    revealed: TwoDVector,
}

export type TwoDVector = {
    matrix: [[any]],
    size: number,
}

export function isGameState(gameState: unknown): gameState is GameState {
    return gameState !== undefined && (gameState as GameState).game !== undefined;
}
