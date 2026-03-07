import Link from "next/link";
import Image from "next/image";

export default function ViewOnGitHub({ small }: { small?: boolean }) {
    return (
        <Link className="
            rounded-xl flex justify-center gap-3 items-center flex-row cursor-pointer border-2 border-transparent px-7 py-2 outline-0 hover:outline-3 transition-all active:opacity-80
            text-gray-900 border-b-gray-900 outline-gray-800/30 
            dark:text-gray-100 dark:border-primary dark:border-r-primary dark:outline-primary/30 
            " href="https://github.com/efimio16/Emittio" style={{ width: small ? 'fit-content' : '100%' }}
        >
            <Image src="/github_logo.svg" className="dark:invert" width={24} height={24} alt="github logo"/>
            {!small && 'View On GitHub'}
        </Link>
    )
}