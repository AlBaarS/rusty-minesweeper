import { useMinesweeper } from "../context/MinesweeperContext";
import Image from 'next/image';
import classNames from "classnames";
import { Switch } from "@mui/material";

import cell0_texture from '../../public/cell_textures/celldown.png';
import cell1_texture from '../../public/cell_textures/cell1.png';
import cell2_texture from '../../public/cell_textures/cell2.png';
import cell3_texture from '../../public/cell_textures/cell3.png';
import cell4_texture from '../../public/cell_textures/cell4.png';
import cell5_texture from '../../public/cell_textures/cell5.png';
import cell6_texture from '../../public/cell_textures/cell6.png';
import cell7_texture from '../../public/cell_textures/cell7.png';
import cell8_texture from '../../public/cell_textures/cell8.png';
import mine_expl_texture from '../../public/cell_textures/blast.png';
import flag_texture from '../../public/cell_textures/cellflag.png';
import blank_texture from '../../public/cell_textures/cellup.png';
import mine_unex_texture from '../../public/cell_textures/cellmine.png';
import doPlayAPI from "../api/doPlayAPI";
import { isGameState, Progress } from "../types/game";
import doFlagAPI from "../api/doFlagAPI";
import getSeed from "../functions/getSeed";
import getGameAPI from "../api/getGameAPI";
import getButtonMarkup from "../functions/getButtonMarkup";

const images = {
    0: cell0_texture,
    1: cell1_texture,
    2: cell2_texture,
    3: cell3_texture,
    4: cell4_texture,
    5: cell5_texture,
    6: cell6_texture,
    7: cell7_texture,
    8: cell8_texture,
    9: mine_expl_texture
};

