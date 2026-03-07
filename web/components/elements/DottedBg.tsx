export default function DottedBg() {
    return (
        <svg width="100%" className="text-gray-500/50 mask-[linear-gradient(white_85%,transparent)]" height="100%" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <pattern
                    id="dots"
                    x="0"
                    y="0"
                    width="18"
                    height="18"
                    patternUnits="userSpaceOnUse"
                >
                    <circle cx="9" cy="9" r="1" fill="currentColor"/>
                </pattern>
            </defs>
            <rect width="100%" height="100%" fill="url(#dots)" />
        </svg>
    )
}