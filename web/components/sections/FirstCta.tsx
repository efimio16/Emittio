"use client";

import { FormEvent, useState } from "react";
import Description from "../elements/Description";
import MeteorsBg from "../elements/MeteorsBg";
import Heading from "../elements/Heading";

export default function FirstCta() {
    const [email, setEmail] = useState('');
    const [success, setSuccess] = useState(false);
    const [error, setError] = useState('');
    const [loading, setLoading] = useState(false);

    async function handleSumbit(e: FormEvent<HTMLFormElement>) {
        e.preventDefault();
        if (loading) return;
        setError('');
        setLoading(true);

        let res = await fetch('/api/subscribe', {
            method: 'POST',
            body: JSON.stringify({ email }),
            headers: {
                'Content-Type': 'application/json'
            }
        });

        const { success, error } = await res.json();

        // MOCK
        // await new Promise(res => setTimeout(res, 800));
        // const { success, error } = { success: true, error: null };

        setLoading(false);

        if (success) {
            setSuccess(true);
            setTimeout(() => {
                setSuccess(false);
            }, 2000);
        } else if (error) {
            setError(error);
        }
    }

    return (
        <section className="relative w-full overflow-hidden" id="join">
            <MeteorsBg/>
            <div
                className="
                    mx-auto max-w-3xl rounded-3xl _border p-10 text-center z-10
                    _border-gray-200 _bg-white _dark:border-gray-800 _dark:bg-gray-900
                ">
                <Heading>Join the Waitlist</Heading>

                <Description>Get early access when it's ready.</Description>

                <form

                    onSubmit={handleSumbit}
                    className="mx-auto mt-8 flex max-w-md flex-col gap-3 sm:flex-row"
                >
                    <input
                        type="email"
                        required
                        placeholder="you@domain.com"
                        value={email}
                        onChange={(e) => setEmail(e.target.value)}
                        className="
                            flex-1/2 rounded-xl border border-gray-300 bg-transparent px-4 py-3
                            text-sm outline-none transition
                            focus:border-gray-900 focus:ring-0
                            dark:border-gray-700 dark:focus:border-gray-200
                        "
                    />

                    <button
                        type="submit"
                        disabled={loading}
                        className="
                            bg-primary text-gray-800 disabled:opacity-50 flex-1/2 outline-2 outline-transparent px-3 py-2 rounded-lg transition-all ease-in cursor-pointer disabled:cursor-not-allowed
                            not-disabled:hover:outline-primary/50 active:opacity-70
                        ">
                            { success
                                ? 'You\'re in ✓'
                                : loading ? 'Loading...' : 'Join'
                            }
                    </button>
                                    
                </form>
            </div>
        </section>
    );
}