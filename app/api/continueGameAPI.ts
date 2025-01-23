import { GameState } from "../types/game";

const continueGameAPI = async (user: any): Promise<object> => {
    console.log("Calling continueGameAPI with user", user)
    const url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${url}/api/continue`, {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            user: user,
        }),
    });
    
    if (response.ok) {
        const gameState = await response.json();
        console.log("continueGameAPI call successful:", gameState);
        return gameState.playboard as GameState;
    } else {
        return {
            statusCode: response.status,
            statusText: response.statusText
        };
    }
};

export default continueGameAPI;