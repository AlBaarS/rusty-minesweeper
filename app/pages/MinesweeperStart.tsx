'use client'

import { useState } from "react";
import { isGameState } from "../types/game";
import { useMinesweeper } from "../context/MinesweeperContext";
import classNames from "classnames";
import getGameAPI from "../api/getGameAPI";
import getSeed from "../functions/getSeed";

export const MinesweeperStart = () => {
    const { setGameState } = useMinesweeper();
    const [alert, setAlert] = useState<string | null>(null);
    const [email, setEmail] = useState("");

    const onSubmit = async (seed?: number) => {

        if (seed == undefined) {
            seed = getSeed();
        }

        console.log("Starting game using API getGameAPI");
        const result = await getGameAPI(seed);

        if (isGameState(result)) {
            setGameState(result);
            console.log("Obtained gamestate:", result);
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
                disabled={email == "" || !(email.includes("@"))}
                onClick={() => onSubmit()}
            >
                Play Minesweeper
            </button>
        )
    }

    return(
        <div className={classNames(
            "relative",
            "h-full", 
            "w-screen", 
            "bg-cover", 
            "bg-center", 
            "bg-no-repeat", 
            "p-12", 
            "text-centerbg-white"
        )}>
            <div className={classNames(
                "absolute2", 
                "bottom-0", 
                "left-8", 
                "right-8", 
                "top-8", 
                "border-4",
                "overflow-hidden", 
                "bg-fixed", 
                "text-white"
            )}>
                <div className="flex h-full items-center justify-center">
                    <div>
                        <h2 className="mb-4 text-4xl font-semibold">Welcome to Minesweeper</h2>
                        <div className={classNames(
                            "px-4", 
                            "pb-4", 
                            "pt-4", 
                            "w-full", 
                            "text-black", 
                            "border-4", 
                            "border-double", 
                            "bg-slate-200", 
                            "border-neutral-400"
                        )}>
                            e-mail: <input onChange={e => setEmail(e.target.value)}/>
                        </div>
                        <br />
                        <br />
                        <PlayButton />
                    </div>
                </div>
            </div>
        </div>
    );
};

export default MinesweeperStart;
