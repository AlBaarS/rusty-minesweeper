const findGameAPI = async (user: any): Promise<any> => {
    console.log("Calling findGameAPI with user", user)
    const url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${url}/api/find`, {
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
        const gamePresence = await response.json();
        console.log("findGameAPI call successful");
        return gamePresence.game as boolean;
    } else {
        return {
            statusCode: response.status,
            statusText: response.statusText
        };
    }
};

export default findGameAPI;