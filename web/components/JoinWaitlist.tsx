import { Button, Field, Input } from "@headlessui/react";
import Image from "next/image";
import Link from "next/link";

export default function JoinWaitlist() {
    return (
        <div id="join" className="bg-gray-200 dark:bg-gray-800 w-md pt-3 border-2 border-primary overflow-hidden flex flex-col justify-around rounded-2xl p-5">
            <div>
                <h2 className="text-3xl pt-3 mb-2">Be first.<br/>Be private.</h2>
                <p className="mb-3 text-gray-500 dark:text-gray-400">Early users shape the future. Join us.</p>
            </div>
            <form action="#" method="post">
                <Field className="flex gap-3 w-full">
                    <Input placeholder="Email" required type="email" className="rounded-lg border-none flex-2/3 bg-gray-500/20 dark:bg-white/5 px-3 py-1.5 text-sm/6 focus:not-data-focus:outline-none data-focus:outline-2 data-focus:-outline-offset-2 data-focus:outline-white/25"/>
                    <Button type="submit" className="bg-primary flex-1/3 outline-2 outline-transparent px-3 py-2 rounded-lg transition-all ease-in cursor-pointer hover:outline-primary/50 active:opacity-70">Join</Button>
                </Field>
            </form>
            <div className="relative flex items-center justify-center m-4">
                <hr className="text-gray-600 w-full"/>
                <span className="absolute bg-gray-200 dark:bg-gray-800 px-2">or</span>
            </div>
            <div className="flex justify-around text-gray-600 dark:text-gray-400 text-sm">
                <Link target="_blanks" className="flex items-center flex-col" href="https://t.me/EmittioMail">
                    <Image src="/telegram_logo.svg" width={32} height={32} alt="telegram logo"/>
                    <span>Telegram</span>
                </Link>
            </div>
        </div>
    )
}