import { useMinesweeper } from "../context/MinesweeperContext";
import Image from 'next/image';

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
import flag_texture from '../../public/cell_textures/cellflag.png'
import blank_texture from '../../public/cell_textures/cellup.png'
import { useState } from "react";
import doPlayAPI from "../api/doPlayAPI";
import { isGameState } from "../types/game";
import classNames from "classnames";

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

    const { gameState, setGameState } = useMinesweeper();
    const { email } = useMinesweeper();
    const mines = gameState?.game.mines.matrix;
    const vicinity = gameState?.game.vicinity.matrix;
    const flagged = gameState?.game.flagged.matrix;
    const revealed = gameState?.game.revealed.matrix;
    const progress = gameState?.progress;

    const playSquare = async (x: number, y: number) => {

        console.log("Clicking square at x =", x, "and y =", y);
        const result = await doPlayAPI(email, x, y);

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

    function FlagsLeft() {
        return(<div></div>)
    }

    function Timer() {
        return(<div></div>)
    }

    function Column({x}) {
        return(
            <div className="grid grid-flow-row">
                {[...Array(16).keys()].map(y => <Square key={`(${x} * 16) + ${y}`} x={x} y={y} />)}
            </div>
        )
    }

    function flagOrNot(bool: boolean) {
        return bool ? flag_texture : blank_texture
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
                disabled={flag}
                onClick = {
                    function(){ playSquare(x, y) }
                }
            >
                <Image src={flagOrNot(flag)} alt="Cell Texture" width={32} height={32} />
            </button>
        )
    }


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
                        <div className="grid">
                            <div><br /></div>
                            <div className="place-self-center">
                                <FlagsLeft /> <Timer />
                                <div><br /></div>
                                <div className={classNames(
                                    "grid", 
                                    "grid-flow-col", 
                                    "justify-start",
                                    "border-4", 
                                    "border-double", 
                                    "bg-slate-200", 
                                    "border-neutral-400"
                                )}>
                                    {[...Array(16).keys()].map(x => <Column key={x} x={x} />)}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}

export default MinesweeperPlay;