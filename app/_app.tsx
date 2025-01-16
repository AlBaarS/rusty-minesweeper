import React from "react";
import { MinesweeperProvider } from "./context/MinesweeperContext"; // Adjust path as needed
import "../styles/globals.css"; // Or your global styles

export default function App({ Component, pageProps }: any) {
    return (
        <MinesweeperProvider>
            <Component {...pageProps} />
        </MinesweeperProvider>
    );
}