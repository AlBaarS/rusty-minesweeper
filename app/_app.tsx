import React from "react";
import { MinesweeperProvider } from "./context/MinesweeperContext";
import "./globals.css";

export default function App({ Component, pageProps }: any) {
    return (
        <MinesweeperProvider>
            <Component {...pageProps} />
        </MinesweeperProvider>
    );
}