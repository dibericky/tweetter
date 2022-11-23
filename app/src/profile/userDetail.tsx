import { useMemo } from "react";
import { User } from "./client";
import Picture from "./picture";

type Props = User

export default function UserDetail(props: Props) {
    const subscribedAt = useMemo(() => {
        let date = new Date(props.createdAt)
        let locale = date.toLocaleString('default', { month: 'long', year: "numeric" })
        return locale
    }, [props.createdAt])

    return (
        <>
                <div className="h-28 flex justify-between pt-2 px-4">
                    <div className="relative bottom-20 h-32">
                        <Picture />
                    </div>

                    <div className="flex items-start">
                        <EditProfileButton />
                    </div>
                </div>
                <div className="px-4">
                    <div className="font-bold text-slate-900 text-lg">
                        {"Ricky"}
                    </div>
                    <div className="font-normal text-gray-600 text-sm">
                        {`@${props.nickname}`}
                    </div>
                    <div>{"my description"}</div>
                    <div className="font-medium text-gray-500">
                        {`Iscrizione: ${subscribedAt}`}
                    </div>
                    <div className="flex gap-5">
                        <FollowDetail label={'following'}>
                            <Number value={props.following.toString()} />
                        </FollowDetail>
                        <FollowDetail label={'follower'}>
                            <Number value={props.follower.toString()} />
                        </FollowDetail>
                    </div>
                </div>
        </>
    )
}



function Number({value}: {value: String}) {
    return (<span className="font-bold text-slate-900 text-sm">{value}</span>)
}

function FollowDetail({label, children}: {label: String, children: React.ReactNode}) {
    return (
        <div className="font-light text-gray-600 text-xs flex gap-1 items-center">
            {children}
            {label}
        </div>
    )
}

function EditProfileButton() {
    return (
        <button className="text-slate-900 font-bold border-2 border-gray-300 hover:bg-slate-100 cursor-pointer text-center py-2 px-4 rounded-full">
            {"Modifica profilo"}
        </button>
    )
}