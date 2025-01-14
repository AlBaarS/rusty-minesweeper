'use client'

import testAPI from './api/testAPI';
import getGameAPI from './api/getGameAPI';
import DisplayBoard from './board';
import getDataFromAPI from './functions/getDataFromAPI'
import getSeed from './functions/getSeed';

export const DeployMinesweeper = () => {
    // Verification of the API
    console.log("Rendering MinesweeperClient");

    // Embedded page functions
    function MineSweeperStartingPage({data}) {
        return (
            <div>
                <p>Received API connection: {data}</p>
            </div>
        )
    }



    // Returning the main page
    const testData = getDataFromAPI(testAPI);
    const seed = getSeed();
    let board = getDataFromAPI(getGameAPI, seed);
    console.log("Starting board:", board);
    console.log("Returning MinesweeperClient");
    return (
        <div>
            {testData ? (
                <div>
                    <MineSweeperStartingPage data={testData} />
                </div>
            ) : (
                <p>Loading...</p>
            )}
        </div>
    );
}

export default DeployMinesweeper;