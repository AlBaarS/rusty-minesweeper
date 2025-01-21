'use client'

import { useState } from "react";
import { isGameState } from "../types/game";
import { email, setEmail, useMinesweeper } from "../context/MinesweeperContext";
import classNames from "classnames";
import getGameAPI from "../api/getGameAPI";
import getSeed from "../functions/getSeed";

export const MinesweeperStart = () => {
    const { setGameState } = useMinesweeper();
    const { email, setEmail } = useMinesweeper();

    const onSubmit = async (seed?: number) => {

        if (seed == undefined) {
            seed = getSeed();
        }

        console.log("Starting game using API getGameAPI");
        const result = await getGameAPI(seed, email);

        if (isGameState(result)) {
            setGameState(result);
            console.log("Obtained gamestate:", result);
        } else {
            console.log("Failed to obtain gameState:", result.statusText);
        }
    }

    function PlayButton() {
        return(
            <button
                type="button"
                className={classNames(
                    "border-2",
                    "border-neutral-500",
                    "px-7",
                    "pb-[8px]",
                    "pt-[10px]",
                    "text-sm",
                    "font-medium",
                    "uppercase",
                    "leading-normal",
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
                    "active:bg-neutral-500",
                    "disabled:border-neutral-100",
                    "disabled:text-neutral-100",
                    "disabled:bg-neutral-200",
                    "hover:bg-neutral-100",
                    "hover:bg-opacity-10",
                )}
                data-te-ripple-init
                data-te-ripple-color="light"
                disabled={email == undefined || !(email.includes("@"))}
                onClick={() => onSubmit()}
            >
                Play Minesweeper
            </button>
        )
    }

    return(
        <div className={classNames(
            "w-full",
            "text-black",
            "flex",
            "items-center",
            "justify-center",
            "content-stretch"
        )}>
            <div className={classNames(
                "relative",
                "content-center",
                "h-max", 
                "w-fit", 
                "px-4", 
                "pb-4", 
                "pt-4", 
                "bg-fixed", 
                "bg-cover", 
                "bg-center", 
                "bg-no-repeat", 
                "bg-slate-300",
                "text-centerbg-white",
                "bottom-0", 
                "left-8", 
                "right-8", 
                "top-8", 
                "border-4",
                "overflow-hidden"
            )}>
                <div className="flex h-full items-center justify-center">
                    <div>
                        <h2 className="mb-4 text-4xl font-semibold">Welcome to Minesweeper</h2>
                        <div className={classNames(
                            "px-4", 
                            "pb-4", 
                            "pt-4", 
                            "w-full",  
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
