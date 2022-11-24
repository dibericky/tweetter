import { useLoaderData } from "react-router-dom"
import {User, Tweet, Response} from "./client"
import Content from "./content"

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
    const userData: Response = useLoaderData() as any as Response
    return (
        <Profile
            {...userData}
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