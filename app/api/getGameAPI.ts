const getGameAPI = async (seed: number): Promise<string> => {
    console.log("Calling getGameAPI with seed", seed)
    const url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${url}/api/create/`, {
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
        throw new Error(`HTTP error! status: ${response.status}`);
    }
    const data = await response.json();
    return data.playboard;
};

export default getGameAPI;