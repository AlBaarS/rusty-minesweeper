export type Gamestate = {
    mines: [[boolean]],
    vicinity: [[number]],
    flagged: [[boolean]],
    revealed: [[boolean]],
}

export enum Progress {
    Win,
    Lose,
    InProgress,
}

export type Play = {
    Gamestate: Gamestate,
    progress: Progress,
}

