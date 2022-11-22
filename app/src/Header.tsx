function items() : Array<{label: String}> {
    return [
        {label: "Home"},
        {label: "Esplora"},
        {label: "Notifiche"},
        {label: "Messaggi"},
        {label: "Segnalibri"},
        {label: "Liste"},
        {label: "Profilo"},
        {label: "Altro"},
    ]
}

export default function Header() {
    return (
        <header>
            <nav className="flex flex-col gap-2">
                {
                    items().map(({label}, i) => {
                        return (
                            <a
                                className="text-slate-900 hover:bg-slate-100 gap-4 cursor-pointer text-center p-5 rounded-full inline-flex"
                                key={`${i}-nav-item`}
                            >
                                <svg viewBox="0 0 24 24" aria-hidden="true" style={{width: "1.75rem"}}><g><path d="M12 9c-2.209 0-4 1.791-4 4s1.791 4 4 4 4-1.791 4-4-1.791-4-4-4zm0 6c-1.105 0-2-.895-2-2s.895-2 2-2 2 .895 2 2-.895 2-2 2zm0-13.304L.622 8.807l1.06 1.696L3 9.679V19.5C3 20.881 4.119 22 5.5 22h13c1.381 0 2.5-1.119 2.5-2.5V9.679l1.318.824 1.06-1.696L12 1.696zM19 19.5c0 .276-.224.5-.5.5h-13c-.276 0-.5-.224-.5-.5V8.429l7-4.375 7 4.375V19.5z"></path></g></svg>
                                {label}</a>)
                    } )
                }
            </nav>
        </header>
    )
}