'use client'
import { useMinesweeper } from './context/MinesweeperContext';
import MinesweeperPlay from './pages/MinesweeperPlay';
import MinesweeperStart from './pages/MinesweeperStart';

export default function HomePage() {
    console.log("Initializing Minesweeper");
    const { gameState } = useMinesweeper()
    return gameState ? <MinesweeperPlay /> : <MinesweeperStart />;
}
