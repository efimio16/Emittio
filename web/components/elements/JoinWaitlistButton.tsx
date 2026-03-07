export default function JoinWaitlistButton({ small }: { small?: boolean }) {
    return (
        <a className="
            block rounded-xl text-center cursor-pointer border-2 px-7 py-2 outline-0 hover:outline-3 transition-[outline-width] active:opacity-80
            text-gray-50 border-gray-900 bg-gray-900 outline-gray-800/30 
            dark:text-gray-800 dark:border-primary dark:bg-primary dark:outline-primary/50 
            " href="#join" style={{ width: small ? 'fit-content' : '100%' }}>
            {small ? 'Join' : 'Join the Waitlist'}
        </a>
    )
}