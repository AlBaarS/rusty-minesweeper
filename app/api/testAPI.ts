const testAPI = async (): Promise<string> => {
    const url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${url}/api/test`);
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }
    const data = await response.json();
    return data.message;
};

export default testAPI;