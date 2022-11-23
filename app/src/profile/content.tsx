import Picture from "./picture";

type Props = {
    nickname: String,
    following: Number,
    follower: Number
}

export default function Content(props: Props) {
    return (
        <div className="w-[600px]">
            <div className="h-[200px] bg-red-200">

            </div>
            <div>
                <div className="h-28 flex justify-between px-4 pt-2">
                    <div className="relative bottom-16 h-32">
                        <Picture />
                    </div>

                    <div className="flex items-start">
                        <EditProfileButton />
                    </div>
                </div>
                <div className="font-bold text-slate-900 text-lg">
                    {"Ricky"}
                </div>
                <div className="font-normal text-gray-600 text-sm">
                    {`@${props.nickname}`}
                </div>
                <div>{"my description"}</div>
                <div className="font-medium text-gray-500">
                    {"Iscrizione: gennario 2010"}
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
        </div>
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