'use client';
import LogoLarge from "../images/LogoLarge"

export default function Header() {
    return (
        <section className="flex items-center justify-center h-screen">
            <div className="w-fit flex items-center flex-col gap-5 text-center">
                <h1 className="max-w-3/4">
                    <LogoLarge />
                </h1>
                <p className="text-gray-700 dark:text-gray-400 text-2xl mb-5">True privacy starts with your inbox.</p>
                <div className="text-gray-700 dark:text-gray-200 w-3/4 flex justify-between gap-10">
                    <button onClick={() => document.getElementById('join')?.scrollIntoView({ behavior: 'smooth' })} className="outline-3 outline-transparent bg-primary transition-colors ease-in active:opacity-70 hover:outline-primary/60 p-2 m-1 rounded-xl text-lg cursor-pointer flex-1/3">Join Waitlist</button>
                    <button onClick={() => document.getElementById('donate')?.scrollIntoView({ behavior: 'smooth' })} className="outline-3 outline-transparent ease-in active:opacity-70 hover:outline-primary/60 border-primary transition-colors hover:text-gray-500 dark:hover:text-gray-300 border-2 p-2 m-1 rounded-xl text-lg cursor-pointer flex-1/3">💸 Donate</button>
                </div>
            </div>
        </section>
    )
}