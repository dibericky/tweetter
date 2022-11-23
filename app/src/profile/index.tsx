import React from "react"
import {useGetUser, QueryState, User, Tweet} from "./client"
import Content from "./content"
import Picture from "./picture"

type Props = {
    user: User,
    userTweets: Tweet[]
}

function Profile(props: Props) {
    return (
        <div className="w-[990px] flex">
            <Content {...props}/>
            <Suggestion />
        </div>
    )
}

export default function() {
    const userData = useGetUser()
    if (userData.status === QueryState.Loading) {
        return <h2>Loading...</h2>
    }
    return (
        <Profile
            {...userData.payload}
        />
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