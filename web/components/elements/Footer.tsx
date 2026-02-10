import Image from "next/image";
import Link from "next/link";

const links = [
    { title: "GitHub", url: "https://github.com/efimio16/Emittio" },
    { title: "Privacy", url: "/privacy" },
    { title: "Contact", url: "/contact" },
]

const socials = [
    {
        label: "Telegram",
        icon: <Image src="/telegram_logo.svg" width={32} height={32} alt="telegram logo"/>,
        url: "https://t.me/EmittioMail"
    }
]

export default function Footer() {
    return (
        <footer>
            <div className="mx-auto max-w-7xl px-6 py-10">
                <div className="flex flex-col gap-8 md:flex-row md:items-center md:justify-between">
                    <div className="text-sm text-gray-500 dark:text-gray-400">© 2026 Emittio</div>

                    <nav className="flex gap-6 text-sm">
                        {links.map((link, i) => 
                            <Link
                                key={i}
                                href={link.url}
                                className="text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100 transition-colors"
                            >
                                {link.title}
                            </Link>
                        )}
                    </nav>

                    <div className="flex items-center gap-4">
                        <span className="text-sm text-gray-500 dark:text-gray-400">Follow us:</span>
                        {socials.map((social, i) =>
                            <a key={i} aria-label={social.label} href={social.url} target="_blank" rel="noopener noreferrer">{social.icon}</a>
                        )}
                    </div>
                </div>
            </div>
        </footer>
    );
}