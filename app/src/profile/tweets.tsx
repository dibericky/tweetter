import { useMemo } from "react"
import { Tweet } from "./client"

type Props = {
    tweets: Tweet[]
}

export default function Tweets(props: Props) {
    return(
        <div>
            {
                props.tweets
                    .map(tweet => <TweetItem key={`tweet-${tweet.id}`} {...tweet} />)
            }
        </div>
    )
}

function TweetItem(props: Tweet) {
    const tweetedAt = useMemo(() => {
        let date = new Date(props.updatedAt)
        let locale = date.toLocaleString('default', { day: 'numeric', month: 'short' })
        return locale
    }, [props.updatedAt])

    return (
        <div className="flex gap-3 px-4 hover:bg-gray-100 cursor-pointer pt-2 border-b-solid border-b border-b-slate-100">
            <div>
                <div className="bg-red-400 rounded-full w-14 h-14"/>
            </div>
            <div className="pb-2 grow">
                <div className="flex gap-2">
                    <div className="font-bold text-sm text-gray-700">
                        {"Foobar"}
                    </div>
                    <div className="text-gray-600 font-light text-sm flex gap-1">
                        <div>
                            {'@foobar'}
                        </div>
                        <span>·</span>
                        <div>
                            {tweetedAt}
                        </div>
                    </div>

                </div>
                <div>{props.message}</div>
                <div className="flex justify-between gap-4 w-[420px] pt-2">
                    <Actionbar />
                </div>
            </div>
        </div>
    )
}

function Actionbar() {
    const classValue = "hover:bg-red-200 rounded-full w-8 h-8 flex items-center justify-center cursor-pointer hover:fill-red-800";
    return (
        <>
            <div className={classValue}><Icon /></div>
            <div className={classValue}><Icon /></div>
            <div className={classValue}><Icon /></div>
            <div className={classValue}><Icon /></div>
            <div className={classValue}><Icon /></div>
        </>
    )
}

function Icon() {
    return (
        <svg viewBox="0 0 24 24" aria-hidden="true" className="w-5 h-5 fill-inherit">
            <g>
                <path d="M1.751 10c0-4.42 3.584-8 8.005-8h4.366c4.49 0 8.129 3.64 8.129 8.13 0 2.96-1.607 5.68-4.196 7.11l-8.054 4.46v-3.69h-.067c-4.49.1-8.183-3.51-8.183-8.01zm8.005-6c-3.317 0-6.005 2.69-6.005 6 0 3.37 2.77 6.08 6.138 6.01l.351-.01h1.761v2.3l5.087-2.81c1.951-1.08 3.163-3.13 3.163-5.36 0-3.39-2.744-6.13-6.129-6.13H9.756z">
                </path>
            </g>
        </svg>
    )
}