export const MinesweeperPlay = () => {

    const { gameState, setGameState, email, flagging, setFlagging } = useMinesweeper();
    const mines = gameState?.game.mines.matrix;
    const vicinity = gameState?.game.vicinity.matrix;
    const flagged = gameState?.game.flagged.matrix;
    const revealed = gameState?.game.revealed.matrix;
    const progress = gameState?.progress as Progress;
    const boardsize = gameState?.game.mines.size;
    const difficulty = gameState?.difficulty;


    
    // Functions
    const doClick = async (x: number, y: number) => {

        console.log("Clicking square at x =", x, "and y =", y, "with flagging =", flagging);
        const result = flagging ? 
            await doFlagAPI(email, x, y) :
            await doPlayAPI(email, x, y);

        if (isGameState(result)) {
            setGameState(result);
            console.log("Obtained gamestate:", result);
        } else {
            console.log("Failed to obtain gameState:", result.statusText);
        }
    }

    const replay = async () =>  {
        let seed = getSeed();
        console.log("Re-starting game using API getGameAPI");
        const result = await getGameAPI(seed, email, difficulty);

        if (isGameState(result)) {
            setGameState(result);
            console.log("Obtained gamestate:", result);
        } else {
            console.log("Failed to obtain gameState:", result.statusText);
        }
    }

    function getImage(type: number) {
        return images[type] || null;
    }

    function flagging_switcher(check: bool) {
        console.log("Setting flag switch to", check);
        setFlagging(check);
    }

    function flagOrNot(bool: boolean) {
        return bool ? flag_texture : blank_texture
    }



    // Game progress
    function DisplayEndgameState() {
        if (progress == Progress.InProgress) {
            return(<div></div>);
        } else {
            return(<div className={classNames(
                "border-4",
                "border-double",
                "border-gray-400",
                "p-1",
                "h-full"
            )}>
                Game over. You {progress}!
            </div>);
        }
        
    }

    function GameProgress() {
        return(<div className={classNames(
            "text-black",
            "grid",
            "grid-cols-2"
        )}>
            <div className="p-2">
                <div className={classNames(
                    "border-4",
                    "border-double",
                    "border-gray-400",
                    "p-1",
                    "h-full"
                )}>
                    Currently playing with e-mail: <br /> {email}
                </div>
            </div>
            <div className="p-2">
                <DisplayEndgameState />
            </div>
        </div>)
    }

    // Game utilities
    function FlagButton() {
        return(
            <div className={classNames(
                "grid",
                "grid-cols-3",
                "place-content-center",
                "justify-center",
                "border-4",
                "border-double",
                "border-gray-400",
                "p-1"
            )}>
                <div className="grid place-content-center">
                    <Image src={mine_unex_texture} alt="Cell Texture" width={32} height={32} />
                </div>
                <div className="grid place-content-center">
                    <Switch 
                        checked={flagging}
                        size="medium" 
                        color="default"
                        onChange={(event) => flagging_switcher(event.target.checked)}
                        disabled={progress != Progress.InProgress}
                    />
                </div>
                <div className="grid place-content-center">
                    <Image src={flag_texture} alt="Cell Texture" width={32} height={32} />
                </div>
            </div>
        )
    }

    // * Unused:
    function Timer() {  // Not used within the scope of the individual project
        return(
            <div></div>
        )
    }

    function FlagsLeft() {  // Not used within the scope of the individual project
        return(
            <div></div>
        )
    }

    // Playing board
    function Column({x}) {
        return(
            <div className="grid grid-flow-row">
                {[...Array(boardsize).keys()].map(y => <Square key={`(${x} * ${boardsize}) + ${y}`} x={x} y={y} />)}
            </div>
        )
    }

    function Square({x, y}) {
        const vic = vicinity[y][x];
        const flag = flagged[y][x];
        const rev = revealed[y][x];
        const cellImg = getImage(vic);

        return rev ? (
            <div className="object-cover">
                <Image src={cellImg} alt="Cell Texture" width={32} height={32} />
            </div>
        ) : (
            <button 
                type = "button"
                disabled={(flag && !flagging) || progress != Progress.InProgress}
                onClick = {
                    function(){ doClick(x, y) }
                }
            >
                <Image src={flagOrNot(flag)} alt="Cell Texture" width={32} height={32} />
            </button>
        )
    }

    // Return/Replay
    function BackToHomeButton() {
        return(
            <button
                type="button"
                className={getButtonMarkup("blue")}
                data-te-ripple-init
                data-te-ripple-color="light"
                onClick={() => window.location.reload(true)}
            >
                Back to menu
            </button>
        )
    }

    function ReplayGameButton() {
        return(
            <button
                type="button"
                className={getButtonMarkup("red")}
                data-te-ripple-init
                data-te-ripple-color="light"
                onClick={() => replay()}
            >
                Replay
            </button>
        )
    }


    // Return page
    return(
        <div className={classNames(
            "flex",
            "place-content-center"
        )}>
            <div className={classNames(
                "box-border",
                "w-2/5"
            )}>
                <div className={classNames(
                    "px-4", 
                    "pb-4", 
                    "pt-4",  
                    "border-4", 
                    "border-double", 
                    "bg-slate-200", 
                    "border-neutral-400"
                    )}>
                    <div>
                        <div className="grid grid-flow-row">
                            <GameProgress />
                            <div><br /></div>
                            <div className="place-self-center">
                                <div className={classNames(
                                    "text-black",
                                    "grid",
                                    "grid-cols-3"
                                )}>
                                    <div className="p-2"><FlagsLeft /> </div>
                                    <div className="p-2"><FlagButton /> </div>
                                    <div className="p-2"><Timer /></div>
                                </div>
                                <div className="grid place-content-center">
                                    <div className={classNames(
                                        "flex",
                                        "w-fit",
                                        "border-4", 
                                        "border-double", 
                                        "bg-slate-200", 
                                        "border-neutral-400"
                                    )}>
                                        <div className={classNames(
                                            "grid", 
                                            "grid-flow-col", 
                                            "justify-start"
                                        )}>
                                            {[...Array(boardsize).keys()].map(x => <Column key={x} x={x} />)}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <br />
                    <div className={classNames(
                        "grid",
                        "grid-cols-2"
                    )}>
                        <div className="px-4"><BackToHomeButton /></div>
                        <div className="px-4"><ReplayGameButton /></div>
                    </div>
                </div>
            </div>
        </div>
    )
}

export default MinesweeperPlay;