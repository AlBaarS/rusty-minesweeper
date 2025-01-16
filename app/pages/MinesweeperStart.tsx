'use client'

import { useState } from "react";
import { isGameState } from "../types/game";
import { useMinesweeper } from "../context/MinesweeperContext";
import classNames from "classnames";
import getGameAPI from "../api/getGameAPI";
import getSeed from "../functions/getSeed";
import MinesweeperPlay from "./MinesweeperPlay";

export const MinesweeperStart = () => {
    const { setGameState } = useMinesweeper();
    const [alert, setAlert] = useState<string | null>(null);
    let [start, setStart] = useState<true | false>(false);

    const onSubmit = async (seed?: number) => {

        if (seed == undefined) {
            seed = getSeed();
        }

        console.log("Starting game using API getGameAPI");
        const result = await getGameAPI(seed);

        if (isGameState(result)) {
            setGameState(result);
            console.log("Obtained gamestate:", result);
            setStart(true);
        } else {
            console.log("Failed to obtain gameState:", result.statusText);
            setAlert(`${result.statusCode} ${result.statusText}`);
        }
    }

    function PlayButton() {
        return(
            <button
                type="button"
                className={classNames(
                    "rounded border-2",
                    "border-neutral-50",
                    "px-7",
                    "pb-[8px]",
                    "pt-[10px]",
                    "text-sm",
                    "font-medium",
                    "uppercase",
                    "leading-normal",
                    "text-neutral-50",
                    "transition duration-150",
                    "ease-in-out",
                    "hover:border-neutral-100",
                    "hover:text-neutral-100",
                    "focus:border-neutral-100",
                    "focus:text-neutral-100",
                    "focus:outline-none",
                    "focus:ring-0",
                    "active:border-neutral-200",
                    "active:text-neutral-200",
                    "hover:bg-neutral-100",
                    "hover:bg-opacity-10",
                )}
                data-te-ripple-init
                data-te-ripple-color="light"
                disabled={false}
                onClick={() => onSubmit()}
            >
                Play Minesweeper
            </button>
        )
    }

    return start ? <MinesweeperPlay /> : (
        <div className="`relative h-full w-screen bg-cover bg-center bg-no-repeat p-12 text-centerbg-white">
            <div className="absolute2 bottom-0 left-0 right-0 top-0 h-full w-full overflow-hidden bg-fixed bg-black/60">
                <div className="flex h-full items-center justify-center">
                    <div className="text-white">
                        <h2 className="mb-4 text-4xl font-semibold">Welcome to Minesweeper</h2>

                        <PlayButton />
                    </div>
                </div>
            </div>
        </div>
    );
};

export default MinesweeperStart;
