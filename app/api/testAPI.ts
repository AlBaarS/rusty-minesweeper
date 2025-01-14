const testAPI = async (): Promise<string> => {
    console.log("Calling testAPI");
    const url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${url}/api/test`, {
        method: "GET",
    });
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }
    const data = await response.json();
    return data.message;
};

export default testAPI;