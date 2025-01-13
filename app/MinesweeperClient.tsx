'use client'

import { useEffect, useState } from 'react';
import testAPI from './api/testAPI';

export const TestAndDeployMinesweeper = () => {
    // Verification of the API
    console.log("Rendering MinesweeperClient");
    console.log("Testing API connection");
    const [testData, setData] = useState<any | null>(null);

    useEffect(() => {
        const fetchData = async () => {
            try {
                const result = await testAPI();
                console.log("Data fetched:", result);
                setData(result);
            } catch (error) {
                console.error("Error fetching data:", error);
            }
        };

        fetchData();
    }, []);

    console.log("Returning MinesweeperClient")



    // Embedded page functions
    function MineSweeperStartingPage({data}) {
        return (
            <div>
                <p>Received API connection: {data}</p>
            </div>
        )
    }



    // Returning the main page
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

export default TestAndDeployMinesweeper;