import { GameState } from "../types/game";

const doPlayAPI = async (user: string, x: number, y: number): Promise<object> => {
    console.log("Calling doPlayAPI with user", user, " and coordinates x =", x, "y =", y);
    const url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${url}/api/play`, {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            user: user,
            x: x,
            y: y,
        }),
    });

    if (response.ok) {
        const gameState = await response.json();
        console.log("doPlayAPI call successful:", gameState);
        return gameState.playboard as GameState;
    } else {
        return {
            statusCode: response.status,
            statusText: response.statusText
        };
    }
}




export default doPlayAPI;