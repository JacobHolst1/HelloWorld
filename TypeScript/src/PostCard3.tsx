

type Props = {
    note: any;
}

const PostCard3 = (props: Props) => {
    return (
        <div
            className="bg-white rounded-lg shadow-md dark:bg-gray"
            style={{cursor: "auto"}}
        >
            <div className="flex items-center justify-between">
                <span className="text-sm font-light">
                    Jan 15, 2024
                </span>
            </div>
        </div>
    )
}