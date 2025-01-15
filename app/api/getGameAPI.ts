import { GameState } from "../types/game";

const getGameAPI = async (seed: number): Promise<object> => {
    console.log("Calling getGameAPI with seed", seed)
    const url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${url}/api/create`, {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            seed: seed,
        }),
    });
    if (!response.ok) {
        throw new Error(`HTTP error in getGameAPI. status: ${response.status}`);
    }
    const data = await response.json();
    return data.playboard as GameState;
};

export default getGameAPI;