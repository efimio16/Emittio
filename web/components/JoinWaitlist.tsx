'use client';
import { Button, Field, Input } from "@headlessui/react";
import { CheckCircleIcon } from "@heroicons/react/20/solid";
import Image from "next/image";
import Link from "next/link";
import { FormEvent, useState } from "react";

export default function JoinWaitlist() {
    const [email, setEmail] = useState('');
    const [success, setSuccess] = useState(false);
    const [error, setError] = useState('');
    const [loading, setLoading] = useState(false);

    function handleSumbit(e: FormEvent<HTMLFormElement>) {
        e.preventDefault();
        if (loading) return;
        setError('');
        setLoading(true);

        fetch('/api/subscribe', {
            method: 'POST',
            body: JSON.stringify({ email }),
            headers: {
                'Content-Type': 'application/json'
            }
        })
        .then(response => response.json())
        .then(json => {
            setLoading(false);
            if (json.success) {
                setSuccess(true);
                setTimeout(() => {
                    setSuccess(false);
                }, 2000);
            } else if (json.error) {
                setError(json.error);
            }
        })
    }

    return (
        <div id="join" className="bg-gray-200 dark:bg-gray-800 w-md pt-3 border-2 border-primary overflow-hidden flex flex-col justify-around rounded-2xl p-5">
            <div>
                <h2 className="text-3xl pt-3 mb-2">Be first.<br/>Be private.</h2>
                <p className="mb-3 text-gray-500 dark:text-gray-400">Early users shape the future. Join us.</p>
            </div>
            <form action="#" method="post" onSubmit={handleSumbit}>
                {error && <p className="text-red-500">{error}</p>}
                <Field className="flex gap-3 w-full">
                    <Input placeholder="Email" required type="email" value={email} onChange={e => setEmail(e.target.value)} className="rounded-lg border-none flex-2/3 bg-gray-500/20 dark:bg-white/5 px-3 py-1.5 text-sm/6 focus:not-data-focus:outline-none data-focus:outline-2 data-focus:-outline-offset-2 data-focus:outline-white/25"/>
                    <Button type="submit" disabled={loading} className="bg-primary disabled:opacity-50 flex-1/3 outline-2 outline-transparent px-3 py-2 rounded-lg transition-all ease-in cursor-pointer disabled:cursor-not-allowed not-disabled:hover:outline-primary/50 active:opacity-70">{ success ? <CheckCircleIcon className="h-6 inline"/> : loading ? 'Loading...' : 'Join' }</Button>
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