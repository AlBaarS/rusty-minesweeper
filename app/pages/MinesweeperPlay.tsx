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
                <Square x={x} y={0} />
                <Square x={x} y={1} />
                <Square x={x} y={2} />
                <Square x={x} y={3} />
                <Square x={x} y={4} />
                <Square x={x} y={5} />
                <Square x={x} y={6} />
                <Square x={x} y={7} />
                <Square x={x} y={8} />
                <Square x={x} y={9} />
                <Square x={x} y={10} />
                <Square x={x} y={11} />
                <Square x={x} y={12} />
                <Square x={x} y={13} />
                <Square x={x} y={14} />
                <Square x={x} y={15} />
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
                    <Column x={0}/>
                    <Column x={1}/>
                    <Column x={2}/>
                    <Column x={3}/>
                    <Column x={4}/>
                    <Column x={5}/>
                    <Column x={6}/>
                    <Column x={7}/>
                    <Column x={8}/>
                    <Column x={9}/>
                    <Column x={10}/>
                    <Column x={11}/>
                    <Column x={12}/>
                    <Column x={13}/>
                    <Column x={14}/>
                    <Column x={15}/>
                </div>
            </div>
        </div>
    )
}

export default MinesweeperPlay;