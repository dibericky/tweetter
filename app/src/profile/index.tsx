import React from "react"
import Content from "./content"
import Picture from "./picture"

type Props = {
    nickname: String
}
export default function Profile(props: Props) {
    return (
        <div className="w-[990px] flex">
            <Content {...props}/>
            <Suggestion />
        </div>
    )
}

function Suggestion() {
    return (
        <ul>
            <li>{'Suggestion 1'}</li>
            <li>{'Suggestion 2'}</li>
            <li>{'Suggestion 3'}</li>
            <li>{'Suggestion 4'}</li>
        </ul>
    )
}