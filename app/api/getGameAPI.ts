import { GameState } from "../types/game";

const getGameAPI = async (seed: number, user: any, difficulty: number): Promise<object> => {
    console.log("Calling getGameAPI with seed", seed, "and user", user)
    const url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${url}/api/create`, {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            seed: seed,
            user: user,
            difficulty: difficulty,
        }),
    });
    
    if (response.ok) {
        const gameState = await response.json();
        console.log("getGameAPI call successful:", gameState);
        return gameState.playboard as GameState;
    } else {
        return {
            statusCode: response.status,
            statusText: response.statusText
        };
    }
};

export default getGameAPI;