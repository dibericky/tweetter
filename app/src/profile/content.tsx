import { useMemo } from "react";
import { Tweet, User } from "./client";
import Tweets from "./tweets";
import UserDetail from "./userDetail";

type Props = {
    user: User,
    userTweets: Tweet[]
}

export default function Content(props: Props) {
    
    return (
        <div className="w-[600px] border-slate-100 border border-solid">
            <div className="h-[200px] bg-red-200 p-2">

            </div>
            <div>
                <UserDetail {...props.user} />
                <Tweets tweets={props.userTweets} />
            </div>  
        </div>
    )
}
