'use client';

import { useEffect, useRef, useState } from 'react';
import clsx from 'clsx';
import Description from '../elements/Description';
import Heading from '../elements/Heading';
import Explanation from '../elements/Explanation';

const steps = [
    {
        title: "Create alias",
        description: [
            "You generate email aliases locally (offline) from your private seed.",
            "No one can see that an alias was created or link it back to you.",
        ]
    },
    {
        title: "Encrypt everything",
        description: [
            "Messages — including sender and recipient information — are encrypted before they leave your device.",
            "Only recipients can decrypt them.",
        ]
    },
    {
        title: "Private routing",
        description: [
            "Messages travel through multiple nodes in the decentralized network to reduce IP and timing correlation.",
        ]
    },
    {
        title: "Decentralized delivery",
        description: [
            "Encrypted messages are temporarily stored across the network, without centralized inboxes or metadata databases.",
        ]
    },
    {
        title: "Receive privately",
        description: [
            "The recipient retrieves and decrypts messages locally, without revealing which messages belong to them.",
        ]
    },
];

export default function HowItWorksSection() {
    const [active, setActive] = useState(0);
    const [highlight, setHighlight] = useState({
        top: 0,
        height: 0,
    });
    const itemRefs = useRef<(HTMLButtonElement | null)[]>([]);

    useEffect(() => {
        const id = setTimeout(() => {
            setActive((i) => (i + 1) % steps.length);
        }, 5000);

        return () => clearTimeout(id);
    }, [active]);

    useEffect(() => {
        const el = itemRefs.current[active];
        if (!el) return;

        setHighlight({
            top: el.offsetTop,
            height: el.offsetHeight,
        });
    }, [active]);

    return (
        <section className="mx-auto py-24 px-6 flex flex-col items-center">
            <Heading>How It Works</Heading>
            <div className="w-full flex flex-row justify-center flex-wrap gap-16">
                <div className="flex min-w-sm flex-col gap-2 relative">
                    {steps.map((step, i) => (
                        <button
                            key={step.title}
                            ref={(el) => void(itemRefs.current[i] = el)}
                            onClick={() => setActive(i)}
                            className={clsx(
                                "w-full flex flex-row px-4 py-3 rounded-lg transition duration-400 cursor-pointer z-1 border",
                                "text-gray-800 dark:text-gray-200",
                                active === i
                                ? "bg-transparent text-white dark:text-gray-800 border-transparent"
                                : "bg-white hover:scale-102 border-gray-200 dark:border-gray-800 dark:bg-gray-900"
                            )}
                        >
                            <div className={clsx("w-1 my-2 mr-4 ml-0 rounded-full transition-[background-size] opacity-90 ease-linear bg-no-repeat",
                                active === i
                                ? "bg-[linear-gradient(white,white)] dark:bg-[linear-gradient(black,black)] bg-size-[100%_100%] duration-4800"
                                : "bg-transparent bg-size-[100%_0%] duration-0"
                            )}/>
                            <div className="text-left">
                                <span className="block text-sm uppercase tracking-wide opacity-60">Step {i + 1}</span>
                                <span className="block text-lg font-medium">{step.title}</span>
                            </div>
                        </button>
                    ))}
                    <div
                        className="absolute h-14 inset-0 rounded-lg -z-1 bg-gray-900 dark:bg-primary transition-all duration-400 ease-out"
                        style={{
                            top: `${highlight.top}px`,
                            height: `${highlight.height}px`,
                        }}
                    />
                </div>

                <div className="flex-1 flex flex-col min-w-30 max-w-lg min-h-50">
                    <div className="flex-1/2">
                        <h3 className="text-2xl font-semibold mb-4">{steps[active].title}</h3>
                        <Description>
                            {steps[active].description.map((line, i) => <span key={i}>{line}<br/></span>)}
                        </Description>
                    </div>
                    <div className="flex-1/2 flex justify-center items-center">
                        <div className="h-25">
                            <Explanation step={active}/>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    );
}