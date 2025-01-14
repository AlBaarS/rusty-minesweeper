import { useEffect, useState } from 'react';

function getDataFromAPI(fun: Function, arg?: any) {
    const [data, setData] = useState<any | null>(null);
    useEffect(() => {
        const fetchData = async () => {
            try {
                const result = arg !== undefined ? await fun(arg) : await fun();
                console.log("Data fetched using", fun, "with argument", arg, ":", result);
                setData(result);
            } catch (error) {
                console.error("Error fetching data:", error);
            }
        };

        fetchData();
    }, [fun, arg]);
    return data;
}

export default getDataFromAPI;