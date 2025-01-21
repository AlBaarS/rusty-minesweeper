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
    const mines = gameState?.game.mines.matrix;
    const vicinity = gameState?.game.vicinity.matrix;
    const flagged = gameState?.game.flagged.matrix;
    const revealed = gameState?.game.revealed.matrix;

    function getImage(type: number, flag?: boolean, revealed?: boolean) {
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
        let [clicked, setClicked] = useState<true | false>(rev);

        return clicked ? (
            <div className="object-cover">
                <Image src={cellImg} alt="Cell Texture" width={32} height={32} />
            </div>
        ) : (
            <button 
                type = "button"
                disabled={flag}
                onClick = {() => setClicked(true)}
            >
                <Image src={flagOrNot(flag)} alt="Cell Texture" width={32} height={32} />
            </button>
        )
    }


    return(
        <div className="grid">
            <div><br /></div>
            <div className="place-self-center">
                <FlagsLeft /> <Timer />
                <div><br /></div>
                <div className="grid grid-flow-col justify-start">
                    {[...Array(16).keys()].map(x => <Column key={x} x={x} />)}
                </div>
            </div>
        </div>
    )
}

export default MinesweeperPlay;