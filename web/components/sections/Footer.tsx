import Link from "next/link";
import Image from "next/image";

export default function() {
    return (
        <footer className="p-5 flex justify-center gap-10">
             <Link target="_blank" className="flex items-center flex-col" href="https://t.me/EmittioMail">
                <Image src="/telegram_logo.svg" width={32} height={32} alt="telegram logo"/>
            </Link>
            <Link target="_blank" className="flex items-center flex-col" href="https://github.com/efimio16/Emittio">
                <Image src="/github_logo.svg" className="dark:invert" width={32} height={32} alt="github logo"/>
            </Link>
        </footer>
    )
}