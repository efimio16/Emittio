'use client';
import { useState } from "react";
import Explanation from "../images/Explanation";
import { ArrowPathIcon, ChevronLeftIcon, ChevronRightIcon } from "@heroicons/react/24/outline";

const captions = [
    "The client encrypts the message and recipient's username.",
    "It sends the encrypted data to a random first node — which doesn't know your identity.",
    "The first node relays it to a second node, which decrypts only the username (still anonymous).",
    "The second node forwards the message to a quorum — a group of nodes that store encrypted mail.",
    "Even if one quorum member fails or goes offline,",
    "the recipient can still retrieve the mail from other nodes in the quorum."
];

export default function How() {
    const [step, setStep] = useState(0);
    
    return (
        <section className="flex flex-col py-4 mb-10 items-center">
            <h2 className="text-5xl text-center w-full mb-7">How does Emittio work?</h2>
            <div className="backdrop-blur-2xl bg-gray-500/10 dark:bg-gray-50/10 max-w-300 w-full rounded-2xl h-2/3 min-h-80 flex flex-col p-4 items-center">
                <p className="mb-2 h-30 w-3/4 min-w-100 flex items-center">
                    <button className="m-4 w-10 h-10 inline-flex justify-center items-center cursor-pointer" onClick={() => setStep(Math.max(step - 1, 0))}>
                        <ChevronLeftIcon className="flex-1"/>
                    </button>
                    <span className="flex-1 text-center">{captions[step]}</span>
                    <button className="m-4 w-10 h-10 inline-flex justify-center items-center cursor-pointer" onClick={() => setStep(Math.min(step + 1, 5))}>
                        <ChevronRightIcon className="flex-1"/>
                    </button>
                </p>
                <Explanation step={step}/>
                <button onClick={() => setStep(0)} className="m-2 flex items-center transition-all duration-200" style={{ opacity: step == 5 ? 100 : 0, cursor: step == 5 ? 'pointer' : '' }}>
                    <ArrowPathIcon className="h-6 mr-2"/>
                    Restart
                </button>
            </div>
        </section>
    )
}