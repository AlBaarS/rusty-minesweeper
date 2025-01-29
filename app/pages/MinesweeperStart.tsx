'use client'

import classNames from "classnames";
import { useState } from "react";
import { FormControl, InputLabel, MenuItem, Select } from "@mui/material";

import { isGameState } from "../types/game";
import { useMinesweeper } from "../context/MinesweeperContext";
import getSeed from "../functions/getSeed";
import getGameAPI from "../api/getGameAPI";
import findGameAPI from "../api/findGameAPI";
import continueGameAPI from "../api/continueGameAPI";
import getButtonMarkup from "../functions/getButtonMarkup";

export const MinesweeperStart = () => {
    const { setGameState } = useMinesweeper();
    const { email, setEmail } = useMinesweeper();
    const [ continue_game, setContinue_game ] = useState<boolean>(undefined!);
    const [ difficulty, setDifficulty ] = useState<number | undefined>(undefined);

    const onSubmitNew = async (seed?: number) => {

        if (seed == undefined) {
            seed = getSeed();
        }

        console.log("Starting game using API getGameAPI");
        const result = await getGameAPI(seed, email, difficulty);

        if (isGameState(result)) {
            setGameState(result);
            console.log("Obtained gamestate:", result);
        } else {
            console.log("Failed to obtain gameState:", result.statusText);
        }
    }

    const onSubmitContinue = async () => {

        console.log("Continuing game using continueGameAPI");
        const result = await continueGameAPI(email);

        if (isGameState(result)) {
            setGameState(result);
            console.log("Obtained gamestate:", result);
        } else {
            console.log("Failed to obtain gameState:", result.statusText);
        }
    }

    const checkDB = async () => {
        console.log("Checking the database for user", email, "with findGameAPI");
        const result = await findGameAPI(email);
        setContinue_game(result);
    }



    // Functions
    // Buttons
    function PlayButton() {
        return(
            <button
                type="button"
                className={getButtonMarkup("green")}
                data-te-ripple-init
                data-te-ripple-color="light"
                disabled={email == undefined || !(email.includes("@"))}
                onClick={() => onSubmitNew()}
            >
                Play new
            </button>
        )
    }

    function CheckButton() {
        let button_text: string = "";
        if (continue_game === undefined) {
            button_text = "Check DB";
        } else if (continue_game === false) {
            button_text = "Game not found"
        } else {
            button_text = "Game found"
        };

        return(
            <button
                type="button"
                className={getButtonMarkup("yellow")}
                data-te-ripple-init
                data-te-ripple-color="light"
                disabled={(email == undefined || !(email.includes("@"))) || continue_game !== undefined}
                onClick={() => checkDB()}
            >
                {button_text}
            </button>
        )
    }

    function ContinueButton() {
        return(
            <button
                type="button"
                className={getButtonMarkup("blue")}
                data-te-ripple-init
                data-te-ripple-color="light"
                disabled={!continue_game}
                onClick={() => onSubmitContinue()}
            >
                Continue
            </button>
        )
    }

    

    // Page
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
                        <h4 className="mb-4 text-xl font-semibold">Please enter your e-mail address so you can save your progress</h4>
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
                            <div className={classNames(
                                "grid",
                                "grid-cols-2",
                            )}>
                                <div>
                                    e-mail: <input 
                                        value={email ?? ""}
                                        onChange={e => {setEmail(e.target.value); setContinue_game(undefined)}}
                                    />
                                </div>
                                <div className="content-center justify-center">
                                    <FormControl variant="standard" sx={{ m: 1, minWidth: 120 }}>
                                        <InputLabel>Difficulty</InputLabel>
                                        <Select
                                            value={difficulty ?? 5}
                                            onChange={e => setDifficulty(e.target.value as number)}
                                            label="Difficulty"
                                        >
                                            <MenuItem value="">
                                                <em>None</em>
                                            </MenuItem>
                                            <MenuItem value={8}>Easy</MenuItem>
                                            <MenuItem value={6}>Medium</MenuItem>
                                            <MenuItem value={5}>Hard</MenuItem>
                                            <MenuItem value={4}>Very hard</MenuItem>
                                        </Select>
                                    </FormControl>
                                </div>
                            </div>
                        </div>
                        <br />
                        <div className={classNames(
                            "grid",
                            "grid-cols-3"
                        )}>
                            <div className="pr-2"><PlayButton /></div>
                            <div className="px-2"><CheckButton /></div>
                            <div className="pl-2"><ContinueButton /></div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default MinesweeperStart;
