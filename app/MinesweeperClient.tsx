'use client'

import { useEffect, useState } from 'react';
import testAPI from './api/testAPI';

function MinesweeperClient() {
    console.log("Rendering MinesweeperClient");
    const [data, setData] = useState<any | null>(null);

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

    return (
        <div>
            <h1>Data from Rust API</h1>
            {data ? (
                <div>
                    <p>Received API connection: {data}</p>
                </div>
            ) : (
                <p>Loading...</p>
            )}
        </div>
    );
}

export default MinesweeperClient;