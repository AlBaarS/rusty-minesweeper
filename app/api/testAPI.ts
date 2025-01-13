// async function testAPI(): Promise<string> {
//     console.log("API Testing called")
//     const response = await fetch("api/minesweeper", {
//         method: "GET",
//         headers: {
//             Accept: "application/json",
//             "Content-Type": "application/json",
//         },
//     });

//     if (response.ok) {
//         const testResult = await response.json();
//         console.log("Returning MinesweeperClient");
//         return (testResult.message as string);
//     } else {
//         console.log("API call for testing failed");
//         return {
//             statusCode: response.status,
//             statusText: response.statusText
//         };
//     }
// }

// export default testAPI;

const testAPI = async (): Promise<string> => {
    const url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${url}/api/minesweeper`);
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }
    const data = await response.json();
    return data.message;
};

export default testAPI;