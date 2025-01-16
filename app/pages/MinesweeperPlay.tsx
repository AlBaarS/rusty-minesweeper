import { useMinesweeper } from "../context/MinesweeperContext";

export const MinesweeperPlay = () => {

    const { gameState, setGameState } = useMinesweeper();
    const mines = gameState?.game.mines.matrix;
    const vicinity = gameState?.game.vicinity.matrix;
    const flagged = gameState?.game.flagged.matrix;
    const revealed = gameState?.game.revealed.matrix;

    function FlagsLeft() {
        return(<div></div>)
    }

    function Timer() {
        return(<div></div>)
    }

    function Row({x}) {
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

    function Square({x, y}) {
        return(
            <div>
                {vicinity[y][x]}
            </div>
        )
    }


    return(
        <div>
            <div>
                <FlagsLeft /> <Timer />
                <div className="grid grid-flow-col">
                    <Row x={0}/>
                    <Row x={1}/>
                    <Row x={2}/>
                    <Row x={3}/>
                    <Row x={4}/>
                    <Row x={5}/>
                    <Row x={6}/>
                    <Row x={7}/>
                    <Row x={8}/>
                    <Row x={9}/>
                    <Row x={10}/>
                    <Row x={11}/>
                    <Row x={12}/>
                    <Row x={13}/>
                    <Row x={14}/>
                    <Row x={15}/>
                </div>
            </div>
        </div>
    )
}

export default MinesweeperPlay;