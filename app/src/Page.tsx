import React from "react";
import Header from "./Header";

export default function Page({children}: {children: React.ReactNode}) {
    return (
        <main className="flex justify-around">
            <div className="grow justify-end flex align-end">
                <Header />
            </div>
            <div className="grow">
                {children}
            </div>
        </main>
    )
}